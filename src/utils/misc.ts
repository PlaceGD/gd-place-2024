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

export const extractFilenames = <T>(
    glob: Record<string, T>
): Record<string, T> => {
    return Object.entries(glob).reduce<Record<string, T>>(
        (prev, [path, data]) => {
            const fname = path.split("/").at(-1)?.split(".")[0];

            if (fname == null) return prev;

            prev[fname] = data;
            return prev;
        },
        {}
    );
};
