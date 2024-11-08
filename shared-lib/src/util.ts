import { Unsubscriber, Writable } from "svelte/store";

export const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
export const clamp = (v: number, min: number, max: number) =>
    Math.max(Math.min(v, max), min);

export const signedClamp = (v: number, min: number, max: number) => {
    let neg = v < 0;
    return clamp(Math.abs(v), min, max) * (neg ? -1 : 1);
};
export const round = (v: number, decimals: number = 0) => {
    let m = 10 ** decimals;
    return Math.round(v * m) / m;
};

export const snap = (v: number, snap: number) => Math.round(v / snap) * snap;

export const map = (v: number, a: number, b: number, c: number, d: number) =>
    ((v - a) / (b - a)) * (d - c) + c;

export const remEuclid = (a: number, b: number): number => {
    let r = a % b;
    if (r < 0.0) {
        return r + Math.abs(b);
    } else {
        return r;
    }
};

export const hexToRgb = (hex: string) => {
    let result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result
        ? {
              r: parseInt(result[1], 16),
              g: parseInt(result[2], 16),
              b: parseInt(result[3], 16),
          }
        : null;
};

export const hsvToRgb = (
    h: number,
    s: number,
    v: number
): [number, number, number] => {
    let h1 = h * 6.0;

    let c = v * s;
    let x = c * (1.0 - Math.abs(remEuclid(h1, 2.0) - 1.0));
    let [r, g, b] = [0, 0, 0];

    if (0.0 <= h1 && h1 < 1.0) {
        [r, g, b] = [c, x, 0.0];
    } else if (1.0 <= h1 && h1 < 2.0) {
        [r, g, b] = [x, c, 0.0];
    } else if (2.0 <= h1 && h1 < 3.0) {
        [r, g, b] = [0.0, c, x];
    } else if (3.0 <= h1 && h1 < 4.0) {
        [r, g, b] = [0.0, x, c];
    } else if (4.0 <= h1 && h1 < 5.0) {
        [r, g, b] = [x, 0.0, c];
    } else {
        [r, g, b] = [c, 0.0, x];
    }

    let m = v - c;
    let [r1, g1, b1] = [r + m, g + m, b + m];

    return [r1 * 255.0, g1 * 255.0, b1 * 255.0];
};

export const getCenterPos = (element: HTMLElement): [number, number] => {
    let rect = element.getBoundingClientRect();
    return [rect.left + rect.width / 2, rect.top + rect.height / 2];
};

export const rotateVec = (
    [x, y]: [number, number],
    angle: number
): [number, number] => {
    let c = Math.cos(angle);
    let s = Math.sin(angle);

    return [c * x - s * y, s * x + c * y];
};

export const timerDisplay = (time?: number) => {
    if (time == null || time < 0 || !isFinite(time)) {
        return "--:--";
    }

    const mins = Math.floor(time / 60);
    const secs = Math.floor(time - mins * 60);
    return `${mins >= 10 ? "" : "0"}${mins}:${secs >= 10 ? "" : "0"}${secs}`;
};

export const semitonesToFactor = (s: number) => Math.pow(2, s / 12);
