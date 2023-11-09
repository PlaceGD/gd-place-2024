import { toast as _toast } from "@zerodevx/svelte-toast";

const toastSuccessTheme = {
    theme: {
        "--toastColor": "mintcream",
        "--toastBackground": "rgba(72, 187, 120, 0.9)",
        "--toastBarBackground": "#2F855A",
    },
};
const toastErrorTheme = {
    theme: {
        "--toastColor": "mintcream",
        "--toastBackground": "rgba(187, 72, 72, 0.9)",
        "--toastBarBackground": "#852F2F",
    },
};

export const toast = {
    showErrorToast: (...message: string[]) => {
        console.error(message);
        _toast.push(message.join(" "), toastErrorTheme);
    },
    showSuccessToast: (...message: string[]) => {
        console.info(message);
        _toast.push(message.join(" "), toastSuccessTheme);
    },
};

export { toastSuccessTheme, toastErrorTheme };
