<script lang="ts">
	import { formatNumber } from '$lib/components/NumberFormat';
	import type { iRelationProp, iOrder } from '$lib/interfaces';
	import {
		Button,
		ComboBox,
		DataTable,
		Toolbar,
		ToolbarContent,
		ToolbarSearch
	} from 'carbon-components-svelte';
	import { Close, Delete, Edit } from 'carbon-icons-svelte';
	import dayjs from 'dayjs';
	import OrderInfo from './OrderInfo.svelte';
	import { isOrderLoading, isOrderUpdating } from './store';
	import { tick } from 'svelte';
	import type { DataTableHeader } from 'carbon-components-svelte/src/DataTable/DataTable.svelte';

	interface Props {
		data: iOrder[] | undefined;
		customers: iRelationProp[] | undefined;
		sales: iRelationProp[] | undefined;
		txt: string;
		selectedCustomerId: number;
		selectedSalesId: number;
		ondelete: (e: number[]) => void;
		onsearch: (e: string | undefined) => void;
		onedit: (e: number) => void;
		oncustomerchange: (e: number) => void;
		onsaleschange: (e: number) => void;
	}

	let {
		data = [],
		customers = [],
		sales = [],
		txt = '',
		selectedCustomerId = 0,
		selectedSalesId = 0,
		onedit,
		ondelete,
		onsearch,
		oncustomerchange,
		onsaleschange
	}: Props = $props();

	let selectedRowIds: number[] = $state([]);
	let searchText = $state(txt);

	const headers: DataTableHeader[] = [
		{ key: 'id', value: '#ID', width: '10%' },
		{ key: 'createdAt', value: 'Tanggal', width: 'auto' },
		{ key: 'customerName', value: 'Customer', width: 'auto' },
		{ key: 'salesName', value: 'Sales', width: 'auto' },
		{ key: 'total', value: 'Total', width: '110px' },
		{ key: 'cmd', value: '', width: '60px' }
	];

	async function editOrder(id: number) {
		isOrderLoading.set(true);
		await tick();
		onedit(id);
	}

	function searchOrder(e: Event): void {
		e.preventDefault();
		onsearch(searchText);
	}

	function searchClear(e: Event): void {
		e.preventDefault();
		onsearch(undefined);
	}

	async function deleteItems(_e: MouseEvent) {
		isOrderUpdating.set(true);
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
		{#if cell.key === 'id'}
			<div>{row['id']}</div>
		{:else if cell.key === 'total'}
			<div class="cell-right">{formatNumber(cell.value)}</div>
		{:else if cell.key === 'createdAt'}
			<div>{dayjs(cell.value).format('DD-MM-YYYY')}</div>
		{:else if cell.key === 'cmd'}
			<Button
				tooltipPosition="left"
				tooltipAlignment="end"
				size="small"
				kind="ghost"
				iconDescription="Edit #{row['id']}"
				disabled={$isOrderUpdating || $isOrderLoading}
				skeleton={$isOrderLoading}
				icon={Edit}
				onclick={() => editOrder(row.id)}
			/>
		{:else}
			{cell.value}
		{/if}
	</svelte:fragment>
	<svelte:fragment slot="expanded-row" let:row>
		<OrderInfo data={data.filter((f) => f.id === row.id)[0]} />
	</svelte:fragment>
	<Toolbar size="sm">
		<ToolbarContent>
			<ToolbarSearch
				on:change={searchOrder}
				bind:value={searchText}
				on:clear={searchClear}
			/>
			<ComboBox
				type="inline"
				light={selectedCustomerId > 0}
				size="sm"
				style="width: 165px; border-bottom: none;"
				class={'supplier'}
				placeholder="Customer"
				selectedId={selectedCustomerId}
				items={customers.map((m) => ({ id: m.id, text: m.text }))}
				on:select={(e) => {
					oncustomerchange(e.detail.selectedId);
				}}
				on:clear={() => {
					oncustomerchange(0);
				}}
			/>
			<ComboBox
				type="inline"
				light={selectedSalesId > 0}
				class={'supplier'}
				size="sm"
				selectedId={selectedSalesId}
				style="width: 165px; border-bottom: none;"
				placeholder="Sales"
				items={sales.map((m) => ({ id: m.id, text: m.text }))}
				on:select={(e) => {
					onsaleschange(e.detail.selectedId);
				}}
				on:clear={() => {
					onsaleschange(0);
				}}
			/>
			<Button
				kind="danger"
				size="small"
				disabled={selectedRowIds.length == 0 || $isOrderLoading}
				skeleton={$isOrderUpdating}
				icon={Delete}
				on:click={deleteItems}>Hapus order</Button
			>
			<Button
				size="small"
				kind={'danger-ghost'}
				disabled={$isOrderUpdating || $isOrderLoading}
				on:click={() => editOrder(0)}
				icon={Close}>Cancel</Button
			>
		</ToolbarContent>
	</Toolbar>
</DataTable>
