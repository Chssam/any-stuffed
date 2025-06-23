(async () => {
    try {
        const src = chrome.runtime.getURL("content.js");
        const wasmPath = chrome.runtime.getURL("content_bg.wasm");

        const contentMain = await import(src);

        if (!contentMain.default) throw new Error("WASM entry point not found!");
        await contentMain.default({ module_or_path: wasmPath });

        // attaching extract function to window
        window.contentMain = contentMain;
    } catch (err) {
        console.error("Failed to initialize WASM module:", err);
    }
})();
