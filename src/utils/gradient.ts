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

const gradients = [
    ["#F9C823", "#FAA03C", "#FB7855", "#FC506E"],
    ["#1ED7B5", "#64E2B0", "#AAEEAC", "#F0F9A7"],
    ["#F8F9C7", "#F6E6D3", "#F4D4DE", "#F2C1EA"],
    ["#0E1C26", "#172A32", "#21373F", "#2A454B"],
    ["#2278FB", "#3A9AF0", "#53BDE6", "#6BDFDB"],
    ["#DFF2CB", "#E5D79B", "#EABC6A", "#F0A13A"],
    ["#ffadad", "#ffd6a5", "#fdffb6", "#caffbf", "#9bf6ff"],
    ["#6b9080", "#cce3de", "#f6fff8"],
    ["#7bdff2", "#b2f7ef", "#eff7f6", "#f7d6e0", "#f2b5d4"],
    ["#001427", "#708d81", "#f4d58d", "#bf0603", "#8d0801"],
    ["#edafb8", "#f7e1d7", "#dedbd2", "#b0c4b1", "#4a5759"],
];

export const getRandomGradientColors = (): string[] => {
    return gradients[Math.floor(Math.random() * gradients.length)];
};
