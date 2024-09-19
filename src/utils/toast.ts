import { toast as _toast } from "@zerodevx/svelte-toast";

const toastThemes = {
    SUCCESS: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "rgba(68, 150, 57, 0.7)",
            "--toastBarBackground": "#6ab55b",
            "--toastBorderRadius": "0.5rem",
            "--toastPadding": "8px",
        },
        duration: 10000,
    },
    ERROR: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "rgba(150, 57, 57, 0.7)",
            "--toastBarBackground": "#b55f5b",
            "--toastBorderRadius": "0.5rem",
            "--toastPadding": "8px",
        },
        duration: 10000,
    },
    WARN: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "rgba(150, 110, 57, 0.7)",
            "--toastBarBackground": "#b58d5b",
            "--toastBorderRadius": "0.5rem",
            "--toastPadding": "8px",
        },
        duration: 10000,
    },
    INFO: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "rgba(57, 97, 150, 0.7)",
            "--toastBarBackground": "#5b82b5",
            "--toastBorderRadius": "0.5rem",
            "--toastPadding": "8px",
        },
        duration: 10000,
    },
    ANNOUNCEMENT: {
        theme: {
            "--toastColor": "#FFF",
            "--toastBackground": "rgba(45, 89, 123, 0.7)",
            "--toastBarBackground": "#234863",
            "--toastBorderRadius": "0.5rem",
            "--toastPadding": "8px",
        },
        initial: 0,
        target: "announcement",
        duration: 10000,
    },
};

export default class Toast {
    static showErrorToast = (...message: string[]) => {
        console.error(...message);
        _toast.push(message.join(" "), toastThemes.ERROR);
    };
    static showWarningToast = (...message: string[]) => {
        console.warn(...message);
        _toast.push(message.join(" "), toastThemes.WARN);
    };
    static showSuccessToast = (...message: string[]) => {
        console.log(...message);
        _toast.push(message.join(" "), toastThemes.SUCCESS);
    };
    static showInfoToast = (...message: string[]) => {
        console.info(...message);
        _toast.push(message.join(" "), toastThemes.INFO);
    };

    static showAnnouncementToast = (...message: string[]) => {
        console.info(...message);
        _toast.push(message.join(" "), toastThemes.ANNOUNCEMENT);
    };
}
