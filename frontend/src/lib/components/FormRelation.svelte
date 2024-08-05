<script lang="ts">
	import type { iRelation, RelationTypeWIthID } from "$lib/interfaces";
	import {
		Checkbox,
		Column,
		FluidForm,
		Grid,
		InlineLoading,
		Modal,
		Row,
		TextInput,
	} from "carbon-components-svelte";
	import { Save } from "carbon-icons-svelte";
	import { createEventDispatcher } from "svelte";

	export let open = false;
	export let data: iRelation;
	export let isError = false;
	export let isUpdating = false;
	export let errorMessage = "";
	export let relationTypes: RelationTypeWIthID[] = [];

	const dispatch = createEventDispatcher();

$: isMember = data.relationType.filter(f => f === 'Customer').length > 0;

	function submit() {

		dispatch("submit", {...data,
			region: isMember ? data.region : undefined,
			isSpecial: isMember ? data.isSpecial : false
		});
	}

	$: isDataValid = (data.name.trim().length > 0 && data.city.trim().length > 0 && data.relationType.length > 0 && (isMember ? data.region?.trim().length !== 0 : true) );
	// $: console.log(relationTypes)
</script>

<Modal
	bind:open
	hasForm
	preventCloseOnClickOutside
	modalHeading={"Relasi"}
	primaryButtonText="Simpan"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={"#rel-name"}
	on:click:button--secondary={() => (open = false)}
	on:click:button--primary={submit}
	size="sm"
	primaryButtonDisabled={isUpdating || !(isDataValid)}
>
	<FluidForm>
		<TextInput
			id="rel-name"
			bind:value={data.name}
			labelText={"Nama"}
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
		<div class="divider" />
		<div style="margin-bottom: 6px;">Tipe relasi:</div>
		<Grid>
			<Row noGutter>
				{#each relationTypes as t}
					<Column md>
						<Checkbox
							labelText={t.text}
							checked={data.relationType.filter((f) => f === t.id).length > 0}
							on:check={(e) => {
								if (e.detail) {
									data = {
										...data,
										relationType: [...data.relationType, t.id],
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
		<div class="divider" />
		<TextInput
			disabled={!isMember}
			bind:value={data.region}
			labelText="Rayon (Khusus pelanggan)"
			placeholder="e.g. Bangkir"
			size="sm"
		/>
		<div class="divider" />
		<Grid>
			<Row noGutter>
				<Column>
					<Checkbox labelText="? Khusus" bind:checked={data.isSpecial} disabled={!isMember} />
				</Column>
				<Column>
					<Checkbox labelText="? Aktif" bind:checked={data.isActive} />
				</Column>
			</Row>
		</Grid>
		<!-- <code>
			{JSON.stringify(data, null, 2)}
		</code> -->
	</FluidForm>

	{#if isUpdating}
		<InlineLoading
			status="active"
			description={data.id === 0 ? "Posting data..." : "Updating data..."}
		/>
	{/if}

	{#if isError}
		<InlineLoading description={errorMessage} status="error" />
	{/if}
</Modal>

<style lang="scss">
	.divider {
		margin-top: 16px;
		// border-top: 1px solid;
		// border-color: #999;
	}
</style>
