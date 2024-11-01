export const getFixedSize = (
    mSprite: { size: [number, number]; offset: [number, number] } | null,
    dSprite: { size: [number, number]; offset: [number, number] } | null
): [number, number] => {
    let width =
        Math.max(
            (mSprite == null ? 0 : mSprite.size[0]) +
                Math.abs(mSprite == null ? 0 : mSprite.offset[0]) * 2,
            (dSprite == null ? 0 : dSprite.size[0]) +
                Math.abs(dSprite == null ? 0 : dSprite.offset[0]) * 2
        ) + 2;
    let height =
        Math.max(
            (mSprite == null ? 0 : mSprite.size[1]) +
                Math.abs(mSprite == null ? 0 : mSprite.offset[1]) * 2,
            (dSprite == null ? 0 : dSprite.size[1]) +
                Math.abs(dSprite == null ? 0 : dSprite.offset[1]) * 2
        ) + 2;
    return [width, height];
};
