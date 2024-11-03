<script lang="ts">
    import { onMount } from "svelte";
    import { KOFI_ID } from "../utils/misc";

    let widgetString = "";
    onMount(() => {
        window.kofiwidget2.init("Support Us!", "#7ADE2D", KOFI_ID);
        widgetString = window.kofiwidget2
            .getHTML()
            .replace("Support me on ko-fi.com", "Support us!");
    });
</script>

<span title="Support us">
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html widgetString}
</span>
