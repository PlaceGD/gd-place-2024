import { writable } from "svelte/store";
import Toast from "./toast";

// // fixes safari toolbar blocking content
// // https://www.reddit.com/r/nextjs/comments/11g3znz/comment/janib69/?utm_source=share&utm_medium=web2x&context=3
// const fixHeight = () => {
//     const vh = window.innerHeight * 0.01;
//     document.documentElement.style.setProperty("--vh", `${vh}px`);
// };
// fixHeight();

// let oldHeight = 0;
// window.addEventListener(
//     "resize",
//     () => {
//         if (oldHeight != window.innerHeight) {
//             oldHeight = window.innerHeight;
//             fixHeight();
//         }
//     },
//     { passive: true }
// );

export const isOverflowing = (element: HTMLElement): boolean => {
    if (!element) return false;

    let curOverflow = element.style.overflow;

    if (!curOverflow || curOverflow === "visible")
        element.style.overflow = "hidden";

    let isOverflowing =
        element.clientWidth < element.scrollWidth ||
        element.clientHeight < element.scrollHeight;

    element.style.overflow = curOverflow;

    return isOverflowing;
};

export const useIsOverflowing = () => {
    const { subscribe: isOverflowingSubscribe, update: updateIsOverflowing } =
        writable(false);

    const updateElement = (element: HTMLElement | null) => {
        let updateElement = () => {
            if (element) {
                updateIsOverflowing(() => isOverflowing(element));
            }
        };
        window.addEventListener("resize", updateElement, { passive: true });
        window.addEventListener("DOMContentLoaded", updateElement, {
            passive: true,
        });

        if (element) {
            updateElement();
        }
    };

    return {
        isOverflowing: { subscribe: isOverflowingSubscribe },
        element: updateElement,
    };
};

export const hasDarkReader = (): boolean => {
    return document.getElementsByClassName("darkreader").length > 0;
};

export const alertHasDarkReader = () => {
    if (hasDarkReader()) {
        Toast.showInfoToast(
            "The DarkReader extension has been detected! Please disable this for a better experience."
        );
    }
};

export const isMobile = (): boolean => {
    if (typeof window != "undefined") {
        return false;
    }

    try {
        document.createEvent("TouchEvent");
        return true;
    } catch (e) {
        return false;
    }
};
