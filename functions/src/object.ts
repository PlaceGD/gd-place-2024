import { database } from "firebase-admin";
// import { initializeApp } from "firebase-admin/app";
import { HttpsError, onCall } from "firebase-functions/v2/https";

import { decodeString, objects, colors } from "shared-lib";

import { CHUNK_SIZE_UNITS, LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from ".";
import { Reader } from "./utils/reader";
import { Level, LogGroup } from "./utils/logger";
import { ChunkID, ObjKey } from "shared-lib";
import { refAllGet, ref } from "./utils/database";

interface GDColor {
    r: number;
    g: number;
    b: number;
    opacity: number;
    blending: boolean;
}

interface GDObjectOpt {
    id: number;
    x: number;
    y: number;
    x_scale_exp: number;
    x_angle: number;
    y_scale_exp: number;
    y_angle: number;
    zLayer: number;
    zOrder: number;
    mainColor: GDColor;
    detailColor: GDColor;
}
const GD_OBJECT_SIZE = 26;

const deserializeObject = (
    data: string,
    logger: LogGroup
): GDObjectOpt | null => {
    let bytes: Uint8Array = decodeString(data, 126); // crazy base

    logger.debug("Reading bytes:", bytes);

    let reader: Reader<Uint8Array>;
    try {
        reader = new Reader(bytes, GD_OBJECT_SIZE);
    } catch (e: any) {
        logger.error("Reader failed:", e);
        throw new HttpsError("invalid-argument", "Invalid object string");
    }

    const id = reader.readU16();
    logger.debug("Object id:", id);
    if (objects[id] === undefined) return null;

    const x = reader.readF32();
    logger.debug("Object x:", x);
    if (x < 0 || x > LEVEL_WIDTH_UNITS) return null;
    const y = reader.readF32();
    logger.debug("Object y:", y);
    if (y < 0 || y > LEVEL_HEIGHT_UNITS) return null;

    const x_scale_exp = reader.readI8();
    logger.debug("Object x_scale_exp:", x_scale_exp);
    const x_angle = reader.readI8();
    logger.debug("Object x_angle:", x_angle);
    const y_scale_exp = reader.readI8();
    logger.debug("Object y_scale_exp:", y_scale_exp);
    const y_angle = reader.readI8();
    logger.debug("Object y_angle:", y_angle);

    const zLayer = reader.readU8();
    logger.debug("Object zLayer:", zLayer);
    if (zLayer < 0 || zLayer > 8) return null;

    const zOrder = reader.readI8();
    logger.debug("Object zOrder:", zOrder);
    if (zOrder < -50 || zOrder > 50) return null;

    const mainColor = deserializeColor(reader, logger);
    if (mainColor === null) return null;

    const detailColor = deserializeColor(reader, logger);
    if (detailColor === null) return null;

    return {
        id,
        x,
        y,
        x_scale_exp,
        x_angle,
        y_scale_exp,
        y_angle,
        zLayer,
        zOrder,
        mainColor,
        detailColor,
    };
};

const deserializeColor = (
    reader: Reader<Uint8Array>,
    logger: LogGroup
): GDColor | null => {
    const r = reader.readU8();
    logger.debug("Object color red", r);
    if (r < 0 || r > 255) return null;

    const g = reader.readU8();
    logger.debug("Object color green", r);
    if (g < 0 || g > 255) return null;

    const b = reader.readU8();
    logger.debug("Object color blue", r);
    if (b < 0 || b > 255) return null;

    const opacity = reader.readU8();
    logger.debug("Object opacity", r);
    if (opacity < 0 || opacity > 255) return null;

    const blending = reader.readBool();

    if (r == 0 && g == 0 && b == 0 && blending) return null;

    const colorsContain = (r: number, g: number, b: number): boolean => {
        for (let { palette } of colors.list) {
            for (let i of palette) {
                for (let j of i) {
                    if (r == j[0] && g == j[1] && b == j[2]) {
                        return true;
                    }
                }
            }
        }
        return false;
    };

    if (!colorsContain(r, g, b)) return null;

    return {
        r,
        g,
        b,
        opacity,
        blending,
    };
};

type PlaceReq = { object: string };

export const placeObject = onCall<PlaceReq>({ cors: true }, async request => {
    if (!request.auth) {
        throw new HttpsError("unauthenticated", "User is not authenticated");
    }

    // /ref/
    //     <key>: gf

    const db = database();

    const data = request.data;
    const uid = request.auth.uid;

    if (!data.object) {
        throw new HttpsError("invalid-argument", "Missing object string");
    }
    const objectString = data.object.toString();

    const objLogger = new LogGroup("placeObject");
    let object;
    try {
        object = deserializeObject(data.object, objLogger);
        objLogger.finish();
    } catch (e) {
        objLogger.finish(Level.ERROR);
        throw e;
    }

    if (object === null) {
        objLogger.finish(Level.ERROR);
        throw new HttpsError("invalid-argument", "Invalid object string");
    }

    let chunkX = Math.floor(object.x / CHUNK_SIZE_UNITS);
    let chunkY = Math.floor(object.y / CHUNK_SIZE_UNITS);

    let [
        // obj_limit,
        // obj_count,
        // { eventStart, placeTimer: timer, canEdit },
        _userName,
        // banned,
    ] = await refAllGet(db, `userData/${uid}/username`);

    let userName = _userName.val();
    if (userName === undefined) {
        throw new HttpsError("invalid-argument", "Missing user data");
    }

    const objRef = await ref(db, `objects/${chunkX},${chunkY}`).push(
        objectString
    );

    ref(db, `userPlaced/${objRef.key}`).set(userName);
});

type DeleteReq = { chunkId: string; objId: string };

export const deleteObject = onCall<DeleteReq>({ cors: true }, async request => {
    if (!request.auth) {
        throw new HttpsError("unauthenticated", "User is not authenticated");
    }

    const db = database();
    const data = request.data;
    const uid = request.auth.uid;

    let [userName] = (await refAllGet(db, `userData/${uid}/username`)).map(v =>
        v.val()
    );

    if (userName === undefined) return;

    if (!data.chunkId) {
        throw new HttpsError("invalid-argument", "Missing chunk id");
    }
    if (!data.objId) {
        throw new HttpsError("invalid-argument", "Missing object id");
    }

    const obj = ref(db, `objects/${data.chunkId as ChunkID}/${data.objId}`);
    obj.set(userName).then(() => obj.remove());

    ref(db, `/userPlaced/${data.objId}`).remove();
});
