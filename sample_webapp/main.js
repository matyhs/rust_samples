import init, {run} from './pkg/sample_webapp.js';

async function main() {
    await init('/pkg/sample_webapp_bg.wasm');
    run();
}

main()