import { svelteTsConfig } from "@jill64/eslint-config-svelte";

/** @type {import('@jill64/eslint-config-svelte').FlatConfig[]} */
export default [
    ...svelteTsConfig({ exclude: { deprecation: true } }),
    {
        languageOptions: {
            parserOptions: {
                project: ["./tsconfig.json", "./functions/tsconfig.json"],
            },
        },
        rules: {
            "@typescript-eslint/no-unused-vars": "warn",
            "@typescript-eslint/no-explicit-any": "allow",
        },
    },
];
