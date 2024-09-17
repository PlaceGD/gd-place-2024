import { sineInOut } from "svelte/easing";

export const menuHeight = (
    node: HTMLElement,
    { duration }: { duration: number }
) => {
    return {
        duration,
        easing: sineInOut,
        css: (t: number) => {
            return `max-height: ${t * 75}%;`;
        },
    };
};
