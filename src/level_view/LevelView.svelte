<script lang="ts">
    import * as wasm from "../../wasm-lib/pkg/wasm_lib";
    import { onMount } from "svelte";
    import { toast } from "../utils/toast";

    export let state: wasm.StateWrapper | null;

    export let canvas: HTMLCanvasElement;
    let view_size = [0, 0];

    onMount(() => {
        state = wasm.create_view(canvas);
    });

    const draw = () => {
        if (state != null) {
            try {
                state.pub_render(0.25);
            } catch (e) {
                toast.showErrorToast(
                    `An error occured in the editor (WASM). 
                    Please report this bug to the developers (the error can be found in the console by pressing \`F12\` or \`CTRL+SHIFT+I\`.
                    Refresh the page and try again.`
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

            state.resize(w, h);
            canvas.width = w;
            canvas.height = h;
        }
    }
</script>

<div
    class="absolute w-full h-full"
    bind:offsetHeight={view_size[1]}
    bind:offsetWidth={view_size[0]}
>
    <canvas bind:this={canvas} />
</div>
