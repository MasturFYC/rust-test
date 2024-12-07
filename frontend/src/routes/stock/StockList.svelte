<script lang="ts">
	import { formatNumber } from '$lib/components/NumberFormat';
	import type { iRelationProp, iStock } from '$lib/interfaces';
	import {
		Button,
		ComboBox,
		DataTable,
		Toolbar,
		ToolbarContent,
		ToolbarSearch
	} from 'carbon-components-svelte';
	import { Delete, Edit, NewTab } from 'carbon-icons-svelte';
	import dayjs from 'dayjs';
	import StockInfo from './StockInfo.svelte';
	import { isStockLoading, isStockUpdating } from './store';
	import { tick } from 'svelte';
	import type { DataTableHeader } from 'carbon-components-svelte/src/DataTable/DataTable.svelte';

	interface Props {
		data: iStock[] | undefined;
		suppliers: iRelationProp[] | undefined;
		employees: iRelationProp[] | undefined;
		txt: string;
		selectedSupplierId: number;
		selectedWarehouseId: number;
		ondelete: (e: number[]) => void;
		onsearch: (e: string | undefined) => void;
		onedit: (e: number) => void;
		onsupplierchange: (e: number) => void;
		onwarehousechange: (e: number) => void;
	}

	let {
		data = [],
		suppliers = [],
		employees = [],
		txt = '',
		selectedSupplierId = 0,
		selectedWarehouseId = 0,
		onedit,
		ondelete,
		onsearch,
		onsupplierchange,
		onwarehousechange
	}: Props = $props();

	let selectedRowIds: number[] = $state([]);
	let searchText = $state(txt);

	const headers: DataTableHeader[] = [
		{ key: 'id', value: 'ID#', width: '12%' },
		{ key: 'createdAt', value: 'Tanggal', width: '120px' },
		{ key: 'invoiceId', value: 'No. Faktur', width: 'auto' },
		{ key: 'supplierName', value: 'Supplier', width: 'auto' },
		{ key: 'warehouseName', value: 'Checker', width: 'auto' },
		{ key: 'total', value: 'Total', width: '110px' },
		{ key: 'cmd', value: '', width: '60px' }
	];

	async function editStock(id: number) {
		isStockLoading.set(true);
		await tick();

		onedit(id);
	}

	function searchStock(e: Event): void {
		e.preventDefault();
		onsearch(searchText);
	}

	function searchClear(e: Event): void {
		e.preventDefault();
		onsearch(undefined);
	}

	async function deleteItems(_e: MouseEvent) {
		isStockUpdating.set(true);
		// await tick();
		ondelete(selectedRowIds);
		await tick();
		selectedRowIds = [];
	}
</script>

<DataTable
	batchSelection
	batchExpansion
	size="medium"
	zebra
	headers={headers}
	rows={data}
	bind:selectedRowIds={selectedRowIds}
>
	<svelte:fragment slot="cell-header" let:header>
		{#if header.key === 'total'}
			<div class="cell-right">{header.value}</div>
		{:else}
			{header.value}
		{/if}
	</svelte:fragment>

	<svelte:fragment slot="cell" let:row let:cell>
		{#if cell.key === 'total'}
			<div class="cell-right">{formatNumber(cell.value)}</div>
		{:else if cell.key === 'createdAt'}
			<div>{dayjs(cell.value).format('DD-MM-YYYY')}</div>
		{:else if cell.key === 'cmd'}
			<Button
				tooltipPosition="left"
				tooltipAlignment="end"
				size="small"
				kind="ghost"
				iconDescription="Edit"
				disabled={$isStockUpdating || $isStockLoading}
				skeleton={$isStockLoading}
				icon={Edit}
				onclick={() => editStock(row.id)}
			/>
		{:else}
			{cell.value}
		{/if}
	</svelte:fragment>
	<svelte:fragment slot="expanded-row" let:row>
		<StockInfo data={data.filter((f) => f.id === row.id)[0]} />
	</svelte:fragment>
	<Toolbar size="sm">
		<ToolbarContent>
			<ToolbarSearch
				on:change={searchStock}
				bind:value={searchText}
				on:clear={searchClear}
			/>
			<ComboBox
				type="inline"
				light={selectedSupplierId > 0}
				size="sm"
				style="width: 165px; border-bottom: none;"
				class={'supplier'}
				placeholder="Supplier"
				selectedId={selectedSupplierId}
				items={suppliers.map((m) => ({ id: m.id, text: m.text }))}
				on:select={(e) => {
					onsupplierchange(e.detail.selectedId);
				}}
				on:clear={() => {
					onsupplierchange(0);
				}}
			/>
			<ComboBox
				type="inline"
				light={selectedWarehouseId > 0}
				class={'supplier'}
				size="sm"
				selectedId={selectedWarehouseId}
				style="width: 165px; border-bottom: none;"
				placeholder="Penjaga gudang"
				items={employees.map((m) => ({ id: m.id, text: m.text }))}
				on:select={(e) => {
					onwarehousechange(e.detail.selectedId);
				}}
				on:clear={() => {
					onwarehousechange(0);
				}}
			/>
			<Button
				kind="danger"
				size="small"
				disabled={selectedRowIds.length == 0 || $isStockLoading}
				skeleton={$isStockUpdating}
				icon={Delete}
				on:click={deleteItems}>Hapus stock</Button
			>
			<Button
				size="small"
				disabled={$isStockUpdating || $isStockLoading}
				on:click={() => editStock(0)}
				icon={NewTab}>Buat baru</Button
			>
		</ToolbarContent>
	</Toolbar>
</DataTable>
