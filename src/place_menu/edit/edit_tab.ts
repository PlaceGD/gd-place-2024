import type { GDObject } from "wasm-lib";

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
    name: string;
    image: string;
    flipped: boolean;
    angle: number;
    cb: (obj: GDObject) => void;
}

export const KEYBINDS: Record<string, Keybind> = {
    move_up_small: {
        cb: (obj: GDObject) => {
            obj.translate(0, 2);
        },
        shortcut: {
            key: "w",
            shift: true,
            alt: false,
        },
    },
    move_down_small: {
        cb: (obj: GDObject) => {
            obj.translate(0, -2);
        },
        shortcut: {
            key: "s",
            shift: true,
            alt: false,
        },
    },
    move_left_small: {
        cb: (obj: GDObject) => {
            obj.translate(-2, 0);
        },
        shortcut: {
            key: "a",
            shift: true,
            alt: false,
        },
    },
    move_right_small: {
        cb: (obj: GDObject) => {
            obj.translate(2, 0);
        },
        shortcut: {
            key: "d",
            shift: true,
            alt: false,
        },
    },

    move_up: {
        cb: (obj: GDObject) => {
            obj.translate(0, 30);
        },
        shortcut: {
            key: "w",
            shift: false,
            alt: false,
        },
    },
    move_down: {
        cb: (obj: GDObject) => {
            obj.translate(0, -30);
        },
        shortcut: {
            key: "s",
            shift: false,
            alt: false,
        },
    },
    move_left: {
        cb: (obj: GDObject) => {
            obj.translate(-30, 0);
        },
        shortcut: {
            key: "a",
            shift: false,
            alt: false,
        },
    },
    move_right: {
        cb: (obj: GDObject) => {
            obj.translate(30, 0);
        },
        shortcut: {
            key: "d",
            shift: false,
            alt: false,
        },
    },

    flip_vert: {
        cb: (obj: GDObject) => {
            obj.apply_matrix(1, 0, 0, -1);
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: true,
        },
    },
    flip_horiz: {
        cb: (obj: GDObject) => {
            obj.apply_matrix(-1, 0, 0, 1);
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: true,
        },
    },

    rotate_ccw: {
        cb: (obj: GDObject) => {
            obj.rotate(-90);
        },
        shortcut: {
            key: "q",
            shift: false,
            alt: false,
        },
    },
    rotate_cw: {
        cb: (obj: GDObject) => {
            obj.rotate(90);
        },
        shortcut: {
            key: "e",
            shift: false,
            alt: false,
        },
    },
    rotate_ccw_5: {
        cb: (obj: GDObject) => {
            obj.rotate(-5);
        },
        shortcut: {
            key: "q",
            shift: true,
            alt: false,
        },
    },
    rotate_cw_5: {
        cb: (obj: GDObject) => {
            obj.rotate(5);
        },
        shortcut: {
            key: "e",
            shift: true,
            alt: false,
        },
    },

    move_up_big: {
        cb: (obj: GDObject) => {
            obj.translate(0, 30 * 5);
        },
        shortcut: {
            key: "w",
            shift: false,
            alt: true,
        },
    },
    move_down_big: {
        cb: (obj: GDObject) => {
            obj.translate(0, -30 * 5);
        },
        shortcut: {
            key: "s",
            shift: false,
            alt: true,
        },
    },
    move_left_big: {
        cb: (obj: GDObject) => {
            obj.translate(-30 * 5, 0);
        },
        shortcut: {
            key: "a",
            shift: false,
            alt: true,
        },
    },
    move_right_big: {
        cb: (obj: GDObject) => {
            obj.translate(30 * 5, 0);
        },
        shortcut: {
            key: "d",
            shift: false,
            alt: true,
        },
    },

    move_up_mini: {
        cb: (obj: GDObject) => {
            obj.translate(0, 0.5);
        },
        shortcut: {
            key: "w",
            shift: true,
            alt: true,
        },
    },
    move_down_mini: {
        cb: (obj: GDObject) => {
            obj.translate(0, -0.5);
        },
        shortcut: {
            key: "s",
            shift: true,
            alt: true,
        },
    },
    move_left_mini: {
        cb: (obj: GDObject) => {
            obj.translate(-0.5, 0);
        },
        shortcut: {
            key: "a",
            shift: true,
            alt: true,
        },
    },
    move_right_mini: {
        cb: (obj: GDObject) => {
            obj.translate(0.5, 0);
        },
        shortcut: {
            key: "d",
            shift: true,
            alt: true,
        },
    },

    scale_up: {
        cb: (obj: GDObject) => {
            obj.scale(2 ** (1 / 12));
        },
        shortcut: {
            key: "e",
            shift: true,
            alt: true,
        },
    },
    scale_down: {
        cb: (obj: GDObject) => {
            obj.scale(2 ** (-1 / 12));
        },
        shortcut: {
            key: "q",
            shift: true,
            alt: true,
        },
    },
    ebonky: {
        cb: (obj: GDObject) => {
            obj.skew(0.1, 0);
        },
        shortcut: {
            key: "f",
            shift: false,
            alt: false,
        },
    },
};

export const TRANSFORM_BUTTONS: TransformButton[] = [
    {
        name: "Move Up 2 Units",
        image: "move_small",
        cb: KEYBINDS.move_up_small.cb,
        flipped: false,
        angle: 180,
    },
    {
        name: "Move Down 2 Units",
        image: "move_small",
        cb: KEYBINDS.move_down_small.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Move Left 2 Units",
        image: "move_small",
        cb: KEYBINDS.move_left_small.cb,
        flipped: false,
        angle: 90,
    },
    {
        name: "Move Right 2 Units",
        image: "move_small",
        cb: KEYBINDS.move_right_small.cb,
        flipped: false,
        angle: -90,
    },

    {
        name: "Move Up 1 Block",
        image: "move_normal",
        cb: KEYBINDS.move_up.cb,
        flipped: false,
        angle: 180,
    },
    {
        name: "Move Down 1 Block",
        image: "move_normal",
        cb: KEYBINDS.move_down.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Move Left 1 Block",
        image: "move_normal",
        cb: KEYBINDS.move_left.cb,
        flipped: false,
        angle: 90,
    },
    {
        name: "Move Right 1 Block",
        image: "move_normal",
        cb: KEYBINDS.move_right.cb,
        flipped: false,
        angle: -90,
    },

    {
        name: "Flip Horizontally",
        image: "flip",
        cb: KEYBINDS.flip_horiz.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Flip Vertically",
        image: "flip",
        cb: KEYBINDS.flip_vert.cb,
        flipped: false,
        angle: 90,
    },

    {
        name: "Rotate Counter-Clockwise",
        image: "rotate",
        cb: KEYBINDS.rotate_ccw.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Rotate Clockwise",
        image: "rotate",
        cb: KEYBINDS.rotate_cw.cb,
        flipped: true,
        angle: 0,
    },
    {
        name: "Rotate 5 Degrees Counter-Clockwise",
        image: "rotate_5_ccw",
        cb: KEYBINDS.rotate_ccw_5.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Rotate 5 Degrees Clockwise",
        image: "rotate_5_cw",
        cb: KEYBINDS.rotate_cw_5.cb,
        flipped: false,
        angle: 0,
    },

    {
        name: "Move Up 5 Blocks",
        image: "move_big",
        cb: KEYBINDS.move_up_big.cb,
        flipped: false,
        angle: 180,
    },
    {
        name: "Move Down 5 Blocks",
        image: "move_big",
        cb: KEYBINDS.move_down_big.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Move Left 5 Blocks",
        image: "move_big",
        cb: KEYBINDS.move_left_big.cb,
        flipped: false,
        angle: 90,
    },
    {
        name: "Move Right 5 Blocks",
        image: "move_big",
        cb: KEYBINDS.move_right_big.cb,
        flipped: false,
        angle: -90,
    },

    {
        name: "Move Up 0.5 Units",
        image: "move_tiny",
        cb: KEYBINDS.move_up_mini.cb,
        flipped: false,
        angle: 180,
    },
    {
        name: "Move Down 0.5 Units",
        image: "move_tiny",
        cb: KEYBINDS.move_down_mini.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Move Left 0.5 Units",
        image: "move_tiny",
        cb: KEYBINDS.move_left_mini.cb,
        flipped: false,
        angle: 90,
    },
    {
        name: "Move Right 0.5 Units",
        image: "move_tiny",
        cb: KEYBINDS.move_right_mini.cb,
        flipped: false,
        angle: -90,
    },

    {
        name: "Scale Up",
        image: "scale_up",
        cb: KEYBINDS.scale_up.cb,
        flipped: false,
        angle: 0,
    },
    {
        name: "Scale Down",
        image: "scale_down",
        cb: KEYBINDS.scale_down.cb,
        flipped: false,
        angle: 0,
    },
];
