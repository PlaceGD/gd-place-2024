import { writable } from "svelte/store";
import LocalSettingsFactory from "../utils/local_settings";
import { reportUser as reportUserCloud } from "./cloud_functions";
import Toast from "../utils/toast";

export const reportedUsers = writable(
    LocalSettingsFactory<{ reported: string[] }>("reported", {
        reported: [],
    })
);

export const reportUser = (name: string) => {
    reportedUsers.update(v => {
        if (!v.reported.includes(name)) {
            reportUserCloud({ username: name }).catch(e => {
                Toast.showErrorToast(`Failed to report user. (${e})`);
            });

            v.reported = [...v.reported, name];
        }
        return v;
    });
};
