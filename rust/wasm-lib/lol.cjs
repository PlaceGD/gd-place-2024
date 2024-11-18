const baseConvert = (digits, fromBase, toBase) => {
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
const decode = (v, base) => new Uint8Array(baseConvert(v, base, 256));
const decodeString = (s, base) => {
    const encoder = new TextEncoder();
    return decode([...encoder.encode(s)], base);
};

const fs = require("node:fs");
fs.readFile("db_objects.json", "utf8", (err, s) => {
    if (err) {
        console.error(err);
        return;
    }
    let data = JSON.parse(s);

    let out = "";

    for (let v of Object.values(data)) {
        for (let [k, obj] of Object.entries(v)) {
            out += k + ",";
            for (let i of decodeString(obj, 126)) {
                out += `${i},`;
            }
        }
    }
    fs.writeFile("glog.txt", out, err => {
        if (err) {
            console.error(err);
        } else {
            // file written successfully
        }
    });
});
