<script lang="ts">
	import init, { wasm_next } from 'elementary-cellular-automaton';
	import { onMount } from 'svelte';
	import { Canvas, Layer, type Render } from 'svelte-canvas';
	import Marker from '../lib/Marker.svelte';

	let ready = false;
	let rule = '90';

	function isValidRule(rule: string): boolean {
		return !isNaN(Number(rule)) && Number(rule) >= 0 && Number(rule) <= 255;
	}

	$: parsedRule = isValidRule(rule) ? Number(rule) : null;

	let width: number;
	let height: number;

	type Color = [r: number, g: number, b: number, a: number];

	function gradient(start: Color, end: Color, percent: number): Color {
		const [r1, g1, b1, a1] = start;
		const [r2, g2, b2, a2] = end;

		const r = Math.floor(r1 + (r2 - r1) * percent);
		const g = Math.floor(g1 + (g2 - g1) * percent);
		const b = Math.floor(b1 + (b2 - b1) * percent);
		const a = Math.floor(a1 + (a2 - a1) * percent);
		return [r, g, b, a];
	}

	let render: Render;
	$: render = ({ context, width, height }) => {
		context.imageSmoothingEnabled = false;
		if (!ready) return;

		if (!parsedRule) {
			context.fillStyle = 'black';
			context.fillRect(0, 0, width, height);
			context.fillStyle = 'white';
			context.font = '30px Arial';
			context.textAlign = 'center';
			context.fillText('Enter a valid rule (0 - 255)', width / 2, height / 2);

			return;
		}

		const starting = new Array<boolean>(width).fill(false);
		starting[Math.floor(width / 2)] = true;

		const arr = next(starting, parsedRule, height - 1);

		context.fillStyle = 'black';
		const { data } = context.getImageData(0, 0, width, height);

		for (let i = 0; i < data.length; i += 4) {
			if (arr[i / 4]) {
				const color = gradient([55, 114, 255, 255], [240, 56, 255, 255], i / data.length);
				data[i] = color[0];
				data[i + 1] = color[1];
				data[i + 2] = color[2];
				data[i + 3] = color[3];
			}
		}

		context.putImageData(new ImageData(data, width, height), 0, 0);
	};

	function ruleToBinary(rule: number): boolean[] {
		const binary = rule.toString(2).padStart(8, '0');
		return binary.split('').map((b) => b === '1');
	}

	const rules = [
		[true, true, true],
		[true, true, false],
		[true, false, true],
		[true, false, false],
		[false, true, true],
		[false, true, false],
		[false, false, true],
		[false, false, false]
	];

	// returns the next N rows of the elementary cellular automaton
	function next(arr: boolean[], rule: number, steps: number): boolean[] {
		const u8 = new Uint8Array(arr.map((b) => (b ? 1 : 0)));
		const next = wasm_next(u8, rule, steps);
		return [...next].map((n) => n === 1);
	}

	onMount(async () => {
		await init();
		ready = true;
	});
</script>

<main>
	<h1>Elementary Cellular Automaton</h1>
	<p>
		Simulates a large grid of every Elementary Cellular Automaton from 0 - 255 (<a
			href="https://en.wikipedia.org/wiki/Elementary_cellular_automaton">Wikipedia</a
		>, <a href="https://mathworld.wolfram.com/Rule.html">Wolfram Alpha</a>).
	</p>

	<div>
		<label for="rule">Rule: </label>
		<input
			type="number"
			id="rule"
			placeholder="Enter Rule (0 - 255)"
			bind:value={rule}
			min="0"
			max="255"
		/>
	</div>

	<div class="rules">
		{#each ruleToBinary(parsedRule ?? 0) as r, i}
			<div class="marker">
				<Marker input={rules[i]} output={[r]} />
			</div>
		{/each}
	</div>

	<div class="container" bind:clientWidth={width} bind:clientHeight={height}>
		<Canvas {width} {height}>
			<Layer {render} />
		</Canvas>
	</div>
</main>

<style>
	input {
		margin-top: 1rem;
	}

	.rules {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		width: 100%;
		flex-wrap: wrap;
	}

	.marker {
		margin: 1rem;
	}

	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		width: 100%;
		height: 100%;
	}

	.container {
		width: 60%;
		height: 60%;
		overflow: hidden;
		padding: 1px;
		border: 1.5px solid black;
		margin: 1rem;
	}
</style>
