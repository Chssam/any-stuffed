import init, { run_popup } from './the_refresher.js';

async function run() {
    try {
        await init();
        console.log("Successful Init foreground");
        run_popup();
    } catch (error) {
        console.error('Failed to initialize:', error);
        document.getElementById('output').textContent = 'Error: Failed to initialize extension';
    }
}

run();
