/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        screens: {
            lg: { max: "1500px" },
            md: { max: "1000px" },
            sm: { max: "750px" },
            xs: { max: "500px" },
        },
        colors: {
            "menu-gray": "#0A0A0A",
            black: "black",
            white: "white",
            "disabled-white": "#444",
            red: "red",
            "button-green": "#75c934",
            "button-cyan-press": "#35CECE",
            transparent: "transparent",
        },
        fontFamily: {
            pusab: "Pusab",
        },
    },
    plugins: [],
};
