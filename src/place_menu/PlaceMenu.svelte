<script lang="ts">
    import { default as cx } from "classnames";
    import { Motion, useAnimation } from 'svelte-motion';

    import Animate from "../components/Animate.svelte"

    import Build from "./icons/build.svg";
    import Edit from "./icons/edit.svg";
    import Delete from "./icons/delete.svg";
    import Minimize from "./icons/caret.svg";

    let isMinimized = false;
    const shrinkTabAnim = useAnimation();
    const minimizeAnim = useAnimation();

    const test = useAnimation();

    $: {
        if (isMinimized) {
            test.start((e: any) => {
                console.log(e);
                return {
                    "grid-template-rows": "48px 0px",
                    transition: {
                        easing: "ease-in-out",
                        duration: 0.5,
                    }
                }
            });
            shrinkTabAnim.start((_: any) => ({
                width: 100,
                transition: {
                    easing: "ease-in-out",
                    duration: 0.5,
                }
            }));
            // minimizeAnim.start((_: any) => ({
            //     height: 0,
            //     transition: {
            //         easing: "ease-in-out",
            //         duration: 0.5,
            //     }
            // }))
        } else {
            test.start((_: any) => ({
                "grid-template-rows": "48px 200px",
                transition: {
                    easing: "ease-in-out",
                    duration: 0.5,
                }
            }));
            shrinkTabAnim.start((_: any) => ({
                width: "auto",
                transition: {
                    easing: "ease-in-out",
                    duration: 0.5,
                }
            }));
            // minimizeAnim.start((_: any) => ({
            //     height: "auto",
            //     transition: {
            //         easing: "ease-in-out",
            //         duration: 0.5,
            //     }
            // }))
        }
    }
    
</script>


<div class="flex flex-col justify-end w-full h-full p-2">
    <Motion animate={test} let:motion layout>
    <div class="container" use:motion>
        <div class="flex flex-col items-center minimize menu-panel justify-evenly" >
            <button on:click={() => {
                isMinimized = !isMinimized;
            }}>
                <Minimize class={cx({
                    "cursor-pointer": true,
                    "rotate-180": isMinimized,
                })}></Minimize>
            </button>
        </div>

        
        <!-- <Motion animate={shrinkTabAnim} let:motion>
            <div class="tabs menu-panel" use:motion>
             
            </div>
        </Motion> -->

        <Animate 
            easing="easeInOut" 
            duration={0.5}
            from={{
                width: "auto",
            }}
            to={{
                isMinimized: {
                    width: 100,
                }
            }}
            conditions={{
                isMinimized: isMinimized,
            }}
        >
            <div class="tabs menu-panel">
             
            </div>
        </Animate>

        <div class="flex flex-col items-center w-full h-full menu-panel justify-evenly">
            <Build class="cursor-pointer"></Build>
            <Edit class="cursor-pointer"></Edit>
            <Delete class="cursor-pointer"></Delete>
            <!--  -->
        </div>
    
        <div class="buttons menu-panel">
            <!--  -->
        </div>
    
        <div class="place-bttn-place">
            <!--  -->
        </div>
        
    </div>
</Motion>
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
