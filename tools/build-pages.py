"""Build the bounded PACKET demo, native references and inspectable source custody."""
import argparse, hashlib, html, json, os, shutil, subprocess, zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]

def capture(*args):return subprocess.check_output(args,cwd=ROOT).decode('utf-8').strip()
def run(*args):subprocess.run(args,cwd=ROOT,check=True)
def pin(path):return {'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,default=Path('dist/PACKET'));a=p.parse_args()
 output=(ROOT/a.output).resolve();dist=(ROOT/'dist').resolve()
 if output==dist or not output.is_relative_to(dist):raise ValueError('Output must be a child of dist')
 if capture('wasm-bindgen','--version')!='wasm-bindgen 0.2.127':raise ValueError('Use wasm-bindgen 0.2.127')
 run('cargo','+1.95.0','build','--locked','--release','--target','wasm32-unknown-unknown','--manifest-path','web/adapter/Cargo.toml','--lib','--features','wasm')
 run('cargo','+1.95.0','build','--locked','--manifest-path','web/adapter/Cargo.toml','--bin','packet-inspect')
 metadata=json.loads(capture('cargo','+1.95.0','metadata','--manifest-path','web/adapter/Cargo.toml','--locked','--format-version','1','--filter-platform','wasm32-unknown-unknown','--features','wasm'))
 if output.exists():
  def linked(path):return path.is_symlink() or bool(getattr(path.lstat(),'st_file_attributes',0)&1024)
  if linked(output) or any(linked(p) for p in output.rglob('*')):raise ValueError('Linked output refused')
  shutil.rmtree(output)
 output.mkdir(parents=True)
 target=Path(metadata['target_directory']);run('wasm-bindgen',str(target/'wasm32-unknown-unknown/release/packet_web.wasm'),'--target','web','--out-dir',str(output/'pkg'))
 binary=target/'debug'/('packet-inspect.exe' if os.name=='nt' else 'packet-inspect')
 examples=json.loads(capture(str(binary),'--examples'));sample=examples['tail']
 page=(ROOT/'web/index.html').read_text(encoding='utf-8')
 page=page.replace('{{CORPUS}}',html.escape(sample['corpus_json']))
 page=page.replace('{{EXAMPLES}}',json.dumps(examples,ensure_ascii=False).replace('<','\\u003c'))
 (output/'index.html').write_text(page,encoding='utf-8')
 for name in ('app.js','worker.js','style.css'):shutil.copyfile(ROOT/'web'/name,output/name)
 for name in ('LICENSE','BUILD.md'):shutil.copyfile(ROOT/name,output/name)
 shutil.copytree(ROOT/'tools/licenses/rust',output/'rust')
 (output/'examples.json').write_text(json.dumps(examples,indent=2)+'\n',encoding='utf-8')
 # Include notices for the portable adapter's actual resolved dependency closure,
 # including build-time proc macros, without adding consumer dependencies.
 packages={p['id']:p for p in metadata['packages']};nodes={n['id']:n for n in metadata['resolve']['nodes']}
 first=next(p['id'] for p in metadata['packages'] if p['name']=='packet-web');pending=[first];visited=set();notices=[];dependencies=[]
 while pending:
  current=pending.pop()
  if current in visited:continue
  visited.add(current);pending.extend(nodes[current]['dependencies']);pkg=packages[current]
  if pkg['source'] is None:continue
  base=Path(pkg['manifest_path']).parent
  files=sorted({f for pattern in ('LICENSE*','COPYRIGHT*','NOTICE*') for f in base.glob(pattern) if f.is_file()})
  if not files:raise ValueError('No license notice found for '+pkg['name'])
  dependencies.append({'name':pkg['name'],'version':pkg['version'],'license':pkg['license']})
  notices.append('\n\n===== '+pkg['name']+' '+pkg['version']+' ('+str(pkg['license'])+') =====\n')
  for f in files:notices.append('\n--- '+f.name+' ---\n'+f.read_text(encoding='utf-8'))
 notice='PACKET portable adapter dependency closure, including build-time proc macros.\nThe workbench calls the in-memory resolver; native filesystem entry points are not used by the browser. Original licenses follow.\nRust standard-library notices: rust/COPYRIGHT-library.html and rust/licenses/.\n'+''.join(notices)
 (output/'THIRD-PARTY-NOTICES.txt').write_text(notice,encoding='utf-8')
 dirty=bool(capture('git','status','--porcelain','--untracked-files=all'));commit=capture('git','rev-parse','HEAD')
 tracked=capture('git','ls-files','-z').split('\0')
 if dirty:
  with zipfile.ZipFile(output/'source.zip','w',compression=zipfile.ZIP_DEFLATED) as archive:
   for name in tracked:
    path=ROOT/name
    if path.is_file():
     if not path.resolve().is_relative_to(ROOT):raise ValueError('External source path refused')
     archive.write(path,name)
 else:run('git','archive','--format=zip','--output='+str(output/'source.zip'),'HEAD')
 with zipfile.ZipFile(output/'source.zip') as archive:
  for name in ('web/adapter/src/lib.rs','crates/packet-gap/src/lib.rs','crates/packet-corpus/src/lib.rs','web/adapter/Cargo.lock','Cargo.lock','BUILD.md'):
   expected=(ROOT/name).read_bytes() if dirty else subprocess.check_output(['git','show',commit+':'+name],cwd=ROOT)
   if archive.read(name)!=expected:raise ValueError('Source ZIP differs from build source: '+name)
 for name in ('web/adapter/src/lib.rs','crates/packet-gap/src/lib.rs','crates/packet-corpus/src/lib.rs','web/adapter/Cargo.lock','Cargo.lock'):
  dest=output/'source'/name;dest.parent.mkdir(parents=True,exist_ok=True)
  dest.write_bytes((ROOT/name).read_bytes() if dirty else subprocess.check_output(['git','show',commit+':'+name],cwd=ROOT))
 provenance={'schema':'packet.pages-build.v1','commit':commit,'dirty':dirty,'source':'https://github.com/giodl73-repo/PACKET',
   'rust':capture('rustc','+1.95.0','--version'),'bindings':capture('wasm-bindgen','--version'),'dependencies':sorted(dependencies,key=lambda p:p['name']),
   'example_requests':7,'delivery_budget_bytes':5_000_000}
 (output/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n',encoding='utf-8');(output/'.nojekyll').touch()
 cache=ROOT/'.cache';cache.mkdir(exist_ok=True);binary=target/'debug'/('packet-inspect.exe' if os.name=='nt' else 'packet-inspect')
 cases=[]
 def case(name,record):
  path=cache/'request.json';path.write_text(record,encoding='utf-8',newline='')
  canonical=capture(str(binary),str(path));json.loads(canonical);cases.append({'name':name,'input':record,'canonical':canonical})
 for name,pair in examples.items():case(name,json.dumps(pair))
 def altered(name,mutate):
  bundle=json.loads(sample['corpus_json']);mutate(bundle);case(name,json.dumps({**sample,'corpus_json':json.dumps(bundle,ensure_ascii=False)},ensure_ascii=False))
 def replace(bundle,old,new):bundle['documents'][0]['markdown']=bundle['documents'][0]['markdown'].replace(old,new)
 altered('missing-id',lambda b:replace(b,'id: synthetic-0','no_id: synthetic-0'))
 altered('missing-scale',lambda b:replace(b,'scale: regional','scale: unknown'))
 altered('missing-market',lambda b:replace(b,'market: Authored market 0','market: '))
 altered('missing-tier',lambda b:replace(b,'tier: T4','tier: invalid'))
 altered('nan-score',lambda b:replace(b,'score.DIM-01: 1','score.DIM-01: NaN'))
 altered('over-score',lambda b:replace(b,'score.DIM-01: 1','score.DIM-01: 11'))
 altered('malformed-score',lambda b:replace(b,'score.DIM-01: 1','score.DIM-01: broken'))
 altered('malformed-quantity',lambda b:replace(b,'quantity: 20','quantity: broken'))
 altered('nonfinite-quantity',lambda b:replace(b,'quantity: 20','quantity: NaN'))
 altered('unknown-field',lambda b:replace(b,'tier: T4','tier: T4\nextra-note: Café 🌿 <img src=x onerror=alert(1)>'))
 altered('duplicate-field',lambda b:replace(b,'id: synthetic-0','id: synthetic-0\nid: shadow'))
 altered('duplicate-id',lambda b:replace(b,'id: synthetic-0','id: synthetic-1'))
 altered('unit-substring',lambda b:replace(b,'| Mbps |','| not-Mbps |'))
 altered('byte-unit-unassessed',lambda b:replace(b,'| Mbps |','| MBps |'))
 altered('over-markdown',lambda b:b['documents'][0].update(markdown='x'*32769))
 altered('over-lines',lambda b:b['documents'][0].update(markdown='\n'*4097))
 altered('over-documents',lambda b:b.update(documents=b['documents']*9))
 altered('duplicate-names',lambda b:b['documents'][0].update(name='synthetic-1.md'))
 altered('empty-documents',lambda b:b.update(documents=[]))
 case('empty-scale',json.dumps({**sample,'scale':'national'}))
 case('invalid-scale',json.dumps({**sample,'scale':'everywhere'}))
 case('over-corpus',json.dumps({**sample,'corpus_json':' '*524289}))
 case('invalid-envelope','{}');case('invalid-json','[broken');case('over-envelope',' '*1048577)
 (cache/'native-references.json').write_text(json.dumps(cases,indent=2)+'\n',encoding='utf-8')
 files={p.relative_to(output).as_posix():pin(p) for p in sorted(output.rglob('*')) if p.is_file()}
 size=sum(p['bytes'] for p in files.values())
 if size>=5_000_000:raise ValueError('Static demo exceeds the 5 MB delivery budget')
 (cache/'site-inventory.json').write_text(json.dumps({'commit':commit,'dirty':dirty,'bytes':size,'files':files},indent=2)+'\n')
 print(json.dumps({'output':str(output),'bytes':size,'files':len(files),'native_reference_cases':len(cases),'dirty':dirty}))

if __name__=='__main__':main()
