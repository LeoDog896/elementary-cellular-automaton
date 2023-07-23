<script lang="ts">
    import init, { wasm_next } from "elementary-cellular-automaton"
	import { onMount } from "svelte";

    // compress it in the <u8, Msb0> format
    function toByteArray(arr: boolean[]): Uint8Array {
        const len = arr.length;
        const u8 = new Uint8Array(Math.ceil(len / 8));
        for (let i = 0; i < len; i++) {
            if (arr[i]) {
                u8[Math.floor(i / 8)] |= 1 << (i % 8);
            }
        }
        return u8;
    }

    // decompress it from the <u8, Msb0> format
    function fromByteArray(u8: Uint8Array): boolean[] {
        const len = u8.length;
        const arr = new Array<boolean>(len * 8);
        for (let i = 0; i < len; i++) {
            for (let j = 0; j < 8; j++) {
                arr[i * 8 + j] = (u8[i] & (1 << j)) !== 0;
            }
        }
        return arr;
    }

    onMount(async () => {
        await init();

        // window.wasm_next = wasm_next;
        window.next = (arr: boolean[], rule: number, steps: number) => {
            const u8 = toByteArray(arr);
            const next = wasm_next(u8, rule, steps);
            return fromByteArray(next);
        }
    })
</script>