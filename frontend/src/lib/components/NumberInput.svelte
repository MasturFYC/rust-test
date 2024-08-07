<script lang="ts">
	import { cardNumber } from "$lib/components/NumberFormat";
	import { TextInput } from "carbon-components-svelte";
	import { onDestroy } from "svelte";

	export let value = "0.0";
	export let inline = false;
	export let labelText = "";
	export let size: "sm" | "xl" = "sm";
	export let placeholder = "";
	export let warn = false;
	export let warnText = "";
	export let hideLabel = false;
	export let disabled = false;
	export let style = "";
	export let tabindex = 0;
	export let classes = "input-number";
	export let id = "";

	let ref_input: HTMLInputElement;

	function updateValue(e: Event) {
		 //e.preventDefault();
		if (ref_input) {
			ref_input.value = cardNumber(ref_input.value);
		}
	}

	onDestroy(() => {
		try {
			if(ref_input) {
				ref_input.removeEventListener("input", updateValue);
				ref_input.removeEventListener("paste", updateValue);
			}
		} catch (ex: any) {
			console.log(ex.message);
		}
	});


	$: if (ref_input) {
		ref_input.addEventListener("input", updateValue);
		ref_input.addEventListener("paste", updateValue);
	}

	// $: value = getNumber(str_value);
	// $: console.log(value);
</script>

<TextInput
	autocomplete="off"
	{id}
	{tabindex}
	class={classes}
	{warn}
	{style}
	{inline}
	{labelText}
	{placeholder}
	{size}
	{warnText}
	{hideLabel}
	{disabled}
	on:change
	on:input
	on:keydown
	bind:ref={ref_input}
	bind:value
/>

