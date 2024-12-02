<style lang="css">
	:global(#menu-gud-ware.bx--list-box__menu) {
		position: fixed;
		z-index: 1;
		margin-left: 17px;
		margin-right: 17px;
		width: auto;
	}
	:global(#gud-ware) {
		margin-top: 6px;
		margin-bottom: 6px;
	}
</style>

<script lang="ts">
	import { type iGudang } from '$lib/interfaces';
	import type { Snippet } from 'svelte';

	import {
		Button,
		DataTable,
		Toolbar,
		ToolbarContent,
		ToolbarMenu,
		ToolbarMenuItem
	} from 'carbon-components-svelte';
	import { NewTab } from 'carbon-icons-svelte';
	import type { DataTableHeader } from 'carbon-components-svelte/src/DataTable/DataTable.svelte';

	interface Props {
		gudangs: iGudang[];
		onNew: (e: iGudang) => void;
		tools: (id: number) => ReturnType<Snippet>;
	}

	let { gudangs = [], onNew, tools }: Props = $props();

	let client_width = $state(0);

	function newGudang() {
		const newData: iGudang = {
			id: 0,
			name: '',
			employeeId: 0,
			employeeName: ''
		};
		onNew(newData);
	}

	let headers: DataTableHeader[] = [
		{ key: 'id', value: '#ID', width: '10%' },
		{ key: 'name', value: 'Nama', width: 'auto' },
		{ key: 'employeeName', value: 'Penjaga Gudang', width: 'auto' },
		{ key: 'locate', value: 'Lokasi Gudang', width: 'auto' },
		{ key: 'cmd', value: '', width: '150px' }
	];
</script>

<svelte:window bind:innerWidth={client_width} />

<DataTable zebra headers={headers} rows={gudangs}>
	<svelte:fragment slot="cell" let:row let:cell>
		{#if cell.key === 'cmd'}
			{@render tools(row.id)}
		{:else if cell.key === 'locate'}
			{cell.value ?? '-'}
		{:else}
			{cell.value}
		{/if}
	</svelte:fragment>

	<Toolbar size="sm">
		<ToolbarContent>
			<!-- <ToolbarSearch /> -->
			<ToolbarMenu>
				<ToolbarMenuItem primaryFocus>Restart all</ToolbarMenuItem>
				<ToolbarMenuItem href="https://cloud.ibm.com/docs/loadbalancer-service"
					>API documentation</ToolbarMenuItem
				>
				<ToolbarMenuItem hasDivider danger>Stop all</ToolbarMenuItem>
			</ToolbarMenu>
			<Button on:click={newGudang} icon={NewTab}>Buat baru</Button>
		</ToolbarContent>
	</Toolbar>
</DataTable>
