<script lang="ts">
    export let src: string;
    export let alt: string = "";
    export let lazyLoad: boolean = false;
    export let cls: string = "";
</script>

<img {src} {alt} class={cls} draggable="false" />
