<script lang="ts">
	import { formatNumber, getNumber } from "$lib/components/NumberFormat";
	import NumberInput from "$lib/components/NumberInput.svelte";
	import {
		baseURL,
		credential_include,
		type iStockDetail,
	} from "$lib/interfaces";
	import {
		Button,
		DataTable,
		TextInput,
		ToastNotification,
		Toolbar,
		ToolbarContent,
	} from "carbon-components-svelte";
	import type { DataTableRow } from "carbon-components-svelte/types/DataTable/DataTable.svelte";
	import { onDestroy, onMount, tick } from "svelte";
	import { createEventDispatcher } from "svelte";
	import ExpandedRow from "./ExpandedRow.svelte";
	import "../../style.css";
	import dayjs from "dayjs";
	import { Add, Delete, InformationSquare, NewTab, Save } from "carbon-icons-svelte";

	const dispatch = createEventDispatcher();

	let reform: HTMLDivElement;
	let isDirty = false;
	let isBarcodeDirty = false;
	let showNotification = false;
	let timeout: number | undefined = undefined;
	let notifyTitle = "Error";
	let notifySubtitle = "";

	let headers = [
		{ key: "barcode", value: "Barcode", width: "15%" },
		{ key: "name", value: "Nama barang", width: "auto" },
		{ key: "qty", value: "Qty", width: "70px" },
		{ key: "unit", value: "Unit", width: "40px" },
		{ key: "price", value: "Harga", width: "100px" },
		{ key: "discount", value: "Disc", width: "80px" },
		{ key: "pot", value: "Hrg-Pot", width: "90px" },
		{ key: "subtotal", value: "Subtotal", width: "100px" },
	];

	export let data: iStockDetail[] = [];
	export let barcodes: { barcode: string }[] = [];
	export let isStockValid = false;
	let items = 0;
	let currentId = 0;
	let isAddNew = false;

	// let currentDetail: iStockDetail;
	let currentKey = "barcode";
	// let strQty = "0";
	// let strDiscount = "0";
	const initDetail: iStockDetail = {
		orderId: 0,
		id: 0,
		productId: 0,
		barcode: "",
		name: "",
		qty: 1,
		direction: 1,
		unit: "",
		hpp: 0,
		price: 0,
		discount: 0,
		subtotal: 0,
	};

	function onRowClick(e: CustomEvent<DataTableRow>) {
		e.preventDefault();
		// const i = data.findIndex((f) => f.id === e.detail.id);
		// let d = data[i];
		// currentDetail = d;
		let setFocus = false;

		const i = data.findIndex((f) => f.id === 0);

		if (i < 0) {
			data = [...data, { ...initDetail }];
		}

		// setStringValue(currentDetail.qty, currentDetail.discount);

		if (currentId != e.detail.id) {
			setFocus = true;
			// console.log(currentKey);
			if (
				currentKey === "name" ||
				currentKey === "unit" ||
				currentKey === "price" ||
				currentKey === "subtotal"||
				currentKey === "pot"
			) {
				currentKey = "barcode";
			}
		}

		currentId = e.detail.id;

		if (setFocus) {
			const ctlId = "#" + currentKey + "-id";
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
			const i = data.findIndex((f) => f.id === 0);
			if (i >= 0) {
				data.splice(i, 1);
				data = [...data];
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

	function discountOnKeyDown(e: KeyboardEvent, id: number) {
		if ((e.key === "Tab" && !e.shiftKey) || e.key === "Enter") {
			e.preventDefault();

			let i = data.findIndex((f) => f.id === id);

			i++;

			if (i === data.length) {
				if (id > 0 && isDirty) {
					data = [...data, { ...initDetail }];
				} else {
					i = 0;
				}
			}

			let d = data[i];
			currentId = d.id;
			isDirty = false;
			currentKey = "barcode";

			setTimeout(() => {
				const ctlId = "#" + currentKey + "-id";
				setFocuse(ctlId);
			}, 100);
		} else if (e.key && e.shiftKey) {
			currentKey = "qty";
		}
	}

	function qtyOnKeyDown(e: KeyboardEvent, id: number) {
		if (e.key === "Enter" || e.key === "Tab") {
			currentKey = "discount";
			if (e.key === "Enter") {
				e.preventDefault();
				const ctlId = "#" + currentKey + "-id";
				setFocuse(ctlId);
			}
		} else if (e.key === "Tab" && e.shiftKey) {
			currentKey = "barcode";
		}
	}

	async function onTablePointerEnter(e: Event) {
		const i = data.findIndex((f) => f.id === 0);

		if (i < 0) {
			data = [...data, { ...initDetail }];
		}

		if (currentId === 0) {
			if (data.length > 0) {
				let currentDetail = data[data.length-1];
				currentId = currentDetail.id;
				currentKey = "barcode";
				await tick();
				setFocuse("#barcode-id")
			}
		}
	}

	async function onTableKeyDown(
		e: KeyboardEvent & { currentTarget: EventTarget & HTMLDivElement },
	) {
		// console.log(e.key)
		if (
			e.key !== "ArrowUp" &&
			e.key !== "ArrowDown" &&
			e.key !== "Escape" &&
			(!e.ctrlKey || e.key !== "+")
		) {
			return true;
		}
		e.preventDefault();

		if (e.key === "Escape") {
			const i = data.findIndex((f) => f.id === 0);
			if (i >= 0) {
				data.splice(i, 1);
				data = [...data];
			}
			return true;
		}

		const i = data.findIndex((f) => f.id === currentId);

		let x = 0;
		if (e.key === "ArrowDown") {
			x = i === data.length - 1 ? 0 : i + 1;
		} else if (e.key === "ArrowUp") {
			x = i === 0 ? data.length - 1 : i - 1;
		} else if (e.key === "+" && e.ctrlKey) {
			const i = data.findIndex((f) => f.id === 0);
			currentKey = "barcode";
			currentId = 0;
			if (i < 0) {
				data = [...data, { ...initDetail }];
			}
			x = data.length - 1;
		}

		if (x >= 0) {
			let currentDetail = data[x];
			currentId = currentDetail.id;
			await tick();
			setFocuse("#" + currentKey + "-id");
		}
	}


	function updateCurrentDetail(e: iStockDetail) {
		const i = data.findIndex((f) => f.id === e.id);
		if (i >= 0) {
			data.splice(i, 1, e);
			data = [...data];
			dispatch("stockChanged", data);
		}
	}

	function createNewId(): number {
		let test = data.reduce((prev, cur) => (prev.id > cur.id) ? prev:cur).id;
		return test+1;
	}

	async function barcodeOnChange(
		e: CustomEvent<string | number | null>,
		id: number,
	) {
		if (typeof e.detail === "string" && isBarcodeDirty) {
			const strCode = e.detail;
			const url = `${baseURL}/products/barcode/${strCode}`;

			const options = {
				headers: {
					"content-type": "application/json",
				},
				method: "GET",
				credentials: credential_include,
			};

			const request = new Request(url, options);

			let result = await fetch(request);

			isDirty = true;

			if (result.ok) {
				isBarcodeDirty = false;
				let json = await result.json();
				let p = json.data;
				let found = true;

				let i = data.findIndex((f) => f.productId === p.id);

				if (i < 0) {
					i = data.findIndex((f) => f.id === id);
					found = false;
				}

				if (i >= 0) {
					let d = data[i];
					d.price = p.price;

					if (found) {
						d.qty += 1;
					} else {
						if (d.qty === 0) {
							d.qty = 1;
						}
						d.id = createNewId();
						d.unit = p.unit;
						d.productId = p.id;
						d.direction = 1;
						d.name = p.name;
						d.barcode = p.barcode;
						d.hpp = p.hpp;
					}
					d.subtotal = (p.price - d.discount) * d.qty;

					updateCurrentDetail(d);

					if (!found) {
						currentId = d.id;
						await tick();
						currentKey = "qty";
						setFocuse("#qty-id");
					}
				}
			} else {
				await tick();
				currentKey = "barcode";
				setFocuse("#barcode-id");
				dispatch("productNotFound", strCode);
			}
		}
	}

	async function barcodeOnKeyDown(e: KeyboardEvent, id: number) {
		if (e.key === "Enter" || (e.key === "Tab" && !e.shiftKey)) {
			if (isBarcodeDirty) return;
			currentKey = "qty";
			if (e.key === "Enter") {
				e.preventDefault();
				const ctlId = "#" + currentKey + "-id";
				setFocuse(ctlId);
			}
		} else if (e.key === "Tab" && e.shiftKey) {
			e.preventDefault();
			let i = data.findIndex((f) => f.id === id);
			currentKey = "discount";
			if (i === 0) {
				i = data.length;
			}

			let currentDetail = data[i - 1];
			currentId = currentDetail.id;

			// setStringValue(currentDetail.qty, currentDetail.discount);

			await tick();
			// setTimeout(() => {
			setFocuse("#" + currentKey + "-id");
			// }, 100);
		}
	}

	function qtyOnChange(e: CustomEvent<string | number | null>, id: number) {
		if (typeof e.detail === "string") {
			const i = data.findIndex((f) => f.id === id);
			if (i >= 0) {
				const qty = getNumber(e.detail);

				const d = data[i];

				const c = {
					...d,
					qty: qty,
					subtotal: (d.price - d.discount) * qty,
				};
				updateCurrentDetail(c);
				isDirty = true;
			}
		}
	}

	function discountOnChange(
		e: CustomEvent<string | number | null>,
		id: number,
	) {
		if (typeof e.detail === "string") {
			const i = data.findIndex((f) => f.id === id);
			if (i >= 0) {
				const discount = getNumber(e.detail);
				const d = data[i];
				if (d.price - discount <= d.hpp) {
					// showNotification = true;
					// timeout = 3_000;
					// notifySubtitle = "Discount " + e.detail + " terlalu besar!";
					return true;
				}
				const c = {
					...d,
					discount: discount,
					subtotal: (d.price - discount) * d.qty,
				};
				updateCurrentDetail(c);
				isDirty = true;
			}
		}
	}
	function discountOnInput(
		e: CustomEvent<string | number | null>,
		price: number,
		hpp: number,
	): void {
		if (typeof e.detail === "string") {
			e.preventDefault();
			const discount = getNumber(e.detail);
			if (price - discount <= hpp) {
				timeout = 3_000;
				notifyTitle = "Maaf";
				notifySubtitle =
					"Discount " + formatNumber(discount) + " terlalu besar!";
				showNotification = true;
			}
		}
	}

	// function setStringValue(qty: number, discount: number) {
	//   strQty = formatNumber(qty);
	//   strDiscount = formatNumber(discount);
	// }

	onDestroy(() => {
		try {
			if (document) {
				document.removeEventListener("click", clickOutSize);
			}
		} catch (ex: any) {
			console.log(ex.message);
		}
	});

	onMount(() => {
		document.addEventListener("click", clickOutSize);
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

	let selectedRowIds: number[] = [];
	$: showNotification = timeout !== undefined;
	$: {
		items = data.filter((f) => f.id > 0).length;
	}


	async function deleteItems(e: MouseEvent) {
		const test =  data.filter(f => !selectedRowIds.includes(f.id));
		data = test;
		dispatch("stockChanged", data);
		await tick();
		selectedRowIds = [];
	}
</script>

{#if showNotification}
	<ToastNotification
		style={"margin-top: 24px; width: 100%"}
		on:click={() => (timeout = 12_000)}
		{timeout}
		kind="warning-alt"
		on:close={(e) => {
			timeout = undefined;
		}}
	>
		<strong slot="title">{notifyTitle} </strong>
		<span slot="subtitle">{notifySubtitle}</span>
		<span slot="caption">{dayjs().format("DD-MM-YYYY HH:mm:ss")}</span>
	</ToastNotification>
{/if}

<div
	bind:this={reform}
	tabindex={0}
	role="row"
	aria-labelledby="table-detail"
	on:keydown={onTableKeyDown}
	on:focus={onTablePointerEnter}
>
	<DataTable
		selectable
		batchSelection
		batchExpansion
		rows={data}
		{headers}
		nonExpandableRowIds={[0]}
		nonSelectableRowIds={[0]}
		expandable
		bind:selectedRowIds
		size="short"
		on:click:row={onRowClick}
	>

	<Toolbar size="sm">
		<ToolbarContent>
			<Button kind="danger" size="small" disabled={selectedRowIds.length==0} icon={Delete} on:click={deleteItems}>Hapus item</Button>
			<Button icon={InformationSquare} on:click={() =>dispatch("showInfo", null)} disabled={items===0}>Informasi stock</Button>
			<Button icon={NewTab} on:click={() => {
				dispatch("createNewStock", null);
				selectedRowIds = [];
			}} disabled={items===0}>Buat stock baru</Button>
			<Button icon={Save} disabled={!isStockValid}>Simpan</Button>
		</ToolbarContent>
	</Toolbar>

		<svelte:fragment slot="cell-header" let:header>
			{#if header.key === "price" || header.key === "qty" || header.key === "discount" || header.key === "subtotal"|| header.key === "pot" }
				<div class="cell-right">{header.value}</div>
			{:else}
				{header.value}
			{/if}
		</svelte:fragment>
		<svelte:fragment slot="cell" let:row let:cell>
			<!-- edit mode -->
			{#if currentId === row["id"]}
				{#if cell.key === "barcode"}
					<TextInput
						autocomplete="off"
						list="barcode-list"
						value={cell.value}
						size={"sm"}
						class="cell-edit"
						id="barcode-id"
						on:input={() => (isBarcodeDirty = true)}
						on:change={(e) => barcodeOnChange(e, row.id)}
						on:focus={(e) => (currentKey = cell.key)}
						on:keydown={(e) => barcodeOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === "qty"}
					<NumberInput
						value={formatNumber(cell.value)}
						size="sm"
						classes="cell-edit input-number"
						id="qty-id"
						on:change={(e) => qtyOnChange(e, row.id)}
						on:focus={() => (currentKey = cell.key)}
						on:keydown={(e) => qtyOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === "discount"}
					<NumberInput
						value={formatNumber(cell.value)}
						size="sm"
						classes="cell-edit input-number"
						id="discount-id"
						on:input={(e) => discountOnInput(e, row["price"], row["hpp"])}
						on:change={(e) => discountOnChange(e, row.id)}
						on:focus={() => (currentKey = cell.key)}
						on:keydown={(e) => discountOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === "price" || cell.key === "qty" || cell.key === "discount" || cell.key === "subtotal"}
					<div
						role="button"
						tabindex={-1}
						on:keyup
						aria-labelledby="btn-101"
						on:click={() => (currentKey = cell.key)}
						class="cell-right"
						class:qty={cell.key === "subtotal"}
						class:qty-alert={row["qty"] === 0}
					>
						{formatNumber(cell.value)}
					</div>
				{:else if cell.key === "pot"}
					<div
						role="button"
						tabindex={-1}
						on:keyup
						aria-labelledby="btn-105"
						on:click={() => (currentKey = cell.key)}
						class:qty-alert={row["qty"] === 0}
						class="cell-right"
					>
						{formatNumber(row["price"] - row["discount"])}
					</div>
				{:else}
					<div
						role="button"
						tabindex={-1}
						on:keyup
						aria-labelledby="btn-102"
						on:click={() => (currentKey = cell.key)}
						class:qty-alert={row["qty"] === 0}
					>
						{cell.value}
					</div>
				{/if}
				<!-- normal mode -->
				<!-- {:else if row.id === 0}
				{#if cell.key === "barcode"}
					Total item: {data.length}
					{:else if cell.key === "subtotal"}
					<div class="cell-right">{formatNumber(data.reduce((o,t) => o + t.subtotal, 0))}</div>
				{:else}
				<span></span>
				{/if} -->
			{:else if cell.key === "price" || cell.key === "hpp" || cell.key === "qty" || cell.key === "discount" || cell.key === "subtotal"}
				<div
					role="button"
					tabindex={-1}
					on:keyup
					aria-labelledby="btn-103"
					on:click={() => (currentKey = cell.key)}
					class="cell-right"
					class:qty={cell.key === "qty"}
					class:subtotal={cell.key === "subtotal"}
					class:qty-alert={row["qty"] === 0}
				>
					{formatNumber(cell.value)}
				</div>
			{:else if cell.key === "pot"}
				<div
					role="button"
					tabindex={-1}
					on:keyup
					aria-labelledby="btn-105"
					on:click={() => (currentKey = cell.key)}
					class:qty-alert={row["qty"] === 0}
					class="cell-right"
				>
					{formatNumber(row["price"] - row["discount"])}
				</div>
			{:else}
				<div
					role="button"
					tabindex={-1}
					on:keyup
					aria-labelledby="btn-104"
					on:click={() => (currentKey = cell.key)}
					class:qty-alert={row["qty"] === 0}
				>
					{cell.value}
				</div>
			{/if}
		</svelte:fragment>
		<svelte:fragment slot="expanded-row" let:row>
			<ExpandedRow productId={row["productId"]} />
		</svelte:fragment>
	</DataTable>
</div>
<hr />

<div>
	<Button
		size="small"
		icon={Add}
		kind="tertiary"
		on:click={totalItemClick}>Total: {items} item{items > 1 ? "s" : ""}</Button
	>
</div>

<datalist id="barcode-list">
	{#each barcodes as c, i}
		<option id="list-{i}" value={c.barcode} />
	{/each}
</datalist>

<!-- <div>{JSON.stringify(selectedRowIds, null,  4)}</div> -->

<style lang="scss">
	.cell-right {
		text-align: right;
	}

	:global(.bx--list-box__field .bx--text-input.supplier) {
		border-bottom: none;
		// background-color: var(--cds-link-01);
		// color: var(--cds-ui-01);
	}
	:global(.bx--table-column-checkbox) {
	width: auto;
	}
	:global(.bx--table-expand__button) {
		width: auto;
		min-height: 16px;
	}
	:global(.bx--text-input--sm.cell-edit) {
		margin: 0;
		padding: 0 2px;
		height: auto;
	}
	:global(.bx--text-input.input-number) {
		text-align: right;
	}
	:global(.bx--table-header-label) {
		margin: 0;
		padding: 0 !important;
	}
	.qty {
		font-weight: 700;
		color: var(--cds-focus);
	}
	.qty-alert {
		color: var(--cds-text-error);
		font-weight: 700;
	}

	.subtotal {
		font-weight: 700;
	}
	hr {
		height: 1px;
		border-width: 1px 0 0 0;
		margin-top: 3px;
	}
</style>
