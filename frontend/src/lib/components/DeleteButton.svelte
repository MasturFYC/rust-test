<script lang="ts">
	import { Button } from 'carbon-components-svelte';
	import { Delete } from 'carbon-icons-svelte';

	let {
		idData = 0,
		disabled = false,
		onDeleteData
	}: {
		idData: number;
		disabled?: boolean;
		onDeleteData: (e: number) => void;
	} = $props();

	let is_deleting = $state(false);

	function delete_data() {
		is_deleting = true;
		setTimeout(() => {
			onDeleteData(idData);
			// is_deleting = false;
		}, 250);
	}
</script>

<Button
	disabled={disabled}
	skeleton={is_deleting}
	tooltipAlignment="end"
	size="small"
	kind="danger-ghost"
	icon={Delete}
	iconDescription={'Delete'}
	on:click={() => delete_data()}
/>
