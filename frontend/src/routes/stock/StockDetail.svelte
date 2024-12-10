<!-- <div>{JSON.stringify(selectedRowIds, null,  4)}</div> -->
<style lang="css">
	:global(#combo-gudang-id.bx--combo-box) {
		height: auto;
		border: 0;
		max-height: 16px;
		padding: 0;
		margin: 0;
	}
	:global(#combo-gudang-id.bx--list-box--sm) {
		border: 0;
		max-height: 16px;
		padding: 0;
		margin: 0;
	}
</style>

<script lang="ts">
	import {
		formatNumber,
		getNumber,
		getPercent
	} from '$lib/components/NumberFormat';
	import NumberInput from '$lib/components/NumberInput.svelte';
	import NumberPercent from '$lib/components/NumberPercent.svelte';
	import {
		baseURL,
		credential_include,
		type iGudang,
		type iStockDetail
	} from '$lib/interfaces';
	import {
		Button,
		DataTable,
		TextInput,
		ToastNotification,
		Toolbar,
		ToolbarContent,
		ComboBox
	} from 'carbon-components-svelte';
	import { onDestroy, onMount, tick } from 'svelte';
	import ExpandedRow from './ExpandedRow.svelte';
	import '$assets/styles.css';
	import dayjs from 'dayjs';
	import {
		Add,
		Delete,
		NewTab,
		Save,
		Logout,
		Money
	} from 'carbon-icons-svelte';
	import { toNumber } from './handler';
	import { stock, details } from './store';
	import { isStockUpdating } from './store';
	import type {
		DataTableHeader,
		DataTableRow
	} from 'carbon-components-svelte/src/DataTable/DataTable.svelte';

	interface Props {
		barcodes: { barcode: string }[] | undefined;
		gudangs: iGudang[] | undefined;
		onclose: (e: number) => void;
		onadddp: () => void;
		onnew: (e: number) => void;
		onsave: (e: iStockDetail[]) => void;
		onnotfound: (e: string) => void;
	}

	let {
		barcodes = [],
		gudangs = [],
		onclose,
		onadddp,
		onnew,
		onsave,
		onnotfound
	}: Props = $props();

	let reform: HTMLDivElement;
	let isDirty = $state(false);
	let isBarcodeDirty = $state(false);
	let timeout: number | undefined = $state(undefined);
	let notifyTitle = $state('Error');
	let notifySubtitle = $state('');
	let isQtyChanged = $state(false);
	let isDiscountChanged = $state(false);

	let headers: DataTableHeader[] = [
		{ key: 'barcode', value: 'Barcode', width: '12%' },
		{ key: 'name', value: 'Nama barang', width: 'auto' },
		{ key: 'qty', value: 'Qty', width: '70px' },
		{ key: 'unit', value: 'Unit', width: '40px' },
		{ key: 'price', value: 'Harga', width: '100px' },
		{ key: 'discount', value: 'Disc', width: '80px' },
		{ key: 'pot', value: 'Hrg-Pot', width: '90px' },
		{ key: 'subtotal', value: 'Subtotal', width: '100px' },
		{ key: 'gudangId', value: 'Simpan di Gudang', width: 'auto' }
	];

	const get_gudang = () => {
		return gudangs.map((m) => ({ id: m.id, text: m.name }));
	};

	let items = $state(0);
	let currentId = $state(0);
	let isAddNew = $state(false);

	// let currentDetail: iStockDetail;
	let currentKey = 'barcode';
	// let strQty = "0";
	// let strDiscount = "0";
	const initDetail: iStockDetail = {
		stockId: 0,
		id: 0,
		productId: 0,
		barcode: '',
		name: '',
		qty: 1,
		direction: 1,
		unit: '',
		hpp: 0,
		price: 0,
		discount: 0,
		subtotal: 0,
		oldQty: 0,
		oldGudangId: 0,
		gudangId: 1,
		gudangName: ''
	};

	function onRowClick(e: CustomEvent<DataTableRow>) {
		e.preventDefault();
		// const i = data.findIndex((f) => f.id === e.detail.id);
		// let d = data[i];
		// currentDetail = d;
		let setFocus = false;

		const i = $details.findIndex((f) => f.id === 0);

		if (i < 0) {
			details.update((o) => [
				...o,
				{ ...initDetail, gudangName: getDefaultGudangName() }
			]);
		}

		// setStringValue(currentDetail.qty, currentDetail.discount);

		if (currentId != e.detail.id) {
			setFocus = true;
			// console.log(currentKey);
			if (
				currentKey === 'name' ||
				currentKey === 'unit' ||
				currentKey === 'price' ||
				currentKey === 'subtotal' ||
				currentKey === 'pot'
			) {
				currentKey = 'barcode';
			}
		}

		currentId = e.detail.id;

		if (setFocus) {
			const ctlId = '#' + currentKey + '-id';
			setTimeout(() => {
				setFocuse(ctlId);
			}, 100);
		}
	}
	// async function onCellClik(e: CustomEvent<DataTableCell>) {
	//   e.preventDefault();
	//   await tick();
	//   currentKey = e.detail.key;
	// }

	function clickOutSize(event: any) {
		const withinBoundaries = event.composedPath().includes(reform);

		if (isAddNew) return true;
		// isDirty = false;

		if (withinBoundaries) {
			// console.log("Click happened inside element");
		} else {
			// console.log("Click happened **OUTSIDE** element");
			currentId = 0;
			const i = $details.findIndex((f) => f.id === 0);
			if (i >= 0) {
				const slices = [...$details];
				slices.splice(i, 1);
				details.update(() => slices);
			}
		}
	}

	function setFocuse(ctlId: string) {
		const elem = document.querySelector(ctlId) as HTMLInputElement;
		if (elem) {
			elem.focus();
			elem.select();
		}
	}

	function change_qty(e: string, id: number) {
		const i = $details.findIndex((f) => f.id === id);
		if (i >= 0) {
			const qty = getPercent(e);
			const d = $details[i];
			const c = {
				...d,
				qty: qty,
				subtotal: (toNumber(d.price) - toNumber(d.discount)) * qty
			};
			updateCurrentDetail(c);
			isDirty = false;
			isQtyChanged = false;
		}
	}

	function qtyOnChange(e: CustomEvent<string | number | null>, id: number) {
		if (typeof e.detail === 'string') {
			change_qty(e.detail, id);
		}
	}

	function qtyOnKeyDown(e: KeyboardEvent, id: number) {
		if (e.key === 'Enter') {
			e.preventDefault();
			currentKey = 'discount';
			const ctlId = '#' + currentKey + '-id';
			setFocuse(ctlId);
		} else if (e.key === 'Tab' && e.shiftKey) {
			currentKey = 'barcode';
		} else if (isQtyChanged && (e.key === 'ArrowDown' || e.key === 'ArrowUp')) {
			e.preventDefault();
			const el = e.currentTarget as HTMLInputElement;
			if (el) {
				currentKey = 'qty';
				change_qty(el.value, id);
			}
		}
	}

	function change_discount(e: string, id: number) {
		const i = $details.findIndex((f) => f.id === id);
		if (i >= 0) {
			const discount = getNumber(e);
			const d = $details[i];
			if (toNumber(d.price) - discount <= toNumber(d.hpp)) {
				return true;
			}
			const c = {
				...d,
				discount: discount,
				subtotal: (toNumber(d.price) - discount) * toNumber(d.qty)
			};
			updateCurrentDetail(c);
			isDirty = false;
			isDiscountChanged = false;
		}
	}

	function discountOnChange(
		e: CustomEvent<string | number | null>,
		id: number
	) {
		if (typeof e.detail === 'string') {
			change_discount(e.detail, id);
		}
	}

	function discountOnInput(
		e: CustomEvent<string | number | null>,
		price: number,
		hpp: number
	): void {
		if (typeof e.detail === 'string') {
			e.preventDefault();
			const discount = getNumber(e.detail);
			if (price - discount <= hpp) {
				timeout = 3_000;
				notifyTitle = 'Maaf';
				notifySubtitle =
					'Discount ' + formatNumber(discount) + ' terlalu besar!';
				showNotification = true;
			}
		}
	}

	async function discountOnKeyDown(e: KeyboardEvent, id: number) {
		if ((e.key === 'Tab' && !e.shiftKey) || e.key === 'Enter') {
			e.preventDefault();

			currentKey = 'barcode';
			const ctlId = '#' + currentKey + '-id';
			setFocuse(ctlId);

			await tick();

			let i = $details.findIndex((f) => f.id === id);

			i++;

			if (i === $details.length) {
				if (id > 0) {
					details.update((o) => [
						...o,
						{ ...initDetail, gudangName: getDefaultGudangName() }
					]);
				} else {
					i = 0;
				}
			}

			let d = $details[i];
			currentId = d.id;
			isDirty = false;
			await tick();
			setFocuse(ctlId);
		} else if (e.key === 'Tab' && e.shiftKey) {
			currentKey = 'qty';
		} else if (
			isDiscountChanged &&
			(e.key === 'ArrowUp' || e.key === 'ArrowDown')
		) {
			if (e.currentTarget) {
				currentKey = 'discount';
				const el = e.currentTarget as HTMLInputElement;
				change_discount(el.value, id);
			}
		}
	}

	async function onTablePointerEnter(_e: Event) {
		const i = $details.findIndex((f) => f.id === 0);

		if (i < 0) {
			details.update((o) => [
				...o,
				{ ...initDetail, gudangName: getDefaultGudangName() }
			]);
		}

		if (currentId === 0) {
			if ($details.length > 0) {
				let currentDetail = $details[$details.length - 1];
				currentId = currentDetail.id;
				currentKey = 'barcode';
				await tick();
				setFocuse('#barcode-id');
			}
		}
	}

	async function onTableKeyDown(
		e: KeyboardEvent & { currentTarget: EventTarget & HTMLDivElement }
	) {
		// console.log(e.key)
		if (
			e.key !== '+' &&
			e.key !== '=' &&
			e.key !== '-' &&
			e.key !== 'ArrowUp' &&
			e.key !== 'ArrowDown' &&
			e.key !== 'Escape' &&
			(!e.ctrlKey || e.key !== '+')
		) {
			return true;
		}

		e.preventDefault();

		if (e.key === 'Escape') {
			if (isBarcodeDirty) {
				const el = document.getElementById('barcode-id') as HTMLInputElement;
				if (el) {
					const i = $details.findIndex((f) => f.id === currentId);
					let currentDetail = $details[i];
					isBarcodeDirty = false;
					el.value = currentDetail.barcode;
					return false;
				}
			}
			const i = $details.findIndex((f) => f.id === 0);
			if (i >= 0) {
				const slices = [...$details];
				slices.splice(i, 1);
				details.update(() => [...slices]);
			}
			return true;
		}

		const i = $details.findIndex((f) => f.id === currentId);

		if ((e.key === '=' || e.key === '+') && !e.ctrlKey) {
			const d = $details[i];
			const qty = toNumber(d.qty);
			d.qty = qty + 1;
			d.subtotal = d.qty * (d.hpp - d.discount);
			updateCurrentDetail(d);
			// const slices = [...$details];
			// slices.splice(i, 1, d);
			// details.update(() => [...slices]);
			// console.log(d.qty);
			return true;
		}

		if (e.key === '-' && !e.ctrlKey) {
			const d = $details[i];
			const qty = toNumber(d.qty) - 1;
			d.qty = qty <= 1 ? 1 : qty;
			d.subtotal = d.qty * (d.hpp - d.discount);
			updateCurrentDetail(d);
			// const slices = [...$details];
			// slices.splice(i, 1, d);
			// details.update(() => [...slices]);
			return true;
		}

		let x = 0;
		if (e.key === 'ArrowDown') {
			if (isBarcodeDirty) return false;
			x = i === $details.length - 1 ? 0 : i + 1;
		} else if (e.key === 'ArrowUp') {
			if (isBarcodeDirty) return false;
			x = i === 0 ? $details.length - 1 : i - 1;
		} else if (e.key === '+' && e.ctrlKey) {
			const i = $details.findIndex((f) => f.id === 0);
			currentKey = 'barcode';
			currentId = 0;
			if (i < 0) {
				const slices = [...$details];
				slices.splice(i, 1);
				details.update(() => [...slices]);
			}
			x = $details.length - 1;
		}

		if (x >= 0) {
			let currentDetail = $details[x];
			currentId = currentDetail.id;
			await tick();
			setFocuse('#' + currentKey + '-id');
		}
	}

	function updateCurrentDetail(e: iStockDetail) {
		const i = $details.findIndex((f) => f.id === e.id);
		if (i >= 0) {
			const slices = [...$details];
			slices.splice(i, 1, e);
			details.update(() => [...slices]);
			updateStock();
		}
	}

	function createNewId(): number {
		let test = $details.reduce((prev, cur) =>
			prev.id > cur.id ? prev : cur
		).id;
		return test + 1;
	}

	async function barcodeOnChange(
		e: CustomEvent<string | number | null>,
		id: number
	) {
		if (typeof e.detail === 'string' && isBarcodeDirty) {
			const strCode = e.detail;
			const url = `${baseURL}/products/barcode/${strCode}`;

			const options = {
				headers: {
					'content-type': 'application/json'
				},
				method: 'GET',
				credentials: credential_include
			};

			const request = new Request(url, options);

			let result = await fetch(request);

			isDirty = true;

			if (result.ok) {
				isBarcodeDirty = false;
				let json = await result.json();
				let p = json.data;
				let found = true;

				let i = $details.findIndex((f) => f.productId === p.id);

				if (i < 0) {
					i = $details.findIndex((f) => f.id === id);
					found = false;
				}

				if (i >= 0) {
					let d = $details[i];
					d.price = toNumber(p.hpp);

					if (found) {
						d.qty = toNumber(d.qty) + 1;
					} else {
						if (toNumber(d.qty) === 0) {
							d.qty = 1;
						}
						d.id = createNewId();
						d.unit = p.unit;
						d.productId = p.id;
						d.direction = 1;
						d.name = p.name;
						d.barcode = p.barcode;
						d.hpp = toNumber(p.hpp);
						d.oldQty = p.oldQty ? 0 : toNumber(p.oldQty);
						// console.log(p.oldStock);
					}
					d.subtotal =
						(toNumber(p.price) - toNumber(d.discount)) * toNumber(d.qty);

					updateCurrentDetail(d);

					if (!found) {
						currentId = d.id;
						await tick();
						currentKey = 'qty';
						setFocuse('#qty-id');
					}
				}
			} else {
				await tick();
				currentKey = 'barcode';
				setFocuse('#barcode-id');
				onnotfound(strCode);
			}
		}
	}

	async function barcodeOnKeyDown(e: KeyboardEvent, id: number) {
		if (e.key === 'Enter' || (e.key === 'Tab' && !e.shiftKey)) {
			if (isBarcodeDirty) return;
			currentKey = 'qty';
			if (e.key === 'Enter') {
				e.preventDefault();
				const ctlId = '#' + currentKey + '-id';
				setFocuse(ctlId);
			}
		} else if (e.key === 'Tab' && e.shiftKey) {
			e.preventDefault();
			let i = $details.findIndex((f) => f.id === id);
			currentKey = 'discount';
			if (i === 0) {
				i = $details.length;
			}

			let currentDetail = $details[i - 1];
			currentId = currentDetail.id;

			// setStringValue(currentDetail.qty, currentDetail.discount);

			await tick();
			// setTimeout(() => {
			setFocuse('#' + currentKey + '-id');
			// }, 100);
		}
	}

	async function gudangChange(gudId: number, rowId: number, name: string) {
		//		console.log(gudId, rowId);
		const i = $details.findIndex((f) => f.id === rowId);
		if (i >= 0) {
			const d = $details[i];
			const c = {
				...d,
				gudangName: name,
				gudangId: gudId
			};
			updateCurrentDetail(c);
			isDirty = true;
		}
	}

	// function setStringValue(qty: number, discount: number) {
	//   strQty = formatNumber(qty);
	//   strDiscount = formatNumber(discount);
	// }

	onDestroy(() => {
		try {
			if (document) {
				document.removeEventListener('click', clickOutSize);
			}
		} catch (ex: any) {
			console.log(ex.message);
		}
	});

	onMount(() => {
		document.addEventListener('click', clickOutSize);
	});

	async function totalItemClick(e: MouseEvent) {
		e.preventDefault();
		isAddNew = true;

		setTimeout(() => {
			if (reform) {
				currentId = 0;
				reform.focus();
			}

			isAddNew = false;
		}, 100);
	}

	let selectedRowIds: number[] = $state([]);
	let showNotification = $derived(timeout !== undefined);

	$effect(() => {
		items = $details.filter((f) => f.id > 0).length;
	});

	function updateStock() {
		let total = $details.reduce((o, t) => o + toNumber(t.subtotal), 0);
		stock.update(
			(s) =>
				(s = {
					...s,
					total: total,
					remain: total - (toNumber(s.dp) + toNumber(s.payment)),
					isModified: true,
					isDetailChanged: true
				})
		);
	}

	async function deleteItems(_e: MouseEvent) {
		const slices = $details.filter((f) => !selectedRowIds.includes(f.id));
		details.update(() => [...slices]);

		updateStock();

		await tick();
		selectedRowIds = [];
	}

	async function saveData(e: MouseEvent) {
		e.preventDefault();

		isStockUpdating.set(true);
		await tick();
		onsave(
			$details
				.filter((f) => f.id !== 0)
				.map((m) => ({ ...m, stockId: $stock.id }))
		);
	}

	const getDefaultGudangName = () => {
		const i = gudangs.findIndex((f) => f.id === 1);

		if (i >= 0) {
			const d = gudangs[i];
			return d.name;
		}
		return '-';
	};

	let isStockValid = $derived.by(() => {
		return (
			$stock.supplierId > 0 &&
			$stock.warehouseId > 0 &&
			$stock.total > 0 &&
			$stock.invoiceId.trim().length > 0
		);
	});

	// $inspect($stock);
</script>

{#if showNotification}
	<ToastNotification
		style={'margin-top: 24px; width: 100%'}
		on:click={() => (timeout = 12_000)}
		timeout={timeout}
		kind="warning-alt"
		on:close={() => {
			timeout = undefined;
		}}
	>
		<strong slot="title">{notifyTitle} </strong>
		<span slot="subtitle">{notifySubtitle}</span>
		<span slot="caption">{dayjs().format('DD-MM-YYYY HH:mm:ss')}</span>
	</ToastNotification>
{/if}
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions (because of reasons) -->
<div
	bind:this={reform}
	tabindex={0}
	role="row"
	aria-labelledby="table-detail"
	onkeydown={onTableKeyDown}
	onfocus={onTablePointerEnter}
>
	<DataTable
		selectable
		batchSelection
		batchExpansion
		rows={$details}
		headers={headers}
		nonExpandableRowIds={[0]}
		nonSelectableRowIds={[0]}
		expandable
		bind:selectedRowIds={selectedRowIds}
		size="medium"
		on:click:row={onRowClick}
	>
		<Toolbar size="sm">
			<ToolbarContent>
				<Button
					kind="danger-ghost"
					size="small"
					disabled={selectedRowIds.length == 0 || $isStockUpdating}
					icon={Delete}
					on:click={deleteItems}>Hapus item</Button
				>
				<Button
					icon={NewTab}
					kind="tertiary"
					style="border: none;"
					size="small"
					on:click={() => {
						onnew(0);
						selectedRowIds = [];
					}}
					disabled={items === 0 || $isStockUpdating || $stock.id === 0}
					>Buat stock baru</Button
				>
				<Button
					icon={Money}
					kind="tertiary"
					style="border: none;"
					size="small"
					on:click={() => onadddp()}
					disabled={items === 0 || $isStockUpdating}>Pembayaran / Dp</Button
				>
				<Button
					icon={Save}
					disabled={!isStockValid}
					on:click={saveData}
					size="small"
					skeleton={$isStockUpdating}>Simpan</Button
				>
				<Button
					icon={Logout}
					kind="danger-ghost"
					size="small"
					disabled={$isStockUpdating}
					on:click={() => onclose(0)}>Close</Button
				>
			</ToolbarContent>
		</Toolbar>

		<svelte:fragment slot="cell-header" let:header>
			{#if header.key === 'price' || header.key === 'qty' || header.key === 'discount' || header.key === 'subtotal' || header.key === 'pot'}
				<div class="cell-right">{header.value}</div>
			{:else}
				{header.value}
			{/if}
		</svelte:fragment>
		<svelte:fragment slot="cell" let:row let:cell>
			<!-- edit mode -->
			{#if currentId === row['id']}
				{#if cell.key === 'barcode'}
					<TextInput
						autocomplete="off"
						list="barcode-list"
						value={cell.value}
						size={'sm'}
						class="cell-edit"
						id="barcode-id"
						on:input={() => (isBarcodeDirty = true)}
						on:change={(e) => barcodeOnChange(e, row.id)}
						on:focus={() => (currentKey = cell.key)}
						on:keydown={(e) => barcodeOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === 'gudangId'}
					<ComboBox
						id="gudang-id"
						size={'sm'}
						placeholder="Pilih gudang"
						selectedId={row['gudangId']}
						style={'margin: 0;padding:0'}
						items={get_gudang()}
						on:select={(e) => {
							if (e.detail.selectedId > 0) {
								gudangChange(
									e.detail.selectedId,
									row.id,
									e.detail.selectedItem.text
								);
							}
						}}
					/>
				{:else if cell.key === 'qty'}
					<NumberPercent
						value={formatNumber(cell.value, 2)}
						size="sm"
						clasess="cell-edit input-number"
						id="qty-id"
						on:input={() => (isQtyChanged = true)}
						on:change={(e) => qtyOnChange(e, row.id)}
						on:focus={() => (currentKey = cell.key)}
						on:keydown={(e) => qtyOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === 'discount'}
					<NumberInput
						value={formatNumber(cell.value)}
						size="sm"
						classes="cell-edit input-number"
						id="discount-id"
						on:change={(e) => discountOnChange(e, row.id)}
						on:input={(e) => {
							isDiscountChanged = true;
							discountOnInput(e, toNumber(row['price']), toNumber(row['hpp']));
						}}
						on:focus={() => (currentKey = cell.key)}
						on:keydown={(e) => discountOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === 'price' || cell.key === 'qty' || cell.key === 'discount' || cell.key === 'subtotal'}
					<div
						role="button"
						tabindex={-1}
						aria-labelledby="btn-101"
						onclick={() => (currentKey = cell.key)}
						class="cell-right"
						class:qty={cell.key === 'subtotal'}
						class:qty-alert={toNumber(row['qty']) === 0}
					>
						{formatNumber(cell.value)}
					</div>
				{:else if cell.key === 'pot'}
					<div
						role="button"
						tabindex={-1}
						aria-labelledby="btn-105"
						onclick={() => (currentKey = cell.key)}
						class:qty-alert={toNumber(row['qty']) === 0}
						class="cell-right"
					>
						{formatNumber(toNumber(row['price']) - toNumber(row['discount']))}
					</div>
				{:else}
					<div
						role="button"
						tabindex={-1}
						aria-labelledby="btn-102"
						onclick={() => (currentKey = cell.key)}
						class:qty-alert={toNumber(row['qty']) === 0}
					>
						{cell.value}
					</div>
				{/if}
				<!-- {:else if row.id === 0}
				{#if cell.key === "barcode"}
					Total item: {data.length}
					{:else if cell.key === "subtotal"}
					<div class="cell-right">{formatNumber(data.reduce((o,t) => o + t.subtotal, 0))}</div>
				{:else}
				<span></span>
				{/if} -->
				<!-- normal mode -->
			{:else if cell.key === 'price' || cell.key === 'hpp' || cell.key === 'discount' || cell.key === 'subtotal'}
				<div
					role="button"
					tabindex={-1}
					aria-labelledby="btn-103"
					onclick={() => (currentKey = cell.key)}
					class="cell-right"
					class:subtotal={cell.key === 'subtotal'}
					class:qty-alert={toNumber(row['qty']) === 0}
				>
					{formatNumber(cell.value)}
				</div>
			{:else if cell.key === 'qty'}
				<div
					role="button"
					tabindex={-1}
					aria-labelledby="btn-107"
					onclick={() => (currentKey = cell.key)}
					class="cell-right"
					class:qty={cell.key === 'qty'}
					class:qty-alert={toNumber(row['qty']) <= 0}
				>
					{formatNumber(cell.value, 2)}
				</div>
			{:else if cell.key === 'gudangId'}
				{row['gudangName'] ?? ''}
			{:else if cell.key === 'pot'}
				<div
					role="button"
					tabindex={-1}
					aria-labelledby="btn-105"
					onclick={() => (currentKey = cell.key)}
					class:qty-alert={toNumber(row['qty']) === 0}
					class="cell-right"
				>
					{formatNumber(toNumber(row['price']) - toNumber(row['discount']))}
				</div>
			{:else}
				<div
					role="button"
					tabindex={-1}
					aria-labelledby="btn-104"
					onclick={() => (currentKey = cell.key)}
					class:qty-alert={toNumber(row['qty']) === 0}
				>
					{cell.value}
				</div>
			{/if}
		</svelte:fragment>
		<svelte:fragment slot="expanded-row" let:row>
			<ExpandedRow
				productId={row['productId']}
				newQty={toNumber(row['qty'])}
				oldQty={toNumber(row['oldQty'])}
				selectedGudangId={row['gudangId']}
				oldGudangId={row['oldGudangId']}
			/>
		</svelte:fragment>
	</DataTable>
</div>
<hr />

<div>
	<Button
		size="small"
		icon={Add}
		kind="tertiary"
		onclick={totalItemClick}
		disabled={$isStockUpdating}
		>Total: {items} item{items > 1 ? 's' : ''}</Button
	>
</div>

<datalist id="barcode-list">
	{#each barcodes as c, i}
		<option id="list-{i}" value={c.barcode}></option>
	{/each}
</datalist>
