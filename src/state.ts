import * as wasm from "wasm-lib";
import { widgetData } from "./stores";

let state: wasm.StateWrapper | null = null;

type Callback = (s: wasm.StateWrapper) => void;

let callbacks: Record<number, Callback> = {};
let callbackCount = 0;
class CallbackID {
    constructor(public id: number) {}
    remove() {
        delete callbacks[this.id];
    }
}

export const addCallback = (f: Callback) => {
    callbacks[callbackCount] = f;
    callbackCount += 1;
    return new CallbackID(callbackCount - 1);
};
export const runCallbacks = () => {
    if (state != null) {
        for (let i of Object.values(callbacks)) {
            i(state);
        }
    }
};

export const loadState = (s: wasm.StateWrapper) => {
    state = s;
};

export const withState = <T>(f: (state: wasm.StateWrapper) => T): T => {
    if (state !== null) {
        return f(state);
    }
    return undefined as any;
};

export const setPreviewObject = (obj: wasm.GDObject) => {
    widgetData.update(data => {
        // console.log("bimby", obj.x_basis_len());
        let s = parseFloat(
            `${Math.max(obj.x_basis_len(), obj.y_basis_len())}`.slice(
                0,
                data.maxScaleLen
            )
        );
        let sx = parseFloat(`${obj.x_basis_len()}`.slice(0, data.maxScaleLen));
        let sy = parseFloat(`${obj.y_basis_len()}`.slice(0, data.maxScaleLen));
        let rot = obj.x_basis_angle();
        return {
            ...data,
            scale: s,
            prevScale: s,
            scaleX: sx,
            prevScaleX: sx,
            scaleY: sy,
            prevScaleY: sy,
            angle: rot,
            prevAngle: rot,
            // ix: obj.ix,
            // iy: obj.iy,
            // jx: obj.jx,
            // jy: obj.jy,
        };
    });
    state?.set_preview_object(obj);
};
