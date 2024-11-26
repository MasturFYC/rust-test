<script lang="ts">
import { type iCategory } from '$lib/interfaces';
import {
	Button,
	DataTable,
	Toolbar,
	ToolbarContent,
	ToolbarMenu,
	ToolbarMenuItem
} from 'carbon-components-svelte';
import { NewTab } from 'carbon-icons-svelte';
import type {
	DataTableHeader,
	DataTableCell,
	DataTableRow
} from 'carbon-components-svelte/src/DataTable/DataTable.svelte';

let {
	categories = [],
	cellSnippet,
	onNewCategory
}: {
	categories: iCategory[];
	cellSnippet: (
		cell: DataTableCell<DataTableRow>,
		row: DataTableRow
	) => ReturnType<import('svelte').Snippet>;
	onNewCategory: () => void;
} = $props();

// let data: iResult = {
// 	count: 0,
// 	data: [],
// 	status: ""
// };

// function delete_category(e: number) {
// 	isUpdating = true;
// 	$deleteData.mutate(e);
// }

let headers: DataTableHeader[] = [
	{ key: 'id', value: '#ID', width: '10%' },
	{ key: 'name', value: 'Nama', width: 'auto' },
	{ key: 'cmd', value: '', width: '150px' }
];

// 	const descriptionMap = [
// 		"Submitting...",
// 		"Success",
// 		"Cancelling...",
// ]	;

// 	const stateMap = [
// 		"finished",
// 		"dormant",
// 		"dormant",
// 	];

// 	let timeout: NodeJS.Timeout;
// 	let state = 1;

// 	function reset(incomingState: number) {
// 		if (incomingState > 2) {
// 			clearTimeout(timeout);
// 		}

// 		if (incomingState) {
// 			timeout = setTimeout(() => {
// 				state = incomingState;
// 			}, 2000);
// 		}
// 	}

// 	onDestroy(() => reset(4));

// 	$: reset(3);

//  let selectedRowIds = [categories.length > 0 ? categories[0].id : 0];

let client_width = $state(0);

// $: console.log("selectedRowIds", selectedRowIds);
</script>

<svelte:window bind:innerWidth={client_width} />

<DataTable
	useStaticWidth={client_width > 640}
	zebra
	size="short"
	headers={headers}
	rows={categories}
>
	<svelte:fragment slot="cell" let:row let:cell>
		{@render cellSnippet(cell, row)}
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
			<Button on:click={() => onNewCategory()} icon={NewTab}>Buat baru</Button>
		</ToolbarContent>
	</Toolbar>
</DataTable>
