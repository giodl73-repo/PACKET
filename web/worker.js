import init,{inspect} from './pkg/packet_web.js';
try{await init({module_or_path:new URL('./pkg/packet_web_bg.wasm',import.meta.url)});postMessage({type:'ready'});}catch(error){postMessage({type:'startup-error',message:String(error)});}
onmessage=({data})=>{if(data.type==='inspect')try{postMessage({type:'result',id:data.id,canonical:inspect(data.request)});}catch(error){postMessage({type:'compute-error',id:data.id,message:String(error)});}};
