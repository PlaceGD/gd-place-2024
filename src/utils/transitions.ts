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

export const disappear = (_: HTMLElement) => {
    return {
        duration: 0,
        css: () => {
            return "";
        },
    };
};

// export const typewriter = (
//     node: HTMLElement,
//     { delay = 0, speed = 1 }: { delay?: number; speed?: number }
// ) => {
//     const text = node.textContent;
//     console.log(text);

//     if (!text) return { duration: 0, tick: () => {} };

//     const duration = text.length / (speed * 0.01);

//     return {
//         duration,
//         delay,
//         tick: (t: number) => {
//             const i = ~~(text.length * t);
//             node.textContent = text.slice(0, i);
//         },
//     };
// };
