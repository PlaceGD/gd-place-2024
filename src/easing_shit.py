factors = [1.0, 0.89, 0.56, 0.02, 0.25, 0.02, 0.06, 0.01, 0.02, 0.0]
percentages = [0, 12, 24, 36, 54, 74, 82, 92, 96, 100]
def lerp(a, b, t):
    return a + (b - a) * t


def keyframes(a, b):

    for (p, t) in zip(percentages, factors):
        print(f"""{p}% {{
    transform: scale({lerp(a, b, 1 - t)});
}}""")

keyframes(1, 1.2)