import * as wasm from "../wasm-lib/pkg/wasm_lib.js";
// import { toast } from "./utils/toast";

import { setTest, getTest } from "./test2.js";

onmessage = e => {
    if (e.data.event === "load") {
        setTest();
        console.log(getTest());
        postMessage({ event: "loaded", test: wasm });
        // init(e.data.resp)
        //     .catch(e => {
        //         toast.showErrorToast("Failed to initialize WASM.", e);
        //     })
        //     .then(w => {
        //         console.log(w as any);
        //         postMessage({ event: "loaded" });
        //     });
    }
};

console.log(self.scheduler.postMessage);
