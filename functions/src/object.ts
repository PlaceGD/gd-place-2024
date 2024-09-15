import { decodeString } from "shared-lib/base_util";
import {
    objects,
    colors,
    GDObjectOpt,
    GD_OBJECT_OPT_BYTE_SIZE,
    GDColor,
    isValidObject,
} from "shared-lib/gd";
import { ChunkID } from "shared-lib/database";
import { PlaceReq, DeleteReq } from "shared-lib/cloud_functions";
import { CHUNK_SIZE_UNITS, LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from ".";
import { Reader } from "./utils/reader";
import { Level, LogGroup } from "./utils/logger";
import { HttpsError, onCall } from "firebase-functions/v2/https";
import { onCallAuth, onCallAuthLogger } from "./utils/on_call";
import { smartDatabase } from "src";
import { getCheckedUserDetails } from "./utils/utils";

// #region deserializeObject
const deserializeObject = (
    data: string,
    logger: LogGroup
): GDObjectOpt | null => {
    let bytes: Uint8Array = decodeString(data, 126); // crazy base

    logger.debug("Reading bytes:", bytes);

    let reader: Reader<Uint8Array>;
    try {
        reader = new Reader(bytes, GD_OBJECT_OPT_BYTE_SIZE);
    } catch (e: unknown) {
        logger.error("Reader failed:", e as string);
        throw new HttpsError("invalid-argument", "Invalid object string");
    }

    const id = reader.readU16();
    logger.debug("Object id:", id);
    if (objects[id] == undefined) return null;

    const x = reader.readF32();
    logger.debug("Object x:", x);
    if (x < 0 || x > LEVEL_WIDTH_UNITS) return null;
    const y = reader.readF32();
    logger.debug("Object y:", y);
    if (y < 0 || y > LEVEL_HEIGHT_UNITS) return null;

    ///// validated inside `isValidObject`
    const x_scale_exp = reader.readI8();
    logger.debug("Object x_scale_exp:", x_scale_exp);
    const x_angle = reader.readI8();
    logger.debug("Object x_angle:", x_angle);
    const y_scale_exp = reader.readI8();
    logger.debug("Object y_scale_exp:", y_scale_exp);
    const y_angle = reader.readI8();
    logger.debug("Object y_angle:", y_angle);

    const z_layer = reader.readU8();
    logger.debug("Object z_layer:", z_layer);

    const z_order = reader.readI8();
    logger.debug("Object z_order:", z_order);
    /////

    const main_color = deserializeColor(reader, logger);
    if (main_color === null) return null;

    const detail_color = deserializeColor(reader, logger);
    if (detail_color === null) return null;

    let obj = {
        id,
        x,
        y,
        x_scale_exp,
        x_angle,
        y_scale_exp,
        y_angle,
        z_layer,
        z_order,
        main_color,
        detail_color,
    };
    if (!isValidObject(obj)) {
        return null;
    }

    return obj;
};

// #region deserializeColor
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

    return {
        r,
        g,
        b,
        opacity,
        blending,
    };
};

// #region placeObject
export const placeObject = onCallAuthLogger<PlaceReq>(
    "placeObject",
    async (request, logger) => {
        const db = smartDatabase();

        const data = request.data;
        const uid = request.auth.uid;

        const { userDetails, userDetailsRef } = await getCheckedUserDetails(
            db,
            uid
        );

        const placeTimerCooldown =
            (await db.ref("metaVariables/placeCooldown").get()).val() ?? 5 * 60;

        let transactionResult = await userDetailsRef
            .child("epochNextPlace")
            .transaction(nextPlace => {
                if (Date.now() < nextPlace ?? 0) {
                    throw new HttpsError(
                        "permission-denied",
                        "Cannot place before timer expired"
                    );
                }

                return Date.now() + placeTimerCooldown * 1000;
            });
        if (!transactionResult.committed) {
            logger.debug("Transaction not committed");
            return;
        }

        if (!data.object) {
            throw new HttpsError("invalid-argument", "Missing object string");
        }
        const objectString = data.object.toString();

        const object = deserializeObject(data.object, logger);
        if (object === null) {
            throw new HttpsError("invalid-argument", "Invalid object string");
        }

        let chunkX = Math.floor(object.x / CHUNK_SIZE_UNITS);
        let chunkY = Math.floor(object.y / CHUNK_SIZE_UNITS);
        const objRef = await db
            .ref(`objects/${chunkX},${chunkY}`)
            .push(objectString);

        db.ref(`userPlaced/${objRef.key}`).set(userDetails.username);
    }
);

// #region deleteObject
export const deleteObject = onCallAuthLogger<DeleteReq>(
    "deleteObject",
    async (request, logger) => {
        const db = smartDatabase();
        const data = request.data;
        const uid = request.auth.uid;

        const { userDetails, userDetailsRef } = await getCheckedUserDetails(
            db,
            uid
        );

        if (!data.chunkId) {
            throw new HttpsError("invalid-argument", "Missing chunk id");
        }
        if (!data.objId) {
            throw new HttpsError("invalid-argument", "Missing object id");
        }
        const deleteTimerCooldown =
            (await db.ref("metaVariables/deleteCooldown").get()).val() ??
            5 * 60;

        let transactionResult = await userDetailsRef
            .child("epochNextDelete")
            .transaction(nextDelete => {
                if (Date.now() < nextDelete ?? 0) {
                    throw new HttpsError(
                        "permission-denied",
                        "Cannot delete before timer expired"
                    );
                }

                return Date.now() + deleteTimerCooldown * 1000;
            });
        if (!transactionResult.committed) {
            logger.debug("Transaction not committed");
            return;
        }

        const obj = db.ref(`objects/${data.chunkId as ChunkID}/${data.objId}`);
        obj.set(userDetails.username).then(() => obj.remove());

        db.ref(`/userPlaced/${data.objId}`).remove();
    }
);
