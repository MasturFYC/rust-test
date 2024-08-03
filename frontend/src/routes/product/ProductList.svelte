<script lang="ts">
	import { cardNumber, cardPercent } from "$lib/components/NumberFormat";
	import type { iProduct, iPropertyID, iRelationProp } from "$lib/interfaces";
	import {
		Button,
		ComboBox,
		DataTable,
		Toolbar,
		ToolbarContent,
		ToolbarSearch
	} from "carbon-components-svelte";
	import type { DataTableRow } from "carbon-components-svelte/types/DataTable/DataTable.svelte";
	import { Edit, NewTab } from "carbon-icons-svelte";
	import { createEventDispatcher } from "svelte";
	import DeleteRelation from "../relation/DeleteRelation.svelte";

	export let data: iProduct[] = [];
	export let innerWidth = 720;
	export let suppliers: iRelationProp[] = [];
	export let categories: iPropertyID[] = [];

	const dispatch = createEventDispatcher();
	const headers = [
		{ key: "name", value: "Nama Barang", width: "auto" },
		{ key: "barcode", value: "Barcode", width: "100px" },
		{ key: "unitInStock", value: "Stock", width: "90px" },
		{ key: "hpp", value: "HPP", width: "80px" },
		{ key: "margin", value: "Margin", width: "80px" },
		{ key: "price", value: "Harga", width: "90px" },
		{ key: "cmd", value: "", width: "118px" },
	];
	const headers2 = [
		{ key: "name", value: "Nama Barang", width: "auto" },
		{ key: "unitInStock", value: "Stock", width: "90px" },
		{ key: "price", value: "Harga", width: "90px" },
		{ key: "cmd", value: "", width: "118px" },
	];

	function get_headers() {
		if (innerWidth < 720) {
			return headers2;
		}
		return headers;
	}

	function edit_product(id: string | undefined) {
		dispatch("edit", id);
	}

	function product_info(r: DataTableRow): string {
		let info: string[] = [];
		let pad = 12;

		if (innerWidth < 720) {
			info.push("HPP:".padEnd(pad, " ") + cardNumber(r["hpp"]));
			info.push("Margin:".padEnd(pad, " ") + cardPercent(r["margin"]) + "%");
			info.push("\n");
		}
		info.push("barcode:".padEnd(pad, " ") + r["barcode"]);
		info.push("Variant:".padEnd(pad, " ") + r["variantName"]);
		info.push("Kategori:".padEnd(pad, " ") + r["categoryName"]);
		info.push("Supplier:".padEnd(pad, " ") + r["supplierName"]);
		info.push("Aktif:".padEnd(pad, " ") + (r["isActive"] ? "Ya" : "Tidak"));
		info.push("Deskripsi:".padEnd(pad, " ") + r["descriptions"]);

		return info.join("\n");
	}

	let txt = "";

	function submit_search(e: Event): void {
		dispatch("search", txt);
	}

	function search_clear(e: any): void {
		dispatch("search", undefined);
	}

	// function get_suppliers() {
	// 	return suppliers.map((m) => ({ id: m.id, text: m.text }));
	// }

	let sup_light = false;
	let cat_light = false;
	// $: 	console.log(suppliers)
</script>


<DataTable

	expandable
	size="compact"
	title="Data Barang"
	description="Tabel data barang / produk"
	headers={get_headers()}
	rows={data}
>
	<svelte:fragment slot="cell-header" let:header>
		{#if header.key === "price" || header.key === "hpp" || header.key === "unitInStock"}
			<div class="cell-right">{header.value}</div>
		{:else}
			{header.value}
		{/if}
	</svelte:fragment>

	<svelte:fragment slot="cell" let:row let:cell>
		{#if cell.key === "cmd"}
			<Button
				tooltipPosition="left"
				tooltipAlignment="end"
				size="small"
				kind="ghost"
				iconDescription="Edit"
				icon={Edit}
				on:click={() => edit_product(row.id)}
			/>
			<DeleteRelation idData={row.id} on:deleteData />
		{:else if cell.key === "relationType"}
			<div style="cell-right">{cell.value.join(", ")}</div>
		{:else if cell.key === "margin"}
			<div
				style="text-align:
				right;"
			>
				{cardPercent(cell.value).replace(/\,0*$|(\,\d*[1-9])0+$/, "$1")}%
			</div>
		{:else if cell.key === "hpp" || cell.key === "price"}
			<div class="cell-right">{cardNumber(cell.value)}</div>
		{:else if cell.key === "unitInStock"}
			<div class="cell-right">{cell.value} {row["unit"]}</div>
		{:else}
			{cell.value}
		{/if}
	</svelte:fragment>
	<svelte:fragment slot="expanded-row" let:row>
		<code><pre class="code-pre">{product_info(row)}</pre></code>
	</svelte:fragment>

	<Toolbar size="sm">
		<ToolbarContent>
			<ToolbarSearch
				on:change={submit_search}
				bind:value={txt}
				on:clear={search_clear}
			/>
			<ComboBox
				type="inline"
				light={cat_light}
				size="sm"
				class={"supplier"}
				placeholder="filter by category"
				items={categories}
				on:select={(e) => {
					dispatch("categoryChange", e.detail.selectedId);
					cat_light = true;
				}}
				on:clear={() => {
					cat_light = false;
					dispatch("categoryChange", 0);
				}}
			/>
			<ComboBox
				type="inline"
				light={sup_light}
				class={"supplier"}
				size="sm"
				placeholder="filter by supplier"
				items={suppliers.map((m) => ({ id: m.id, text: m.text }))}
				on:select={(e) => {
					dispatch("supplierChange", e.detail.selectedId);
					sup_light = true;
				}}
				on:clear={() => {
					dispatch("supplierChange", undefined);
					sup_light = false;
				}}
			/>
			<!-- <ToolbarMenu>
				<ToolbarMenuItem primaryFocus>Restart all</ToolbarMenuItem>
				<ToolbarMenuItem href="https://cloud.ibm.com/docs/loadbalancer-service"
					>API documentation</ToolbarMenuItem
				>
				<ToolbarMenuItem hasDivider danger>Stop all</ToolbarMenuItem>
			</ToolbarMenu> -->
			<Button on:click={() => edit_product(undefined)} icon={NewTab}
				>Buat baru</Button
			>
		</ToolbarContent>
	</Toolbar>
</DataTable>

<style lang="scss">
	.cell-right {
		text-align: right;
	}
	.code-pre {
		font-size: small;
	}
	/*
		 :global(table.bx--data-table th, table.bx--data-table--zebra) {
		background-color: #000;
		 }
		 */

	:global(.bx--list-box__field .bx--text-input.supplier) {
		border-bottom: none;
		// background-color: var(--cds-link-01);
		// color: var(--cds-ui-01);
	}
	:global(.bx--table-expand__button) {
		width: auto;
		min-height: 16px;
	}
</style>
