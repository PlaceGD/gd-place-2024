import type { GDObjectOpt } from "wasm-lib";

export enum EditTab {
    Transform = "Transform",
    Layers = "Layers",
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
        image: "move_mini",
        amount: "1/60",
        keybinds: MOVE_KEYBINDS.MOVE_TINY,
    },
    MOVE_SMALL: {
        name: "Move 1/15th",
        image: "move_small",
        amount: "1/15",
        keybinds: MOVE_KEYBINDS.MOVE_SMALL,
        class: "hide-small",
    },
    MOVE_NORMAL: {
        name: "Move 1",
        image: "move_normal",
        amount: "1",
        keybinds: MOVE_KEYBINDS.MOVE_NORMAL,
    },
    MOVE_BIG: {
        name: "Move 5",
        image: "move_big",
        amount: "5",
        keybinds: MOVE_KEYBINDS.MOVE_BIG,
        class: "hide-big",
    },
};

export const TRANSFORM_KEYBINDS = {
    flip_vert: {
        cb: (obj: GDObjectOpt) => {
            obj.x_angle *= -1;
            obj.y_angle *= -1;
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: true,
        },
    },
    flip_horiz: {
        cb: (obj: GDObjectOpt) => {
            obj.x_angle -= 18;
            obj.y_angle -= 18;
            obj.x_angle *= -1;
            obj.y_angle *= -1;
            obj.x_angle += 18;
            obj.y_angle += 18;
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: true,
        },
    },

    rotate_ccw: {
        cb: (obj: GDObjectOpt) => {
            obj.x_angle += 18;
            obj.y_angle += 18;
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: false,
        },
    },
    rotate_cw: {
        cb: (obj: GDObjectOpt) => {
            obj.x_angle -= 18;
            obj.y_angle -= 18;
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: false,
        },
    },
    rotate_ccw_5: {
        cb: (obj: GDObjectOpt) => {
            obj.x_angle += 1;
            obj.y_angle += 1;
        },
        shortcut: {
            key: "q",
            shift: true,
            alt: false,
        },
    },
    rotate_cw_5: {
        cb: (obj: GDObjectOpt) => {
            obj.x_angle -= 1;
            obj.y_angle -= 1;
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
        image: "flip",
        cb: TRANSFORM_KEYBINDS.flip_horiz.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Flip Vertically",
        image: "flip",
        cb: TRANSFORM_KEYBINDS.flip_vert.cb,
        flipped: false,
        angle: 90,
    },

    {
        name: "Rotate Counter-Clockwise",
        image: "rotate",
        cb: TRANSFORM_KEYBINDS.rotate_ccw.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Rotate Clockwise",
        image: "rotate",
        cb: TRANSFORM_KEYBINDS.rotate_cw.cb,
        flipped: true,
        angle: 0,
    },

    {
        name: "Scale Up",
        image: "scale_up",
        cb: TRANSFORM_KEYBINDS.scale_up.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Scale Down",
        image: "scale_down",
        cb: TRANSFORM_KEYBINDS.scale_down.cb,
        flipped: false,
        angle: 0,
    },
];
