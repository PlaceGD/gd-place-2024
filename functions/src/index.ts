/**
 * Import function triggers from their respective submodules:
 *
 * import {onCall} from "firebase-functions/v2/https";
 * import {onDocumentWritten} from "firebase-functions/v2/firestore";
 *
 * See a full list of supported triggers at https://firebase.google.com/docs/functions
 */
import * as functions from "firebase-functions/v2";

import { database } from "firebase-admin";
import { initializeApp } from "firebase-admin/app";
import { HttpsError } from "firebase-functions/v2/https";

const CHUNK_SIZE = { x: 20 * 30, y: 20 * 30 };

initializeApp();

export const placeObject = functions.https.onCall(
    { cors: true },
    async request => {
        const db = database();

        if (!request.data.object) {
            throw new HttpsError("invalid-argument", "Missing object string");
        }
        const object_string = request.data.object.toString();
        const object = deserialize_object(request.data.object);

        let chunk_x = Math.floor(object.x / CHUNK_SIZE.x);
        let chunk_y = Math.floor(object.y / CHUNK_SIZE.y);

        const ref = db.ref(`/objects/${chunk_x},${chunk_y}/`);

        await ref.push(object_string);
    }
);

export const deleteObject = functions.https.onCall(
    { cors: true },
    async request => {
        const db = database();
        const data = request.data;

        if (!data.chunkId) {
            throw new HttpsError("invalid-argument", "Missing chunk id");
        }
        if (!data.objId) {
            throw new HttpsError("invalid-argument", "Missing object id");
        }

        const ref = db.ref(`/objects/${data.chunkId}/${data.objId}`);
        ref.remove();
    }
);

function deserialize_object(object: string) {
    /*
    id: u16,
    x: f32,
    y: f32,
    rotation: f32,
    flip_x: bool,
    flip_y: bool,
    scale: f32,
    z_layer: ZLayer,
    z_order: i8,
    main_color: GDColor,
    detail_color: GDColor
    */
    let props = object.toString().split(",");

    if (props.length != 11) {
        throw new HttpsError("invalid-argument", "Invalid object string");
    }

    return {
        id: parseInt(props[0]),
        x: parseFloat(props[1]),
        y: parseFloat(props[2]),
        rotation: parseFloat(props[3]),
        flip_x: props[4] == "1",
        flip_y: props[5] == "1",
        scale: parseFloat(props[6]),
        z_layer: parseInt(props[7]),
        z_order: parseInt(props[8]),
        main_color: deserialize_color(props[9]),
        detail_color: deserialize_color(props[10]),
    };
}

function deserialize_color(color: string) {
    /*
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub opacity: u8,
    pub blending: bool,
    */
    let props = color.toString().split("|");

    if (props.length != 5) {
        throw new HttpsError("invalid-argument", "Invalid color string");
    }

    return {
        r: parseInt(props[0]),
        g: parseInt(props[1]),
        b: parseInt(props[2]),
        opacity: parseInt(props[3]),
        blending: props[4] == "1",
    };
}
