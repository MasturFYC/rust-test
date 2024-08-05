<script lang="ts">
	import type { iProduct, iPropertyID, iRelationProp } from "$lib/interfaces";
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
	} from "carbon-components-svelte";
	import { Save } from "carbon-icons-svelte";
	import { createEventDispatcher, onMount } from "svelte";

	import type { ComboBoxItem } from "carbon-components-svelte/types/ComboBox/ComboBox.svelte";
	import { formatRupiah, getNumber, getPercent } from "./NumberFormat";
	import InputNumber from "./NumberInput.svelte";
	import NumberPercent from "./NumberPercent.svelte";

	export let innerWidth = 720;
	export let open = false;
	export let data: iProduct;
	export let isError = false;
	export let isUpdating = false;
	export let errorMessage = "";
	export let suppliers: iRelationProp[] = [];
	export let categories: iPropertyID[] = [];

	const dispatch = createEventDispatcher();
	// let cat_id = "" + data.categoryId;
	//   let prop_id = "test: " + cat_id;
	// let category: ComboBoxItem = {
	//      id: cat_id,
	//      text: ""
	//  }

	function submit() {
		isUpdating = true;
		dispatch("submit", data);
	}

	function on_hpp_change(e: CustomEvent<string | number | null>): void {
		data.price = data.hpp + (data.margin * data.hpp) / 100;
		str_price = formatRupiah(data.price);
	}

	function on_price_change(e: CustomEvent<string | number | null>): void {
		data.margin = ((data.price - data.hpp) / data.hpp) * 100;
		str_percent = formatRupiah(data.margin, 4);
	}

	function on_percent_change(e: CustomEvent<string | number | null>): void {
		data.price = data.hpp + (data.margin * data.hpp) / 100;
		str_price = formatRupiah(data.price);
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
				info = info + " - " + item.city;
			} else {
				info = item.city;
			}
			if (item.phone) {
				info = info + ", " + item.phone;
			}

			return info;
		}
		return "-";
	}

	function get_suppliers() {
		return suppliers.map((m) => ({ id: m.id, text: m.text }));
	}

	let str_price = formatRupiah(data.price);
	let str_hpp = formatRupiah(data.hpp);
	let str_percent = formatRupiah(data.margin, 4);
	let str_heavy = formatRupiah(data.heavy, 2);
	//	let str_stock = cardNumber(data.unitInStock.toString());

	// $:	console.log(str_price, str_hpp, str_percent)

	//$: data.categoryId = +cat_id;

	onMount(() => {
		isError = false;
	})

	$: category_invalid = data.categoryId === 0;
	$: supplier_invalid = data.supplierId === 0;
	$: name_invalid = data.name.trim().length === 0;
	$: barcode_invalid = data.barcode.trim().length < 8;
	$: unit_invalid = data.unit.trim().length === 0;
	$: is_barcode_invalid = data.barcode.trim().length === 0;

	$: data.price = getNumber(str_price);
	$: data.margin = getPercent(str_percent);
	$: data.hpp = getNumber(str_hpp);
	$: data.heavy = getPercent(str_heavy);
	//	$: data.unitInStock = getNumber(str_stock);
	// $: console.log(getNumber(str_price));

	$: price_invalid = data.price <= data.hpp;

	$: noGutter = innerWidth > 640;
	$: md = innerWidth < 640;
	$: isDataValid = (category_invalid || supplier_invalid  || is_barcode_invalid || name_invalid || unit_invalid || price_invalid);
</script>

<Modal
	bind:open
	hasForm
	preventCloseOnClickOutside
	modalHeading={"Data Barang"}
	primaryButtonText="Simpan"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={"#prod-name"}
	on:click:button--secondary={() => (open = false)}
	on:click:button--primary={submit}
	size="sm"
	primaryButtonDisabled={isUpdating || isDataValid}
>
	<Form>
		<Grid bind:noGutter>
			<Row>
				<Column bind:md>
					<TextInput
						inline={innerWidth<720}
						warn={name_invalid}
						size="sm"
						id="prod-name"
						bind:value={data.name}
						labelText={"Nama"}
						placeholder="e.g. Djarum Super"
						maxlength={50}
						required
					/>
					<TextInput
						inline={innerWidth<720}
						warn={barcode_invalid}
						size="sm"
						bind:value={data.barcode}
						labelText="Barcode"
						placeholder="e.g. 0899984564698"
						maxlength={25}
					/>
					<TextInput
						inline={innerWidth<720}
						warn={unit_invalid}
						bind:value={data.unit}
						labelText="Unit"
						placeholder="e.g pcs"
						size="sm"
						maxlength={6}
					/>
					{#if innerWidth>720}
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
				<Column bind:md>
					<ComboBox
						size="sm"
						titleText="Kategori"
						placeholder="Pilih kategori"
						selectedId={data.categoryId}
						warn={category_invalid}
						items={categories}
						{shouldFilterItem}
						on:select={(e) => (data.categoryId = e.detail.selectedId)}
						on:clear={() => (data.categoryId = 0)}
					/>
					<ComboBox
						titleText="Supplier"
						selectedId={data.supplierId}
						placeholder="Pilih supplier"
						warn={supplier_invalid}
						items={get_suppliers()}
						{shouldFilterItem}
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
			description={data.id === 0 ? "Posting data..." : "Updating data..."}
		/>
	{/if}

	{#if isError}
		<InlineLoading description={errorMessage} status="error" />
	{/if}
</Modal>

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
