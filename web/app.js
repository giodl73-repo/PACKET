const $=id=>document.getElementById(id),examples=JSON.parse($('examples-data').textContent);$('static-help').hidden=true;
let worker,epoch=0,revision=0,serial=0,active=null,ready=false,busy=false,reading=false,startupTimer,computeTimer,fileTimer,fileToken=0,original=null,result=null,canonical=null;
function controls(){$('run').disabled=!ready||busy||reading;$('report-download').disabled=!canonical;$('native-download').disabled=!result?.analysis_json;$('original-download').disabled=!$('corpus').value&&original===null;}
function clear(){canonical=result=null;$('results').hidden=true;$('error').hidden=true;$('coverage').replaceChildren();controls();}
function invalidate(){revision++;clear();if(ready)$('status').textContent=busy?'Inputs changed; waiting for previous work.':'Inputs changed. Inspect again.';}
function cancelFile(){fileToken++;reading=false;clearTimeout(fileTimer);}
function edited(id){cancelFile();if(id==='corpus')original=null;invalidate();}
function diagnostic(detail){$('error').hidden=false;$('error-message').textContent=detail.message;$('diagnostic').textContent=JSON.stringify(detail,null,2);}
function engineError(message){epoch++;worker?.terminate();worker=null;ready=busy=false;active=null;clearTimeout(startupTimer);clearTimeout(computeTimer);clear();diagnostic({message});$('retry').hidden=false;$('status').textContent='Engine unavailable. Inputs and independent downloads are retained.';controls();}
function request(){return JSON.stringify({scale:$('scale').value,corpus_json:$('corpus').value});}
function inspect(){
 if(!ready||busy||reading)return;clear();const text=request();if(new TextEncoder().encode(text).length>1048576){diagnostic({message:'Current request exceeds 1 MiB UTF-8.'});return;}
 busy=true;active={id:++serial,revision};const id=active.id,generation=epoch;controls();$('status').textContent='Inspecting in Rust…';computeTimer=setTimeout(()=>{if(generation===epoch&&active?.id===id)engineError('Inspection exceeded ten seconds. Retry with retained inputs.');},10000);
 try{worker.postMessage({type:'inspect',id,request:text});}catch{engineError('The worker could not accept this request.');}
}
function render(){
 $('outcome').textContent=result.outcome.replaceAll('-',' ');$('records').textContent=JSON.stringify(result.records,null,2);$('authority').textContent=JSON.stringify(result.authority,null,2);
 const selection=result.selection;$('selection').textContent=selection?`Scale: ${result.scale} · Selected: ${selection.ids.join(', ')} · Markets: ${selection.markets.join(', ')} · Excluded IDs: ${selection.excluded_ids.join(', ')||'none'}`:`Scale: ${result.scale} · Held/rejected: ${result.held_or_rejected}. ${result.outcome==='insufficient-data'?'No records at this scale; no analysis.':'Resolve admission reasons before analysis.'}`;
 const complete=result.summary?.complete_dimensions??[];const coverage=result.coverage??[];const missing=coverage.some(row=>row.provided<row.total);
 $('coverage-warning').hidden=!missing;$('coverage-warning').textContent='Incomplete score coverage. Native missing scores default to zero. The complete-dimension summary excludes these defaults; the full native JSON retains them for inspection.';
 $('summary').textContent=result.summary?`${complete.length} of 13 dimensions fully supplied · ${result.summary.mean_regions.length} mean flags · ${result.summary.tail_regions.length} tail flags · ${result.summary.capacity_gaps.length} provisional capacity flags. No complete-market or SLA adequacy claim.`:'No modeled results computed.';
 for(const item of coverage){const row=document.createElement('tr');for(const value of [item.dimension,item.provided,item.total,item.provided===item.total?'Fully supplied':item.provided?'Partial':'Unassessed']){const cell=document.createElement('td');cell.textContent=value;row.append(cell);}$('coverage').append(row);}
 $('signals').textContent=JSON.stringify(result.summary?{mean_regions:result.summary.mean_regions,tail_regions:result.summary.tail_regions}:null,null,2);$('capacity').textContent=JSON.stringify(result.summary?{capacity_gaps:result.summary.capacity_gaps,assessed_ids:result.summary.capacity_assessed_ids,unassessed_ids:result.summary.capacity_unassessed_ids,limits:result.summary.other_sla_terms}:null,null,2);$('results').hidden=false;
}
function start(){
 epoch++;const generation=epoch;worker?.terminate();clearTimeout(startupTimer);clearTimeout(computeTimer);ready=busy=false;active=null;clear();$('retry').hidden=true;$('status').textContent='Loading Rust engine…';startupTimer=setTimeout(()=>{if(generation===epoch&&!ready)engineError('Rust did not start within twelve seconds. Retry to load it again.');},12000);
 try{worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});}catch{engineError('This browser could not create the Rust worker.');return;}
 worker.onerror=e=>{e.preventDefault();if(generation===epoch)engineError('The Rust worker failed.');};worker.onmessage=({data})=>{
  if(generation!==epoch)return;if(data.type==='startup-error'){engineError('Rust could not be downloaded or initialized.');return;}
  if(data.type==='ready'){clearTimeout(startupTimer);ready=true;$('status').textContent='Rust engine ready.';controls();if(revision===0)inspect();return;}
  if(!active||data.id!==active.id)return;clearTimeout(computeTimer);const submitted=active.revision;busy=false;active=null;
  if(data.type==='compute-error'){engineError('The worker could not complete inspection.');return;}if(submitted!==revision){$('status').textContent='Inputs changed. Inspect again.';controls();return;}
  try{result=JSON.parse(data.canonical);}catch{engineError('The worker returned an unreadable report.');return;}canonical=data.canonical;
  if(!result.ok){diagnostic(result.diagnostic);$('status').textContent='Fix the input and inspect again.';controls();return;}render();$('status').textContent='Inspection complete. Results remain provisional research tooling.';controls();
 };
}
function download(name,text){const url=URL.createObjectURL(new Blob([text],{type:'application/json;charset=utf-8'}));const link=document.createElement('a');link.href=url;link.download=name;link.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
function example(name){cancelFile();original=null;$('corpus').value=examples[name].corpus_json;$('scale').value=examples[name].scale;for(const id of ['corpus-file','markdown-files'])$(id).value='';invalidate();$('corpus').focus();}
async function openFiles(id){
 const files=Array.from($(id).files);if(!files.length)return;cancelFile();invalidate();const token=fileToken;const markdown=id==='markdown-files';
 if((markdown&&(files.length>32||files.some(f=>f.size>32768)||files.reduce((n,f)=>n+f.size,0)>262144))||(!markdown&&files[0].size>524288)){diagnostic({message:'Bundle limit: 512 KiB. Markdown: up to 32 files, 32 KiB each, 256 KiB combined.'});$(id).value='';return;}
 reading=true;controls();fileTimer=setTimeout(()=>{if(token===fileToken){cancelFile();diagnostic({message:'Local read exceeded five seconds. Previous draft retained.'});controls();}},5000);
 try{const texts=await Promise.all(files.map(async f=>new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(await f.arrayBuffer())));if(token===fileToken){const text=markdown?JSON.stringify({documents:files.map((f,i)=>({name:f.name,markdown:texts[i]}))},null,2):texts[0];$('corpus').value=text;$(id).value='';edited('corpus');original=text;}}
 catch{if(token===fileToken)diagnostic({message:'File could not be read as valid UTF-8. Previous draft retained.'});}finally{if(token===fileToken){reading=false;clearTimeout(fileTimer);$(id).value='';}controls();}
}
$('corpus').addEventListener('input',()=>edited('corpus'));$('scale').addEventListener('change',()=>edited('scale'));for(const id of ['corpus-file','markdown-files'])$(id).addEventListener('change',()=>openFiles(id));
for(const button of document.querySelectorAll('[data-example]'))button.addEventListener('click',()=>example(button.dataset.example));$('workbench').addEventListener('submit',e=>{e.preventDefault();inspect();});$('reset').addEventListener('click',()=>example('tail'));$('retry').addEventListener('click',start);
$('original-download').addEventListener('click',()=>download('packet-corpus.json',original??$('corpus').value));$('request-download').addEventListener('click',()=>download('packet-request.json',request()));$('report-download').addEventListener('click',()=>{if(canonical)download('packet-workbench.json',canonical);});$('native-download').addEventListener('click',()=>{if(result?.analysis_json)download('packet-native-gap.json',result.analysis_json);});start();
