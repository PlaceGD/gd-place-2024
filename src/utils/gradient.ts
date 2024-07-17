export const complement = (colors: string[]): string => {
    // returns "black" or "white"

    const rbg = colors.map(color => {
        const colorN = parseInt(color.replace("#", ""), 16);
        return [(colorN >> 16) & 255, (colorN >> 8) & 255, colorN & 255];
    });

    let [r, g, b] = rbg.reduce<[number, number, number]>(
        (acc, cur) => [acc[0] + cur[0], acc[1] + cur[1], acc[2] + cur[2]],
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
