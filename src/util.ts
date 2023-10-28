export const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
export const clamp = (a: number, min: number, max: number) =>
    Math.max(Math.min(a, max), min);

export const hexToRgb = (hex: string) => {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result
        ? {
              r: parseInt(result[1], 16),
              g: parseInt(result[2], 16),
              b: parseInt(result[3], 16),
          }
        : null;
};

export const isOverflow = (element: HTMLElement): boolean => {
    let curOverflow = element.style.overflow;

    if (!curOverflow || curOverflow === "visible")
        element.style.overflow = "hidden";

    let isOverflowing =
        element.clientWidth < element.scrollWidth ||
        element.clientHeight < element.scrollHeight;

    element.style.overflow = curOverflow;

    return isOverflowing;
};
