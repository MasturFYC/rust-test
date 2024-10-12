<script lang="ts">
import type { iGudang, iRelationProp } from "$lib/interfaces";
import {
	InlineLoading,
	Modal,
	TextInput,
	ComboBox,
} from "carbon-components-svelte";
import { Save } from "carbon-icons-svelte";
import { createEventDispatcher } from "svelte";

const dispatch = createEventDispatcher();
export let open = false;
export let gudang: iGudang;
export let isUpdating = false;
export let employees: iRelationProp[] = [];
export let isError = false;
export let errorMessage = "";

function get_employees() {
	return employees.map((m) => ({ id: m.id, text: m.text }));
}

function submit() {
	isUpdating = true;
	dispatch("submit", gudang);
}

$: employee_invalid = gudang.employeeId === 0;

</script>

<Modal
  bind:open
  hasForm
  id="gud-modal"
  preventCloseOnClickOutside
  modalHeading="Gudang"
  primaryButtonText="Simpan"
  primaryButtonIcon={Save}
  secondaryButtonText="Batal"
  selectorPrimaryFocus={"#gud-name"}
  on:click:button--secondary={() => (open = false)}
  on:click:button--primary={submit}
  size="xs"
  primaryButtonDisabled={isUpdating || employee_invalid}
>
  <form>
    <TextInput
      id="gud-name"
      bind:value={gudang.name}
      labelText="Nama"
      placeholder="e.g. Junaedi"
    />
    <ComboBox
      id="gud-ware"
      titleText="Penjaga gudang"
      placeholder="Pilih penjaga gudang"
      selectedId={gudang.employeeId}
      warn={employee_invalid}
      items={get_employees()}
      on:select={(e) => {
        if (e.detail.selectedId > 0) {
          gudang.employeeId = e.detail.selectedId;
        }
      }}
      on:clear={() => (gudang.employeeId = 0)}
    />
    <TextInput
      id="gud-locate"
      bind:value={gudang.locate}
      labelText="Lokasi"
      placeholder="Jl. Merdeka Barat"
    />
  </form>

  {#if isUpdating}
    <InlineLoading
      status="active"
      description={gudang.id === 0 ? "Posting data..." : "Updating data..."}
    />
  {/if}

  {#if isError}
    <InlineLoading description={errorMessage} status="error" />
  {/if}
</Modal>

<style lang="css">
  :global(#gud-modal .bx--modal-container.bx--modal-container--xs) {
    max-height: 100%;
  }
</style>
