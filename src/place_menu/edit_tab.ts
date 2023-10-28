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

interface Button {
    image: string;
    scale: number;
    cb: (obj: GDObject) => void;
    shortcut: Shortcut;
}

export const EDIT_BUTTONS: { [key: string]: { buttons: Button[] } } = {
    [EditTab.Transform]: {
        buttons: [
            {
                image: "left",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.x -= 30;
                },
                shortcut: {
                    key: "a",
                    shift: false,
                    alt: false,
                },
            },
            {
                image: "up",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.y += 30;
                },
                shortcut: {
                    key: "w",
                    shift: false,
                    alt: false,
                },
            },
            {
                image: "right",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.x += 30;
                },
                shortcut: {
                    key: "d",
                    shift: false,
                    alt: false,
                },
            },
            {
                image: "down",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.y -= 30;
                },
                shortcut: {
                    key: "s",
                    shift: false,
                    alt: false,
                },
            },
            {
                image: "left_small",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.x -= 2;
                },
                shortcut: {
                    key: "a",
                    shift: true,
                    alt: false,
                },
            },
            {
                image: "up_small",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.y += 2;
                },
                shortcut: {
                    key: "w",
                    shift: true,
                    alt: false,
                },
            },
            {
                image: "right_small",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.x += 2;
                },
                shortcut: {
                    key: "d",
                    shift: true,
                    alt: false,
                },
            },
            {
                image: "down_small",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.y -= 2;
                },
                shortcut: {
                    key: "s",
                    shift: true,
                    alt: false,
                },
            },
            {
                image: "left_big",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.x -= 30 * 5;
                },
                shortcut: {
                    key: "a",
                    shift: false,
                    alt: true,
                },
            },
            {
                image: "up_big",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.y += 30 * 5;
                },
                shortcut: {
                    key: "w",
                    shift: false,
                    alt: true,
                },
            },
            {
                image: "right_big",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.x += 30 * 5;
                },
                shortcut: {
                    key: "d",
                    shift: false,
                    alt: true,
                },
            },
            {
                image: "down_big",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.y -= 30 * 5;
                },
                shortcut: {
                    key: "s",
                    shift: false,
                    alt: true,
                },
            },
            {
                image: "flip_horiz",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.flip_x = !obj.flip_x;
                },
                shortcut: {
                    key: "q",
                    shift: false,
                    alt: true,
                },
            },
            {
                image: "flip_vert",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.flip_y = !obj.flip_y;
                },
                shortcut: {
                    key: "e",
                    shift: false,
                    alt: true,
                },
            },
            {
                image: "ccw",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.rotation -= 90;
                },
                shortcut: {
                    key: "q",
                    shift: false,
                    alt: false,
                },
            },
            {
                image: "cw",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.rotation += 90;
                },
                shortcut: {
                    key: "e",
                    shift: false,
                    alt: false,
                },
            },
            {
                image: "ccw_5",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.rotation -= 5;
                },
                shortcut: {
                    key: "q",
                    shift: true,
                    alt: false,
                },
            },
            {
                image: "cw_5",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.rotation += 5;
                },
                shortcut: {
                    key: "e",
                    shift: true,
                    alt: false,
                },
            },
            {
                image: "scale_up",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.scale += 0.1;
                },
                shortcut: {
                    key: "e",
                    shift: true,
                    alt: true,
                },
            },
            {
                image: "scale_down",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.scale -= 0.1;
                },
                shortcut: {
                    key: "q",
                    shift: true,
                    alt: true,
                },
            },
            {
                image: "left_small",
                scale: 0.5,
                cb: (obj: GDObject) => {
                    obj.x -= 0.5;
                },
                shortcut: {
                    key: "a",
                    shift: true,
                    alt: true,
                },
            },
            {
                image: "up_small",
                scale: 0.5,
                cb: (obj: GDObject) => {
                    obj.y += 0.5;
                },
                shortcut: {
                    key: "w",
                    shift: true,
                    alt: true,
                },
            },
            {
                image: "right_small",
                scale: 0.5,
                cb: (obj: GDObject) => {
                    obj.x += 0.5;
                },
                shortcut: {
                    key: "d",
                    shift: true,
                    alt: true,
                },
            },
            {
                image: "down_small",
                scale: 0.5,
                cb: (obj: GDObject) => {
                    obj.y -= 0.5;
                },
                shortcut: {
                    key: "s",
                    shift: true,
                    alt: true,
                },
            },
        ],
    },
    [EditTab.Layers]: {
        buttons: [
            {
                image: "z_plus",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.z_order += 1;
                },
                shortcut: {
                    key: "z",
                    shift: false,
                    alt: false,
                },
            },
            {
                image: "z_minus",
                scale: 1.0,
                cb: (obj: GDObject) => {
                    obj.z_order -= 1;
                },
                shortcut: {
                    key: "z",
                    shift: true,
                    alt: false,
                },
            },
            // {
            //     image: "ccw",
            //     scale: 1.0,
            // },
            // {
            //     image: "cw",
            //     scale: 1.0,
            // },
        ],
    },
    [EditTab.Colors]: {
        buttons: [],
    },
};
