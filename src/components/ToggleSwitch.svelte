<script lang="ts">
    import { default as cx } from "classnames";

    export let isToggled: boolean;
    export let disabled: boolean = false;

    export let tabIndex: number = 0;

    let { id, ...rest } = $$restProps;
</script>

<div
    class={cx({
        "flex items-center justify-center w-full": true,
        "opacity-50 pointer-events-none cursor-not-allowed": disabled,
    })}
    {...rest}
>
    <button
        class="flex items-center rounded-full cursor-pointer bg-black/40 outline-2 outline outline-white/20 -outline-offset-2 focus:outline-white focus:outline-offset-0"
        tabindex={tabIndex}
        role="checkbox"
        aria-checked={isToggled}
        on:click={() => {
            if (disabled) return;
            isToggled = !isToggled;
        }}
        on:keydown={e => {
            if ((e.key === "Enter" || e.key === " ") && !disabled) {
                isToggled = !isToggled;
                e.preventDefault();
            }
        }}
    >
        <div class="relative">
            <input
                tabindex={tabIndex}
                type="checkbox"
                disabled
                class="sr-only"
                {id}
            />
            <div
                class="block h-8 bg-gray-600 rounded-full w-14"
                tabindex="-1"
            ></div>
            <div
                class={cx({
                    "absolute w-6 h-6 transition rounded-full dot left-1 top-1": true,
                    "bg-white": !isToggled,
                    "bg-button-green translate-x-full": isToggled,
                })}
                tabindex="-1"
            ></div>
        </div>
    </button>
</div>
