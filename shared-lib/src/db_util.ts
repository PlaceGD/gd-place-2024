import {
    CommonDatabase,
    CommonDataSnapshot,
    CommonReference,
} from "@firebase-parity/db-interfaces";
import { DatabaseSchema } from "./database";

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

export class SmartDataSnapshot<T> {
    constructor(private data: CommonDataSnapshot) {}
    exists(): boolean {
        return this.data.exists();
    }
    forEach(
        action: (a: SmartDataSnapshot<Child<T>>) => boolean | void
    ): boolean {
        return this.data.forEach(d => action(new SmartDataSnapshot(d)));
    }
    val(): T {
        return this.data.val();
    }
}

export class SmartReference<T> {
    constructor(private ref: CommonReference) {}

    get key() {
        return this.ref.key;
    }

    async get(): Promise<SmartDataSnapshot<T>> {
        return new Promise((res, rej) => {
            this.ref
                .get()
                .then(d => res(new SmartDataSnapshot(d)))
                .catch(e => rej(e));
        });
    }

    async set(value: T) {
        return this.ref.set(value);
    }

    async remove() {
        return this.ref.remove();
    }

    async push(value: Child<T>): Promise<SmartReference<Child<T>>> {
        return new SmartReference(await this.ref.push(value));
    }

    async transaction(cb: (value: T) => T) {
        return this.ref.transaction(cb);
    }

    onValue(cb: (s: SmartDataSnapshot<T>) => void) {
        return this.ref.on("value", s => cb(new SmartDataSnapshot(s)));
    }
    onChildAdded(cb: (s: SmartDataSnapshot<Child<T>>) => void) {
        return this.ref.on("child_added", s => cb(new SmartDataSnapshot(s)));
    }
    onChildRemoved(cb: (s: SmartDataSnapshot<Child<T>>) => void) {
        return this.ref.on("child_removed", s => cb(new SmartDataSnapshot(s)));
    }
    onChildChanged(cb: (s: SmartDataSnapshot<Child<T>>) => void) {
        return this.ref.on("child_changed", s => cb(new SmartDataSnapshot(s)));
    }
}

type MapSnapshot<
    T extends string[],
    D,
    Out extends SmartDataSnapshot<any>[] = [],
> = T extends []
    ? Out
    : T extends [infer A, ...infer B]
      ? A extends string
          ? B extends string[]
              ? MapSnapshot<B, D, [...Out, SmartDataSnapshot<PathType<A, D>>]>
              : never
          : never
      : never;

export class SmartDatabase {
    constructor(private db: CommonDatabase) {}

    ref<T extends string, D = DatabaseSchema>(
        path: T
    ): SmartReference<PathType<T, D>> {
        return new SmartReference(this.db.ref(path));
    }
    async refAllGet<T extends string[], D = DatabaseSchema>(
        ...paths: T
    ): Promise<MapSnapshot<T, D>> {
        return Promise.all(
            paths.map(async p => await this.db.ref(p).get())
        ) as any;
    }
}
