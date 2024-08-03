<script lang="ts">
	import { Button } from "carbon-components-svelte";
	import { Delete } from "carbon-icons-svelte";
	import { createEventDispatcher } from "svelte";

	const dispatch = createEventDispatcher();
	export let idData = "";

	export let isDeleting = false;

	function delete_data() {
		isDeleting = true;
      setTimeout(() => {
         dispatch("deleteData", idData);
      }, 250);
		
	}
</script>

<Button
	skeleton={isDeleting}
	tooltipPosition="top"
	tooltipAlignment="end"
	size="small"
	kind="danger-ghost"
	icon={Delete}
	iconDescription={"Delete"}
	on:click={() => delete_data()}
/>
