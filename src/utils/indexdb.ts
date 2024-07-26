export class PlaceDB {
    private constructor(private db: IDBDatabase) {}

    static async open(): Promise<PlaceDB> {
        return new Promise((res, rej) => {
            const request = indexedDB.open("PlaceDB", 1);

            request.onerror = e => {
                rej(e);
            };

            request.onupgradeneeded = () => {
                const db = request.result;

                db.createObjectStore("spriteCache", {
                    keyPath: "id",
                    autoIncrement: false,
                });

                db.createObjectStore("randomCache", {
                    autoIncrement: false,
                });
            };

            request.onsuccess = () => {
                const db = request.result;

                res(new PlaceDB(db));
            };
        });
    }

    async putSpriteCache(id: number, data: Blob): Promise<void> {
        return new Promise((res, rej) => {
            const tx = this.db.transaction("spriteCache", "readwrite");

            const spriteCache = tx.objectStore("spriteCache");

            const put = spriteCache.put({ id, data });

            put.onsuccess = () => res();
            put.onerror = e => rej(e);
        });
    }
    async getSpriteCache(id: number): Promise<Blob | null> {
        return new Promise((res, rej) => {
            const tx = this.db.transaction("spriteCache", "readwrite");

            const spriteCache = tx.objectStore("spriteCache");
            const query = spriteCache.get(id);

            query.onsuccess = () => res(query.result?.data);
            query.onerror = e => rej(e);
        });
    }

    async putWasmCache(data: ArrayBuffer): Promise<void> {
        return new Promise((res, rej) => {
            const tx = this.db.transaction("randomCache", "readwrite");

            const randomCache = tx.objectStore("randomCache");

            const put = randomCache.put(data, "wasm");

            put.onsuccess = () => res();
            put.onerror = e => rej(e);
        });
    }
    async getWasmCache(): Promise<ArrayBuffer | null> {
        return new Promise((res, rej) => {
            const tx = this.db.transaction("randomCache", "readwrite");

            const randomCache = tx.objectStore("randomCache");
            const query = randomCache.get("wasm");

            query.onsuccess = () => res(query.result);
            query.onerror = e => rej(e);
        });
    }
}
