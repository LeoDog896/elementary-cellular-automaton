<script lang="ts">
    import init from "elementary-cellular-automaton"
	import { onMount } from "svelte";

    onMount(async () => {
        await init();
    })
</script>