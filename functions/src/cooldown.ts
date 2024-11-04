import { smartDatabase } from "src";
import { onCallAuth } from "./utils/on_call";
import {
    GRADIENT_COOLDOWN_SECONDS,
    REPORT_COOLDOWN_SECONDS,
} from "shared-lib/user";
import { CHARACTER_COOLDOWN_SECONDS } from "shared-lib/ending";

export const getPlaceCooldown = onCallAuth<{}, Promise<number>>(
    async request => {
        const db = smartDatabase();

        const placeCooldown =
            (await db.ref("metaVariables/placeCooldown").get()).val() * 1000;

        const userLastPlaced =
            (
                await db
                    .ref(`userDetails/${request.auth.uid}/lastPlaceTimestamp`)
                    .get()
            ).val() ?? 0;

        return placeCooldown - (Date.now() - userLastPlaced);
    }
);

export const getDeleteCooldown = onCallAuth<{}, Promise<number>>(
    async request => {
        const db = smartDatabase();

        const deleteCooldown =
            (await db.ref("metaVariables/deleteCooldown").get()).val() * 1000;

        const userLastDeleted =
            (
                await db
                    .ref(`userDetails/${request.auth.uid}/lastDeleteTimestamp`)
                    .get()
            ).val() ?? 0;

        return deleteCooldown - (Date.now() - userLastDeleted);
    }
);

export const getReportCooldown = onCallAuth<{}, Promise<number>>(
    async request => {
        const db = smartDatabase();

        const userLastReported =
            (
                await db
                    .ref(`userDetails/${request.auth.uid}/lastReportTimestamp`)
                    .get()
            ).val() ?? 0;

        return REPORT_COOLDOWN_SECONDS * 1000 - (Date.now() - userLastReported);
    }
);

export const getGradientCooldown = onCallAuth<{}, Promise<number>>(
    async request => {
        const db = smartDatabase();

        const userLastGradient =
            (
                await db
                    .ref(
                        `userDetails/${request.auth.uid}/lastGradientTimestamp`
                    )
                    .get()
            ).val() ?? 0;

        return (
            GRADIENT_COOLDOWN_SECONDS * 1000 - (Date.now() - userLastGradient)
        );
    }
);

export const getCharacterCooldown = onCallAuth<{}, Promise<number>>(
    async request => {
        const db = smartDatabase();

        const userLastCharacter =
            (
                await db
                    .ref(
                        `userDetails/${request.auth.uid}/lastCharacterTimestamp`
                    )
                    .get()
            ).val() ?? 0;

        return (
            CHARACTER_COOLDOWN_SECONDS * 1000 - (Date.now() - userLastCharacter)
        );
    }
);
