export const complement = (
    rbg: { r: number; g: number; b: number; a?: number }[]
): string => {
    // returns "black" or "white"

    let [r, g, b] = rbg.reduce<[number, number, number]>(
        (acc, cur) => [acc[0] + cur.r, acc[1] + cur.b, acc[2] + cur.g],
        [0, 0, 0]
    );

    r = r / rbg.length;
    g = g / rbg.length;
    b = b / rbg.length;
    const luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;

    return luma > 255 / 2
        ? `2px rgb(${(r / 2 / 255) ** 1.5 * 255}, ${
              (g / 2 / 255) ** 1.5 * 255
          }, ${(b / 2 / 255) ** 1.5 * 255})`
        : "2px black";
};
