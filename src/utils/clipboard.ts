export const setClipboard = async (text: string) => {
    await navigator.clipboard.writeText(text);
};
