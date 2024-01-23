export declare const objects: Record<string, any>;
export declare const objectOrder: number[];
export * from "./util";
export declare const baseConvert: (digits: number[], fromBase: number, toBase: number) => number[];
export declare const encode: (v: Uint8Array, base: number) => number[];
export declare const decode: (v: number[], base: number) => Uint8Array;
export declare const encodeString: (data: Uint8Array, base: number) => string;
export declare const decodeString: (s: string, base: number) => Uint8Array;
