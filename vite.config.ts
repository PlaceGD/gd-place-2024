import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import topLevelAwait from "vite-plugin-top-level-await";
import FullReload from "vite-plugin-full-reload";
import svelteSVG from "vite-plugin-svelte-svg";
import UnpluginInjectPreload from "unplugin-inject-preload/vite";
import obfuscatorPlugin from "vite-plugin-javascript-obfuscator";
import { existsSync } from "fs";

const TURNSTILE_LOGIN_SITE_KEY = "'0x4AAAAAAARPU_AxoWb2X1wE'";
const TURNSTILE_REPORT_SITE_KEY = "'0x4AAAAAAARP5tpK_cioW-QN'";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => ({
    define: {
        __DEBUG: mode == "development",
        __HAS_OPT_WASM: existsSync("wasm-lib/pkg/wasm_lib_bg.wasm-opt.wasm"),
        __TURNSTILE_LOGIN_SITE_KEY: TURNSTILE_LOGIN_SITE_KEY,
        __TURNSTILE_REPORT_SITE_KEY: TURNSTILE_REPORT_SITE_KEY,
    },
    plugins: [
        svelte(),
        svelteSVG({
            svgoConfig: {},
            requireSuffix: false,
        }),
        topLevelAwait(),
        // preload image assets (only works on `vite build`)
        UnpluginInjectPreload({
            files: [
                {
                    entryMatch: /\/assets\/ui\/.*\.(png|svg)/,
                    attributes: {
                        rel: "preload",
                    },
                },
                {
                    entryMatch: /Pusab\.oft/,
                    attributes: {
                        rel: "preload",
                    },
                },
            ],
            injectTo: "head-prepend",
        }),
        obfuscatorPlugin({
            include: ["**/*.js"],
            exclude: [/node_modules/, "wasm-lib/pkg/**"],
            // debugger: true,
            apply: "build",
            options: {
                domainLock: [mode == "development" ? "localhost" : "place.gd"],
                identifierNamesCache: {},
                identifierNamesGenerator: "mangled-shuffled",
                selfDefending: true,
                stringArrayCallsTransform: true,
                deadCodeInjection: false,
            },
        }),
        FullReload(["src/**/*"]),
    ],
}));
