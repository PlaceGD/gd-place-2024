<script lang="ts">
    export let min: number = 0.2;
    export let max: number = 1;
    export let step: number = 0.1;

    export let currentColor: string;
</script>

<input
    class="w-full h-full rounded-lg opacity-slider"
    aria-label="Opacity Slider"
    type="range"
    {min}
    {max}
    {step}
    aria-valuemin={min}
    aria-valuemax={max}
    style={`--currentColor: #${currentColor}`}
/>

<style>
    .opacity-slider {
        position: relative;
        background: repeating-conic-gradient(#474747 0% 25%, transparent 0% 50%)
            50% / 20px 20px;
        appearance: none;
    }

    .opacity-slider::-moz-range-track {
        height: 100%;
        -moz-appearance: none;
        background: linear-gradient(
            to right,
            #00000000 0%,
            var(--currentColor) 100%
        );

        background-color: transparent;
        border-radius: 6px;
    }
    .opacity-slider::-webkit-slider-runnable-track {
        height: 100%;
        background: linear-gradient(
            to right,
            #00000000 0%,
            var(--currentColor) 100%
        );
        background-color: transparent;
        border-radius: 6px;
        -webkit-appearance: none;
    }

    /* webkit sucks go use firefox pls thanks :3 */
    .opacity-slider::-webkit-slider-thumb {
        /* position: absolute;
        bottom: -3px; */
        background: white;
        border-radius: 50%;
        border: 2.5px black solid;
        -webkit-appearance: none;
        width: 20px;
        height: 20px;
        box-sizing: content-box;
    }
    .opacity-slider::-moz-range-thumb {
        -moz-appearance: none;
        background: white;
        border: 2.5px black solid;
        border-radius: 50%;
    }
</style>
