import { HttpsError, onCall, Request } from "firebase-functions/v2/https";
import { onMessagePublished } from "firebase-functions/v2/pubsub";
import { getAuth } from "firebase-admin/auth";
import { REPORT_COOLDOWN_SECONDS, VALID_USERNAME } from "shared-lib/user";
import type {
    InitWithUsernameReq,
    ReportUserReq,
    ReportedUserOperationReq,
    BanReq,
    ReportUserRes,
} from "shared-lib/cloud_functions";
import { Level, LogGroup } from "./utils/logger";
import { LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS } from "shared-lib/nexusgen";
import { onCallAuth, onCallAuthLogger } from "./utils/on_call";
import { UserDetails } from "shared-lib/database";
import { DEV_UIDS } from "shared-lib/cloud_functions";
import { auth, mailjetClient, smartDatabase } from "./exports";
import { checkedTransaction, getCheckedUserDetails } from "./utils/utils";
import { FILTERS } from "./utils/username_filter";
import Error from "./utils/errors";

// #region validateTurnstile
// const validateTurnstile = async (
//     key: string | undefined,
//     token: string,
//     request: Request,
//     logger: LogGroup
// ) => {
//     key =
//         process.env["NODE_ENV"] == "development"
//             ? "1x0000000000000000000000000000000AA"
//             : key;

//     if (key == undefined) {
//         throw new HttpsError(
//             "permission-denied",
//             "Missing turnstile key! Please contact a developer."
//         );
//     }

//     const ip = request.get("CF-Connecting-IP");

//     let formData = new FormData();
//     formData.append("secret", key);
//     formData.append("response", token!);
//     formData.append("remoteip", ip!);

//     const url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
//     const result = await fetch(url, {
//         body: formData,
//         method: "POST",
//     });

//     const outcome = await result.json();

//     logger.info("Turnstile outcome:", JSON.stringify(outcome));

//     if (!outcome.success) {
//         throw new HttpsError(
//             "permission-denied",
//             "Something went wrong. Please try again."
//         );
//     }
// };

const GD_PLACE_CONTACT_LIST_ID = "10488916";

// #region initUserWithUsername
export const initUserWithUsername = onCallAuthLogger<
    InitWithUsernameReq,
    Promise<UserDetails>
>("initUserWithUsername", async (request, logger) => {
    const data = request.data;

    // await validateTurnstile(
    //     process.env["TURNSTILE_LOGIN_PRIV_KEY"],
    //     data.turnstileResp,
    //     request.rawRequest,
    //     logger
    // );

    logger.debug("Username:", data.username);
    if (!data.username.match(VALID_USERNAME)) {
        throw Error.code(100, "invalid-argument");
    }

    const db = smartDatabase();

    const usernameLower = data.username.toLowerCase();
    FILTERS.forEach(regFilter => {
        if (regFilter.test(usernameLower)) {
            throw Error.code(100, "invalid-argument");
        }
    });

    const usernameDataRef = db.ref(`userName/${usernameLower}`);

    if ((await usernameDataRef.get()).exists()) {
        logger.error("Username already exists");
        throw Error.code(300, "already-exists");
    }

    const userDetailsRef = db.ref(`userDetails/${data.uid}`);

    if ((await userDetailsRef.get()).exists()) {
        logger.error("User data already exists");
        throw Error.code(301, "already-exists");
    }

    const isBanned = (await db.ref(`bannedUsers/${data.uid}`).get()).exists();
    if (isBanned) {
        throw Error.code(201, "permission-denied");
    }

    const user: UserDetails = {
        username: data.username,
        lastPlaceTimestamp: 0,
        lastDeleteTimestamp: 0,
        lastReportTimestamp: 0,
        lastGradientTimestamp: 0,
        lastCharacterTimestamp: 0,
        moderator: false,
        hasDonated: false,
        signupdate: Date.now(),
    };

    logger.info("User created sucessfully");

    await Promise.all([
        userDetailsRef.set(user),
        usernameDataRef.set({
            uid: data.uid,
            displayColor: "white",
        }),
        db.ref("userCount").transaction(count => {
            return count + 1;
        }),
    ]);

    try {
        const authData = await auth.getUser(request.auth.uid);

        if (authData.email != null) {
            const contactRequest = mailjetClient
                .post(
                    `contactslist/${GD_PLACE_CONTACT_LIST_ID}/managecontact`,
                    { version: "v3" }
                )
                .request({
                    Action: "addforce",
                    Email: authData.email,
                    Properties: {
                        firstName: data.username,
                    },
                });

            contactRequest.catch(() => {
                logger.error(
                    `Failed to add user to contact list: ${usernameLower}).`
                );
            });
        }
    } catch {
        logger.error(`Failed to add user to contact list: ${usernameLower}).`);
        return user;
    }

    return user;
});

// #region reportUser
export const reportUser = onCallAuthLogger<
    ReportUserReq,
    Promise<ReportUserRes>
>("reportUser", async (request, logger) => {
    const data = request.data;

    // await validateTurnstile(
    //     process.env["TURNSTILE_GENERAL_PRIV_KEY"],
    //     data.turnstileResp,
    //     request.rawRequest,
    //     logger
    // );

    const db = smartDatabase();

    const now = Date.now();

    const samaritanDetails = await getCheckedUserDetails(db, request.auth.uid);

    const [_, badUser] = await Promise.all([
        checkedTransaction(
            samaritanDetails.ref.child("lastReportTimestamp"),
            lastReport =>
                now >= (lastReport ?? 0) + REPORT_COOLDOWN_SECONDS * 1000,
            () => Error.code(204, "permission-denied"),
            () => now // + REPORT_COOLDOWN_SECONDS * 1000
        ),
        db
            .ref(`userName/${data.username.toLowerCase()}`)
            .get()
            .then(v => v.val()),
    ]);

    if (badUser == undefined) {
        throw Error.code(100, "invalid-argument");
    }

    if (isNaN(data.x) || data.x < 0 || data.x > LEVEL_WIDTH_UNITS) {
        logger.debug("User stinkily reported at X", data.x);
        throw Error.code(101, "invalid-argument");
    }
    if (isNaN(data.y) || data.y < 0 || data.y > LEVEL_HEIGHT_UNITS) {
        logger.debug("User stinkily reported at Y", data.y);
        throw Error.code(101, "invalid-argument");
    }

    // const reported = db.ref("reportedUsers");

    await db.ref(`reports`).push({
        badUserID: badUser.uid,
        badUsername: data.username,

        samaritanUsername: samaritanDetails.val.username,
        samaritanID: request.auth.uid,

        x: data.x,
        y: data.y,

        timestamp: Date.now(),
    });

    // await checkedTransaction(
    //     db.ref("reportedUsers").child(badUser.uid),
    //     () => true,
    //     () => 0,
    //     reportData => {
    //         if (reportData == undefined) {
    //             return {
    //                 username: data.username,
    //                 timestamp: Date.now(),
    //                 count: 1,
    //                 avg_x: data.x,
    //                 avg_y: data.y,
    //             };
    //         } else {
    //             return {
    //                 username: data.username,
    //                 timestamp: Date.now(),
    //                 count: reportData.count + 1,
    //                 avg_x:
    //                     (reportData.avg_x * reportData.count + data.x) /
    //                     (reportData.count + 1),
    //                 avg_y:
    //                     (reportData.avg_y * reportData.count + data.y) /
    //                     (reportData.count + 1),
    //             };
    //         }
    //     }
    // );

    logger.info(`Reported user ${data.username}`);

    return { cooldown: REPORT_COOLDOWN_SECONDS * 1000 };
});

// #region banUserInner
const banUserInner = async (
    db: ReturnType<typeof smartDatabase>,
    requesterUid: string,
    userToBanUid: string,
    reason: string,
    modUsername: string
) => {
    const isBanned = (
        await db.ref(`bannedUsers/${requesterUid}`).get()
    ).exists();
    if (isBanned) {
        throw Error.code(201, "permission-denied");
    }

    const userToBanDetailsRef = db.ref(`userDetails/${userToBanUid}`);
    const userToBanDetails = (await userToBanDetailsRef.get()).val();
    if (userToBanDetails == null) {
        throw Error.code(102, "invalid-argument");
    }

    if (!DEV_UIDS.includes(requesterUid)) {
        const reportedUserIsMod = (
            await userToBanDetailsRef.child("moderator").get()
        ).val();

        if (reportedUserIsMod) {
            throw Error.code(206, "permission-denied");
        }
    }

    await Promise.all([
        db.ref(`bannedUsers/${userToBanUid}`).set({
            username: userToBanDetails.username.toLowerCase(),
            modName: modUsername,
            reason,
        }),

        db.ref("userCount").transaction(count => {
            return count - 1;
        }),
        // db.ref(`reports`).
        // db.ref(`reportedUsers/${userToBanUid}`).remove(),
    ]);
};

// #region clearReports
// projects/gd-place-2023/topics/clearOldReports
export const clearReports = onMessagePublished("clearOldReports", _ => {
    const now = Date.now();
    const db = smartDatabase();

    db.ref("reports")
        .orderByChild("timestamp")
        .endAt(now - 15 * 60 * 1000)
        .get()
        .then(snapshot => {
            snapshot.forEach(child => {
                child.ref().remove();
            });
        });
});

// #region reportedUserOperation
export const reportedUserOperation = onCallAuth<ReportedUserOperationReq>(
    async request => {
        const db = smartDatabase();

        const modData = (
            await db.ref(`userDetails/${request.auth.uid}`).get()
        ).val();
        if (modData == null) {
            throw Error.code(103, "invalid-argument");
        }

        if (!modData.moderator) {
            throw Error.code(200, "permission-denied");
        }

        const data = request.data;

        if (data.operation == "ban") {
            await banUserInner(
                db,
                request.auth.uid,
                data.userUid,
                data.reason,
                modData.username
            );
        } else if (data.operation == "ignore") {
            await Promise.all(
                data.reportKeys.map(k => db.ref(`reports/${k}`).remove())
            );
        } else {
            throw Error.code(500, "aborted");
        }
    }
);

// #region banUser
export const banUser = onCallAuthLogger<BanReq>(
    "banUser",
    async (request, logger) => {
        const db = smartDatabase();

        const modData = (
            await db.ref(`userDetails/${request.auth.uid}`).get()
        ).val();
        if (modData == null) {
            throw Error.code(103, "invalid-argument");
        }

        if (!modData.moderator) {
            logger.debug("User is not mod");
            throw Error.code(200, "permission-denied");
        }

        const data = request.data;

        logger.info("Username:", data.username, data.username.toLowerCase());

        const userToBanUid = (
            await db.ref(`userName/${data.username.toLowerCase()}`).get()
        ).val()?.uid;

        if (userToBanUid == null) {
            throw Error.code(100, "invalid-argument");
        }

        logger.info("User id:", userToBanUid);

        await banUserInner(
            db,
            request.auth.uid,
            userToBanUid,
            data.reason,
            modData.username
        );
    }
);
