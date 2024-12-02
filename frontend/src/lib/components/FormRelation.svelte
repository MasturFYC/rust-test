<style lang="css">
	.divider {
		margin-top: 6px;
		/* // border-top: 1px solid;
    // border-color: #999; */
	}
	:global(#rel-mod .bx--modal-container.bx--modal-container--sm) {
		max-height: 100%;
		/* height: 640px; */
	}
</style>

<script lang="ts">
	import type { iRelation, RelationTypeWIthID } from '$lib/interfaces';
	import {
		Checkbox,
		Column,
		FluidForm,
		// FluidForm,
		Grid,
		InlineLoading,
		Modal,
		Row,
		TextInput
	} from 'carbon-components-svelte';
	import { Save } from 'carbon-icons-svelte';

	let {
		open = $bindable(false),
		initdata,
		isError = false,
		isUpdating = false,
		errorMessage = '',
		relationTypes = [],
		saveRelation
	}: {
		open: boolean;
		initdata: iRelation;
		isError?: boolean;
		isUpdating?: boolean;
		errorMessage: string;
		relationTypes: RelationTypeWIthID[] | undefined;
		saveRelation: (data: iRelation) => void;
	} = $props();

	let data = $state(initdata);
	let isMember = $derived(
		data.relationType.filter((f) => f === 'Customer').length > 0
	);

	function save(e: CustomEvent<null>) {
		isUpdating = true;
		e.preventDefault();
		e.stopPropagation();

		// if (x === 2) return false;

		const newData = {
			...data,
			relationType: [...data.relationType],
			region: isMember ? data.region : undefined,
			isSpecial: isMember ? data.isSpecial : false
		};

		saveRelation(newData);
		// return true;
	}

	let isDataValid = $derived(
		data.name.trim().length > 0 &&
			data.city.trim().length > 0 &&
			data.relationType.length > 0 &&
			(isMember ? data.region?.trim().length !== 0 : true)
	);
	// $: console.log(relationTypes
	// $inspect(data);
</script>

<Modal
	id="rel-mod"
	bind:open={open}
	preventCloseOnClickOutside
	modalHeading={'Relasi'}
	hasForm
	primaryButtonText="Simpan"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={'#rel-name'}
	on:click:button--secondary={() => (open = false)}
	on:submit={save}
	size="sm"
	primaryButtonDisabled={isUpdating || !isDataValid}
>
	<FluidForm>
		<TextInput
			id="rel-name"
			bind:value={data.name}
			labelText={'Nama'}
			placeholder="Junaedi"
			required
		/>
		<TextInput
			bind:value={data.street}
			labelText="Alamat"
			placeholder="e.g. Jl. Graha Sudirman Blok Acasia-4 No. 11 Kel. Lemahmekar"
		/>
		<TextInput
			bind:value={data.city}
			labelText="Kota"
			placeholder="Indramayu"
			size="sm"
		/>
		<TextInput
			bind:value={data.phone}
			labelText="Telepon/mobile"
			placeholder="08999998347"
			size="sm"
		/>
		<div class="divider"></div>
		<div style="margin-bottom: 3px;">Tipe relasi:</div>
		<Grid>
			<Row noGutter>
				{#each relationTypes as t (t.id)}
					<Column md>
						<Checkbox
							labelText={t.text}
							checked={data.relationType.filter((f) => f === t.id).length > 0}
							on:check={(e) => {
								if (e.detail) {
									const types = data.relationType.filter((f) => f !== t.id);
									data = {
										...data,
										relationType: [...types, t.id]
									};
								} else {
									const test = data.relationType.filter((f) => f !== t.id);
									data = { ...data, relationType: [...test] };
								}
							}}
						/>
					</Column>
				{/each}
			</Row>
		</Grid>
		<div class="divider"></div>
		<TextInput
			disabled={!isMember}
			bind:value={data.region}
			labelText="Rayon (Khusus pelanggan)"
			placeholder="e.g. Bangkir"
			size="sm"
		/>
		<div class="divider"></div>
		<Grid>
			<Row noGutter>
				<Column>
					<Checkbox
						labelText="? Khusus"
						bind:checked={data.isSpecial}
						disabled={!isMember}
					/>
				</Column>
				<Column>
					<Checkbox labelText="? Aktif" bind:checked={data.isActive} />
				</Column>
			</Row>
		</Grid>
	</FluidForm>
	<!-- <code>
			{JSON.stringify(data, null, 2)}
		</code> -->

	{#if isUpdating}
		<InlineLoading
			status="active"
			description={data.id === 0 ? 'Posting data...' : 'Updating data...'}
		/>
	{/if}

	{#if isError}
		<InlineLoading description={errorMessage} status="error" />
	{/if}
</Modal>
