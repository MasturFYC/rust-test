<script lang="ts">
	import { Button } from 'carbon-components-svelte';
	import { Delete } from 'carbon-icons-svelte';

	let {
		idData = 0,
		isDeleting = $bindable(false),
		onDeleteData
	}: {
		idData: number;
		isDeleting: boolean;
		onDeleteData: (id: number) => void;
	} = $props();

	async function delete_data() {
		isDeleting = true;

		setTimeout(() => {
			onDeleteData(idData);
		}, 500);
	}
</script>

<Button
	skeleton={isDeleting}
	tooltipPosition="right"
	tooltipAlignment="end"
	size="small"
	kind="danger-ghost"
	icon={Delete}
	iconDescription={'Delete'}
	on:click={delete_data}
/>
