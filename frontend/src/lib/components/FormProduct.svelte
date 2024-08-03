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
		TextInput,
	} from "carbon-components-svelte";
	import { Save } from "carbon-icons-svelte";
	import { createEventDispatcher } from "svelte";

	import type { ComboBoxItem } from "carbon-components-svelte/types/ComboBox/ComboBox.svelte";
	import {
		cardNumber,
		cardPercent,
		getNumber,
		getPercent,
	} from "./NumberFormat";
	import NumberInput from "./NumberInput.svelte";
	import NumberPercent from "./NumberPercent.svelte";

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
		str_price = cardNumber(data.price.toString());
	}

	function on_price_change(e: CustomEvent<string | number | null>): void {
		data.margin = ((data.price - data.hpp) / data.hpp) * 100;
		str_percent = cardPercent(data.margin.toString());
	}

	function on_percent_change(e: CustomEvent<string | number | null>): void {
		data.price = data.hpp + (data.margin * data.hpp) / 100;
		str_price = cardNumber(data.price.toString());
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

	let str_price = cardNumber(data.price.toString());
	let str_hpp = cardNumber(data.hpp.toString());
	let str_percent = cardPercent(data.margin.toString());
//	let str_stock = cardNumber(data.unitInStock.toString());

	//$: data.categoryId = +cat_id;

	$: category_invalid = data.categoryId === 0;
	$: supplier_invalid = data.supplierId === 0;
	$: name_invalid = data.name.trim().length === 0;
	$: barcode_invalid = data.barcode.trim().length < 8;
	$: unit_invalid = data.unit.trim().length === 0;
	$: data.price = getNumber(str_price);
	$: data.margin = getPercent(str_percent);
	$: data.hpp = getNumber(str_hpp);
//	$: data.unitInStock = getNumber(str_stock);

	$: price_invalid = data.price <= data.hpp;
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
	primaryButtonDisabled={isUpdating}
>
	<Form>
		<Grid noGutter>
			<Row>
				<Column>
					<TextInput
						warn={name_invalid}
						size="sm"
						id="prod-name"
						bind:value={data.name}
						labelText={data.id === 0 ? "Nama" : "ID: " + data.id}
						placeholder="e.g. Djarum Super"
						maxlength={50}
						required
					/>
					<TextInput
						warn={barcode_invalid}
						size="sm"
						bind:value={data.barcode}
						labelText="Barcode"
						placeholder="e.g. 0899984564698"
						maxlength={25}
					/>
					<TextInput
						warn={unit_invalid}
						bind:value={data.unit}
						labelText="Unit"
						placeholder="e.g pcs"
						size="sm"
						maxlength={6}
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
				</Column>
				<Column>
					<ComboBox
						size="sm"
						titleText="Kategori"
						placeholder="Pilih kategori"
						selectedId={data.categoryId}
						warn={category_invalid}
						items={categories}
						{shouldFilterItem}
						on:select={(e) =>
							(data.categoryId = e.detail.selectedId)}
						on:clear={() => (data.categoryId = 0)}
					/>
					<ComboBox
						titleText="Supplier"
						selectedId={data.supplierId}
						placeholder="Pilih supplier"
						warn={supplier_invalid}
						items={get_suppliers()}
						{shouldFilterItem}
						on:select={(e) =>
							(data.supplierId = e.detail.selectedId)}
						on:clear={() => (data.supplierId = 0)}
						let:item
					>
						<div><strong>{item.text}</strong></div>
						<div class="supplier-info">
							{get_supplier_info(item.id)}
						</div>
					</ComboBox>
					<br />
					<NumberInput
						inline
						bind:value={str_hpp}
						labelText="Harga beli"
						placeholder="e.g pcs"
						size="sm"
						on:change={on_hpp_change}
					/>
					<NumberPercent
						inline
						bind:value={str_percent}
						labelText="Margin: (%)"
						placeholder="e.g 10%"
						size="sm"
						on:change={on_percent_change}
					/>
					<NumberInput
						inline
						bind:value={str_price}
						labelText="Harga jual"
						placeholder="e.g pcs"
						size="sm"
						on:change={on_price_change}
						warn={price_invalid}
						warnText="Harga jual harus lebih besar dari harga beli / hpp."
					/>
					<Checkbox
						labelText="? Aktif"
						bind:checked={data.isActive}
					/>
				</Column>
			</Row>
		</Grid>
	</Form>

	{#if isUpdating}
		<InlineLoading
			status="active"
			description={data.id === 0
				? "Posting data..."
				: "Updating data..."}
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
