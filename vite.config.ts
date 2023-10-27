import { defineConfig } from "vite"
import { svelte } from "@sveltejs/vite-plugin-svelte"
import wasm from "vite-plugin-wasm"
import topLevelAwait from "vite-plugin-top-level-await"
import FullReload from "vite-plugin-full-reload"
import svelteSVG from "vite-plugin-svelte-svg"

// https://vitejs.dev/config/
export default defineConfig({
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
})
