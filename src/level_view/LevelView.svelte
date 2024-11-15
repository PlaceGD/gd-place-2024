<script lang="ts">
    import * as wasm from "wasm-lib";

    import { onMount } from "svelte";
    import Toast, { WASM_ERROR } from "../utils/toast";
    import { DEBUG, stats } from "../utils/debug";
    import { spritesheetProgress } from "../load_wasm";
    // import Widget from "../widgets/Widget.svelte";
    import { editorSettings, rawSpritesheetData } from "../stores";
    import { handleSub } from "./view_controls";
    import { isMobile } from "../utils/document";
    import { toast } from "@zerodevx/svelte-toast";
    import { showGpuAccelWarning } from "../utils/misc";
    // import { loadState, runCallbacks } from "../state";

    export let state: wasm.State | null;

    export let canvas: HTMLCanvasElement;
    export let canvasWidth: number;
    export let canvasHeight: number;

    let offscreenCanvas: OffscreenCanvas | null = null;

    const setView = async () => {
        offscreenCanvas = canvas?.transferControlToOffscreen();

        if (canvas == null || offscreenCanvas == null) {
            Toast.showErrorToast(
                "There was an error creating the canvas. Please report this!"
            );
        }

        try {
            state = await wasm.create_view(
                offscreenCanvas,
                $rawSpritesheetData!.data,
                $rawSpritesheetData!.width,
                $rawSpritesheetData!.height
            );
        } catch (e: unknown) {
            if (e instanceof wasm.StateError) {
                console.error(e.display(), "(Failed in `wasm.create_view`)");

                if (e.kind === 0) {
                    showGpuAccelWarning(
                        "An error occurred during loading. This error may be due to an outdated browser or operating system."
                    );
                } else {
                    Toast.showErrorToast(WASM_ERROR);
                    Toast.showInfoToast(
                        "This error may be due to an outdated browser or operating system."
                    );
                }
            } else {
                console.error(e, "(Failed in `wasm.create_view`)");
                Toast.showErrorToast(WASM_ERROR);
            }
        }
    };

    $: {
        if (canvas != null && offscreenCanvas == null) {
            setView();
        }
    }

    let prevTime = 0;

    let fpsSum = 0;
    let fpsCount = 0;
    let numTestFrames = 60;
    let qualityStep = ["low", "medium", "high"].indexOf(
        $editorSettings.quality
    ); // 3 = high, 2 = med, 1 = low + warning, 0 = finished

    const draw = (time: number) => {
        if (state != null) {
            try {
                if (document.visibilityState === "visible") {
                    stats.begin();

                    state.render((time - prevTime) / 1000);
                    fpsSum += 1000 / (time - prevTime);
                    numTestFrames -= 1;
                    fpsCount += 1;
                    if (numTestFrames <= 0) {
                        let avg = fpsSum / fpsCount;

                        if (avg < 20.0 && qualityStep > 0) {
                            qualityStep--;

                            switch (qualityStep) {
                                case 1:
                                    $editorSettings.quality = "medium";
                                    console.info("Quality set to medium.");
                                    numTestFrames = 240;
                                    fpsSum = 0;
                                    fpsCount = 0;
                                    break;
                                case 0:
                                    $editorSettings.quality = "low";
                                    console.info("Quality set to low.");
                                    showGpuAccelWarning("Low FPS detected.");
                                    break;
                                default:
                                    break;
                            }
                        }
                    }
                    stats.end();

                    prevTime = time;
                }
            } catch (e: unknown) {
                console.error(e, "(Failed in `state.render`)");
                Toast.showErrorToast(WASM_ERROR);
                return;
            }
        }
        requestAnimationFrame(draw);
    };
    requestAnimationFrame(draw);

    const resize = () => {
        if (canvas == null || offscreenCanvas == null) {
            Toast.showErrorToast(
                "There was an error creating the canvas. Please report this!"
            );
            return;
        }

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

        // high = window.dpr
        // low = 1

        // high = 1
        // med = 1/window.dpr / 2
        // low = 1/window.dpr

        const quality = {
            high: 1,
            medium: 0.6,
            low: 0.35,
        }[$editorSettings.quality ?? (isMobile() ? "medium" : "high")];

        // state.resize(w, h);
        state!.resize(dprWidth, dprHeight);
        state!.set_quality(quality);
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        // canvas.width = dprWidth * quality;
        // canvas.height = dprHeight * quality;
        // canvas.width = w;
        // canvas.height = h;
        offscreenCanvas.width = dprWidth * quality;
        offscreenCanvas.height = dprHeight * quality;

        // handleSub(state!);
    };

    $: {
        $editorSettings.quality;
        canvasWidth || canvasHeight;
        if (state != null) {
            resize();
        }
    }
</script>

<div
    class="absolute w-full h-full"
    bind:offsetHeight={canvasHeight}
    bind:offsetWidth={canvasWidth}
    aria-label="Level Canvas"
    id="level-canvas"
>
    <canvas bind:this={canvas} />
</div>
