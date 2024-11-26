<style lang="scss">
p {
	margin-top: 12px;
}
</style>

<script lang="ts">
import { browser } from '$app/environment';
import { baseURL, credential_include, type iCategory } from '$lib/interfaces';
import { useMutation, useQuery, useQueryClient } from '@sveltestack/svelte-query';
import ListCategory from './list.svelte';
import { Category } from 'carbon-icons-svelte';
import {
	Button,
	FluidForm,
	InlineLoading,
	Modal,
	TextInput,
	ToastNotification
} from 'carbon-components-svelte';
import type {
	DataTableRow,
	DataTableCell
} from 'carbon-components-svelte/src/DataTable/DataTable.svelte';
import DeleteCategory from './DeleteCategory.svelte';
import { Edit, Save } from 'carbon-icons-svelte';

type iResult = {
	count: number;
	data: iCategory[];
	status: string;
};

const client = useQueryClient();
const title = 'Kategori Barang';
const url = `${baseURL}/categories`;

let isError = $state(false);
let err_msg = $state('');
let isUpdating = $state(false);
let open = $state(false);
let category: iCategory = $state({
	id: 0,
	name: ''
});
let timeout: undefined | number = $state(undefined);
let showNotification = $state(false);

$effect(() => {
	showNotification = timeout !== undefined;
});

async function fetchCategories(): Promise<iResult> {
	const options = {
		headers: {
			'content-type': 'application/json'
		},
		method: 'GET',
		credentials: credential_include
	};

	const request = new Request(url, options);
	let result = await fetch(request);

	return (await result.json()) as iResult;
}

const fetchUpdateData = async (_e: iCategory): Promise<iCategory> => {
	const url = `${baseURL}/categories/${category.id}`;
	const request = new Request(url, {
		headers: {
			'content-type': 'application/json'
		},
		body: JSON.stringify(category),
		method: 'PUT',
		credentials: credential_include
	});

	const result = await fetch(request);
	return await result.json();
};

const fetchCreateData = async (_e: iCategory): Promise<iCategory> => {
	const url = `${baseURL}/categories`;
	const request = new Request(url, {
		headers: {
			'content-type': 'application/json'
		},
		body: JSON.stringify(category),
		method: 'POST',
		credentials: credential_include
	});

	return await (await fetch(request)).json();
};

const fetchDeleteData = async (e: number) => {
	const url = `${baseURL}/categories/${e}`;
	const request = new Request(url, {
		method: 'DELETE',
		credentials: credential_include
	});

	return await (await fetch(request)).json();
};

const createData = useMutation(fetchCreateData, {
	onMutate: async (_e: iCategory) => {
		// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
		await client.cancelQueries();

		// Snapshot the previous value
		const previousData = client.getQueryData<iResult>(['category', 'list']);

		// Optimistically update to the new value
		if (previousData) {
			client.setQueryData<iResult>(['category', 'list'], previousData);
		}

		return previousData;
	},
	onSuccess: async (data: any, _variable: iCategory, context) => {
		if (context) {
			setTimeout(() => {
				isUpdating = false;
				if (data.status !== 'fail') {
					open = false;
				} else {
					isError = true;
					err_msg = data.message;
				}
			}, 250);
			//await client.invalidateQueries(["category", "list"]);
			//client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
		}
	},
	// If the mutation fails, use the context returned from onMutate to roll back
	onError: (_err: any, _variables: any, context: any) => {
		// console.log(err);
		if (context?.previousData) {
			client.setQueryData<iResult>(['category', 'list'], context.previousData);
		}
		//      selectedCategoryId.set($category.id)
		// errorMesage.set(`Nama kategori '${$category.name}'' sudah ada!`);
	},
	// Always refetch after error or success:
	onSettled: async () => {
		await client.invalidateQueries(['category', 'list']);
	}
});

const updateData = useMutation(fetchUpdateData, {
	onMutate: async (_e: iCategory) => {
		// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
		await client.cancelQueries();

		// Snapshot the previous value
		const previousCategories = client.getQueryData<iResult>(['category', 'list']);

		// Optimistically update to the new value
		if (previousCategories) {
			client.setQueryData<iResult>(['category', 'list'], previousCategories);
		}

		return previousCategories;
	},
	onSuccess: async (data: any, _variable: iCategory, context) => {
		if (context) {
			setTimeout(() => {
				isUpdating = false;
				if (data.status !== 'fail') {
					open = false;
				} else {
					isError = true;
					err_msg = data.message;
				}
			}, 1500);
			//        await client.invalidateQueries(["category", "list"]);
			//client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
		}
	},
	// If the mutation fails, use the context returned from onMutate to roll back
	onError: (_err: any, _variables: any, context: any) => {
		if (context?.previousCategories) {
			client.setQueryData<iResult>(['category', 'list'], context.previousCategories);
			//        selectedCategoryId.set($category.id)
		}
		// errorMesage.set(`Nama kategori '${$category.name}' sudah ada!`);
	},
	onSettled: async (
		_data: any,
		_error: any,
		_variables: iCategory,
		_context: iResult | undefined
	) => {
		await client.invalidateQueries(['category', 'list']);
	}
});

const deleteData = useMutation(fetchDeleteData, {
	onMutate: async (_e: number) => {
		// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
		await client.cancelQueries();

		// Snapshot the previous value
		const previousCategories = client.getQueryData<iResult>(['category', 'list']);

		// Optimistically update to the new value
		if (previousCategories) {
			client.setQueryData<iResult>(['category', 'list'], previousCategories);
		}

		return previousCategories;
	},
	onSuccess: async () => {
		setTimeout(() => {
			isUpdating = false;
		}, 1500);

		category = {
			id: 0,
			name: ''
		};
	},
	// If the mutation fails, use the context returned from onMutate to roll back
	onError: (_err: any, _variables: any, context: any) => {
		if (context?.previousCategories) {
			client.setQueryData<iResult>(['category', 'list'], context.previousCategories);
		}
	},
	onSettled: async (data: any, _error: any, _variables: number, _context: iResult | undefined) => {
		if (data.status === 'fail') {
			showNotification = true;
			err_msg = data.message;
			timeout = 3_000;
		}
		await client.invalidateQueries(['category', 'list']);
		isUpdating = false;
	}
});

const queryCategoryOptions = () => ({
	queryKey: ['category', 'list'],
	queryFn: async () => await fetchCategories(),
	enabled: browser
});

const query = useQuery<iResult, Error>(queryCategoryOptions());

let categories = $derived.by<iCategory[]>(() => {
	if ($query.isSuccess && $query.data) {
		return $query.data.data;
	}
	return [];
});

function showErrorMessage() {
	if ($query.error instanceof Error) {
		return $query.error.message;
	}
	return 'Cannot load category list.';
}

function edit_category(id: number) {
	isError = false;
	err_msg = '';
	// timeout = undefined;

	const test = categories.filter((f) => f.id == id);

	if (test.length > 0) {
		category = test[0];
		open = true;
	}
}

function new_category() {
	isError = false;
	err_msg = '';
	// timeout = undefined;

	category = {
		id: 0,
		name: ''
	};

	open = true;
}

function delete_category(e: number) {
	isUpdating = true;
	$deleteData.mutate(e);
}

function submit() {
	isError = false;
	isUpdating = true;
	if (category.id > 0) {
		$updateData.mutate(category);
	} else {
		$createData.mutate(category);
	}
}

$effect(() => query.setEnabled(browser));
$inspect(categories);
</script>

{#snippet cellSnippet(cell: DataTableCell<DataTableRow>, row: DataTableRow)}
	{#if cell.key === 'cmd'}
		<Button
			tooltipPosition="left"
			tooltipAlignment="end"
			size="small"
			kind="ghost"
			iconDescription="Edit"
			icon={Edit}
			on:click={() => edit_category(row.id)}
		/>
		<DeleteCategory categoryId={row.id} onDeleteData={delete_category} />
	{:else}
		{cell.value}
	{/if}
{/snippet}

<svelte:head>
	<title>{title}</title>
	<meta name="description" content="Category this app" />
</svelte:head>

<h2><Category size={24} /> {title}</h2>

{#if $query.isLoading}
	<p>Loading...</p>
{:else if $query.isError}
	<p>Error: {showErrorMessage()}</p>
{/if}

<ListCategory
	categories={categories}
	cellSnippet={cellSnippet}
	onNewCategory={() => new_category()}
/>
<p>Total: {categories.length} item{categories.length > 1 ? 's' : ''}</p>

<Modal
	open={open}
	on:close={() => {
		open = false;
		isUpdating = false;
	}}
	hasForm
	preventCloseOnClickOutside
	modalHeading="Kategori"
	primaryButtonText="Simpan"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={'#cat-name'}
	on:click:button--secondary={() => (open = false)}
	on:submit={submit}
	size="xs"
	primaryButtonDisabled={isUpdating}
>
	<FluidForm>
		<TextInput id="cat-name" bind:value={category.name} labelText="Nama" />
	</FluidForm>

	{#if isUpdating}
		<InlineLoading
			status="active"
			description={category.id === 0 ? 'Posting data...' : 'Updating data...'}
		/>
	{/if}

	{#if isError}
		<InlineLoading description={err_msg} status="error" />
	{/if}
</Modal>

{#if showNotification}
	<ToastNotification
		timeout={timeout}
		title="Error"
		subtitle={err_msg}
		caption={new Date().toLocaleString()}
		on:close={(_e) => {
			timeout = undefined;
		}}
	/>
{/if}
