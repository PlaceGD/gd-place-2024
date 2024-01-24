<script lang="ts">
    import * as wasm from "wasm-lib";

    import { onMount } from "svelte";
    import Toast from "../utils/toast";
    import { DEBUG } from "../utils/debug";
    import { spritesheetProgress } from "../load_wasm";
    import Widget from "../widgets/Widget.svelte";
    import { runCallbacks } from "../state";

    export let state: wasm.StateWrapper | null;

    export let canvas: HTMLCanvasElement;
    let view_size = [0, 0];
    // let text_draws: wasm.TextDraw[] = [];

    onMount(() => {
        try {
            state = wasm.create_view(canvas, $spritesheetProgress.arrayBuffer!);
        } catch (e: any) {
            console.error(e, "(Failed in `wasm.create_view`)");
            Toast.showErrorToast(
                `A fatal error occured in the WASM.\nPlease report this bug to the developers (the error can be found in the console by pressing \`F12\` or \`CTRL+SHIFT+I\`.\nRefresh the page and try again. (${e})`
            );
        }
    });

    let prevTime = 0;

    const draw = (time: number) => {
        if (state != null) {
            try {
                state.pub_render((time - prevTime) / 1000);
                prevTime = time;
                runCallbacks(state);
                // text_draws = state.get_text_draws();
            } catch (e: any) {
                console.error(e, "(Failed in `state.pub_render`)");
                Toast.showErrorToast(
                    `A fatal error occured in the WASM.\nPlease report this bug to the developers (the error can be found in the console by pressing \`F12\` or \`CTRL+SHIFT+I\`.\nRefresh the page and try again. (${e})`
                );
                return;
            }
        }
        requestAnimationFrame(draw);
    };
    requestAnimationFrame(draw);

    $: {
        if (state != null && canvas != null) {
            let [w, h] = view_size;
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
    bind:offsetHeight={view_size[1]}
    bind:offsetWidth={view_size[0]}
    aria-label="Level Canvas"
>
    <canvas bind:this={canvas} />
</div>
