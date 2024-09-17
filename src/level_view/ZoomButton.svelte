<script lang="ts">
    import Image from "../components/Image.svelte";

    import zoomIn from "./assets/zoom_in.png?url";
    import zoomOut from "./assets/zoom_out.png?url";
    import { zoomCentral, zoomGoal } from "./view_controls";

    export let zoom: "in" | "out";
    export let canvas: HTMLCanvasElement;
</script>

<button
    class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
    on:click={() => {
        zoomCentral($zoomGoal + (zoom == "in" ? 2 : -2), canvas);
    }}
>
    <Image
        src={zoom == "in" ? zoomIn : zoomOut}
        class="object-contain aspect-square"
    ></Image>
</button>
