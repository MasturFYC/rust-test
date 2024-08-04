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

	function submit() {
        dispatch("submit", data);
	}

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
	primaryButtonDisabled={isUpdating}
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
		<Grid noGutter>
			<Row>
				{#each relationTypes as t}
					<Column>
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
		<Grid noGutterLeft>
			<Row>
				<Column>
					<Checkbox labelText="? Khusus" bind:checked={data.isSpecial} />
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
		margin-top: 6px;
		// border-top: 1px solid;
		// border-color: #999;
	}
</style>
