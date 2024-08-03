<script lang="ts">
	import { cardPercent } from "$lib/components/NumberFormat";
	import { TextInput } from "carbon-components-svelte";
	import { onDestroy } from "svelte";

	export let value = "11.0";
	export let inline = false;
	export let labelText = "";
	export let size: "sm" | "xl" = "sm";
	export let placeholder = "";
	export let hideLabel = false;
	let ref_input: HTMLInputElement;


	function updateValue(e: Event) {
		// e.preventDefault();
		if (ref_input) {
			// str_value = cardPercent(ref_input.value);
			// value = getPercent(str_value);
			ref_input.value = cardPercent(ref_input.value);
		}
	}

	onDestroy(() => {
		ref_input.removeEventListener("input", updateValue);
		ref_input.removeEventListener("paste", updateValue);
	});

	$: if (ref_input) {
		ref_input.addEventListener("input", updateValue);
		ref_input.addEventListener("paste", updateValue);
	}

	// $: value = getPercent(str_value);
</script>

<TextInput
	class="input-number"
	{inline}
	bind:ref={ref_input}
	bind:value
	{labelText}
	{placeholder}
	{size}
	on:change
	on:input
	{hideLabel}
/>
