import { isValidObject, objects } from "shared-lib/gd";
import * as wasm from "wasm-lib";

export const setCheckedPreviewObject = (
    state: wasm.State,
    obj: wasm.GDObjectOpt
): boolean => {
    if (
        objects[obj.id].hitboxType == "Solid" &&
        (obj.x_angle % 18 != 0 || obj.y_angle % 18 != 0)
    ) {
        obj.x_angle = 0;
        obj.y_angle = 18;
    }

    if (isValidObject(obj)) {
        state.set_preview_object(obj);
        return true;
    }
    return false;
};
