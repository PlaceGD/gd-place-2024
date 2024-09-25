<script lang="ts">
    import * as wasm from "wasm-lib";

    import { onMount } from "svelte";
    import Toast from "../utils/toast";
    import { DEBUG } from "../utils/debug";
    import { spritesheetProgress } from "../load_wasm";
    import Widget from "../widgets/Widget.svelte";
    import { rawSpritesheetData } from "../stores";
    // import { loadState, runCallbacks } from "../state";

    export let state: wasm.State | null;

    export let canvas: HTMLCanvasElement;
    export let canvasWidth: number;
    export let canvasHeight: number;

    const WASM_ERROR = `
        <strong>A fatal error occured in the WASM.</strong><br/>Please report this bug to the developers!
            <span style="color:white;text-decoration:underline;cursor:pointer;pointer-events:all;" onclick='navigator.clipboard.writeText(window.consoleErrors.join("\\n"));'>
                (click this text to copy the errors and include this in the report)
            </span>`;

    onMount(async () => {
        try {
            console.time("create_view");
            state = await wasm.create_view(
                canvas,
                $rawSpritesheetData!.data,
                $rawSpritesheetData!.width,
                $rawSpritesheetData!.height
            );
            console.timeEnd("create_view");

            // loadState(state);
        } catch (e: unknown) {
            console.error(e, "(Failed in `wasm.create_view`)");
            Toast.showErrorToast(WASM_ERROR);
        }
    });

    let prevTime = 0;

    const draw = (time: number) => {
        if (state != null) {
            try {
                state.render((time - prevTime) / 1000);
                prevTime = time;

                // runCallbacks();
            } catch (e: unknown) {
                console.error(e, "(Failed in `state.render`)");
                Toast.showErrorToast(WASM_ERROR);
                return;
            }
        }
        requestAnimationFrame(draw);
    };
    requestAnimationFrame(draw);

    $: {
        if (state != null && canvas != null) {
            let [w, h] = [canvasWidth, canvasHeight];
            if (w % 2 != 0) {
                w += 1;
            }
            if (h % 2 != 0) {
                h += 1;
            }

            // https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio#correcting_resolution_in_a_canvas
            const scale = window.devicePixelRatio;
            const dprWidth = Math.floor(w * scale);
            const dprHeight = Math.floor(h * scale);

            state.resize(dprWidth, dprHeight);
            canvas.style.width = `${w}px`;
            canvas.style.height = `${h}px`;
            canvas.width = dprWidth;
            canvas.height = dprHeight;
        }
    }
</script>

<div
    class="absolute w-full h-full"
    bind:offsetHeight={canvasHeight}
    bind:offsetWidth={canvasWidth}
    aria-label="Level Canvas"
>
    <canvas bind:this={canvas} />
</div>
