<style lang="css">
	:global(label.bx--label) {
		margin-bottom: 3px;
		margin-top: 9px;
	}
	:global(.bx--checkbox-wrapper) {
		margin-top: 12px;
	}
	:global(.bx--text-input.input-number) {
		text-align: right;
	}
	:global(.bx--label--inline--sm) {
		min-width: 4.5rem;
		padding-right: 0;
		margin-right: 0;
	}
	:global(.bx--list-box__menu-item, .bx--list-box__menu-item__option) {
		height: auto;
	}

	.supplier-info {
		height: auto;
		font-size: x-small;
		margin-top: 3px;
		margin-bottom: 0;
		white-space: wrap;
	}
</style>

<script lang="ts">
	import type { iProduct, iPropertyID, iRelationProp } from '$lib/interfaces';
	import {
		Checkbox,
		Column,
		ComboBox,
		Form,
		Grid,
		InlineLoading,
		Modal,
		Row,
		TextArea,
		TextInput
	} from 'carbon-components-svelte';
	import { Save } from 'carbon-icons-svelte';

	// import type { ComboBoxItem } from "carbon-components-svelte/types/ComboBox/ComboBox.svelte";
	import { formatNumber, getNumber, getPercent } from './NumberFormat';
	import InputNumber from './NumberInput.svelte';
	import NumberPercent from './NumberPercent.svelte';
	import type { ComboBoxItem } from 'carbon-components-svelte/src/ComboBox/ComboBox.svelte';
	import { onMount } from 'svelte';

	interface Props {
		innerWidth: number;
		open: boolean;
		data: iProduct;
		isError: boolean;
		isUpdating: boolean;
		errorMessage: string;
		suppliers: iRelationProp[];
		categories: iPropertyID[];
		onSubmit: (e: iProduct) => void;
		onReset?: () => void;
	}

	let {
		innerWidth = $bindable(720),
		open = $bindable(false),
		isError = $bindable(false),
		isUpdating = $bindable(false),
		data = $bindable(),
		errorMessage = $bindable(''),
		suppliers = [],
		categories = [],
		onSubmit,
		onReset
	}: Props = $props();

	// let cat_id = "" + data.categoryId;
	//   let prop_id = "test: " + cat_id;
	// let category: ComboBoxItem = {
	//      id: cat_id,
	//      text: ""
	//  }
	function toNumber(v: string | number | undefined): number {
		if (v == undefined || v === null) return 0;
		if (typeof v === 'string') {
			return +v;
		}
		return v;
	}

	function submit() {
		isUpdating = true;
		// console.log(data)
		onSubmit(data);
	}

	function on_hpp_change(_e: CustomEvent<string | number | null>): void {
		data.price = data.hpp + (data.margin * data.hpp) / 100;
		str_price = formatNumber(data.price);
	}

	function on_price_change(_e: CustomEvent<string | number | null>): void {
		data.margin = ((data.price - data.hpp) / data.hpp) * 100;
		str_percent = formatNumber(data.margin, 4);
	}

	function on_percent_change(_e: CustomEvent<string | number | null>): void {
		data.price = data.hpp + (data.margin * data.hpp) / 100;
		str_price = formatNumber(data.price);
		// console.log(cardNumber(data.price.toString()), data.price.toString());
	}

	function shouldFilterItem(item: ComboBoxItem, value: string) {
		if (!value) return true;
		return item.text.toLowerCase().includes(value.toLowerCase());
	}

	function get_supplier_info(id: number): string {
		let item = suppliers.filter((f) => f.id === id)[0];

		if (item) {
			let info: string;

			if (item.street) {
				info = item.street;
				info = info + ' - ' + item.city;
			} else {
				info = item.city;
			}

			if (item.phone) {
				info = info + ', ' + item.phone;
			}
			return info;
		}
		return '-';
	}

	function get_suppliers() {
		return suppliers.map((m) => ({ id: m.id, text: m.text }));
	}

	let str_price = $state('0');
	let str_hpp = $state('0');
	let str_percent = $state('0');
	let str_heavy = $state('0.0');

	$effect.pre(() => {
		str_price = formatNumber(toNumber(data.price));
		str_hpp = formatNumber(toNumber(data.hpp));
		str_percent = formatNumber(toNumber(data.margin), 4);
		str_heavy = formatNumber(toNumber(data.heavy), 2);
	});
	//	let str_stock = cardNumber(data.unitInStock.toString());

	// $:	console.log(str_price, str_hpp, str_percent)

	//$: data.categoryId = +cat_id;

	onMount(() => {
		isError = false;
	});

	let category_invalid = $derived(data.categoryId === 0);
	let supplier_invalid = $derived(data.supplierId === 0);
	let name_invalid = $derived(data.name.trim().length === 0);
	let barcode_invalid = $derived(data.barcode.trim().length < 8);
	let unit_invalid = $derived(data.unit.trim().length === 0);
	let is_barcode_invalid = $derived(data.barcode.trim().length === 0);
	let price_invalid = $derived(data.price <= data.hpp);

	$effect(() => {
		data.price = getNumber(str_price);
		data.margin = getPercent(str_percent);
		data.hpp = getNumber(str_hpp);
		data.heavy = getPercent(str_heavy);
	});

	//	$: data.unitInStock = getNumber(str_stock);
	// $: console.log(getNumber(str_price));

	let noGutter = $state(innerWidth > 640);
	let md = $state(innerWidth < 640);

	let isDataValid = $derived(
		category_invalid ||
			supplier_invalid ||
			is_barcode_invalid ||
			name_invalid ||
			unit_invalid ||
			price_invalid
	);

	$effect(() => {
		noGutter = innerWidth > 640;
		md = innerWidth < 640;
	});

	$inspect(str_price, str_hpp);
</script>

<Modal
	bind:open={open}
	on:close={() => {
		if (onReset) {
			onReset();
		}
	}}
	hasForm
	preventCloseOnClickOutside
	modalHeading={'Data Barang'}
	primaryButtonText="Simpan"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={'#prod-name'}
	on:click:button--secondary={() => {
		if (onReset) {
			onReset();
		}
	}}
	on:click:button--primary={submit}
	size="sm"
	primaryButtonDisabled={isUpdating || isDataValid}
>
	<Form>
		<Grid bind:noGutter={noGutter}>
			<Row>
				<Column bind:md={md}>
					<TextInput
						inline={innerWidth < 720}
						warn={name_invalid}
						size="sm"
						id="prod-name"
						bind:value={data.name}
						labelText={'Nama'}
						placeholder="e.g. Djarum Super"
						maxlength={50}
						required
					/>
					<TextInput
						inline={innerWidth < 720}
						warn={barcode_invalid}
						size="sm"
						bind:value={data.barcode}
						labelText="Barcode"
						placeholder="e.g. 0899984564698"
						maxlength={25}
					/>
					<TextInput
						inline={innerWidth < 720}
						warn={unit_invalid}
						bind:value={data.unit}
						labelText="Unit"
						placeholder="e.g pcs"
						size="sm"
						maxlength={6}
					/>
					{#if innerWidth > 720}
						<br />
					{/if}
					<InputNumber
						inline
						bind:value={str_hpp}
						labelText="Harga beli"
						size="sm"
						on:change={on_hpp_change}
					/>
					<NumberPercent
						inline
						bind:value={str_percent}
						labelText="Margin: (%)"
						size="sm"
						on:change={on_percent_change}
					/>
					<InputNumber
						inline
						bind:value={str_price}
						labelText="Harga jual"
						size="sm"
						on:change={on_price_change}
						warn={price_invalid}
						warnText="Harga jual harus lebih besar dari harga beli / hpp."
					/>
				</Column>
				<Column bind:md={md}>
					<ComboBox
						size="sm"
						titleText="Kategori"
						placeholder="Pilih kategori"
						selectedId={data.categoryId}
						warn={category_invalid}
						items={categories}
						shouldFilterItem={shouldFilterItem}
						on:select={(e) => (data.categoryId = e.detail.selectedId)}
						on:clear={() => (data.categoryId = 0)}
					/>
					<ComboBox
						titleText="Supplier"
						selectedId={data.supplierId}
						placeholder="Pilih supplier"
						warn={supplier_invalid}
						items={get_suppliers()}
						shouldFilterItem={shouldFilterItem}
						on:select={(e) => (data.supplierId = e.detail.selectedId)}
						on:clear={() => (data.supplierId = 0)}
						let:item
					>
						<div><strong>{item.text}</strong></div>
						<div class="supplier-info">
							{get_supplier_info(item.id)}
						</div>
					</ComboBox>
					<NumberPercent
						bind:value={str_heavy}
						labelText="Berat: (kg)"
						size="sm"
					/>
					<TextInput
						bind:value={data.variantName}
						labelText="Variant"
						placeholder="e.g. isi 12 batang"
						size="sm"
						maxlength={50}
					/>
					<TextArea
						rows={2}
						bind:value={data.descriptions}
						labelText="Deskripsi"
						placeholder="e.g. diproduksi oleh PT. Djarum Super Kudus"
						maxlength={128}
					/>
					<Checkbox labelText="? Aktif" bind:checked={data.isActive} />
				</Column>
			</Row>
		</Grid>
	</Form>

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
