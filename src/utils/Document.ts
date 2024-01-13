import Toast from "./Toast";

export const isOverflow = (element: HTMLElement): boolean => {
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
