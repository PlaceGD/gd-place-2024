import {
    push as firebasePush,
    remove as firebaseRemove,
    set as firebaseSet,
    get as firebaseGet,
    runTransaction,
    update as firebaseUpdate,
    onValue as firebaseOnValue,
    onChildAdded as firebaseOnChildAdded,
    onChildRemoved as firebaseOnChildRemoved,
    ref as firebaseRef,
    type Database,
    type DatabaseReference,
    type DataSnapshot,
    type IteratedDataSnapshot,
} from "firebase/database";
import type {
    CommonDatabase,
    CommonDataSnapshot,
    CommonIteratedDataSnapshot,
    CommonReference,
    CommonTransactionResult,
} from "shared-lib/db_util";

const convertIteratedDataSnapshot = (
    snapshot: IteratedDataSnapshot
): CommonIteratedDataSnapshot => {
    return {
        ...convertDataSnapshot(snapshot),
        key: snapshot.key,
    };
};
const convertDataSnapshot = (snapshot: DataSnapshot): CommonDataSnapshot => {
    return {
        child(path: string): CommonDataSnapshot {
            return convertDataSnapshot(snapshot.child(path));
        },
        exists(): boolean {
            return snapshot.exists();
        },
        forEach(
            action: (a: CommonIteratedDataSnapshot) => boolean | void
        ): boolean {
            return snapshot.forEach(c =>
                action(convertIteratedDataSnapshot(c))
            );
        },
        key: snapshot.key,
        val(): any {
            return snapshot.val();
        },
    };
};
const convertReference = (ref: DatabaseReference): CommonReference => {
    return {
        key: ref.key,
        async push(value?: any): Promise<CommonReference> {
            return convertReference(await firebasePush(ref, value));
        },
        remove(): Promise<void> {
            return firebaseRemove(ref);
        },
        set(value: any): Promise<void> {
            return firebaseSet(ref, value);
        },
        async get(): Promise<CommonDataSnapshot> {
            return convertDataSnapshot(await firebaseGet(ref));
        },
        async transaction(
            transactionUpdate: (a: any) => any
        ): Promise<CommonTransactionResult> {
            const v = await runTransaction(ref, transactionUpdate);
            return {
                committed: v.committed,
                snapshot: convertDataSnapshot(v.snapshot),
            };
        },
        update(values: Object): Promise<void> {
            return firebaseUpdate(ref, values);
        },

        onValue(cb: (data: CommonDataSnapshot) => void): () => void {
            return firebaseOnValue(ref, cb);
        },
        onChildAdded(cb: (data: CommonDataSnapshot) => void): () => void {
            return firebaseOnChildAdded(ref, cb);
        },
        onChildRemoved(cb: (data: CommonDataSnapshot) => void): () => void {
            return firebaseOnChildRemoved(ref, cb);
        },
    };
};

export const convertDatabase = (db: Database): CommonDatabase => {
    return {
        ref(path?: string): CommonReference {
            return convertReference(firebaseRef(db, path));
        },
    };
};
