import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import topLevelAwait from "vite-plugin-top-level-await";
import FullReload from "vite-plugin-full-reload";
import svelteSVG from "vite-plugin-svelte-svg";

import { existsSync } from "fs";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => ({
    define: {
        __DEBUG: mode == "development",
        __HAS_OPT_WASM: existsSync("wasm-lib/pkg/wasm_lib_bg.wasm-opt.wasm"),
    },
    plugins: [
        svelte(),
        // svelteSVG({ svgo: {}, enforce: "pre" }),
        svelteSVG({
            svgoConfig: {}, // See https://github.com/svg/svgo#configuration
            requireSuffix: false, // Set false to accept '.svg' without the '?component'
        }),
        //wasm(),
        topLevelAwait(),
        FullReload(["src/**/*"]),
    ],
}));
