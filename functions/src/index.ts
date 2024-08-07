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
    // clearReports,
} from "./user";

export {
    onKofiDonation,
    submitKofiTxId,
    changeNameGradient,
} from "./donations";

// TODO: add our uids
export const DEV_UIDS: string[] = [];

/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
export const CHUNK_SIZE_BLOCKS = 20;
export const CHUNK_SIZE_UNITS = CHUNK_SIZE_BLOCKS * 30;

/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
export const LEVEL_WIDTH_BLOCKS = 800;
export const LEVEL_HEIGHT_BLOCKS = 800;
export const LEVEL_WIDTH_UNITS = LEVEL_WIDTH_BLOCKS * 30;
export const LEVEL_HEIGHT_UNITS = LEVEL_HEIGHT_BLOCKS * 30;

export const smartDatabase = () => convertDatabase<DatabaseSchema>(database());

initializeApp();
