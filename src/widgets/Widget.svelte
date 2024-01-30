<script lang="ts">
    export let position: [number, number];
    export let scale: number;
    export let screenCenter: boolean = true;
</script>

<div
    class="absolute w-full h-full overflow-visible pointer-events-none {screenCenter
        ? 'flex-center'
        : ''}"
    style={`
        left: ${position[0]}px;
        top: ${-position[1]}px;
        transform: scale(${scale});
    `}
>
    <slot />
</div>
