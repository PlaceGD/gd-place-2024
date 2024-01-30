import {
    Database,
    Reference,
    DataSnapshot as FDataSnapshot,
} from "firebase-admin/database";
import { DatabaseSchema } from "shared-lib";

type Child<T> = Record<any, any> extends T ? NonNullable<T>[keyof T] : never;

type PathType<
    P extends string,
    T = DatabaseSchema,
    U extends boolean = false,
> = P extends `${infer A}/${infer B}`
    ? A extends keyof T
        ? PathType<
              B,
              T[A],
              U extends true ? true : Record<any, any> extends T ? true : false
          >
        : never
    : P extends keyof T
      ? U extends true
          ? T[P] | undefined
          : Record<any, any> extends T
            ? T[P] | undefined
            : T[P]
      : never;

class DataSnapshot<T> {
    constructor(private data: FDataSnapshot) {}
    exists(): boolean {
        return this.data.exists();
    }
    forEach(action: (a: DataSnapshot<Child<T>>) => boolean | void): boolean {
        return this.data.forEach(d => action(new DataSnapshot(d)));
    }
    val(): T {
        return this.data.val();
    }
}

class Ref<T> {
    constructor(private ref: Reference) {}

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
        return new Ref(this.ref.push(value));
    }

    async transaction(cb: (value: T) => T) {
        return this.ref.transaction(cb);
    }
}

// type F<X, Y> = never;

// type B = F<...[string, number]>

export const ref = <T extends string, D = DatabaseSchema>(
    db: Database,
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
    db: Database,
    ...paths: T
): Promise<MapSnapshot<T, D>> => {
    return Promise.all(paths.map(async p => await ref(db, p).get())) as any;
};
