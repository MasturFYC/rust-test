<style lang="scss">
.cell-right {
	text-align: right;
}
/*
		 :global(table.bx--data-table th, table.bx--data-table--zebra) {
		background-color: #000;
		 }
		 */
</style>

<script lang="ts">
import { formatNumber } from '$lib/components/NumberFormat';
import type { iProduct, iProductStock, iPropertyID, iRelationProp } from '$lib/interfaces';
import {
	Button,
	ComboBox,
	DataTable,
	Loading,
	Toolbar,
	ToolbarContent,
	ToolbarSearch
} from 'carbon-components-svelte';
import { Edit, NewTab } from 'carbon-icons-svelte';
import ExapandedList from './ExapandedList.svelte';
import type { DataTableHeader } from 'carbon-components-svelte/src/DataTable/DataTable.svelte';
import type { Snippet } from 'svelte';

interface Props {
    data: iProduct[],
    innerWidth: number,
    suppliers: iRelationProp[],
    categories: iPropertyID[],
    onEdit: (id: number) => void,
    onSearch: (e: string | undefined) => void,
    onSupplierChange: (e: number) => void,
    onCategoryChange: (e: number) => void,
    deleteTool: (id: number) => ReturnType<Snippet>,
    isProductLoading?: boolean
}

let {
    data = [],
    innerWidth = $bindable(720),
    suppliers = [],
    categories = [],
    onEdit,
    onSearch,
    onSupplierChange,
    onCategoryChange,
    deleteTool,
    isProductLoading
}: Props = $props();

const headers: DataTableHeader[] = [
	{ key: 'name', value: 'Nama Barang', width: 'auto' },
	{ key: 'barcode', value: 'Barcode', width: '100px' },
	{ key: 'stocks', value: 'Stock', width: '90px' },
	{ key: 'hpp', value: 'HPP', width: '80px' },
	{ key: 'margin', value: 'Margin', width: '80px' },
	{ key: 'price', value: 'Harga', width: '90px' },
	{ key: 'cmd', value: '', width: '60px' }
];
const headers2: DataTableHeader[] = [
	{ key: 'name', value: 'Nama Barang', width: 'auto' },
	{ key: 'stocks', value: 'Stock', width: '90px' },
	{ key: 'price', value: 'Harga', width: '90px' },
	{ key: 'cmd', value: '', width: '60px' }
];

function get_headers() {
	if (innerWidth < 720) {
		return headers2;
	}
	return headers;
}

function edit_product(id: number) {
	onEdit(id);
}

let txt = $state('');

function submit_search(e: Event): void {
    e.preventDefault();
	onSearch(txt);
}

function search_clear(): void {
	onSearch(undefined);
}

// function get_suppliers() {
// 	return suppliers.map((m) => ({ id: m.id, text: m.text }));
// }
const defaultStock = (stocks: iProductStock[]) => {
	const d = stocks.filter((f) => f.gudangId === 1)[0];
	if (d) {
		return d.qty;
	}
	return 0;
};

let sup_light = $state(false);
let cat_light = $state(false);
// $: 	console.log(suppliers)
</script>

<DataTable expandable size="medium" headers={get_headers()} rows={data}>
	<svelte:fragment slot="cell-header" let:header>
		{#if header.key === 'price' || header.key === 'hpp' || header.key === 'stocks' || header.key === 'margin'}
			<div class="cell-right">{header.value}</div>
		{:else}
			{header.value}
		{/if}
	</svelte:fragment>

	<svelte:fragment slot="cell" let:row let:cell>
        {#if isProductLoading}  <Loading withOverlay={false} /> {:else}

        {#if cell.key === 'cmd'}
			<Button
				tooltipPosition="left"
				tooltipAlignment="end"
				size="small"
				kind="ghost"
				iconDescription="Edit"
				icon={Edit}
				on:click={() => edit_product(row.id)}
			/>
			<!-- <div></div> -->
		{:else if cell.key === 'relationType'}
			<div style="cell-right">{cell.value.join(', ')}</div>
		{:else if cell.key === 'hpp' || cell.key === 'price'}
			<div class="cell-right">{formatNumber(cell.value)}</div>
		{:else if cell.key === 'margin'}
			<div class="cell-right">{formatNumber(cell.value, 2)}%</div>
		{:else if cell.key === 'stocks'}
			<div class="cell-right">
				{formatNumber(defaultStock(cell.value))}
				{row['unit']}
			</div>
		{:else}
			{cell.value}
		{/if}
    {/if}
	</svelte:fragment>
	<svelte:fragment slot="expanded-row" let:row>
		<ExapandedList row={row} bind:innerWidth={innerWidth} {deleteTool} />
	</svelte:fragment>

	<Toolbar size="sm">
		<ToolbarContent>
			<ToolbarSearch on:change={submit_search} bind:value={txt} on:clear={search_clear} />
			<ComboBox
				type="inline"
				light={cat_light}
				size="sm"
				style="border-bottom: none"
				class={'supplier'}
				placeholder="filter by category"
				items={categories}
				on:select={(e) => {
					onCategoryChange(e.detail.selectedId);
					cat_light = true;
				}}
				on:clear={() => {
					cat_light = false;
					onCategoryChange(0);
				}}
			/>
			<ComboBox
				type="inline"
				light={sup_light}
				style="border-bottom: none"
				class={'supplier'}
				size="sm"
				placeholder="filter by supplier"
				items={suppliers.map((m) => ({ id: m.id, text: m.text }))}
				on:select={(e) => {
					onSupplierChange(e.detail.selectedId);
					sup_light = true;
				}}
				on:clear={() => {
					onSupplierChange(0);
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
			<Button on:click={() => edit_product(0)} icon={NewTab}>Buat baru</Button>
		</ToolbarContent>
	</Toolbar>
</DataTable>
