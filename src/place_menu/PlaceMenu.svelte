<script lang="ts">
    import { default as cx } from "classnames";
    import { Motion, useAnimation } from 'svelte-motion';

    import Build from "./icons/build.svg";
    import Edit from "./icons/edit.svg";
    import Delete from "./icons/delete.svg";
    import Minimize from "./icons/caret.svg";

    let isMinimized = false;
    const shrinkTabAnim = useAnimation();
    const minimizeAnim = useAnimation();

    $: {
        if (isMinimized) {
            shrinkTabAnim.start((_: any) => ({
                width: 100,
                transition: {
                    easing: "ease-in-out",
                    duration: 0.5,
                }
            }));
            minimizeAnim.start((_: any) => ({
                height: 0,
                transition: {
                    easing: "ease-in-out",
                    duration: 0.5,
                }
            }))
        } else {
            shrinkTabAnim.start((_: any) => ({
                width: "auto",
                transition: {
                    easing: "ease-in-out",
                    duration: 0.5,
                }
            }));
            minimizeAnim.start((_: any) => ({
                height: "auto",
                transition: {
                    easing: "ease-in-out",
                    duration: 0.5,
                }
            }))
        }
    }
    
</script>


<div class="w-full h-full p-2 flex flex-col justify-end">
    <div class="container">
        <div class="minimize menu-panel flex flex-col items-center justify-evenly" >
            <button on:click={() => {
                isMinimized = !isMinimized;
            }}>
                <Minimize class={cx({
                    "cursor-pointer": true,
                    "rotate-180": isMinimized,
                })}></Minimize>
            </button>
        </div>

        
        <Motion animate={shrinkTabAnim} let:motion>
            <div class="tabs menu-panel" use:motion>
                <!--  -->
            </div>
        </Motion>

        <Motion animate={minimizeAnim} let:motion>
            <div class="side-menu menu-panel flex flex-col items-center justify-evenly" use:motion>
                <Build class="cursor-pointer"></Build>
                <Edit class="cursor-pointer"></Edit>
                <Delete class="cursor-pointer"></Delete>
                <!--  -->
            </div>
        </Motion>
        <Motion animate={minimizeAnim} let:motion>
            <div class="buttons menu-panel" use:motion>
                <!--  -->
            </div>
        </Motion>
        <Motion animate={minimizeAnim} let:motion>
            <div class="place-bttn-place" use:motion>
                <!--  -->
            </div>
        </Motion>
    </div>
</div>

<style>
    .container {
        display: grid;
        grid-template-columns: 72px auto 256px;
        grid-template-rows: 48px 200px;
        gap: 8px 8px;
        grid-template-areas:
            "minimize tabs place-bttn"
            "side-menu buttons place-bttn";
    }

    .minimize {
        grid-area: minimize;
    }

    .tabs {
        grid-area: tabs;
    }

    .side-menu {
        grid-area: side-menu;
    }

    .buttons {
        grid-area: buttons;
    }

    .place-bttn-place {
        grid-area: place-bttn;
        border-radius: 16px;
        background: #7ADE2D;
        box-shadow: 0px 0px 0px 4px #FFF inset, 0px 0px 0px 8px #000 inset, -4px -4px 0px 8px #49851B inset, 4px 4px 0px 8px #C6F249 inset;
    }
    .place-bttn-delete {
        grid-area: place-bttn;
        border-radius: 16px;
        background: #DE2D30;
        box-shadow: 0px 0px 0px 4px #FFF inset, 0px 0px 0px 8px #000 inset, -4px -4px 0px 8px #851B1D inset, 4px 4px 0px 8px #F24980 inset;
    }
</style>
