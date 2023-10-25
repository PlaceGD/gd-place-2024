<script lang="ts">
    import Editor from "./Editor.svelte"
    import Image from "./components/Image.svelte"

    let bgUrl =
        "https://cdn.discordapp.com/attachments/1017864804561588246/1166852967908003892/image.png";



	import { flip } from 'svelte/animate';
	import { quintOut } from 'svelte/easing';
</script>

<div class="w-screen h-screen relative">
    <Image src={bgUrl} alt="" cls="w-full h-full object-cover absolute" />
    <Editor />
</div>
