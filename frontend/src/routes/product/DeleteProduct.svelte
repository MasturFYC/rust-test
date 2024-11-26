<style lang="scss">
.cell-right {
	text-align: right;
}
</style>

<script lang="ts">
import { Button } from 'carbon-components-svelte';
import { Delete } from 'carbon-icons-svelte';
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();
export let productId: number = 0;

export let isDeleting = false;

function delete_data() {
	isDeleting = true;
	setTimeout(() => {
		dispatch('deleteData', productId);
	}, 250);
}
</script>

<div class="cell-right">
	<Button
		skeleton={isDeleting}
		tooltipPosition="left"
		tooltipAlignment="end"
		size="small"
		kind="danger-ghost"
		icon={Delete}
		iconDescription={'Delete'}
		on:click={() => delete_data()}
	/>
</div>
