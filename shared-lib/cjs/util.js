"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.hsvToRgb = exports.hexToRgb = exports.remEuclid = exports.map = exports.clamp = exports.lerp = void 0;
const lerp = (a, b, t) => a + (b - a) * t;
exports.lerp = lerp;
const clamp = (a, min, max) => Math.max(Math.min(a, max), min);
exports.clamp = clamp;
const map = (v, a, b, c, d) => ((v - a) / (b - a)) * (d - c) + c;
exports.map = map;
const remEuclid = (a, b) => {
    let r = a % b;
    if (r < 0.0) {
        return r + Math.abs(b);
    }
    else {
        return r;
    }
};
exports.remEuclid = remEuclid;
const hexToRgb = (hex) => {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result
        ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16),
        }
        : null;
};
exports.hexToRgb = hexToRgb;
const hsvToRgb = (h, s, v) => {
    let h1 = h * 6.0;
    let c = v * s;
    let x = c * (1.0 - Math.abs((0, exports.remEuclid)(h1, 2.0) - 1.0));
    let [r, g, b] = [0, 0, 0];
    if (0.0 <= h1 && h1 < 1.0) {
        [r, g, b] = [c, x, 0.0];
    }
    else if (1.0 <= h1 && h1 < 2.0) {
        [r, g, b] = [x, c, 0.0];
    }
    else if (2.0 <= h1 && h1 < 3.0) {
        [r, g, b] = [0.0, c, x];
    }
    else if (3.0 <= h1 && h1 < 4.0) {
        [r, g, b] = [0.0, x, c];
    }
    else if (4.0 <= h1 && h1 < 5.0) {
        [r, g, b] = [x, 0.0, c];
    }
    else {
        [r, g, b] = [c, 0.0, x];
    }
    let m = v - c;
    let [r1, g1, b1] = [r + m, g + m, b + m];
    return [r1 * 255.0, g1 * 255.0, b1 * 255.0];
};
exports.hsvToRgb = hsvToRgb;
