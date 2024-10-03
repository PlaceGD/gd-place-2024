import typescript from "@rollup/plugin-typescript";
import json from "@rollup/plugin-json";
import terser from "@rollup/plugin-terser";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import "dotenv/config";

const REQUIRED_ENV_VARS = [
    "KOFI_VERIFICATION_TOKEN",
    "MAILJET_API_KEY",
    "MAILJET_API_SECRET",
];
const missingVars = REQUIRED_ENV_VARS.filter(key => !process.env[key]);

if (missingVars.length > 0) {
    throw new Error(
        `Missing required environment variables: ${missingVars.join(", ")}`
    );
}

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
