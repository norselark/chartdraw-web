import init, { run_app } from './pkg/yew_test.js';

async function main() {
   await init('/pkg/yew_test.wasm');
   run_app();
}
main()
