<script lang="ts">
	import { formatRupiah } from "$lib/components/NumberFormat";
	import type { iStockDetail } from "$lib/interfaces";
	import { DataTable, TextInput } from "carbon-components-svelte";
	import type {
		DataTableCell,
		DataTableHeader,
		DataTableRow,
	} from "carbon-components-svelte/types/DataTable/DataTable.svelte";
	import { onDestroy, onMount } from "svelte";

	let reform: HTMLDivElement;

	let headers = [
		{ key: "barcode", value: "Barcode", width: "20%" },
		{ key: "name", value: "Nama barang", width: "auto" },
		{ key: "qty", value: "Qty", width: "80px" },
		{ key: "price", value: "Harga", width: "100px" },
		{ key: "discount", value: "Discount", width: "120px" },
		{ key: "subtotal", value: "Subtotal", width: "120px" },
	];

	export let data: iStockDetail[] = [];
	let currentId = 0;
	let currentDetail: iStockDetail;
	let currentKey = "barcode";

	function onRowClick(e: CustomEvent<DataTableRow>) {
		e.preventDefault();
		let d = e.detail as iStockDetail;
		currentDetail = d;
		let setFocus = false;

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

		if (withinBoundaries) {
			// console.log("Click happened inside element");
		} else {
			// console.log("Click happened **OUTSIDE** element");
			currentId = 0;
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

	onDestroy(() => {
		document.removeEventListener("click", clickOutSize);
	});

	onMount(() => {
		document.addEventListener("click", clickOutSize);
	});
</script>

<div bind:this={reform}>
	<DataTable
		rows={data}
		{headers}
		zebra
		on:click:row={onRowClick}
		on:click:cell={onCellClik}
		on:mouseenter:row={(e) => {
			// currentId = e.detail.id;
			// console.log(currentId);
			currentDetail = data.filter((f) => f.id === currentId)[0];
		}}
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
				{#if cell.key === "qty"}
					<TextInput
						bind:value={currentDetail.qty}
						size={"sm"}
						class="cell-edit"
						id="qty-id"
						on:keydown={(e) => {
							if ((e.key === "Tab" && !e.shiftKey) || e.key === "Enter") {
								e.preventDefault();
								let i = data.findIndex((f) => f.id === row.id);
								i++;
								currentKey = "barcode";
								if (i === data.length) {
									currentDetail = data[0];
									currentId = currentDetail.id;
									// e.preventDefault();
								} else {
									currentDetail = data[i];
									currentId = currentDetail.id;
								}

								setTimeout(() => {
									const ctlId = "#" + currentKey + "-id";
									const elem = document.querySelector(ctlId);
									if (elem) {
										elem.focus();
										elem.select();
									}
								}, 100);
							}
						}}
					/>
				{:else if cell.key === "barcode"}
					<TextInput
						list="barcode-list"
						bind:value={currentDetail.barcode}
						size={"sm"}
						class="cell-edit"
						id="barcode-id"
						on:keydown={(e) => {
							if (e.key === "Enter") {
								e.preventDefault();
								const ctlId = "#qty-id";
								const elem = document.querySelector(ctlId);
								if (elem) {
									elem.focus();
									elem.select();
								}
							} else if ((e.key === "Tab" || e.key === "Enter") && e.shiftKey) {
								const i = data.findIndex((f) => f.id === row.id);
								if (i > 0) {
									e.preventDefault();
									currentDetail = data[i - 1];
									currentKey = "qty";
									currentId = currentDetail.id;
								}
							}
						}}
					/>
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
	<option value="GG12" />
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
		padding: 0;
	}
</style>
