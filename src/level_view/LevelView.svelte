<script lang="ts">
    import * as wasm from "wasm-lib";

    import { onMount } from "svelte";
    import Toast from "../utils/toast";
    import { DEBUG } from "../utils/Debug";

    export let state: wasm.StateWrapper | null;

    export let canvas: HTMLCanvasElement;
    let view_size = [0, 0];
    let text_draws: wasm.TextDraw[] = [];

    onMount(() => {
        try {
            state = wasm.create_view(canvas);
        } catch (e: any) {
            Toast.showErrorToast(
                `A fatal error occured in the WASM. 
                    Please report this bug to the developers (the error can be found in the console by pressing \`F12\` or \`CTRL+SHIFT+I\`.
                    Refresh the page and try again. (${e})`
            );
        }
    });

    let prevTime = 0;

    const draw = (time: number) => {
        if (state != null) {
            try {
                state.pub_render((time - prevTime) / 1000);
                prevTime = time;
                //text_draws = state.get_text_draws();
            } catch (e: any) {
                Toast.showErrorToast(
                    `A fatal error occured in the WASM. 
                    Please report this bug to the developers (the error can be found in the console by pressing \`F12\` or \`CTRL+SHIFT+I\`.
                    Refresh the page and try again. (${e})`
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

{#if $DEBUG}
    <button
        class="absolute z-50 p-1 ml-20 text-white rounded-lg font-pusab text-md bg-white/10"
        on:click={() => {
            localStorage.clear();
            window.location.reload();
        }}>Clear LS & Refresh</button
    >
    <button
        class="absolute z-50 p-1 text-white rounded-lg font-pusab text-md ml-72 bg-white/10 cummer"
        on:click={() => {
            localStorage.clear();
            window.location.reload();
        }}
        >Hammod 😘😘😘😂😂
        <img
            src="https://media.tenor.com/-OpJG9GeK3EAAAAC/kanye-west-stare.gif"
            alt=""
        /></button
    >
{/if}

<div
    class="absolute w-full h-full"
    bind:offsetHeight={view_size[1]}
    bind:offsetWidth={view_size[0]}
>
    <canvas bind:this={canvas} />
</div>
<div class="absolute w-full h-full overflow-visible">
    {#each text_draws as text_draw}
        <div
            class="absolute overflow-visible font-semibold text-center whitespace-nowrap"
            style={`
            left: ${canvas.offsetWidth / 2}px;
            top: ${canvas.offsetHeight / 2}px;
            font-size: ${text_draw.font_size}px;
            transform: translate(-50%, -50%) ${text_draw.get_css_transform()} scaleY(-1);
            text-shadow: 0px ${text_draw.font_size / 10}px ${
                text_draw.font_size / 6
            }px rgba(0, 0, 0, 1.0);
            ${text_draw.get_extra_style()}
        `}
        >
            {text_draw.get_text()}
        </div>
    {/each}
</div>

<style>
    .cummer > img {
        display: none;
    }
    .cummer:hover > img {
        display: block;
    }
</style>
