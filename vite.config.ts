import {
    defineConfig,
    Plugin,
    PluginOption,
    searchForWorkspaceRoot,
    splitVendorChunk,
    splitVendorChunkPlugin,
} from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import topLevelAwait from "vite-plugin-top-level-await";
import FullReload from "vite-plugin-full-reload";
import UnpluginInjectPreload from "unplugin-inject-preload/vite";
import { ViteImageOptimizer } from "vite-plugin-image-optimizer";
import { sveltekit } from "@sveltejs/kit/vite";

const TURNSTILE_LOGIN_SITE_KEY = "'0x4AAAAAAAkCQrZbhWcKuz_T'";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => ({
    define: {
        __DEBUG: mode == "development",
        // __HAS_OPT_WASM: existsSync("wasm-lib/pkg/wasm_lib_bg.wasm-opt.wasm"),
        __TURNSTILE_LOGIN_SITE_KEY: TURNSTILE_LOGIN_SITE_KEY,
    },
    json: {
        stringify: true,
    },
    server: {
        fs: {
            allow: [
                // search up for workspace root
                searchForWorkspaceRoot(process.cwd()),
                // your custom rules
                "/shared-lib",
            ],
        },
        watch: {
            ignored: ["**/firebase/**", "**/emulator_data/**"],
        },
        hmr: false,
    },
    optimizeDeps: {
        esbuildOptions: {
            target: "esnext",
            define: {
                global: "globalThis",
            },
            supported: {
                bigint: true,
            },
        },
    },
    worker: {
        format: "es",
        rollupOptions: {
            output: {
                manualChunks(id: string) {
                    if (id.includes("shared-lib")) {
                        return "shared";
                    }
                    if (id.includes("pngjs")) {
                        return "pngjs-vendor";
                    }
                    if (id.includes("node_modules")) {
                        return "worker-vendor";
                    }
                },
            },
        },
    },
    esbuild: {
        legalComments: "none",
    },
    plugins: [
        sveltekit(),
        topLevelAwait(),
        // preload image assets (only works on `vite build`)
        // UnpluginInjectPreload({
        //     files: [
        //         {
        //             outputMatch: /ui\/.*\.(png|svg|otf|mp3)/,
        //             attributes: {
        //                 rel: "preload",
        //             },
        //         },
        //         {
        //             outputMatch: /.*\.(png|svg|otf|mp3)/,
        //             attributes: {
        //                 rel: "preload",
        //             },
        //         },
        //     ],
        //     injectTo: "head-prepend",
        // }),
        mode !== "development"
            ? ViteImageOptimizer({
                  exclude: ["spritesheet.png"],
                  test: /\.(jpe?g|png|gif|tiff|webp|svg|avif)$/i,
                  cache: false,
                  svg: {
                      plugins: [
                          {
                              name: "preset-default",
                              params: {
                                  overrides: {
                                      removeViewBox: false,
                                      collapseGroups: false,
                                      moveGroupAttrsToElems: false,
                                  },
                              },
                          },
                      ],
                  },
              })
            : null,
        FullReload(["**/*"], { always: false, root: "./src/", log: true }),
    ],
    build: {
        rollupOptions: {
            output: {
                // @ts-ignore
                manualChunks(id: string) {
                    if (id.includes("shared-lib")) {
                        return "shared";
                    }
                    if (id.includes("wasm-lib")) {
                        return "wasm";
                    }
                    if (id.includes("@firebase")) {
                        return "firebase-vendor";
                    }
                    if (id.includes("devtools-fps")) {
                        return "devtools-fps-vendor";
                    }
                    if (id.includes("node_modules")) {
                        return "client-vendor";
                    }
                },
            },
        },
        target: ["esnext"],
    },
}));
