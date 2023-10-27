export default class LocalSettings<T extends { [key: string]: any }> {
    // i wish this could depend on `T`
    [key: string]: any
    readonly #id: string
    #value: T

    constructor(id: string, initial: T) {
        this.#id = id

        const ls = localStorage.getItem(id)

        if (ls) {
            this.#value = JSON.parse(ls) as T
        } else {
            this.#value = initial
        }

        return new Proxy<LocalSettings<T>>(this, this)
    }

    get(target: this, prop: string): any | undefined {
        return this.#value[prop]
    }

    set(target: this, prop: string, value: any, _reciever: this): boolean {
        if (prop in this.#value) {
            ;(this.#value as any)[prop] = value
            localStorage.setItem(this.#id, JSON.stringify(this.#value))
        } else {
            return false
        }

        return true
    }
}
