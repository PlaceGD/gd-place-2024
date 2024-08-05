import typescript from "@rollup/plugin-typescript";
import json from "@rollup/plugin-json";
import { globSync } from "glob";
import path from "node:path";
import { fileURLToPath } from "node:url";
import terser from "@rollup/plugin-terser";

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
            json({ compact: true, preferConst: true }),
            terser(),
        ],
    },
];
