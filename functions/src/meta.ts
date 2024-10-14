import { DEV_UIDS, MetaReq } from "shared-lib/cloud_functions";
import Error from "./utils/errors";
import { onCallAuth } from "./utils/on_call";
import { smartDatabase } from "src";

export const setMeta = onCallAuth<MetaReq>(async request => {
    if (!DEV_UIDS.includes(request.auth.uid)) {
        throw Error.code(200, "permission-denied");
    }
    const data = request.data;
    const db = smartDatabase();

    switch (data.op.type) {
        case "place_timer":
            db.ref("metaVariables/placeCooldown").set(data.op.to);
            break;
        case "delete_timer":
            db.ref("metaVariables/deleteCooldown").set(data.op.to);
            break;
        case "event_start":
            db.ref("metaVariables/eventStartTime").set(data.op.to);
            break;
        case "event_end":
            db.ref("metaVariables/eventEndTime").set(data.op.to);
            break;
        case "postpone_start":
            db.ref("metaVariables/eventStartTime").transaction((time: any) => {
                return parseInt(time) + data.op.secs * 1000;
            });
            break;
        case "postpone_end":
            db.ref("metaVariables/eventEndTime").transaction((time: any) => {
                return parseInt(time) + data.op.secs * 1000;
            });
            break;
        case "change_mod_status":
            let userID1 = (
                await db.ref(`userName/${data.op.user.toLowerCase()}/uid`).get()
            ).val();
            if (userID1 == undefined) {
                return;
            }

            db.ref(`userDetails/${userID1}/moderator`).set(data.op.to == "mod");
            break;
        case "unban":
            let userID2 = (
                await db.ref(`userName/${data.op.user.toLowerCase()}/uid`).get()
            ).val();
            if (userID2 == undefined) {
                return;
            }

            db.ref(`bannedUsers/${userID2}`).remove();
            db.ref("userCount").transaction(count => {
                return count + 1;
            });
            break;

        case "announcement":
            db.ref("announcement").set({
                text: data.op.text,
                time: Date.now(),
            });
            break;
        case "clear_announcement":
            db.ref("announcement").remove();
            break;
        case "log_donation":
            db.ref(`userDetails/${data.op.uid}/hasDonated`).set(true);
            break;
        default:
            throw Error.code(500, "aborted");
    }
});
