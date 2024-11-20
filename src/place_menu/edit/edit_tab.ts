import { objects, type ObjectCategory } from "shared-lib/gd";
import { remEuclid, rotateVec } from "shared-lib/util";
import { type GDObjectOpt, convert_opt_transform } from "wasm-lib";
import { extractFilenames } from "../../utils/misc";
import {
    menuBuildTab,
    menuEditTab,
    menuTabGroup,
    TabGroup,
} from "../../stores";
import { get } from "svelte/store";
import { CATEGORY_ICONS } from "../../gd/object";

export const EDIT_TAB_ICONS = extractFilenames<string>(
    import.meta.glob("../assets/edit_tab/*.svg", {
        eager: true,
        query: "?url",
        import: "default",
    })
);

export enum EditTab {
    Transform = "Transform",
    Layers = "Layers",

    // this is colors for most stuff but it will be sfx and song for the respective triggers! we
    Colors = "Colors",
}

export enum WidgetType {
    None,
    Rotate,
    Scale,
    Warp,
}

interface Shortcut {
    key: string;
    shift: boolean;
    alt: boolean;
}

interface Keybind {
    cb: (obj: GDObjectOpt) => void;
    shortcut: Shortcut;
}

interface TransformButton {
    name: string;
    image: string;
    flipped: boolean;
    angle: number;
    cb: (obj: GDObjectOpt) => void;
}

interface MoveKeybind {
    up: Keybind;
    down: Keybind;
    left: Keybind;
    right: Keybind;
}

export const MOVE_KEYBINDS = {
    MOVE_TINY: {
        up: {
            cb: (obj: GDObjectOpt) => {
                obj.y += 0.5;
            },
            shortcut: {
                key: "w",
                shift: true,
                alt: true,
            },
        },
        down: {
            cb: (obj: GDObjectOpt) => {
                obj.y -= 0.5;
            },
            shortcut: {
                key: "s",
                shift: true,
                alt: true,
            },
        },
        left: {
            cb: (obj: GDObjectOpt) => {
                obj.x -= 0.5;
            },
            shortcut: {
                key: "a",
                shift: true,
                alt: true,
            },
        },
        right: {
            cb: (obj: GDObjectOpt) => {
                obj.x += 0.5;
            },
            shortcut: {
                key: "d",
                shift: true,
                alt: true,
            },
        },
    },

    MOVE_SMALL: {
        up: {
            cb: (obj: GDObjectOpt) => {
                obj.y += 2;
            },
            shortcut: {
                key: "w",
                shift: true,
                alt: false,
            },
        },
        down: {
            cb: (obj: GDObjectOpt) => {
                obj.y += -2;
            },
            shortcut: {
                key: "s",
                shift: true,
                alt: false,
            },
        },
        left: {
            cb: (obj: GDObjectOpt) => {
                obj.x -= 2;
            },
            shortcut: {
                key: "a",
                shift: true,
                alt: false,
            },
        },
        right: {
            cb: (obj: GDObjectOpt) => {
                obj.x += 2;
            },
            shortcut: {
                key: "d",
                shift: true,
                alt: false,
            },
        },
    },

    MOVE_NORMAL: {
        up: {
            cb: (obj: GDObjectOpt) => {
                obj.y += 30;
            },
            shortcut: {
                key: "w",
                shift: false,
                alt: false,
            },
        },
        down: {
            cb: (obj: GDObjectOpt) => {
                obj.y += -30;
            },
            shortcut: {
                key: "s",
                shift: false,
                alt: false,
            },
        },
        left: {
            cb: (obj: GDObjectOpt) => {
                obj.x -= 30;
            },
            shortcut: {
                key: "a",
                shift: false,
                alt: false,
            },
        },
        right: {
            cb: (obj: GDObjectOpt) => {
                obj.x += 30;
            },
            shortcut: {
                key: "d",
                shift: false,
                alt: false,
            },
        },
    },

    MOVE_BIG: {
        up: {
            cb: (obj: GDObjectOpt) => {
                obj.y += 30 * 5;
            },
            shortcut: {
                key: "w",
                shift: false,
                alt: true,
            },
        },
        down: {
            cb: (obj: GDObjectOpt) => {
                obj.y -= 30 * 5;
            },
            shortcut: {
                key: "s",
                shift: false,
                alt: true,
            },
        },
        left: {
            cb: (obj: GDObjectOpt) => {
                obj.x -= 30 * 5;
            },
            shortcut: {
                key: "a",
                shift: false,
                alt: true,
            },
        },
        right: {
            cb: (obj: GDObjectOpt) => {
                obj.x += 30 * 5;
            },
            shortcut: {
                key: "d",
                shift: false,
                alt: true,
            },
        },
    },
};

interface MoveButton {
    name: string;
    image: string;
    amount: string;
    keybinds: MoveKeybind;
    class?: string;
}

export const MOVE_BUTTONS: Record<string, MoveButton> = {
    MOVE_TINY: {
        name: "Move 1/60th",
        image: EDIT_TAB_ICONS["move_mini"],
        amount: "1/60",
        keybinds: MOVE_KEYBINDS.MOVE_TINY,
    },
    MOVE_SMALL: {
        name: "Move 1/15th",
        image: EDIT_TAB_ICONS["move_small"],
        amount: "1/15",
        keybinds: MOVE_KEYBINDS.MOVE_SMALL,
        class: "hide-small",
    },
    MOVE_NORMAL: {
        name: "Move 1",
        image: EDIT_TAB_ICONS["move_normal"],
        amount: "1",
        keybinds: MOVE_KEYBINDS.MOVE_NORMAL,
    },
    MOVE_BIG: {
        name: "Move 5",
        image: EDIT_TAB_ICONS["move_big"],
        amount: "5",
        keybinds: MOVE_KEYBINDS.MOVE_BIG,
        class: "hide-big",
    },
};

export const getTransformedPlaceOffset = (
    obj: GDObjectOpt
): [number, number] => {
    let info = objects[obj.id];
    let [vx, vy] = [-info.placeOffsetX, -info.placeOffsetY];

    let [ix, iy, jx, jy] = convert_opt_transform(
        obj.x_scale_exp,
        obj.x_angle,
        obj.y_scale_exp,
        obj.y_angle
    );

    return [ix * vx + jx * vy, iy * vx + jy * vy];
};

export const TRANSFORM_KEYBINDS = {
    flip_vert: {
        cb: (obj: GDObjectOpt) => {
            let [_, ay] = getTransformedPlaceOffset(obj);

            obj.x_angle *= -1;
            obj.y_angle *= -1;

            obj.y += ay * 2;
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: true,
        },
    },
    flip_horiz: {
        cb: (obj: GDObjectOpt) => {
            let [ax, _] = getTransformedPlaceOffset(obj);

            obj.x_angle -= 18;
            obj.y_angle -= 18;
            obj.x_angle *= -1;
            obj.y_angle *= -1;
            obj.x_angle += 18;
            obj.y_angle += 18;

            obj.x += ax * 2;
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: true,
        },
    },

    rotate_ccw: {
        cb: (obj: GDObjectOpt) => {
            let [ax, ay] = getTransformedPlaceOffset(obj);

            obj.x_angle += 18;
            obj.y_angle += 18;

            let [ox, oy] = rotateVec([ax, ay], (90 / 180) * Math.PI);
            [obj.x, obj.y] = [obj.x + ax - ox, obj.y + ay - oy];
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: false,
        },
    },
    rotate_cw: {
        cb: (obj: GDObjectOpt) => {
            let [ax, ay] = getTransformedPlaceOffset(obj);

            obj.x_angle -= 18;
            obj.y_angle -= 18;

            let [ox, oy] = rotateVec([ax, ay], (-90 / 180) * Math.PI);
            [obj.x, obj.y] = [obj.x + ax - ox, obj.y + ay - oy];
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: false,
        },
    },
    rotate_ccw_5: {
        cb: (obj: GDObjectOpt) => {
            let [ax, ay] = getTransformedPlaceOffset(obj);

            obj.x_angle += 1;
            obj.y_angle += 1;

            let [ox, oy] = rotateVec([ax, ay], (5 / 180) * Math.PI);
            [obj.x, obj.y] = [obj.x + ax - ox, obj.y + ay - oy];
        },
        shortcut: {
            key: "q",
            shift: true,
            alt: false,
        },
    },
    rotate_cw_5: {
        cb: (obj: GDObjectOpt) => {
            let [ax, ay] = getTransformedPlaceOffset(obj);

            obj.x_angle -= 1;
            obj.y_angle -= 1;

            let [ox, oy] = rotateVec([ax, ay], (-5 / 180) * Math.PI);
            [obj.x, obj.y] = [obj.x + ax - ox, obj.y + ay - oy];
        },
        shortcut: {
            key: "e",
            shift: true,
            alt: false,
        },
    },
    scale_up: {
        cb: (obj: GDObjectOpt) => {
            obj.x_scale_exp += 1;
            obj.y_scale_exp += 1;
        },
        shortcut: {
            key: "e",
            shift: true,
            alt: true,
        },
    },
    scale_down: {
        cb: (obj: GDObjectOpt) => {
            obj.x_scale_exp -= 1;
            obj.y_scale_exp -= 1;
        },
        shortcut: {
            key: "q",
            shift: true,
            alt: true,
        },
    },
};

export const TRANSFORM_BUTTONS: TransformButton[] = [
    {
        name: "Flip Horizontally",
        image: EDIT_TAB_ICONS["flip"],
        cb: TRANSFORM_KEYBINDS.flip_horiz.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Flip Vertically",
        image: EDIT_TAB_ICONS["flip"],
        cb: TRANSFORM_KEYBINDS.flip_vert.cb,
        flipped: false,
        angle: 90,
    },

    {
        name: "Rotate Counter-Clockwise",
        image: EDIT_TAB_ICONS["rotate"],
        cb: TRANSFORM_KEYBINDS.rotate_ccw.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Rotate Clockwise",
        image: EDIT_TAB_ICONS["rotate"],
        cb: TRANSFORM_KEYBINDS.rotate_cw.cb,
        flipped: true,
        angle: 0,
    },

    {
        name: "Scale Up",
        image: EDIT_TAB_ICONS["scale_up"],
        cb: TRANSFORM_KEYBINDS.scale_up.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Scale Down",
        image: EDIT_TAB_ICONS["scale_down"],
        cb: TRANSFORM_KEYBINDS.scale_down.cb,
        flipped: false,
        angle: 0,
    },
];

const moveBuildTab = (dir: "left" | "right") => {
    const objectCategories = Object.keys(CATEGORY_ICONS) as ObjectCategory[];

    const currentBlockTab = objectCategories.indexOf(get(menuBuildTab));

    if (currentBlockTab === -1) return;

    menuBuildTab.set(
        objectCategories[
            remEuclid(
                currentBlockTab + (dir === "left" ? -1 : 1),
                objectCategories.length
            )
        ]
    );
};

const moveEditTab = (dir: "left" | "right") => {
    const editTabs = Object.values(EditTab);
    const currentEditTab = editTabs.indexOf(get(menuEditTab));

    if (currentEditTab === -1) return;

    menuEditTab.set(
        editTabs[
            remEuclid(
                currentEditTab + (dir === "left" ? -1 : 1),
                editTabs.length
            )
        ]
    );
};

export const MISC_KEYBINDS = {
    build_tab: {
        cb: () => {
            menuTabGroup.set(TabGroup.Build);
        },
        shortcut: {
            key: "1",
            shift: false,
            alt: false,
        },
    },
    edit_tab: {
        cb: () => {
            menuTabGroup.set(TabGroup.Edit);
        },
        shortcut: {
            key: "2",
            shift: false,
            alt: false,
        },
    },
    delete_tab: {
        cb: () => {
            menuTabGroup.set(TabGroup.Delete);
        },
        shortcut: {
            key: "3",
            shift: false,
            alt: false,
        },
    },

    build_tab_left: {
        cb: () => {
            const tabGroup = get(menuTabGroup);
            if (tabGroup == TabGroup.Build) {
                moveBuildTab("left");
            } else if (tabGroup == TabGroup.Edit) {
                moveEditTab("left");
            }
        },
        shortcut: {
            key: "ArrowLeft",
            shift: false,
            alt: false,
        },
    },
    build_tab_right: {
        cb: () => {
            const tabGroup = get(menuTabGroup);
            if (tabGroup == TabGroup.Build) {
                moveBuildTab("right");
            } else if (tabGroup == TabGroup.Edit) {
                moveEditTab("right");
            }
        },
        shortcut: {
            key: "ArrowRight",
            shift: false,
            alt: false,
        },
    },
};
