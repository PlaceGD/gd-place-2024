export class Reader<T extends ArrayBufferView> {
    private view: DataView;
    private byteOffset: number;

    constructor(array: T, expectedSize: number) {
        if (array.byteLength != expectedSize) {
            throw new Error();
        }
        this.view = new DataView(array.buffer);
        this.byteOffset = 0;
    }

    readBool(): boolean {
        return this.readU8() == 1;
    }

    readI8(): number {
        let v = this.view.getInt8(this.byteOffset);
        this.byteOffset += 1;
        return v;
    }

    readU8(): number {
        let v = this.view.getUint8(this.byteOffset);
        this.byteOffset += 1;
        return v;
    }

    readU16(): number {
        let v = this.view.getUint16(this.byteOffset, true);
        this.byteOffset += 2;
        return v;
    }

    readF32(): number {
        let v = this.view.getFloat32(this.byteOffset, true);
        this.byteOffset += 4;
        return v;
    }
}
