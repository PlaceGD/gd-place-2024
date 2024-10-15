import { sineInOut } from "svelte/easing";

export const menuHeight = (
    node: HTMLElement,
    { duration }: { duration: number }
) => {
    return {
        duration,
        easing: sineInOut,
        css: (t: number) => {
            return `height: ${t * 50}%;`;
        },
    };
};

export const fakeModalTransition = (_: HTMLElement) => {
    return {
        duration: 0,
        css: () => {
            return "";
        },
    };
};
