import * as wasm from "wasm-lib";
import { subChunk, unsubChunk } from "../firebase/chunks";
import { decodeString } from "shared-lib/base_util";
import Toast from "../utils/toast";
import { addDeleteText, editorData } from "../stores";
import debounce from "lodash.debounce";

export const handleSub = (state: wasm.StateWrapper) => {
    for (let chunk of state.get_chunks_to_sub()) {
        subChunk(
            [chunk.x, chunk.y],
            data => {
                let key = data.key;
                if (key != null) {
                    try {
                        let obj = wasm.GDObjectOpt.from_bytes(
                            decodeString(data.val(), 126)
                        );

                        state.add_object(key, obj);
                    } catch (e: any) {
                        console.error("(Failed in `GDObjectOpt.from_bytes`)");
                        Toast.showErrorToast(e.display());
                    }
                }
            },
            data => {
                let key = data.key;
                if (key != null) {
                    let coords = state.delete_object(key);
                    if (coords != null) {
                        addDeleteText(data.val(), coords[0], coords[1]);
                    }
                }
            }
        );
    }
};

export const handleUnsub = (state: wasm.StateWrapper) => {
    for (let chunk of state.get_chunks_to_unsub()) {
        unsubChunk([chunk.x, chunk.y]);
    }
};

const savePos = debounce((state: wasm.StateWrapper) => {
    let zoom = Math.round(state.get_zoom());
    let [x, y] = state.get_camera_pos().map(Math.round);

    history.replaceState({}, "", `?x=${x}&y=${y}&zoom=${zoom}`);

    editorData.update(data => {
        data.x = x;
        data.y = y;
        data.zoom = zoom;
        return data;
    });
}, 200);

export const moveCamera = (state: wasm.StateWrapper, x: number, y: number) => {
    state.set_camera_pos(x, y);

    savePos(state);
    handleSub(state);
};

export const getCameraPos = (state: wasm.StateWrapper): [number, number] => {
    const [x, y] = state.get_camera_pos();
    return [x, y];
};
