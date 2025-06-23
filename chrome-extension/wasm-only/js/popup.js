import init, { run_in_foreground } from './refresher.js';

async function run() {
    try {
        await init();
        console.log("Successful Init foreground");
        run_in_foreground();
    } catch (error) {
        console.error('Failed to initialize:', error);
        document.getElementById('output').textContent = 'Error: Failed to initialize extension';
    }
}

run();
