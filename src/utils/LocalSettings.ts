class LocalSettings<T extends Record<string, any>> {
    private readonly id: string;
    private value: T;

    constructor(id: string, initial: T) {
        this.id = id;

        const ls = localStorage.getItem(id);

        if (ls) {
            this.value = JSON.parse(ls) as T;
        } else {
            this.value = initial;
        }

        return new Proxy<LocalSettings<T>>(this, this);
    }

    get(_: this, prop: string): any | undefined {
        return this.value[prop];
    }

    set(_: this, prop: string, value: any, _reciever: this): boolean {
        if (prop in this.value) {
            (this.value as any)[prop] = value;
            localStorage.setItem(this.id, JSON.stringify(this.value));
        } else {
            localStorage.setItem("didErrorOccur", "1");
            window.location.reload();
        }

        return true;
    }
}

type ExtendedProperties<T> = { [P in keyof T]: T[P] };

export default function LocalSettingsFactory<T extends Record<string, any>>(
    id: string,
    value: T
): LocalSettings<T> & ExtendedProperties<T> {
    return new LocalSettings(id, value) as LocalSettings<T> &
        ExtendedProperties<T>;
}
