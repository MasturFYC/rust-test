<script lang="ts">
	import type { iRelationProp, iStock } from "$lib/interfaces";
	import {
		Column,
		ComboBox,
		DatePicker,
		DatePickerInput,
		Form,
		Grid,
		Row,
		TextInput,
	} from "carbon-components-svelte";
	import type { ComboBoxItem } from "carbon-components-svelte/types/ComboBox/ComboBox.svelte";
	import dayjs from "dayjs";
	import InputNumber from "$lib/components/NumberInput.svelte";
	import { formatRupiah, getNumber } from "$lib/components/NumberFormat";

	export let data: iStock;
	export let suppliers: iRelationProp[] = [];
	export let employees: iRelationProp[] = [];
	export let innerWidth = 0;
	let ref_invoice: HTMLInputElement;

	function get_suppliers() {
		return suppliers.map((m) => ({ id: m.id, text: m.text }));
	}
	function get_employees() {
		return employees.map((m) => ({ id: m.id, text: m.text }));
	}

	function shouldFilterItem(item: ComboBoxItem, value: string) {
		if (!value) return true;
		return item.text.toLowerCase().includes(value.toLowerCase());
	}
	function get_employee_info(id: number): string {
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
	function on_employee_changed(
		e: CustomEvent<{ selectedId: any; selectedItem: ComboBoxItem }>,
	): void {
		const empl = employees.filter((f) => f.id === e.detail.selectedId)[0];
		if (empl) {
			data.warehouseId = empl.id;
			data.warehouseName = empl.text;
		}
	}
	function on_supplier_changed(
		e: CustomEvent<{ selectedId: any; selectedItem: ComboBoxItem }>,
	): void {
		const empl = suppliers.filter((f) => f.id === e.detail.selectedId)[0];
		if (empl) {
			data.supplierId = empl.id;
			data.supplierName = empl.text;
		}
	}
	function on_employee_clear(e: any): void {
		data.warehouseId = 0;
		data.warehouseName = undefined;
	}
	function on_supplier_clear(e: any): void {
		data.supplierId = 0;
		data.supplierName = undefined;
	}

	type DatePict =
		| string
		| {
				selectedDates: [dateFrom: Date, dateTo?: Date];
				dateStr:
					| string
					| {
							from: string;
							to: string;
					  };
		  };

	function onDateChange(e: CustomEvent<DatePict>) {
		e.preventDefault();
		if (typeof e.detail === "string") {
		} else {
			let d = e.detail.selectedDates[0];
			let date = dayjs();
			date = date.set("date", d.getDate());
			date = date.set("month", d.getMonth());
			date = date.set("year", d.getFullYear());
			data.createdAt = date.format();
		}
	}
	function onDueDateChange(e: CustomEvent<DatePict>) {
		e.preventDefault();
		if (typeof e.detail === "string") {
		} else {
			let d = e.detail.selectedDates[0];
			let date = dayjs();
			date = date.set("date", d.getDate());
			date = date.set("month", d.getMonth());
			date = date.set("year", d.getFullYear());
			data.dueAt = date.add(7, "day").format();
		}
	}

	function onDpChange(e: CustomEvent<string | number | null>): void {
		data.remain = data.total - (data.payment + data.dp);
	}

	let strDate = dayjs(data.createdAt).format("DD-MM-YYYY");
	let strDueAt = dayjs(data.dueAt).format("DD-MM-YYYY");
	let strTotal = formatRupiah(data.total);
	let strDp = formatRupiah(data.dp);

	$: if (ref_invoice) {
		ref_invoice.focus();
	}

	$: data.dp = getNumber(strDp);
	$: strRemain = formatRupiah(data.remain);
	// $: data.createdAt = strDate;
	// $: console.log(strDate);
</script>

<Form on:submit>
	<Grid noGutter={innerWidth > 720}>
		<Row>
			<Column sm noGutterRight>
				<DatePicker
					datePickerType="single"
					bind:value={strDate}
					dateFormat="d-m-Y"
					on:change={onDateChange}
				>
					<DatePickerInput
					tabindex={1}
						style={`width: ${innerWidth < 720 ? "163px" : "none"}`}
						labelText="Tanggal pembelian"
						placeholder="mm/dd/yyyy"
					/>
				</DatePicker>
			</Column>
			<Column sm noGutter>
				<TextInput
					tabindex={2}
					style={`width: ${innerWidth < 720 ? "none" : "auto"}`}
					id="invoice-id"
					bind:ref={ref_invoice}
					labelText="No. faktur pembelian"
					bind:value={data.invoiceId}
				/>
			</Column>
			<Column sm noGutter>
				<ComboBox
					tabindex={3}
					style={`width: ${innerWidth < 720 ? "163px" : "none"}`}
					id="supplier-id"
					titleText="Supplier"
					selectedId={data.supplierId}
					placeholder="Pilih supplier"
					items={get_suppliers()}
					{shouldFilterItem}
					on:select={on_supplier_changed}
					on:clear={on_supplier_clear}
					let:item
				>
					<div><strong>{item.text}</strong></div>
					<div class="supplier-info">
						{get_supplier_info(item.id)}
					</div>
				</ComboBox>
			</Column>
			<Column sm noGutterLeft>
				<ComboBox
					tabindex={4}
					id="warehouse-id"
					titleText="Penjaga gudang"
					selectedId={data.warehouseId}
					placeholder="Pilih penjaga gudang"
					items={get_employees()}
					{shouldFilterItem}
					on:select={on_employee_changed}
					on:clear={on_employee_clear}
					let:item
				>
					<div><strong>{item.text}</strong></div>
					<div class="supplier-info">
						{get_employee_info(item.id)}
					</div>
				</ComboBox>
			</Column>
		</Row>
		<Row>
			<Column sm noGutterRight>
				<DatePicker
					datePickerType="single"
					bind:value={strDueAt}
					dateFormat="d-m-Y"
					on:change={onDueDateChange}
				>
					<DatePickerInput
					tabindex={5}
						size="sm"
						style={`width: ${innerWidth < 720 ? "163px" : "none"}`}
						labelText="Jatuh tempo"
						placeholder="mm/dd/yyyy"
					/>
				</DatePicker>
			</Column>
			<Column sm noGutter>
				<InputNumber tabindex={6} labelText="Total" bind:value={strTotal} />
			</Column>
			<Column sm noGutter>
				<InputNumber
					tabindex={7}
					style={`width: ${innerWidth < 720 ? "163px" : "none"}`}
					labelText="Cash / DP"
					bind:value={strDp}
					on:change={onDpChange}
				/>
			</Column>
			<Column sm noGutterLeft>
				<InputNumber
					tabindex={8}
					labelText="Sisa pembayaran"
					bind:value={strRemain}
				/>
			</Column>
		</Row>
	</Grid>
</Form>

<div>
	<!-- <code><pre>{JSON.stringify(data, null, 4)}</pre></code> -->
</div>

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
