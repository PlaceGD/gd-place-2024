import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import topLevelAwait from "vite-plugin-top-level-await";
import FullReload from "vite-plugin-full-reload";
import svelteSVG from "vite-plugin-svelte-svg";
import UnpluginInjectPreload from "unplugin-inject-preload/vite";
import { existsSync } from "fs";
import { splitVendorChunkPlugin } from "vite";

const TURNSTILE_LOGIN_SITE_KEY = "'0x4AAAAAAARPU_AxoWb2X1wE'";
const TURNSTILE_REPORT_SITE_KEY = "'0x4AAAAAAARP5tpK_cioW-QN'";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => ({
    define: {
        __DEBUG: mode == "development",
        // __HAS_OPT_WASM: existsSync("wasm-lib/pkg/wasm_lib_bg.wasm-opt.wasm"),
        __TURNSTILE_LOGIN_SITE_KEY: TURNSTILE_LOGIN_SITE_KEY,
        __TURNSTILE_REPORT_SITE_KEY: TURNSTILE_REPORT_SITE_KEY,
    },
    json: {
        stringify: true,
    },
    worker: {
        format: "es",
    },
    esbuild: {
        legalComments: "none",
    },
    plugins: [
        svelte(),
        svelteSVG({
            svgoConfig: {},
            requireSuffix: false,
        }),
        topLevelAwait(),
        // preload image assets (only works on `vite build`)
        // UnpluginInjectPreload({
        //     files: [
        //         {
        //             // entryMatch: /\/assets\/ui\/.*\.(png|svg)/,
        //             outputMatch: /\/assets\/ui\/.*\.(png|svg|otf|mp3)/,
        //             attributes: {
        //                 rel: "preload",
        //             },
        //         },
        //         {
        //             // entryMatch: /\/assets\/ui\/.*\.(png|svg)/,
        //             outputMatch: /\/assets\/.*\.(png|svg|otf|mp3)/,
        //             attributes: {
        //                 rel: "preload",
        //             },
        //         },
        //         // {
        //         //     entryMatch: /Pusab\.oft/,
        //         //     attributes: {
        //         //         rel: "preload",
        //         //     },
        //         // },
        //     ],
        //     injectTo: "head-prepend",
        // }),
        FullReload(["src/**/*"]),
        splitVendorChunkPlugin(),
    ],
}));
