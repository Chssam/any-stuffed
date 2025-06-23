import init from './background.js';

(async () => {
    try {
        await init({ module_or_path: "/background_bg.wasm" });
        console.log("Successful Init background");
    } catch (error) {
        console.error('Failed to run_background:', error);
    }
})();
