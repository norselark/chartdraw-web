import init, { run_app } from './pkg/yew_chartdraw.js';

async function main() {
   await init('/pkg/yew_chartdraw_bg.wasm');
   run_app();
}
main()
