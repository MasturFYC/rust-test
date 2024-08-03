<script lang="ts">
	import { Button } from "carbon-components-svelte";
	import { Delete } from "carbon-icons-svelte";
	import { createEventDispatcher } from "svelte";

	const dispatch = createEventDispatcher();
	export let categoryId = 0;

	let is_deleting = false;

	function delete_category(e: number) {
		is_deleting = true;
      setTimeout(() => {
         dispatch("deleteCategory", e);
         is_deleting = false; 
      }, 250);
		
	}
</script>

<Button
	skeleton={is_deleting}
	tooltipAlignment="end"
	size="small"
	kind="danger-ghost"
	icon={Delete}
	iconDescription={"Delete"}
	on:click={() => delete_category(categoryId)}
/>
