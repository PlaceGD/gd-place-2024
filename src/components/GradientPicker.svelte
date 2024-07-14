<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";
    import ColorPicker from "svelte-awesome-color-picker";
    import ColorPickerWrapper from "./ColorPickerWrapper.svelte";
    import Input from "./Input.svelte";
    import Button from "./Button.svelte";

    function random(min: number, max: number) {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    export let maxStops: number = 10;

    export let gradientString = "";

    let clickedNub = false;

    let rawValues: number[] = new Array(maxStops)
        .fill(0)
        .map((_, i) => (i / (maxStops - 1)) * 100);

    export let pickerValues = rawValues.map(() => ({
        r: random(0, 255),
        g: random(0, 255),
        b: random(0, 255),
        a: 1,
    }));

    let currentStops = rawValues.map((v, i) => ({
        position: v,
        rgb: pickerValues[i],
    }));

    let userInput = new Array(maxStops)
        .fill("")
        .map((_, i) => `${Math.round(rawValues[i])}`);

    let gradientAngle = 90;
    let gradientAngleString = gradientAngle.toString();
    $: gradientAngleString = Math.round(gradientAngle).toString();

    let isRotating: { rect: DOMRect } | null = null;

    $: {
        userInput.forEach((v, i) => {
            let vn = parseInt(v);

            vn = isNaN(vn) ? 0 : vn;

            rawValues[i] = vn;
            currentStops[i] = {
                ...currentStops[i],
                position: vn,
            };
        });

        currentStops = currentStops.sort((a, b) => a.position - b.position);

        gradientString = `linear-gradient(${gradientAngle}deg, ${currentStops.map(c => `rgba(${c.rgb?.r ?? 0},${c.rgb?.g ?? 0},${c.rgb?.b ?? 0},1) ${c.position ?? 0}%`)})`;
    }

    const handlePointerDown = (
        e: PointerEvent & { currentTarget: EventTarget & HTMLDivElement }
    ) => {
        if ((e.target as HTMLElement | null)?.classList.contains("rangeNub")) {
            clickedNub = true;
        }
    };
</script>

<svelte:window
    on:pointerup={() => {
        if (isRotating != null) {
            isRotating = null;
        }
    }}
    on:pointermove={e => {
        if (isRotating == null) return;

        const rect = isRotating.rect;

        // Calculate the center of the circle
        const centerX = rect.left + rect.width / 2;
        const centerY = rect.top + rect.height / 2;

        // Get the mouse position relative to the document
        const mouseX = e.clientX;
        const mouseY = e.clientY;

        const deltaX = mouseX - centerX;
        const deltaY = mouseY - centerY;

        let angle = (Math.atan2(deltaY, deltaX) * 180) / Math.PI;

        if (angle < 0) {
            angle += 360;
        }

        gradientAngle = angle;
    }}
/>

<div class="grid w-full h-full pointer-events-auto">
    <div
        class="h-16 min-h-0 cursor-copy"
        style={`--bg: ${gradientString}`}
        on:pointerdown={handlePointerDown}
        on:pointerup={e => {
            if (clickedNub) {
                clickedNub = false;
                return;
            }

            console.log("ADD STOP", e);
        }}
    >
        <RangeSlider
            bind:values={rawValues}
            id="gradient-slider"
            springValues={{ stiffness: 1, damping: 1 }}
            step={1}
            min={0}
            max={100}
            float
            hoverable={false}
        ></RangeSlider>
    </div>
    <div class="absolute dark left-1/4">
        <div id="color-picker-portal" />
    </div>

    <div class="flex w-full min-h-0 mt-16 overflow-hidden">
        <div class="flex flex-col items-center justify-center gap-2 pr-4">
            <div
                class="relative flex flex-center"
                on:pointerdown={e => {
                    isRotating = {
                        rect: e.currentTarget.getBoundingClientRect(),
                    };
                }}
            >
                <div
                    class="box-content flex w-10 border-2 border-white rounded-full cursor-pointer aspect-square flex-center"
                >
                    <div
                        class="relative w-2 h-full"
                        style:transform={`rotate(${gradientAngle}deg)`}
                    >
                        <span
                            class="absolute w-2 bg-white rounded-full top-2 aspect-square"
                        ></span>
                    </div>
                </div>
                <input
                    type="range"
                    min={0}
                    max={360}
                    aria-label="Gradient Angle"
                    class="absolute sr-only"
                    bind:value={gradientAngle}
                />
            </div>
            <Input
                maxLength={3}
                bind:value={gradientAngleString}
                class="p-2 text-center rounded-lg outline-none w-14 text-stroke bg-black/40 outline-2 outline outline-white/20 -outline-offset-2"
            />
        </div>
        <ul class="overflow-scroll rounded-lg alternating-bg stop-list">
            {#each currentStops as stop, i}
                <li class="grid grid-cols-3">
                    <div
                        class="flex items-center justify-center flex-auto p-1 gradient-picker-color"
                    >
                        <ColorPicker
                            bind:rgb={stop.rgb}
                            label=""
                            isAlpha={false}
                            components={{ wrapper: ColorPickerWrapper }}
                            textInputModes={["hex"]}
                        />
                    </div>
                    <div class="flex items-center justify-center flex-auto p-1">
                        <Input
                            maxLength={3}
                            bind:value={userInput[i]}
                            class="w-full p-2 text-center rounded-lg outline-none text-stroke bg-black/40 outline outline-white/20 -outline-offset-2"
                        />
                    </div>
                    <div class="flex items-center justify-center flex-auto p-1">
                        <Button type="decline" class="w-8 aspect-square"
                        ></Button>
                    </div>
                </li>
            {/each}
        </ul>
    </div>
</div>

<style lang="postcss">
    .dark {
        --cp-bg-color: #333;
        --cp-border-color: white;
        --cp-text-color: white;
        --cp-input-color: #555;
        --cp-button-hover-color: #777;
    }

    :global(#gradient-slider) {
        --range-handle: white;
        --range-handle-border: transparent;
        --range-handle-focus: white;
        --range-handle-inactive: white;
        @apply pointer-events-none m-0 h-full rounded-md border-2 border-black;
        background: var(--bg);
    }

    :global(#gradient-slider .rangeHandle) {
        @apply pointer-events-auto bottom-0 top-0 h-full w-2 -translate-x-1/2 translate-y-0 rounded-md;
    }

    :global(#gradient-slider .rangeNub) {
        @apply rounded-md bg-transparent outline outline-4 outline-white ring-2 ring-black ring-offset-4;
    }

    :global(#gradient-slider .rangeFloat) {
        @apply pointer-events-auto top-full mt-4 h-8 w-12 -translate-x-1/2 translate-y-0 rounded-md border-2 border-white bg-black text-white opacity-100 transition-none;
    }

    .gradient-picker-color :global(.container) :global(.alpha) {
        background: none;
    }
</style>
