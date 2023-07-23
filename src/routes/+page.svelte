<script lang="ts">
    import init, { wasm_next } from "elementary-cellular-automaton"
	import { onMount } from "svelte";
    import { Canvas, Layer, type Render } from 'svelte-canvas';

    let ready = false;

    let render: Render;
    $: render = ({ context, width, height }) => {
        if (!ready) return;
        const starting = new Array<boolean>(width).fill(false);
        starting[Math.floor(width / 2)] = true;
        
        const arr = next(
            starting,
            90,
            height - 1
        );

        context.fillStyle = "black";
        const { data } = context.getImageData(0, 0, width, height);

        for (let i = 0; i < data.length; i += 4) {
            if (arr[i / 4]) {
                data[i] = 0;
                data[i + 1] = 0;
                data[i + 2] = 255;
                data[i + 3] = 255;
            } else {
                data[i] = 255;
                data[i + 1] = 255;
                data[i + 2] = 255;
                data[i + 3] = 255;
            }
        }

        context.putImageData(new ImageData(data, width, height), 0, 0);
    };

    // returns the next N rows of the elementary cellular automaton
    // e.g. if given 0 1 0 and rule 90, with 1 step, it will return 1 0 1 (with extra padding).
    // if it's given more than 1 step, it will return the next step appended to the previous one.
    // e.g. 1 0 1 0 1 0 for 2 steps.
    function next(arr: boolean[], rule: number, steps: number): boolean[] {
        const u8 = new Uint8Array(arr.map((b) => (b ? 1 : 0)));
        const next = wasm_next(u8, rule, steps);
        return [...next].map((n) => n === 1);
    }

    onMount(async () => {
        await init();
        window.next = next;
        ready = true;
    });
</script>

<Canvas width={320} height={320}>
    <Layer {render} />
</Canvas>
