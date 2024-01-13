import { toast as _toast } from "@zerodevx/svelte-toast";

const toastThemes = {
    SUCCESS: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "#48BB78",
            "--toastBarBackground": "#215E40",
        },
    },
    ERROR: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "#BB4848",
            "--toastBarBackground": "#852F2F",
        },
    },
    INFO: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "#2D597B",
            "--toastBarBackground": "#234863",
        },
    },
};

export default class Toast {
    static showErrorToast = (...message: string[]) => {
        console.error(...message);
        _toast.push(message.join(" "), toastThemes.ERROR);
    };
    static showSuccessToast = (...message: string[]) => {
        console.log(...message);
        _toast.push(message.join(" "), toastThemes.SUCCESS);
    };
    static showInfoToast = (...message: string[]) => {
        console.info(...message);
        _toast.push(message.join(" "), toastThemes.INFO);
    };
}
