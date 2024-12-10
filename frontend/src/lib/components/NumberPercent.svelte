<script lang="ts">
	import { cardPercent } from '$lib/components/NumberFormat';
	import { TextInput } from 'carbon-components-svelte';
	import { onDestroy } from 'svelte';

	interface Props {
		value: string;
		inline?: boolean;
		labelText?: string;
		size?: 'sm' | 'xl';
		placeholder?: string;
		hideLabel?: boolean;
		clasess?: string;
		id?: string;
	}

	let {
		value = $bindable('11.0'),
		inline = false,
		labelText = '',
		size = 'sm',
		placeholder = '',
		hideLabel = false,
		clasess = 'input-number',
		id = ''
	}: Props = $props();

	let ref_input: HTMLInputElement | undefined = $state(undefined);

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

	$effect(() => {
		if (ref_input) {
			ref_input.addEventListener('input', updateValue);
			ref_input.addEventListener('paste', updateValue);
		}
	});
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
	on:focus
	on:keydown
	hideLabel={hideLabel}
/>
