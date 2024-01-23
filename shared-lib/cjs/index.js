"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.decodeString = exports.encodeString = exports.decode = exports.encode = exports.baseConvert = exports.objectOrder = exports.objects = void 0;
const objects_json_1 = __importDefault(require("./gd/objects.json"));
const object_order_json_1 = __importDefault(require("./gd/object_order.json"));
exports.objects = objects_json_1.default;
exports.objectOrder = object_order_json_1.default;
__exportStar(require("./util"), exports);
const baseConvert = (digits, fromBase, toBase) => {
    const BIG_FROM_BASE = BigInt(fromBase);
    const BIG_TO_BASE = BigInt(toBase);
    let bigSum = 0n;
    let zeroes = 0;
    for (let i = 0; i < digits.length; i++) {
        if (digits[i] == 0) {
            zeroes += 1;
        }
        else {
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
exports.baseConvert = baseConvert;
const encode = (v, base) => (0, exports.baseConvert)(Array.from(v), 256, base);
exports.encode = encode;
const decode = (v, base) => new Uint8Array((0, exports.baseConvert)(v, base, 256));
exports.decode = decode;
const encodeString = (data, base) => {
    let encoder = new TextDecoder();
    return encoder.decode(new Uint8Array((0, exports.encode)(data, base)));
};
exports.encodeString = encodeString;
const decodeString = (s, base) => {
    const encoder = new TextEncoder();
    return (0, exports.decode)([...encoder.encode(s)], base);
};
exports.decodeString = decodeString;
