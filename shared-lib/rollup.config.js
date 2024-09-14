import typescript from "@rollup/plugin-typescript";
import { globSync } from "glob";
import path from "node:path";
import { fileURLToPath } from "node:url";
import terser from "@rollup/plugin-terser";

// adapted from https://github.com/timyourivh/vite-plugin-json5/blob/dev/src/index.ts
// rollup's builtin json plugin does not allow for stringifying the json
const jsonRE = /\.(json)$/;
function jsonPlugin() {
    let isBuild = false;

    return {
        name: "json",

        configResolved(config) {
            isBuild = config.command === "build";
        },

        transform(json, id) {
            if (!jsonRE.test(id)) return null;

            const parsed = JSON.parse(json);
            json = JSON.stringify(parsed);

            if (isBuild) {
                return {
                    code: `export default JSON.parse(${JSON.stringify(json)})`,
                    map: { mappings: "" },
                };
            } else {
                return {
                    code: `export default JSON.parse(${JSON.stringify(json)})`,
                    map: null,
                };
            }
        },
    };
}

export default [
    {
        input: Object.fromEntries(
            globSync("src/**/*.ts").map(file => [
                path.relative(
                    "src",
                    file.slice(0, file.length - path.extname(file).length)
                ),

                fileURLToPath(new URL(file, import.meta.url)),
            ])
        ),
        cache: false,
        output: {
            dir: "dist",
            format: "es",
            preserveModules: true,
            preserveModulesRoot: "src",
        },
        plugins: [
            typescript({ tsconfig: "./tsconfig.json" }),
            jsonPlugin(),
            terser(),
        ],
    },
];
