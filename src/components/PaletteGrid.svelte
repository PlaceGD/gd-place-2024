<script lang="ts">
    import { colors } from "shared-lib/gd";
    import {
        bgColor,
        ground1Color,
        ground2Color,
        mainColorRGB,
        menuSelectedObject,
    } from "../stores";
    import {
        BG_TRIGGER,
        GROUND_2_TRIGGER,
        GROUND_TRIGGER,
    } from "shared-lib/nexusgen";

    export let hue: number;

    export let currentRow: number;
    export let currentColumn: number;

    export let tabIndex: number = 0;

    $: palette = colors.list[hue].palette;
</script>

<div class="flex flex-col w-full h-full gap-1">
    {#each Array(colors.rows).fill(0) as _, r}
        <div class="flex flex-1 h-auto gap-1">
            {#each Array(colors.columns).fill(0) as _, c}
                <button
                    class="flex-1 w-auto h-auto rounded-md"
                    style={`
                        background: rgb(${palette[r][c].join(", ")});
                        ${
                            r == currentRow && c == currentColumn
                                ? "box-shadow: 0px 0px 0px 2px #000 inset, 0px 0px 0px 2px #FFF;"
                                : ""
                        }
                    `}
                    on:click={() => {
                        currentRow = r;
                        currentColumn = c;

                        if ($menuSelectedObject == BG_TRIGGER) {
                            $bgColor = {
                                r: $mainColorRGB[0],
                                g: $mainColorRGB[1],
                                b: $mainColorRGB[2],
                            };
                        }

                        if ($menuSelectedObject == GROUND_TRIGGER) {
                            $ground1Color = {
                                r: $mainColorRGB[0],
                                g: $mainColorRGB[1],
                                b: $mainColorRGB[2],
                            };
                        }

                        if ($menuSelectedObject == GROUND_2_TRIGGER) {
                            $ground2Color = {
                                r: $mainColorRGB[0],
                                g: $mainColorRGB[1],
                                b: $mainColorRGB[2],
                            };
                        }
                    }}
                    tabindex={tabIndex}
                    aria-label={`Color with Red: ${palette[r][c][0]}, Green: ${palette[r][c][1]}, Blue: ${palette[r][c][2]}`}
                />
            {/each}
        </div>
    {/each}
</div>
