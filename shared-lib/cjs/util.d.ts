export declare const lerp: (a: number, b: number, t: number) => number;
export declare const clamp: (a: number, min: number, max: number) => number;
export declare const map: (v: number, a: number, b: number, c: number, d: number) => number;
export declare const remEuclid: (a: number, b: number) => number;
export declare const hexToRgb: (hex: string) => {
    r: number;
    g: number;
    b: number;
} | null;
export declare const hsvToRgb: (h: number, s: number, v: number) => [number, number, number];
