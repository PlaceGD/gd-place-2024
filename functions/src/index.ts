import { initializeApp } from "firebase-admin/app";
import { convertDatabase } from "@smart-firebase/admin";
import { DatabaseSchema } from "shared-lib/database";
import pkg from "firebase-admin";
const { database } = pkg;

export { deleteObject, placeObject } from "./object";
export {
    initUserWithUsername,
    reportUser,
    reportedUserOperation,
    banUser,
    clearReports,
} from "./user";

export {
    onKofiDonation,
    submitKofiTxId,
    changeNameGradient,
} from "./donations";

export { setMeta } from "./meta";

export { setLevelNameLetter } from "./ending";

export {
    getPlaceCooldown,
    getDeleteCooldown,
    getReportCooldown,
    getGradientCooldown,
} from "./cooldown";

export { getServerTime } from "./status_scheduler";

export const smartDatabase = () => convertDatabase<DatabaseSchema>(database());

initializeApp();
