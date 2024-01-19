<script lang="ts">
    import { default as cx } from "classnames";

    export let isToggled: boolean;
    export let disabled: boolean = false;

    let { id, ...rest } = $$restProps;
</script>

<div
    class={cx({
        "flex items-center justify-center w-full": true,
        "opacity-50 pointer-events-none cursor-not-allowed": disabled,
    })}
    {...rest}
>
    <label
        for={id}
        class="flex items-center rounded-full cursor-pointer bg-black/40"
    >
        <div class="relative">
            <input
                type="checkbox"
                {id}
                class="sr-only"
                disabled
                on:click={() => {
                    isToggled = !isToggled;
                }}
            />
            <div class="block h-8 bg-gray-600 rounded-full w-14"></div>
            <div
                class={cx({
                    "absolute w-6 h-6 transition rounded-full dot left-1 top-1": true,
                    "bg-white": !isToggled,
                    "bg-button-green translate-x-full": isToggled,
                })}
            ></div>
        </div>
    </label>
</div>
