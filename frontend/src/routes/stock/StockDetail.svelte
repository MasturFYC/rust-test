<script lang="ts">
	import { formatRupiah, getNumber } from "$lib/components/NumberFormat";
	import NumberInput from "$lib/components/NumberInput.svelte";
	import { baseURL, credential_include, type iProduct, type iStockDetail } from "$lib/interfaces";
	import { DataTable, TextInput } from "carbon-components-svelte";
	import type {
		DataTableCell,
		DataTableRow,
	} from "carbon-components-svelte/types/DataTable/DataTable.svelte";
	import { onDestroy, onMount, tick } from "svelte";
	import { createEventDispatcher } from "svelte";

	const dispatch = createEventDispatcher();

	let reform: HTMLDivElement;
	let isDirty = false;
	let isBarcodeDirty = false;

	let headers = [
		{ key: "barcode", value: "Barcode", width: "18%" },
		{ key: "name", value: "Nama barang", width: "auto" },
		{ key: "qty", value: "Qty", width: "80px" },
		{ key: "unit", value: "Unit", width: "60px" },
		{ key: "price", value: "Harga", width: "100px" },
		{ key: "discount", value: "Discount", width: "100px" },
		{ key: "subtotal", value: "Subtotal", width: "120px" },
	];

	export let data: iStockDetail[] = [];
	let currentId = 0;
	let currentDetail: iStockDetail;
	let currentKey = "barcode";
	let strQty = "0";
	let strDiscount = "0";
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
		let d = e.detail as iStockDetail;
		currentDetail = d;
		let setFocus = false;
		setStringValue(currentDetail.qty, currentDetail.discount);

		if (currentId != d.id) {
			setFocus = true;
		}

		currentId = d.id;

		if (setFocus) {
			const ctlId = "#" + currentKey + "-id";
			setTimeout(() => {
				const elem = document.querySelector(ctlId) as HTMLInputElement;
				if (elem) {
					elem.focus();
					elem.select();
				}
			}, 100);
		}
	}
	function onCellClik(e: CustomEvent<DataTableCell>): void {
		e.preventDefault();
		currentKey = e.detail.key;
	}

	function clickOutSize(event: any) {
		const withinBoundaries = event.composedPath().includes(reform);

		// isDirty = false;

		if (withinBoundaries) {
			// console.log("Click happened inside element");
		} else {
			// console.log("Click happened **OUTSIDE** element");
			currentId = 0;
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
			currentKey = "barcode";

			if (i === data.length) {
				if (id > 0 && isDirty) {
						data = [...data, { ...initDetail }];
				} else {
					i = 0;
				}
			}

			isDirty = false;
			currentDetail = data[i];
			currentId = currentDetail.id;
			setStringValue(currentDetail.qty, currentDetail.discount);

			setTimeout(() => {
				const ctlId = "#" + currentKey + "-id";
				setFocuse(ctlId);
			}, 100);
		}
	}

	function qtyOnKeyDown(e: KeyboardEvent, id: number) {
		if (e.key === "Enter" || e.key === "Tab") {
			currentKey = "discount";
			if(e.key === "Enter") {
				e.preventDefault();
				const ctlId = "#" + currentKey + "-id";
				setFocuse(ctlId);
			}
		} else if(e.key === "Tab" && e.shiftKey) {
			currentKey = "barcode";
		}
	}

	function onTablePointerEnter() {
		// console.log(currentId)
		if (currentId === 0) {
			currentDetail = data[0];
			currentId = currentDetail.id;
			setStringValue(currentDetail.qty, currentDetail.discount);
			setTimeout(() => {
				setFocuse("#barcode-id");
			}, 100);
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

		if(e.key === "Escape") {
			const i = data.findIndex(f => f.id === 0);
			if(i >= 0) {
				data.splice(i,1);
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
			if (i < 0) {
				data = [...data, { ...initDetail }];
			}
			x = data.length - 1;
		}

		if (i >= 0) {
			currentDetail = data[x];
			currentId = currentDetail.id;
			setStringValue(currentDetail.qty, currentDetail.discount);

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
		}
	}

	async function onBarcodeChange(e: CustomEvent<string | number | null>, id: number) {
		if(typeof e.detail === 'string' && isBarcodeDirty) {
			const url = `${baseURL}/products/barcode/${e.detail}`;

			const options = {
				headers: {
					"content-type": "application/json",
				},
				method: "GET",
				credentials: credential_include,
			};

			const request = new Request(url,options);

			let result = await fetch(request);

			isDirty = true;

			if(result.ok) {
				isBarcodeDirty = false;
				let json = await result.json();
				let p = json.data;
				let i = data.findIndex(f => f.id === id);
				if(i >= 0) {
					let d = data[i];
					d.name = p.name;
					d.barcode = p.barcode;
					d.price = p.price;
					if(d.qty === 0) {
						d.qty = 1;
					}
					d.subtotal = (p.price - d.discount) * d.qty;
					d.unit = p.unit;
					d.productId = p.id;
					d.id = data.length;
					d.direction = 1;
					currentId = d.id;
					currentDetail = d;
					updateCurrentDetail(d);
					// setTimeout(() =>{
					// 	setFocuse("#qty-id");
					// }, 100);

					await tick();
					currentKey = "qty";
					setFocuse("#qty-id");
					setStringValue(d.qty, d.discount);
					// console.log("ON-CHANGE")
				}
			} else {
				await tick();
				currentKey = "barcode";
				setFocuse("#barcode-id");
			}
		}
	}

	async function barcodeOnKeyDown(e: KeyboardEvent, id: number) {
		if (e.key === "Enter" || (e.key === "Tab" && !e.shiftKey)) {

			if(isBarcodeDirty) return;
			currentKey = "qty";
			if(e.key === "Enter") {
				e.preventDefault();
				const ctlId = "#"+currentKey+"-id";
				setFocuse(ctlId);
			}
		} else if (e.key === "Tab" && e.shiftKey) {
			e.preventDefault();
			let i = data.findIndex((f) => f.id === id);
			currentKey = "discount";
			if (i === 0) {
				i = data.length;
			}

			currentDetail = data[i - 1];
			currentId = currentDetail.id;

			setStringValue(currentDetail.qty, currentDetail.discount);

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

	function setStringValue(qty: number, discount: number) {
		strQty = formatRupiah(qty);
		strDiscount = formatRupiah(discount);
	}

	onDestroy(() => {

		try {
			document.removeEventListener("click", clickOutSize);
		} catch (ex: any) {
			console.log(ex.message);
		}
	});

	onMount(() => {
		document.addEventListener("click", clickOutSize);
	});


	$: console.log(currentKey);
</script>

<div
	bind:this={reform}
	tabindex={9}
	role="row"
	aria-labelledby="tablw-detail"
	on:keydown={onTableKeyDown}
	on:focus={onTablePointerEnter}
>
	<DataTable
		rows={data}
		{headers}
		zebra
		size="short"
		on:click:row={onRowClick}
		on:click:cell={onCellClik}
	>
		<svelte:fragment slot="cell-header" let:header>
			{#if header.key === "price" || header.key === "hpp" || header.key === "qty" || header.key === "discount" || header.key === "subtotal"}
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
						tabindex={10}
						list="barcode-list"
						bind:value={currentDetail.barcode}
						size={"sm"}
						class="cell-edit"
						id="barcode-id"
						on:input={() => isBarcodeDirty = true}
						on:change={(e) => onBarcodeChange(e, row.id)}
						on:focus={(e) => (currentKey = "barcode")}
						on:keydown={(e) => barcodeOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === "qty"}
					<NumberInput
						tabindex={11}
						bind:value={strQty}
						size="sm"
						classes="cell-edit input-number"
						id="qty-id"
						on:change={(e) => qtyOnChange(e, row.id)}
						on:focus={() => (currentKey = "qty")}
						on:keydown={(e) => qtyOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === "discount"}
					<NumberInput
						tabindex={12}
						bind:value={strDiscount}
						size="sm"
						classes="cell-edit input-number"
						id="discount-id"
						on:change={(e) => discountOnChange(e, row.id)}
						on:focus={() => (currentKey = "discount")}
						on:keydown={(e) => discountOnKeyDown(e, row.id)}
					/>
				{:else if cell.key === "price" || cell.key === "hpp" || cell.key === "qty" || cell.key === "discount" || cell.key === "subtotal"}
					<div class="cell-right">{formatRupiah(cell.value)}</div>
				{:else}
					{cell.value}
				{/if}
				<!-- </form> -->
			{:else if cell.key === "price" || cell.key === "hpp" || cell.key === "qty" || cell.key === "discount" || cell.key === "subtotal"}
				<div class="cell-right">{formatRupiah(cell.value)}</div>
			{:else}
				{cell.value}
			{/if}
		</svelte:fragment>
	</DataTable>
</div>

<datalist id="barcode-list">
	<option value="SY12" />
	<option value="SY16" />
	<option value="ST40" />
	<option value="MK" />
	<option value="MRK12" />
	<option value="GF" />
	<option value="JS12" />
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
</style>
