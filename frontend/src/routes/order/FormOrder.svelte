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

	.customer-info {
		height: auto;
		font-size: x-small;
		margin-top: 3px;
		margin-bottom: 0;
		white-space: wrap;
	}
</style>

<script lang="ts">
	import type { iRelationProp } from '$lib/interfaces';
	import {
		Column,
		ComboBox,
		DatePicker,
		DatePickerInput,
		Form,
		Grid,
		Row
	} from 'carbon-components-svelte';
	import dayjs from 'dayjs';
	// import { formatNumber, getNumber } from '$lib/components/NumberFormat';
	import { order } from './store';
	// import { toNumber } from './handler';
	import type { ComboBoxItem } from 'carbon-components-svelte/src/ComboBox/ComboBox.svelte';

	// const dispatch = createEventDispatcher();

	interface Props {
		customers: iRelationProp[] | undefined;
		sales: iRelationProp[] | undefined;
		innerWidth: number;
	}

	let {
		customers = [],
		sales = [],
		innerWidth = $bindable(0)
	}: Props = $props();

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

	let refCombo: null | HTMLInputElement = $state(null);

	function get_customers() {
		return customers.map((m) => ({ id: m.id, text: m.text }));
	}

	function get_sales() {
		return sales.map((m) => ({ id: m.id, text: m.text }));
	}

	function shouldFilterItem(item: ComboBoxItem, value: string) {
		if (!value) return true;
		return item.text.toLowerCase().includes(value.toLowerCase());
	}

	function get_sales_info(id: number): string {
		let item = customers.filter((f) => f.id === id)[0];

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

	function get_customer_info(id: number): string {
		let item = customers.filter((f) => f.id === id)[0];

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

	function on_sales_changed(
		e: CustomEvent<{ selectedId: any; selectedItem: ComboBoxItem }>
	): void {
		const empl = sales.filter((f) => f.id === e.detail.selectedId)[0];
		if (empl) {
			order.update((s) => ({
				...s,
				salesId: empl.id,
				salesName: empl.text,
				isModified: true
			}));
		}
	}

	function on_customer_changed(
		e: CustomEvent<{ selectedId: any; selectedItem: ComboBoxItem }>
	): void {
		const cust = customers.filter((f) => f.id === e.detail.selectedId)[0];
		if (cust) {
			order.update((s) => ({
				...s,
				customerId: cust.id,
				customerName: cust.text,
				isModified: true
			}));
		}
	}
	function on_sales_clear(_e: any): void {
		order.update((s) => ({ ...s, salesId: 0, salesName: undefined }));
	}

	function on_customer_clear(_e: any): void {
		order.update((s) => ({ ...s, customerId: 0, customerName: undefined }));
	}

	function onDateChange(e: CustomEvent<DatePict>) {
		e.preventDefault();
		if (typeof e.detail === 'string') {
		} else {
			let d = e.detail.selectedDates[0];
			let date = dayjs();
			date = date.set('date', d.getDate());
			date = date.set('month', d.getMonth());
			date = date.set('year', d.getFullYear());
			order.update((s) => ({
				...s,
				createdAt: date.format(),
				isModified: true
			}));
		}
	}
	function onTempoChange(e: CustomEvent<DatePict>) {
		e.preventDefault();
		if (typeof e.detail === 'string') {
		} else {
			let d = e.detail.selectedDates[0];
			let date = dayjs();
			date = date.set('date', d.getDate());
			date = date.set('month', d.getMonth());
			date = date.set('year', d.getFullYear());
			order.update((s) => ({
				...s,
				dueAt: date.format(),
				isModified: true
			}));
		}
	}

	let strDate = $state(dayjs($order.createdAt).format('DD-MM-YYYY'));
	let strTempo = $state(dayjs($order.dueAt).format('DD-MM-YYYY'));
	// let strDp = $derived(formatNumber(toNumber($order.dp)));

	$effect(() => {
		if (refCombo) {
			refCombo.focus();
		}
	});

	// $inspect(refCombo);
	// function updateDp(str: string) {
	// 	const dp = getNumber(str);
	// 	order.update((s) => ({
	// 		...s,
	// 		dp: dp,
	// 		total: toNumber(s.total) - (toNumber(s.payment) + dp)
	// 	}));
	// }

	// $effect(() => updateDp(strDp));
</script>

<Form on:submit>
	<Grid noGutter={innerWidth > 720} fullWidth>
		<Row>
			<Column noGutterRight as sm={8} md>
				<Grid>
					<Row>
						<Column as noGutterLeft>
							<DatePicker
								short
								datePickerType="single"
								bind:value={strDate}
								dateFormat="d-m-Y"
								on:change={onDateChange}
							>
								<DatePickerInput
									style={'width:100%'}
									accesskey="t"
									labelText="Tanggal penjualan"
									placeholder="mm/dd/yyyy"
								/>
							</DatePicker>
						</Column>
						<Column noGutter as>
							<DatePicker
								datePickerType="single"
								bind:value={strTempo}
								dateFormat="d-m-Y"
								on:change={onTempoChange}
							>
								<DatePickerInput
									accesskey="t"
									style={'width:100%'}
									labelText="Jatuh tempo"
									placeholder="mm/dd/yyyy"
								/>
							</DatePicker>
						</Column>
					</Row>
				</Grid>
			</Column>

			<Column noGutterLeft sm={8} md>
				<Grid noGutter={innerWidth > 720}>
					<Row>
						<Column sm={2} noGutter>
							<ComboBox
								accesskey="s"
								bind:ref={refCombo}
								id="customer-id"
								titleText="Pelanggan"
								selectedId={$order.customerId}
								placeholder="Pilih pelanggan"
								items={get_customers()}
								shouldFilterItem={shouldFilterItem}
								on:select={on_customer_changed}
								on:clear={on_customer_clear}
								let:item
							>
								<div><strong>{item.text}</strong></div>
								<div class="customer-info">
									{get_customer_info(item.id)}
								</div>
							</ComboBox>
						</Column>
						<Column sm={2} noGutterLeft>
							<ComboBox
								accesskey="g"
								id="sales-id"
								titleText="Sales"
								selectedId={$order.salesId}
								placeholder="Pilih sales"
								items={get_sales()}
								shouldFilterItem={shouldFilterItem}
								on:select={on_sales_changed}
								on:clear={on_sales_clear}
								let:item
							>
								<div><strong>{item.text}</strong></div>
								<div class="customer-info">
									{get_sales_info(item.id)}
								</div>
							</ComboBox>
						</Column>
					</Row>
				</Grid>
			</Column>
		</Row>
	</Grid>
</Form>
