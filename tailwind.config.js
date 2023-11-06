/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        screens: {
            lg: { max: "1500px" },
            md: { max: "1000px" },
            sm: { max: "600px" },
        },
        colors: {
            "menu-gray": "#0A0A0A",
            black: "black",
            white: "white",
            red: "red",
            "button-light-green": "#b7f782",
            "button-dark-green": "#407530",
        },
        fontFamily: {
            pusab: "Pusab",
        },
    },
    plugins: [],
};
