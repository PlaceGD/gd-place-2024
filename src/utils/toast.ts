import { toast as _toast } from "@zerodevx/svelte-toast";
import { writable } from "svelte/store";

export const toastPortals = writable<HTMLElement[]>([]);

export const WASM_ERROR = `
<strong>A fatal error occured in the WASM. <span style="color:white;text-decoration:underline;cursor:pointer;pointer-events:all;" onclick='localStorage.removeItem("wasmVer");indexedDB.deleteDatabase("PlaceDB");window.location.reload();'>
Click here to refresh it.</span></strong>
<br/>
<span style="color:white;text-decoration:underline;cursor:pointer;pointer-events:all;" onclick='navigator.clipboard.writeText(window.consoleErrors.join("\\n"));'>
<i>(click here to copy the error)</i>
</span>`;

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
