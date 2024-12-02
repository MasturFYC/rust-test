<script lang="ts">
	import { cardPercent } from '$lib/components/NumberFormat';
	import { TextInput } from 'carbon-components-svelte';
	import { onDestroy } from 'svelte';

	export let value = '11.0';
	export let inline = false;
	export let labelText = '';
	export let size: 'sm' | 'xl' = 'sm';
	export let placeholder = '';
	export let hideLabel = false;
	export let clasess = 'input-number';
	export let id = '';
	let ref_input: HTMLInputElement;

	function updateValue(_e: Event) {
		// e.preventDefault();
		if (ref_input) {
			// str_value = cardPercent(ref_input.value);
			// value = getPercent(str_value);
			ref_input.value = cardPercent(ref_input.value);
		}
	}

	onDestroy(() => {
		if (ref_input) {
			ref_input.removeEventListener('input', updateValue);
			ref_input.removeEventListener('paste', updateValue);
		}
	});

	$: if (ref_input) {
		ref_input.addEventListener('input', updateValue);
		ref_input.addEventListener('paste', updateValue);
	}

	// $: value = getPercent(str_value);
</script>

<TextInput
	autocomplete="off"
	class={clasess}
	inline={inline}
	bind:ref={ref_input}
	bind:value={value}
	id={id}
	labelText={labelText}
	placeholder={placeholder}
	size={size}
	on:change
	on:input
	on:keydown
	hideLabel={hideLabel}
/>
