import { LevelNameReq } from "shared-lib/cloud_functions";
import { onCallAuth } from "./utils/on_call";
import { smartDatabase } from "src";
import {
    checkedTransaction,
    getCheckedUserDetails,
    refAllGet,
} from "./utils/utils";
import Error from "./utils/errors";

import {
    VALID_LEVEL_NAME,
    TOTAL_ENDING_INPUTS,
    CHARACTER_COOLDOWN_SECONDS,
} from "shared-lib/ending";

export const setLevelNameLetter = onCallAuth<LevelNameReq>(async request => {
    const db = smartDatabase();
    const data = request.data;
    const authUID = request.auth.uid;

    const userDetails = await getCheckedUserDetails(db, authUID);

    if (data.letter.length > 1 || !VALID_LEVEL_NAME.test(data.letter)) {
        throw Error.code(108, "invalid-argument");
    }

    if (data.index < 0 || data.index > TOTAL_ENDING_INPUTS) {
        throw Error.code(109, "invalid-argument");
    }

    const [eventEndTime, setNameTime] = await refAllGet(
        db,
        "metaVariables/eventEndTime",
        "metaVariables/setNameTime"
    );

    const now = Date.now();

    if (
        now < eventEndTime.val() ||
        now > eventEndTime.val() + setNameTime.val() * 1000
    ) {
        Error.code(211, "permission-denied");
    }

    await checkedTransaction(
        userDetails.ref.child("lastCharacterTimestamp"),
        lastCharacter =>
            now >= (lastCharacter ?? 0) + CHARACTER_COOLDOWN_SECONDS * 1000,
        () => Error.code(202, "permission-denied"),
        () => now
    );

    if (
        (
            await db.ref("levelName/inputs").transaction(v => {
                let v0 = v ?? {};
                v0[data.index] = data.letter;
                return v0;
            })
        ).committed
    ) {
        db.ref("levelName/history").push(data);
    }

    // db.ref(`/levelName/inputs/${data.index}`).set(data.letter);
});
