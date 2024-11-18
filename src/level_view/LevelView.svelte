<script lang="ts">
    import * as wasm from "wasm-lib";

    import Toast, { WASM_ERROR } from "../utils/toast";
    // import Widget from "../widgets/Widget.svelte";
    import {
        bgColor,
        editorSettings,
        ground1Color,
        ground2Color,
        rawSpritesheetData,
    } from "../stores";
    import { showGpuAccelWarning } from "../utils/misc";
    import { handleSub } from "./view_controls";

    // import { loadState, runCallbacks } from "../state";

    export let state: wasm.State | null;

    export let canvas: HTMLCanvasElement = document.createElement("canvas");

    let links = [];

    const setView = async () => {
        if (canvas == null) {
            Toast.showErrorToast(
                "There was an error creating the canvas. Please report this!"
            );
        }

        try {
            state = await wasm.create_view(
                canvas,
                $rawSpritesheetData!.data,
                $rawSpritesheetData!.width,
                $rawSpritesheetData!.height
            );
        } catch (e: unknown) {
            if (e instanceof wasm.StateError) {
                console.error(e.display(), "(Failed in `wasm.create_view`)");

                if (e.kind === 0) {
                    showGpuAccelWarning(
                        "An error occurred during loading. This error may be due to an outdated browser or operating system."
                    );
                } else {
                    Toast.showErrorToast(WASM_ERROR);
                    Toast.showInfoToast(
                        "This error may be due to an outdated browser or operating system."
                    );
                }
            } else {
                console.error(e, "(Failed in `wasm.create_view`)");
                Toast.showErrorToast(WASM_ERROR);
            }
        }

        // downloadWithProgress()

        if (state != null) {
            state.set_zoom(12);

            state.set_bg_color($bgColor.r, $bgColor.g, $bgColor.b);

            state.set_ground1_color(
                $ground1Color.r,
                $ground1Color.g,
                $ground1Color.b
            );

            state.set_ground2_color(
                $ground2Color.r,
                $ground2Color.g,
                $ground2Color.b
            );
            state.set_hide_grid(true);
            state.set_hide_ground(true);
            resize();
            state.eboba();

            for (let i = 0; i <= 12; i++) {
                for (let j = 0; j <= 12; j++) {
                    state.set_camera_pos(2000 * i, 2000 * j);

                    await new Promise((res: (v: void) => void) => {
                        state!.render(7);
                        console.log(i, j);

                        canvas.toBlob(blob => {
                            if (blob == null) {
                                console.log("guh");
                                return;
                            }

                            let image = URL.createObjectURL(blob);
                            // Create a link
                            let aDownloadLink = document.createElement("a");
                            // Add the name of the file to the link
                            aDownloadLink.download = `${i},${j}.png`;
                            // Attach the data to the link
                            aDownloadLink.href = image;
                            // Get the code to click the download link
                            // aDownloadLink.click();
                            aDownloadLink.click();
                            URL.revokeObjectURL(image);
                            res();
                        }, "image/png");
                    });
                }
            }

            // for (let i of links) {
            //     i.click();
            // }
        }
    };

    setView();

    let prevTime = 0;

    let fpsSum = 0;
    let fpsCount = 0;
    let numTestFrames = 60;
    let qualityStep = ["low", "medium", "high"].indexOf(
        $editorSettings.quality
    ); // 3 = high, 2 = med, 1 = low + warning, 0 = finished

    const draw = (i: number, j: number) => {
        if (state != null) {
            try {
                if (document.visibilityState === "visible") {
                    state.render(7);
                    console.log("agagaga");

                    // setTimeout(() => {
                    console.log("gaga");
                    canvas.toBlob(blob => {
                        console.log(canvas.width, canvas.height);
                        if (blob == null) {
                            console.log("guh");
                            return;
                        }

                        let image = URL.createObjectURL(blob);
                        // Create a link
                        let aDownloadLink = document.createElement("a");
                        // Add the name of the file to the link
                        aDownloadLink.download = `${i},${j}.png`;
                        // Attach the data to the link
                        aDownloadLink.href = image;
                        // Get the code to click the download link
                        // aDownloadLink.click();
                        links.push(aDownloadLink);
                    }, "image/png");
                    // }, 50);
                }
            } catch (e: unknown) {
                console.error(e, "(Failed in `state.render`)");
                Toast.showErrorToast(WASM_ERROR);
                return;
            }
        }
        // requestAnimationFrame(draw);
    };
    // requestAnimationFrame(draw);

    const resize = () => {
        if (canvas == null) {
            Toast.showErrorToast(
                "There was an error creating the canvas. Please report this!"
            );
            return;
        }

        let [w, h] = [4000, 4000];
        if (w % 2 != 0) {
            w += 1;
        }
        if (h % 2 != 0) {
            h += 1;
        }

        // high = window.dpr
        // low = 1

        // high = 1
        // med = 1/window.dpr / 2
        // low = 1/window.dpr

        const quality = 1;

        // state.resize(w, h);
        state!.resize(w, h);
        state!.set_quality(quality);
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        // canvas.width = dprWidth * quality;
        // canvas.height = dprHeight * quality;
        // canvas.width = w;
        // canvas.height = h;

        // handleSub(state!);
    };
</script>

<!-- <div class="absolute w-full h-full" aria-label="Level Canvas" id="level-canvas">
    <canvas bind:this={canvas} />
</div> -->
