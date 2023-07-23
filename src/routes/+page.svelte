<script lang="ts">
	import init, { wasm_next } from 'elementary-cellular-automaton';
	import { onMount } from 'svelte';
	import { Canvas, Layer, type Render } from 'svelte-canvas';

	let ready = false;
	let rule = 90;

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
		const starting = new Array<boolean>(width).fill(false);
		starting[Math.floor(width / 2)] = true;

		const arr = next(starting, rule, height - 1);

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
    <input type="number" bind:value={rule} min="0" max="255" />

    <div class="container" bind:clientWidth={width} bind:clientHeight={height}>
        <Canvas {width} {height}>
            <Layer {render} />
        </Canvas>
    </div>
</main>

<style>
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