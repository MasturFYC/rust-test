<script lang="ts">
	import { formatNumber, getNumber } from "$lib/components/NumberFormat";
	import NumberInput from "$lib/components/NumberInput.svelte";
	import {
		baseURL,
		credential_include,
	type iStockDetail,
	} from "$lib/interfaces";
	import {
		DataTable,
		TextInput,
		ToastNotification,
	} from "carbon-components-svelte";
	import type {
		DataTableRow,
	} from "carbon-components-svelte/types/DataTable/DataTable.svelte";
	import { onDestroy, onMount, tick } from "svelte";
	import { createEventDispatcher } from "svelte";
	import ExpandedRow from "./ExpandedRow.svelte";
	import "../../style.css";
	import dayjs from "dayjs";

	const dispatch = createEventDispatcher();

	let reform: HTMLDivElement;
	let isDirty = false;
	let isBarcodeDirty = false;
	let showNotification = false;
	let timeout: number | undefined = undefined;
	let notifyTitle = "Error";
	let notifySubtitle = "";

	let headers = [
		{ key: "barcode", value: "Barcode", width: "18%" },
		{ key: "name", value: "Nama barang", width: "auto" },
		{ key: "qty", value: "Qty", width: "70px" },
		{ key: "unit", value: "Unit", width: "40px" },
		{ key: "price", value: "Harga", width: "100px" },
		{ key: "discount", value: "Discount", width: "90px" },
		{ key: "subtotal", value: "Subtotal", width: "120px" },
	];

	export let data: iStockDetail[] = [];
	export let barcodes: { barcode: string }[] = [];
	let currentId = 0;

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
		direction: 0,
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
				currentKey === "subtotal"
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

	function onTablePointerEnter(e: Event) {
		const i = data.findIndex((f) => f.id === 0);

		if (i < 0) {
			data = [...data, { ...initDetail }];
		}
		if (currentId === 0) {
			if (data.length > 0) {
				let currentDetail = data[0];
				currentId = currentDetail.id;
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
			// setStringValue(currentDetail.qty, currentDetail.discount);

			// if (currentKey === "name" || currentKey === "hpp") {
			// currentKey = "barcode";
			// }
			// setTimeout(() => {
			await tick();
			setFocuse("#" + currentKey + "-id");
			// }, 100);
		}
	}

	// function onTableClick(
	// 	e: CustomEvent<{
	// 		header?: DataTableHeader | undefined;
	// 		row?: DataTableRow | undefined;
	// 		cell?: DataTableCell | undefined;
	// 	}>,
	// ) {
	// 	if (e.detail.cell) {
	// 		currentDetail = e.detail.row as iStockDetail;
	// 		currentId = e.detail.row?.id;
	// 		currentKey = e.detail?.cell?.key;
	// 		console.log("TABLE", currentDetail);
	// 	}
	// }

	function updateCurrentDetail(e: iStockDetail) {
		const i = data.findIndex((f) => f.id === e.id);
		if (i >= 0) {
			data.splice(i, 1, e);
			data = [...data];
			dispatch("stockChanged", data);
		}
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
						d.id = data.length;
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
	function discountOnInput(e: CustomEvent<string | number | null>, price: number, hpp: number): void {
		if(typeof e.detail === 'string') {
			e.preventDefault();
			const discount = getNumber(e.detail);
			if((price - discount) <= hpp) {
				timeout = 3_000;
				notifyTitle = "Maaf";
				notifySubtitle = "Discount " + formatNumber(discount) + " terlalu besar!";
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

	// $: console.log(currentKey);
	$: showNotification = timeout !== undefined;


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
		rows={data}
		{headers}
		nonExpandableRowIds={[0]}
		expandable
		size="short"
		on:click:row={onRowClick}
	>
		<svelte:fragment slot="cell-header" let:header>
			{#if header.key === "price" || header.key === "qty" || header.key === "discount" || header.key === "subtotal"}
				<div class="cell-right">{header.value}</div>
			{:else}
				{header.value}
			{/if}
		</svelte:fragment>
		<svelte:fragment slot="cell" let:row let:cell>
			{#if currentId === row["id"]}
				<!-- <form
					on:submit={(e) => {
						e.preventDefault();
						const i = data.findIndex((f) => f.id === currentId);
						if (i >= 0) {
							data.splice(i, 1, currentDetail);
							data = [...data];
						}
					}}
				> -->
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
					>
						{formatNumber(cell.value)}
					</div>
				{:else}
					<div
						role="button"
						tabindex={-1}
						on:keyup
						aria-labelledby="btn-102"
						on:click={() => (currentKey = cell.key)}
					>
						{cell.value}
					</div>
				{/if}
				<!-- </form> -->
			{:else if cell.key === "price" || cell.key === "hpp" || cell.key === "qty" || cell.key === "discount" || cell.key === "subtotal"}
				<div
					role="button"
					tabindex={-1}
					on:keyup
					aria-labelledby="btn-103"
					on:click={() => (currentKey = cell.key)}
					class="cell-right"
				>
					{formatNumber(cell.value)}
				</div>
			{:else}
				<div
					role="button"
					tabindex={-1}
					on:keyup
					aria-labelledby="btn-104"
					on:click={() => (currentKey = cell.key)}
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

<datalist id="barcode-list">
	{#each barcodes as c, i}
		<option id="list-{i}" value={c.barcode} />
	{/each}
</datalist>

<style lang="scss">
	.cell-right {
		text-align: right;
	}

	:global(.bx--list-box__field .bx--text-input.supplier) {
		border-bottom: none;
		// background-color: var(--cds-link-01);
		// color: var(--cds-ui-01);
	}
	:global(.bx--table-expand__button) {
		width: auto;
		min-height: 16px;
	}
	:global(.bx--text-input--sm.cell-edit) {
		margin: 0;
		padding: 0 3px;
		height: auto;
	}
	:global(.bx--text-input.input-number) {
		text-align: right;
	}
	:global(.bx--table-header-label) {
		margin: 0;
		padding: 0 !important;
	}
</style>
