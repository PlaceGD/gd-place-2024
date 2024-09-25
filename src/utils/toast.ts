import { toast as _toast } from "@zerodevx/svelte-toast";
import { writable } from "svelte/store";

export const toastPortals = writable<HTMLElement[]>([]);

export default class Toast {
    static showErrorToast = (...message: string[]) => {
        console.error(...message);
        _toast.push(message.join(" "), { classes: ["error"] });
    };
    static showWarningToast = (...message: string[]) => {
        console.warn(...message);
        _toast.push(message.join(" "), { classes: ["warning"] });
    };
    static showSuccessToast = (...message: string[]) => {
        console.log(...message);
        _toast.push(message.join(" "), { classes: ["success"] });
    };
    static showInfoToast = (...message: string[]) => {
        console.info(...message);
        _toast.push(message.join(" "), { classes: ["info"], initial: 0 });
    };

    static showAnnouncementToast = (
        message: string,
        onClose: () => void = () => {}
    ) => {
        _toast.push(message, {
            target: "announcement",
            initial: 0,
            onpop: onClose,
        });
    };
}
