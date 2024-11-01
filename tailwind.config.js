/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        screens: {
            lg: [{ max: "1500px" }],
            md: [{ max: "1000px" }],
            sm: [{ max: "750px" }],
            xs: [{ max: "500px" }],
        },
        colors: {
            "menu-gray": "#0A0A0A",
            black: "black",
            white: "white",
            "disabled-white": "#444",
            red: "red",
            orange: "orange",
            "button-green": "#7ADE2D",
            "button-green-dark": "#61b91d",
            "button-cyan": "#35CECE",
            "button-cyan-dark": "#28a7a7",
            transparent: "transparent",
        },
        fontFamily: {
            pusab: "Pusab",
            saira: "Saira",
            exo: "Share Tech Mono",
            // exo: "Exo",
        },
    },
    plugins: [],
};
