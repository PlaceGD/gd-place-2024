import typescript from "@rollup/plugin-typescript";
import json from "@rollup/plugin-json";
import terser from "@rollup/plugin-terser";

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
            typescript({ tsconfig: "./tsconfig.json" }),
            json({ compact: true, preferConst: true }),
            terser(),
        ],
    },
];
