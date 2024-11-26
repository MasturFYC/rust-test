<script lang="ts">
import { Button } from 'carbon-components-svelte';
import { Delete } from 'carbon-icons-svelte';

let { categoryId, onDeleteData }: { categoryId: number; onDeleteData: (e: number) => void } =
	$props();
let is_deleting = $state(false);

function delete_data(e: number) {
	is_deleting = true;
	setTimeout(() => {
		onDeleteData(e);
		// is_deleting = false;
	}, 250);
}
</script>

<Button
	skeleton={is_deleting}
	tooltipAlignment="end"
	size="small"
	kind="danger-ghost"
	icon={Delete}
	iconDescription={'Delete'}
	on:click={() => delete_data(categoryId)}
/>
