import {test,expect} from '@playwright/test';
import {readFile,mkdir} from 'node:fs/promises';
const fixtures=JSON.parse(await readFile('.cache/native-references.json','utf8'));
const examples=JSON.parse(await readFile('dist/PACKET/examples.json','utf8'));
async function open(page){await page.goto('./');await expect(page.locator('#results')).toBeVisible();}
async function download(page,id){const waiting=page.waitForEvent('download');await page.locator('#'+id).click();return readFile(await(await waiting).path(),'utf8');}
test('all native responses exactly match real WASM worker',async({page})=>{
 await page.goto('./');const actual=await page.evaluate(async cases=>{const w=new Worker(new URL('worker.js',location.href),{type:'module'});await new Promise((resolve,reject)=>{w.onmessage=e=>e.data.type==='ready'?resolve():reject(e.data);w.onerror=reject;});const results=[];for(let id=0;id<cases.length;id++)results.push(await new Promise((resolve,reject)=>{w.onmessage=e=>resolve(e.data.canonical);w.onerror=reject;w.postMessage({type:'inspect',id,request:cases[id].input});}));w.terminate();return results;},fixtures);expect(actual).toEqual(fixtures.map(f=>f.canonical));
});
for(const name of Object.keys(examples))test('synthetic example '+name,async({page})=>{
 await open(page);await page.locator(`[data-example="${name}"]`).click();await page.locator('#run').click();await expect(page.locator('#report-download')).toBeEnabled();const canonical=fixtures.find(f=>f.name===name).canonical;expect(await download(page,'report-download')).toBe(canonical);const result=JSON.parse(canonical);
 await expect(page.locator('#outcome')).toHaveText(result.outcome.replaceAll('-',' '));if(result.analysis_json)expect(await download(page,'native-download')).toBe(result.analysis_json);else await expect(page.locator('#native-download')).toBeDisabled();
 await expect(page.locator('#authority')).toContainText('"public_finding_release_approved": false');
});
test('missing scores are unassessed, not complete observed deficits',async({page})=>{
 await open(page);await page.locator('[data-example="missing-scores"]').click();await page.locator('#run').click();await expect(page.locator('#summary')).toContainText('1 of 13 dimensions fully supplied');await expect(page.locator('#coverage-warning')).toBeVisible();await expect(page.locator('#coverage')).toContainText('Unassessed');await expect(page.locator('#summary')).toContainText('0 mean flags');const raw=JSON.parse(await download(page,'native-download'));expect(raw.regions.length).toBe(12);
});
test('scale selection excludes other records and empty scale is insufficient',async({page})=>{
 await open(page);await page.locator('[data-example="scale-isolation"]').click();await page.locator('#run').click();await expect(page.locator('#selection')).toContainText('Excluded IDs: synthetic-local');await page.locator('#scale').selectOption('local');await expect(page.locator('#report-download')).toBeDisabled();await page.locator('#run').click();await expect(page.locator('#selection')).toContainText('Selected: synthetic-local');await page.locator('#scale').selectOption('national');await page.locator('#run').click();await expect(page.locator('#outcome')).toHaveText('insufficient data');await expect(page.locator('#native-download')).toBeDisabled();
});
test('own Markdown imports preserve document strings and native values',async({page})=>{
 await open(page);const source='---\r\nid: own-id\r\ntype: synthetic-market\r\nscale: local\r\nmarket: Authored Café 🌿\r\ntier: T4\r\nscore.DIM-01: 2\r\nquantity: 12 | Mbps | provisional | own-fixture\r\n---\r\n';
 await page.locator('#markdown-files').setInputFiles([{name:'own.md',mimeType:'text/markdown',buffer:Buffer.from(source)}]);await expect.poll(()=>page.locator('#corpus').inputValue()).toContain('own-id');const body=JSON.parse(await download(page,'original-download'));expect(body.documents[0].markdown).toBe(source);await page.locator('#scale').selectOption('local');await page.locator('#run').click();await expect(page.locator('#selection')).toContainText('own-id');await expect(page.locator('#records')).toContainText('own-fixture');await expect(page.locator('#records')).toContainText('Provisional');
});
test('opened JSON CRLF survives invalid UTF-8 replacement',async({page})=>{
 await open(page);const source=examples.tail.corpus_json.replaceAll('\n','\r\n');await page.locator('#corpus-file').setInputFiles({name:'corpus.json',mimeType:'application/json',buffer:Buffer.from(source)});await expect.poll(()=>page.locator('#corpus').inputValue()).toContain('synthetic-0');expect(await download(page,'original-download')).toBe(source);await page.locator('#corpus-file').setInputFiles({name:'bad.json',mimeType:'application/json',buffer:Buffer.from([255])});await expect(page.locator('#error')).toContainText('valid UTF-8');expect(await download(page,'original-download')).toBe(source);
});
test('oversize JSON and Markdown retain previous draft',async({page})=>{
 await open(page);const before=await download(page,'original-download');for(const [id,size] of [['corpus-file',524289],['markdown-files',32769]]){await page.locator('#'+id).setInputFiles({name:'large.md',mimeType:'text/plain',buffer:Buffer.alloc(size,32)});await expect(page.locator('#error')).toBeVisible();expect(await download(page,'original-download')).toBe(before);}
});
test('native admission holds and inert unknown frontmatter stay visible',async({page})=>{
 await open(page);const req=JSON.parse(fixtures.find(f=>f.name==='unknown-field').input);await page.locator('#corpus').fill(req.corpus_json);await page.locator('#run').click();await expect(page.locator('#records')).toContainText('extra-note');expect(await page.locator('#records img').count()).toBe(0);await page.locator('#corpus').fill('{broken');await page.locator('#run').click();await expect(page.locator('#error')).toBeVisible();await expect(page.locator('#results')).toBeHidden();expect(JSON.parse(await download(page,'report-download')).ok).toBe(false);
});
test('late file completion cannot overwrite an edit or reset',async({page})=>{
 await open(page);await page.evaluate(()=>{File.prototype.arrayBuffer=()=>new Promise(resolve=>window.finishRead=()=>resolve(new TextEncoder().encode('stale').buffer));});await page.locator('#corpus-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});await page.locator('#corpus').fill('new draft');await page.evaluate(()=>window.finishRead());expect(await page.locator('#corpus').inputValue()).toBe('new draft');await expect(page.locator('#run')).toBeEnabled();
 await page.locator('#corpus-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});await page.locator('#reset').click();await page.evaluate(()=>window.finishRead());expect(await page.locator('#corpus').inputValue()).toBe(examples.tail.corpus_json);
});
test('file deadline recovers while retaining original draft',async({page})=>{
 await open(page);await page.clock.install();const before=await page.locator('#corpus').inputValue();await page.evaluate(()=>{File.prototype.arrayBuffer=()=>new Promise(()=>{});});await page.locator('#corpus-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});await expect(page.locator('#run')).toBeDisabled();await page.clock.fastForward(5001);await expect(page.locator('#error')).toContainText('five seconds');await expect(page.locator('#run')).toBeEnabled();expect(await page.locator('#corpus').inputValue()).toBe(before);
});
test('startup timeout retains independent downloads',async({page})=>{
 await page.addInitScript(()=>{window.Worker=class{postMessage(){}terminate(){}};});await page.clock.install();await page.goto('./');await page.locator('#corpus').fill('retained');await page.clock.fastForward(12001);await expect(page.locator('#retry')).toBeVisible();expect(await download(page,'original-download')).toBe('retained');expect(JSON.parse(await download(page,'request-download')).corpus_json).toBe('retained');await expect(page.locator('a[href="./source.zip"]')).toBeVisible();
});
test('compute timeout survives edits and fences late results',async({page})=>{
 await page.addInitScript(()=>{window.Worker=class{constructor(){window.fakeWorker=this;setTimeout(()=>this.onmessage?.({data:{type:'ready'}}),20);}postMessage(data){this.last=data;}terminate(){}};});await page.clock.install();await page.goto('./');await page.clock.fastForward(21);await expect(page.locator('#run')).toBeDisabled();await page.locator('#corpus').fill('retained');await page.clock.fastForward(10001);await expect(page.locator('#retry')).toBeVisible();await page.evaluate(()=>window.fakeWorker.onmessage({data:{id:window.fakeWorker.last.id,canonical:'{}'}}));await expect(page.locator('#results')).toBeHidden();expect(await download(page,'original-download')).toBe('retained');
});
test('keyboard chooses held example and restores input',async({page})=>{
 await open(page);await page.locator('[data-example="held"]').focus();await page.keyboard.press('Enter');await page.locator('#run').focus();await page.keyboard.press('Enter');await expect(page.locator('#outcome')).toHaveText('admission held');await page.locator('#reset').focus();await page.keyboard.press('Enter');await expect(page.locator('#corpus')).toBeFocused();await page.locator('#run').focus();await page.keyboard.press('Enter');await expect(page.locator('#outcome')).toHaveText('provisional signals');
});
test('no JavaScript retains synthetic input and source downloads',async({browser,baseURL})=>{
 const context=await browser.newContext({javaScriptEnabled:false});const page=await context.newPage();await page.goto(baseURL);await expect(page.locator('#static-help')).toBeVisible();await expect(page.locator('#corpus')).toContainText('synthetic-0');const response=await page.request.get(new URL('source.zip',baseURL).href);expect((await response.body()).subarray(0,2).toString()).toBe('PK');await context.close();
});
test('WASM magic, MIME and independent artifact routes',async({request,baseURL})=>{
 const wasm=await request.get(new URL('pkg/packet_web_bg.wasm',baseURL).href);expect(wasm.headers()['content-type']).toContain('application/wasm');expect([...((await wasm.body()).subarray(0,4))]).toEqual([0,97,115,109]);for(const path of ['BUILD.md','LICENSE','THIRD-PARTY-NOTICES.txt','rust/COPYRIGHT-library.html','examples.json','source/crates/packet-gap/src/lib.rs','source/web/adapter/Cargo.lock'])expect((await request.get(new URL(path,baseURL).href)).ok(),path).toBeTruthy();
});
for(const width of [1440,390])test('readable research results at '+width,async({page})=>{
 await page.setViewportSize({width,height:1000});await open(page);expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBeTruthy();await mkdir('.cache/review',{recursive:true});await page.screenshot({path:`.cache/review/results-${width}.png`,fullPage:true});
});
