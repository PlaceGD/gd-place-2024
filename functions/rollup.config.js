import typescript from "@rollup/plugin-typescript";
import json from "@rollup/plugin-json";
import terser from "@rollup/plugin-terser";
import { nodeResolve } from "@rollup/plugin-node-resolve";

export default [
    {
        input: "./src/index.ts",
        cache: false,
        output: {
            dir: "dist",
            format: "es",
            preserveModules: true,
            preserveModulesRoot: "src",
        },
        plugins: [
            typescript({
                tsconfig: "./tsconfig.json",
                moduleResolution: "node",
            }),
            // nodeResolve(),
            json({ compact: true, preferConst: true }),
            terser(),
        ],
    },
];
