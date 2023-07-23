<script lang="ts">
	export let input: boolean[];
	export let output: boolean[];
</script>

<div class="marker">
	<div>
		{#each input as i}
			<div class="box" class:active={i} />
		{/each}
	</div>
	<div>
		{#each output as i}
			<div class="box" class:active={i} />
		{/each}
	</div>
</div>

<style>
	.marker {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
	}

	.marker > div {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
	}

	.box {
		display: inline-block;
		width: 20px;
		height: 20px;
		border: 1px solid black;
		padding: 1px;
		margin: 1px;
	}

	.box.active {
		background-color: black;
	}
</style>
