<style lang="css">
	:global(#gud-modal .bx--modal-container.bx--modal-container--xs) {
		max-height: 100%;
	}
</style>

<script lang="ts">
	import type { iGudang, iRelationProp } from '$lib/interfaces';
	import {
		InlineLoading,
		Modal,
		TextInput,
		ComboBox
	} from 'carbon-components-svelte';
	import { Save } from 'carbon-icons-svelte';

	let {
		open = $bindable(false),
		gudang = $bindable(),
		isUpdating = false,
		employees = [],
		isError = false,
		errorMessage = '',
		onSubmit
	}: {
		open: boolean;
		gudang: iGudang;
		isUpdating: boolean;
		employees: iRelationProp[] | undefined;
		isError: boolean;
		errorMessage: string;
		onSubmit: (e: iGudang) => void;
	} = $props();

	function get_employees() {
		return employees.map((m) => ({ id: m.id, text: m.text }));
	}

	function submit() {
		isUpdating = true;
		onSubmit(gudang);
	}

	let employee_invalid = $derived(gudang.employeeId === 0);

	$inspect(gudang);
</script>

<Modal
	bind:open={open}
	on:close={() => {
		open = false;
	}}
	hasForm
	id="gud-modal"
	preventCloseOnClickOutside
	modalHeading="Gudang"
	primaryButtonText="Simpan"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={'#gud-name'}
	on:click:button--secondary={() => (open = false)}
	on:submit={submit}
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
			description={gudang.id === 0 ? 'Posting data...' : 'Updating data...'}
		/>
	{/if}

	{#if isError}
		<InlineLoading description={errorMessage} status="error" />
	{/if}
</Modal>
