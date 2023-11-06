import type { GDObject } from "../../wasm-lib/pkg/wasm_lib";

export enum EditTab {
    Transform = "Transform",
    Layers = "Layers",
    Colors = "Colors",
}

interface Shortcut {
    key: string;
    shift: boolean;
    alt: boolean;
}

interface Keybind {
    cb: (obj: GDObject) => void;
    shortcut: Shortcut;
}

interface TransformButton {
    image: string;
    scale: number;
    cb: (obj: GDObject) => void;
}

export const KEYBINDS: Record<string, Keybind> = {
    move_up_small: {
        cb: (obj: GDObject) => {
            obj.y += 2;
        },
        shortcut: {
            key: "w",
            shift: true,
            alt: false,
        },
    },
    move_down_small: {
        cb: (obj: GDObject) => {
            obj.y -= 2;
        },
        shortcut: {
            key: "s",
            shift: true,
            alt: false,
        },
    },
    move_left_small: {
        cb: (obj: GDObject) => {
            obj.x -= 2;
        },
        shortcut: {
            key: "a",
            shift: true,
            alt: false,
        },
    },
    move_right_small: {
        cb: (obj: GDObject) => {
            obj.x += 2;
        },
        shortcut: {
            key: "d",
            shift: true,
            alt: false,
        },
    },

    move_up: {
        cb: (obj: GDObject) => {
            obj.y += 30;
        },
        shortcut: {
            key: "w",
            shift: false,
            alt: false,
        },
    },
    move_down: {
        cb: (obj: GDObject) => {
            obj.y -= 30;
        },
        shortcut: {
            key: "s",
            shift: false,
            alt: false,
        },
    },
    move_left: {
        cb: (obj: GDObject) => {
            obj.x -= 30;
        },
        shortcut: {
            key: "a",
            shift: false,
            alt: false,
        },
    },
    move_right: {
        cb: (obj: GDObject) => {
            obj.x += 30;
        },
        shortcut: {
            key: "d",
            shift: false,
            alt: false,
        },
    },

    flip_vert: {
        cb: (obj: GDObject) => {
            obj.flip_y = !obj.flip_y;
            // obj.rotation = obj.rotation - 180;
            obj.rotation = -obj.rotation;
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: true,
        },
    },
    flip_horiz: {
        cb: (obj: GDObject) => {
            obj.flip_x = !obj.flip_x;
            obj.rotation = -obj.rotation;
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: true,
        },
    },

    rotate_ccw: {
        cb: (obj: GDObject) => {
            obj.rotation -= 90;
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: false,
        },
    },
    rotate_cw: {
        cb: (obj: GDObject) => {
            obj.rotation += 90;
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: false,
        },
    },
    rotate_ccw_5: {
        cb: (obj: GDObject) => {
            obj.rotation -= 5;
        },
        shortcut: {
            key: "q",
            shift: true,
            alt: false,
        },
    },
    rotate_cw_5: {
        cb: (obj: GDObject) => {
            obj.rotation += 5;
        },
        shortcut: {
            key: "e",
            shift: true,
            alt: false,
        },
    },

    move_up_big: {
        cb: (obj: GDObject) => {
            obj.y += 30 * 5;
        },
        shortcut: {
            key: "w",
            shift: false,
            alt: true,
        },
    },
    move_down_big: {
        cb: (obj: GDObject) => {
            obj.y -= 30 * 5;
        },
        shortcut: {
            key: "s",
            shift: false,
            alt: true,
        },
    },
    move_left_big: {
        cb: (obj: GDObject) => {
            obj.x -= 30 * 5;
        },
        shortcut: {
            key: "a",
            shift: false,
            alt: true,
        },
    },
    move_right_big: {
        cb: (obj: GDObject) => {
            obj.x += 30 * 5;
        },
        shortcut: {
            key: "d",
            shift: false,
            alt: true,
        },
    },

    move_up_mini: {
        cb: (obj: GDObject) => {
            obj.y += 0.5;
        },
        shortcut: {
            key: "w",
            shift: true,
            alt: true,
        },
    },
    move_down_mini: {
        cb: (obj: GDObject) => {
            obj.y -= 0.5;
        },
        shortcut: {
            key: "s",
            shift: true,
            alt: true,
        },
    },
    move_left_mini: {
        cb: (obj: GDObject) => {
            obj.x -= 0.5;
        },
        shortcut: {
            key: "a",
            shift: true,
            alt: true,
        },
    },
    move_right_mini: {
        cb: (obj: GDObject) => {
            obj.x += 0.5;
        },
        shortcut: {
            key: "d",
            shift: true,
            alt: true,
        },
    },

    scale_up: {
        cb: (obj: GDObject) => {
            obj.scale += 0.1;
        },
        shortcut: {
            key: "e",
            shift: true,
            alt: true,
        },
    },
    scale_down: {
        cb: (obj: GDObject) => {
            obj.scale -= 0.1;
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
        image: "up_small",
        cb: KEYBINDS.move_up_small.cb,
        scale: 1.0,
    },
    {
        image: "down_small",
        cb: KEYBINDS.move_down_small.cb,
        scale: 1.0,
    },
    {
        image: "left_small",
        cb: KEYBINDS.move_left_small.cb,
        scale: 1.0,
    },
    {
        image: "right_small",
        cb: KEYBINDS.move_right_small.cb,
        scale: 1.0,
    },

    {
        image: "up",
        cb: KEYBINDS.move_up.cb,
        scale: 1.0,
    },
    {
        image: "down",
        cb: KEYBINDS.move_down.cb,
        scale: 1.0,
    },
    {
        image: "left",
        cb: KEYBINDS.move_left.cb,
        scale: 1.0,
    },
    {
        image: "right",
        cb: KEYBINDS.move_right.cb,
        scale: 1.0,
    },

    {
        image: "flip_horiz",
        cb: KEYBINDS.flip_horiz.cb,
        scale: 1.0,
    },
    {
        image: "flip_vert",
        cb: KEYBINDS.flip_vert.cb,
        scale: 1.0,
    },

    {
        image: "ccw",
        cb: KEYBINDS.rotate_ccw.cb,
        scale: 1.0,
    },
    {
        image: "cw",
        cb: KEYBINDS.rotate_cw.cb,
        scale: 1.0,
    },
    {
        image: "ccw_5",
        cb: KEYBINDS.rotate_ccw_5.cb,
        scale: 1.0,
    },
    {
        image: "cw_5",
        cb: KEYBINDS.rotate_cw_5.cb,
        scale: 1.0,
    },

    {
        image: "up_big",
        cb: KEYBINDS.move_up_big.cb,
        scale: 1.0,
    },
    {
        image: "down_big",
        cb: KEYBINDS.move_down_big.cb,
        scale: 1.0,
    },
    {
        image: "left_big",
        cb: KEYBINDS.move_left_big.cb,
        scale: 1.0,
    },
    {
        image: "right_big",
        cb: KEYBINDS.move_right_big.cb,
        scale: 1.0,
    },

    {
        image: "up_small",
        cb: KEYBINDS.move_up_mini.cb,
        scale: 0.5,
    },
    {
        image: "down_small",
        cb: KEYBINDS.move_down_mini.cb,
        scale: 0.5,
    },
    {
        image: "left_small",
        cb: KEYBINDS.move_left_mini.cb,
        scale: 0.5,
    },
    {
        image: "right_small",
        cb: KEYBINDS.move_right_mini.cb,
        scale: 0.5,
    },

    {
        image: "scale_up",
        cb: KEYBINDS.scale_up.cb,
        scale: 1.0,
    },
    {
        image: "scale_down",
        cb: KEYBINDS.scale_down.cb,
        scale: 1.0,
    },
];
