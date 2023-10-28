export const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
export const clamp = (a: number, min: number, max: number) =>
    Math.max(Math.min(a, max), min);
