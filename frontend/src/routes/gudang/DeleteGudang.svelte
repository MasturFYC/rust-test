<script lang="ts">
import { Button } from "carbon-components-svelte";
import { Delete } from "carbon-icons-svelte";
import { createEventDispatcher } from "svelte";

const dispatch = createEventDispatcher();
export let gudangId = 0;

let is_deleting = false;

function delete_gudang(e: number) {
  is_deleting = true;
  setTimeout(() => {
    dispatch("deleteGudang", e);
    is_deleting = false;
  }, 250);
}
</script>

<Button
  disabled={gudangId === 1}
  skeleton={is_deleting}
  tooltipAlignment="end"
  size="small"
  kind="danger-ghost"
  icon={Delete}
  iconDescription={"Delete"}
  on:click={() => delete_gudang(gudangId)}
/>
