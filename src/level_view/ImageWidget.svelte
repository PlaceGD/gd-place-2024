<script lang="ts">
    import Widget from "../widgets/Widget.svelte";

    export let position: [number, number];
    export let screenScale: number;
    export let scale: number;
    export let screenOrigin: [number, number];
</script>

<Widget
    position={[
        screenOrigin[0] / window.devicePixelRatio + position[0] * screenScale,
        screenOrigin[1] / window.devicePixelRatio + position[1] * screenScale,
    ]}
    scale={screenScale * scale}
>
    <slot class="flex justify-center items-center" />
</Widget>
