import _objects from "./gd/objects.json";
import _objectOrder from "./gd/object_order.json";
import _spritesheet from "./gd/spritesheet.json";

export interface SpriteData {
    pos: [number, number];
    size: [number, number];
    offset: [number, number];
    rotated: boolean;
}
export interface ObjectData {
    placeOffsetX: number;
    placeOffsetY: number;
    tintable: boolean;
    solid: boolean;
    category: string; // CATEGORY_ICONS key
    builtinScale: number;
}

export const objects: Record<string, ObjectData> = _objects;
export const spritesheet: {
    mainSprites: Record<string, SpriteData>;
    detailSprites: Record<string, SpriteData>;
} = _spritesheet as any;
export const objectOrder: number[] = _objectOrder;

export * from "./util";

export const baseConvert = (
    digits: number[],
    fromBase: number,
    toBase: number
): number[] => {
    const BIG_FROM_BASE = BigInt(fromBase);
    const BIG_TO_BASE = BigInt(toBase);
    let bigSum = 0n;

    let zeroes = 0;
    for (let i = 0; i < digits.length; i++) {
        if (digits[i] == 0) {
            zeroes += 1;
        } else {
            break;
        }
    }

    for (let i = 0; i < digits.length; i++) {
        let p = digits.length - 1 - i;
        bigSum += BigInt(digits[i]) * BIG_FROM_BASE ** BigInt(p);
    }

    let ret = [];
    while (bigSum > 0n) {
        ret.push(Number(bigSum % BIG_TO_BASE));
        bigSum /= BIG_TO_BASE;
    }
    ret.push(...Array(zeroes).fill(0));
    ret.reverse();
    return ret;
};

export const encode = (v: Uint8Array, base: number): number[] =>
    baseConvert(Array.from(v), 256, base);
export const decode = (v: number[], base: number): Uint8Array =>
    new Uint8Array(baseConvert(v, base, 256));

export const encodeString = (data: Uint8Array, base: number): string => {
    let encoder = new TextDecoder();
    return encoder.decode(new Uint8Array(encode(data, base)));
};
export const decodeString = (s: string, base: number): Uint8Array => {
    const encoder = new TextEncoder();
    return decode([...encoder.encode(s)], base);
};
