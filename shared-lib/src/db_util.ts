import { DatabaseSchema } from "./database";

export interface CommonDataSnapshot {
    child(path: string): CommonDataSnapshot;
    exists(): boolean;
    forEach(action: (a: CommonIteratedDataSnapshot) => boolean | void): boolean;
    readonly key: string | null;
    val(): any;
}
export interface CommonIteratedDataSnapshot extends CommonDataSnapshot {
    readonly key: string;
}

export interface CommonTransactionResult {
    committed: boolean;
    snapshot: CommonDataSnapshot;
}

export interface CommonReference {
    readonly key: string | null;
    push(value?: any): Promise<CommonReference>;
    remove(): Promise<void>;
    set(value: any): Promise<void>;
    get(): Promise<CommonDataSnapshot>;
    transaction(
        transactionUpdate: (a: any) => any
    ): Promise<CommonTransactionResult>;
    update(values: Object): Promise<void>;

    onValue(cb: (data: CommonDataSnapshot) => void): () => void;
    onChildAdded(cb: (data: CommonDataSnapshot) => void): () => void;
    onChildRemoved(cb: (data: CommonDataSnapshot) => void): () => void;
}

export interface CommonDatabase {
    ref(path?: string): CommonReference;
}

// ----------------------------------------------------------------------------

type Child<T> =
    Record<never, never> extends T ? NonNullable<T>[keyof T] : never;

type PathType<
    P extends string,
    T = DatabaseSchema,
    U extends boolean = false,
> = P extends `${infer A}/${infer B}`
    ? A extends keyof T
        ? PathType<
              B,
              T[A],
              U extends true
                  ? true
                  : Record<never, never> extends T
                    ? true
                    : false
          >
        : never
    : P extends keyof T
      ? U extends true
          ? T[P] | undefined
          : Record<never, never> extends T
            ? T[P] | undefined
            : T[P]
      : never;

export class DataSnapshot<T> {
    constructor(private data: CommonDataSnapshot) {}
    exists(): boolean {
        return this.data.exists();
    }
    forEach(action: (a: DataSnapshot<Child<T>>) => boolean | void): boolean {
        return this.data.forEach(d => action(new DataSnapshot(d)));
    }
    val(): T {
        return this.data.val();
    }
    get key(): string | null {
        return this.data.key;
    }
}

export class Ref<T> {
    constructor(private ref: CommonReference) {}

    get key() {
        return this.ref.key;
    }

    async get(): Promise<DataSnapshot<T>> {
        return new Promise((res, rej) => {
            this.ref
                .get()
                .then(d => res(new DataSnapshot(d)))
                .catch(e => rej(e));
        });
    }

    async set(value: T) {
        return this.ref.set(value);
    }

    async remove() {
        return this.ref.remove();
    }

    async push(value: Child<T>): Promise<Ref<Child<T>>> {
        return new Ref(await this.ref.push(value));
    }

    async transaction(cb: (value: T) => T) {
        return this.ref.transaction(cb);
    }

    onValue(cb: (data: DataSnapshot<T>) => void): () => void {
        return this.ref.onValue(v => cb(new DataSnapshot(v)));
    }
    onChildAdded(cb: (data: DataSnapshot<Child<T>>) => void): () => void {
        return this.ref.onChildAdded(v => cb(new DataSnapshot(v)));
    }
    onChildRemoved(cb: (data: DataSnapshot<Child<T>>) => void): () => void {
        return this.ref.onChildRemoved(v => cb(new DataSnapshot(v)));
    }
}

export const ref = <T extends string, D = DatabaseSchema>(
    db: CommonDatabase,
    path: T
): Ref<PathType<T, D>> => new Ref(db.ref(path));

type MapSnapshot<
    T extends string[],
    D,
    Out extends DataSnapshot<any>[] = [],
> = T extends []
    ? Out
    : T extends [infer A, ...infer B]
      ? A extends string
          ? B extends string[]
              ? MapSnapshot<B, D, [...Out, DataSnapshot<PathType<A, D>>]>
              : never
          : never
      : never;

export const refAllGet = async <T extends string[], D = DatabaseSchema>(
    db: CommonDatabase,
    ...paths: T
): Promise<MapSnapshot<T, D>> => {
    return Promise.all(paths.map(async p => await ref(db, p).get())) as any;
};
