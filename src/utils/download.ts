type RequestTypeMap = {
    arraybuffer: ArrayBuffer;
    blob: Blob;
    document: Document;
    json: JSON;
    text: string;
};

type Event = ProgressEvent<XMLHttpRequestEventTarget>;

export const downloadWithProgress = async <T extends keyof RequestTypeMap>(
    url: string,
    responseMime: T,
    progress: (e: Event) => void
): Promise<RequestTypeMap[T]> => {
    return new Promise((res, rej) => {
        const request = new XMLHttpRequest();

        request.responseType = responseMime;
        request.addEventListener("progress", progress);

        request.addEventListener("load", () => {
            res(request.response as RequestTypeMap[T]);
        });
        request.addEventListener("error", e => {
            rej(e);
        });
        request.addEventListener("timeout", e => {
            rej(e);
        });

        request.open("GET", url);
        request.send();
    });
};
