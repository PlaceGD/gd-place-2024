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
import {
    PlaceReq,
    DeleteReq,
    PlaceRes,
    DeleteRes,
} from "shared-lib/cloud_functions";
import {
    CHUNK_SIZE_UNITS,
    END_RADIUS,
    LEVEL_HEIGHT_UNITS,
    LEVEL_WIDTH_UNITS,
} from "shared-lib/nexusgen";
import { Reader } from "./utils/reader";
import { Level, LogGroup } from "./utils/logger";
import { HttpsError, onCall } from "firebase-functions/v2/https";
import { onCallAuth, onCallAuthLogger } from "./utils/on_call";
import { smartDatabase } from "./exports";
import {
    checkedTransaction,
    getCheckedUserDetails,
    refAllGet,
} from "./utils/utils";
import Error from "./utils/errors";
import { write } from "firebase-functions/logger";

// #region deserializeObject
const deserializeObject = (
    data: string,
    logger: LogGroup
): GDObjectOpt | null => {
    let bytes: Uint8Array = decodeString(data, 126); // crazy base

    //logger.debug("Reading bytes:", bytes);

    let reader: Reader<Uint8Array>;
    try {
        reader = new Reader(bytes, GD_OBJECT_OPT_BYTE_SIZE);
    } catch (e: unknown) {
        logger.error("Reader failed:", e as string);
        throw Error.code(106, "invalid-argument");
    }

    const id = reader.readU16();
    //logger.debug("Object id:", id);
    if (objects[id] == undefined) return null;

    const x = reader.readF32();
    //logger.debug("Object x:", x);
    if (x < 0 || x > LEVEL_WIDTH_UNITS) return null;
    const y = reader.readF32();
    //logger.debug("Object y:", y);
    if (y < 0 || y > LEVEL_HEIGHT_UNITS) return null;

    if (objects[id].hitboxType != "NoHitbox") {
        let top_right = [LEVEL_WIDTH_UNITS, LEVEL_HEIGHT_UNITS];
        let pos = [x, y];

        let safezone_funny_len = ([x, y]: [number, number]): number =>
            Math.pow(Math.pow(x, 4.0) + Math.pow(y, 4.0), 1.0 / 4.0);

        if (
            safezone_funny_len([top_right[0] - pos[0], top_right[1] - pos[1]]) <
            END_RADIUS
        ) {
            return null;
        }
    }

    ///// validated inside `isValidObject`
    const x_scale_exp = reader.readI8();
    //logger.debug("Object x_scale_exp:", x_scale_exp);
    const x_angle = reader.readI8();
    //logger.debug("Object x_angle:", x_angle);
    const y_scale_exp = reader.readI8();
    //logger.debug("Object y_scale_exp:", y_scale_exp);
    const y_angle = reader.readI8();
    //logger.debug("Object y_angle:", y_angle);

    const z_layer = reader.readU8();
    //logger.debug("Object z_layer:", z_layer);

    const z_order = reader.readI8();
    //logger.debug("Object z_order:", z_order);
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
    //logger.debug("Object color red", r);
    if (r < 0 || r > 255) return null;

    const g = reader.readU8();
    //logger.debug("Object color green", r);
    if (g < 0 || g > 255) return null;

    const b = reader.readU8();
    //logger.debug("Object color blue", r);
    if (b < 0 || b > 255) return null;

    const opacity = reader.readU8();
    //logger.debug("Object opacity", r);
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
export const placeObject = onCallAuthLogger<PlaceReq, Promise<PlaceRes>>(
    "placeObject",
    async (request, logger) => {
        const db = smartDatabase();

        const data = request.data;
        const authUID = request.auth.uid;

        const userDetails = await getCheckedUserDetails(db, authUID);

        const [eventStartTime, eventEndTime, placeCooldown, chunkObjectLimit] =
            await refAllGet(
                db,
                "metaVariables/eventStartTime",
                "metaVariables/eventEndTime",
                "metaVariables/placeCooldown",
                "metaVariables/chunkObjectLimit"
            );

        const now = Date.now();

        if (now < eventStartTime.val() || now > eventEndTime.val()) {
            throw Error.code(209, "permission-denied");
        }

        await checkedTransaction(
            userDetails.ref.child("lastPlaceTimestamp"),
            lastPlaced => now >= (lastPlaced ?? 0) + placeCooldown.val() * 1000,
            () => Error.code(202, "permission-denied"),
            () => now
        );

        if (!data.object) {
            throw Error.code(400, "invalid-argument");
        }

        const object = deserializeObject(data.object, logger);
        if (object === null) {
            throw Error.code(106, "invalid-argument");
        }

        const chunkX = Math.floor(object.x / CHUNK_SIZE_UNITS);
        const chunkY = Math.floor(object.y / CHUNK_SIZE_UNITS);
        const chunkID: ChunkID = `${chunkX},${chunkY}`;

        await checkedTransaction(
            db.ref(`objectCount/${chunkID}`),
            count => (count ?? 0) < chunkObjectLimit.val(),
            () => Error.code(600, "resource-exhausted"),
            c => (c ?? 0) + 1
        );

        const objRef = await db.ref(`objects/${chunkID}`).push(data.object);

        await Promise.all([
            db.ref(`userPlaced/${objRef.key}`).set(userDetails.val.username),
            db.ref(`history`).push({
                object: data.object,
                objKey: objRef.key!,
                username: userDetails.val.username,
                time: now,
            }),
            db.ref(`totalObjectsPlaced`).transaction(v => (v ?? 0) + 1),
        ]);

        return { key: objRef.key ?? "", cooldown: placeCooldown.val() * 1000 };
    }
);

// #region deleteObject
export const deleteObject = onCallAuthLogger<DeleteReq, Promise<DeleteRes>>(
    "deleteObject",
    async (request, logger) => {
        const db = smartDatabase();
        const data = request.data;
        const uid = request.auth.uid;

        const userDetails = await getCheckedUserDetails(db, uid);

        if (!data.chunkId) {
            throw Error.code(403, "invalid-argument");
        }
        if (!data.objId) {
            throw Error.code(401, "invalid-argument");
        }

        const [deleteCooldown, eventStartTime, eventEndTime] = await refAllGet(
            db,
            "metaVariables/deleteCooldown",
            "metaVariables/eventStartTime",
            "metaVariables/eventEndTime"
        );

        const now = Date.now();

        if (now < eventStartTime.val() || now > eventEndTime.val()) {
            throw Error.code(209, "permission-denied");
        }

        await checkedTransaction(
            userDetails.ref.child("lastDeleteTimestamp"),
            lastDelete =>
                now >= (lastDelete ?? 0) + deleteCooldown.val() * 1000,
            () => Error.code(203, "permission-denied"),
            () => now
        );

        const obj = db.ref(`objects/${data.chunkId}/${data.objId}`);

        await Promise.all([
            // do not remove the %%
            // do not remove the %%
            // do not remove the %%
            // do not remove the %%
            // do not remove the %%
            // do not remove the %%
            // do not remove the %%
            // do not remove the %%
            obj.set(`%%${userDetails.val.username}`).then(() => obj.remove()),
            db.ref(`/userPlaced/${data.objId}`).remove(),
            db.ref(`history`).push({
                objKey: data.objId,
                username: userDetails.val.username,
                time: now,
                chunk: data.chunkId,
            }),
            db
                .ref(`objectCount/${data.chunkId}`)
                .transaction(v => (v ?? 1) - 1),
            db.ref(`totalObjectsDeleted`).transaction(v => (v ?? 0) + 1),
        ]);

        return { cooldown: deleteCooldown.val() * 1000 };
    }
);
