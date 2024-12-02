<style lang="scss">
	p {
		margin-top: 12px;
	}
</style>

<script lang="ts">
	import { browser } from '$app/environment';
	import { baseURL, credential_include, type iGudang } from '$lib/interfaces';
	import { useQuery } from '@sveltestack/svelte-query';
	import ListCategory from './list.svelte';
	import { getRelationProp } from '$lib/fetchers';
	import { IbmDb2Warehouse as Warehouse } from 'carbon-icons-svelte';
	import { useMutation, useQueryClient } from '@sveltestack/svelte-query';
	import FormGudang from './form.svelte';
	import { Button, ToastNotification } from 'carbon-components-svelte';
	import DeleteGudang from '$lib/components/DeleteButton.svelte';
	import { Edit } from 'carbon-icons-svelte';

	type iResult = {
		count: number;
		data: iGudang[];
		status: string;
	};

	const title = 'Gudang';
	const client = useQueryClient();
	const url = `${baseURL}/gudangs`;
	const qaKey = ['gudang', 'list'];
	const initGudang: iGudang = {
		id: 0,
		name: '',
		employeeId: 0,
		employeeName: '',
		locate: ''
	};

	let open = $state(false);
	let isError = $state(false);
	let gudang: iGudang = $state(initGudang);
	let timeout: undefined | number = $state(undefined);
	let isUpdating = $state(false);
	let errorMessage = $state('');
	let showNotification = $derived(timeout !== undefined);

	const fetchUpdateData = async (e: iGudang): Promise<iGudang> => {
		const url = `${baseURL}/gudangs/${e.id}`;
		const request = new Request(url, {
			headers: {
				'content-type': 'application/json'
			},
			body: JSON.stringify(e),
			method: 'PUT',
			credentials: credential_include
		});

		const result = await fetch(request);
		return await result.json();
	};

	const fetchCreateData = async (e: iGudang): Promise<iGudang> => {
		const url = `${baseURL}/gudangs`;
		const request = new Request(url, {
			headers: {
				'content-type': 'application/json'
			},
			body: JSON.stringify(e),
			method: 'POST',
			credentials: credential_include
		});

		return await (await fetch(request)).json();
	};

	const fetchDeleteData = async (e: number) => {
		const url = `${baseURL}/gudangs/${e}`;
		const request = new Request(url, {
			method: 'DELETE',
			credentials: credential_include
		});

		return await (await fetch(request)).json();
	};

	const createData = useMutation(fetchCreateData, {
		onMutate: async (_: iGudang) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousData = client.getQueryData<iResult>(qaKey);

			// Optimistically update to the new value
			if (previousData) {
				client.setQueryData<iResult>(['category', 'list'], previousData);
			}

			return previousData;
		},
		onSuccess: async (data: any, _variable: iGudang, context) => {
			if (context) {
				setTimeout(() => {
					isUpdating = false;
					if (data.status !== 'fail') {
						open = false;
					} else {
						isError = true;
						errorMessage = data.message;
					}
				}, 1000);
				//await client.invalidateQueries(["category", "list"]);
				//client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, _variables: any, context: any) => {
			console.log(err);
			if (context?.previousData) {
				client.setQueryData<iResult>(qaKey, context.previousData);
			}
			//      selectedCategoryId.set($category.id)
			// errorMesage.set(`Nama kategori '${$category.name}'' sudah ada!`);
		},
		// Always refetch after error or success:
		onSettled: async () => {
			await client.invalidateQueries(qaKey);
		}
	});

	const updateData = useMutation(fetchUpdateData, {
		onMutate: async (_: iGudang) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousGudang = client.getQueryData<iResult>(qaKey);

			// Optimistically update to the new value
			if (previousGudang) {
				client.setQueryData<iResult>(qaKey, previousGudang);
			}

			return previousGudang;
		},
		onSuccess: async (data: any, _variables: iGudang, context) => {
			if (context) {
				setTimeout(() => {
					isUpdating = false;
					if (data.status !== 'fail') {
						open = false;
					} else {
						isError = true;
						errorMessage = data.message;
					}
				}, 1000);
				//        await client.invalidateQueries(["category", "list"]);
				//client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (_err: any, _variables: any, context: any) => {
			if (context?.previousGudang) {
				client.setQueryData<iResult>(qaKey, context.previousGudang);
				//        selectedCategoryId.set($category.id)
			}
			// errorMesage.set(`Nama kategori '${$category.name}' sudah ada!`);
		},
		onSettled: async (
			_data: any,
			_error: any,
			_variables: iGudang,
			_context: iResult | undefined
		) => {
			await client.invalidateQueries(qaKey);
		}
	});

	const deleteData = useMutation(fetchDeleteData, {
		onMutate: async (_e: number) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousGudang = client.getQueryData<iResult>(qaKey);

			// Optimistically update to the new value
			if (previousGudang) {
				client.setQueryData<iResult>(qaKey, previousGudang);
			}

			return previousGudang;
		},
		onSuccess: async () => {
			setTimeout(() => {
				isUpdating = false;
			}, 1000);

			gudang = {
				id: 0,
				name: '',
				employeeId: 0,
				employeeName: ''
			};
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (_err: any, _variables: any, context: any) => {
			if (context?.previousGudang) {
				client.setQueryData<iResult>(qaKey, context.previousGudang);
			}
		},

		onSettled: async (
			data: any,
			_error: any,
			_variables: number,
			_context: iResult | undefined
		) => {
			if (data.status === 'fail') {
				errorMessage = data.message;
				timeout = 3_000;
			}
			await client.invalidateQueries(qaKey);
			isUpdating = false;
		}
	});

	async function fetchGudangs(): Promise<iResult> {
		const options = {
			headers: {
				'content-type': 'application/json'
			},
			method: 'GET',
			credentials: credential_include
		};

		const request = new Request(url, options);
		const result = await fetch(request);

		return (await result.json()) as iResult;
	}

	const gudangQueryOptions = () => ({
		queryKey: qaKey,
		queryFn: async () => await fetchGudangs(),
		enabled: browser
	});

	const employeeQueryOptions = () => ({
		queryKey: ['relation', 'employee'],
		queryFn: async () => await getRelationProp(['Employee']),
		enabled: browser
	});

	function showErrorMessage() {
		if ($query.error instanceof Error) {
			return $query.error.message;
		}
		return 'Cannot load gudang.';
	}

	function submit(e: iGudang) {
		isError = false;
		isUpdating = true;
		if (e.id > 0) {
			$updateData.mutate(e);
		} else {
			$createData.mutate(e);
		}
	}

	function deleteGudang(e: number) {
		isUpdating = true;
		$deleteData.mutate(e);
	}

	function newGudang(e: iGudang) {
		isError = false;
		errorMessage = '';
		gudang = { ...e };
		open = true;
	}

	function editGudang(e: number) {
		isError = false;
		errorMessage = '';
		// timeout = undefined

		if ($query.data) {
			let test = gudangs.filter((f) => f.id === e);
			if (test.length > 0) {
				gudang = { ...test[0] };
				open = true;
			}
		}
	}

	const query = useQuery<iResult, Error>(gudangQueryOptions());
	const employeeQuery = useQuery(employeeQueryOptions());

	let gudangs = $derived.by(() => {
		if ($query.isSuccess && $query.data) {
			return $query.data.data;
		}
		return [];
	});

	let employees = $derived.by(() => {
		if ($employeeQuery.isSuccess && $employeeQuery.data) {
			return $employeeQuery.data.data;
		}
		return [];
	});

	let countOfGudangs = $derived(gudangs.length);

	$effect.pre(() => {
		employeeQuery.setEnabled(browser);
		query.setEnabled(browser);
	});
</script>

{#snippet tools(id: number)}
	<Button
		tooltipPosition="left"
		tooltipAlignment="end"
		size="small"
		kind="ghost"
		iconDescription="Edit"
		icon={Edit}
		on:click={() => editGudang(id)}
	/>
	<DeleteGudang idData={id} onDeleteData={deleteGudang} disabled={id === 1} />
{/snippet}

<svelte:head>
	<title>{title}</title>
	<meta name="description" content="Gudang this app" />
</svelte:head>

<h2><Warehouse size={24} /> {title}</h2>

{#if $query.isLoading || $employeeQuery.isLoading}
	<p>Loading...</p>
{:else if $query.isError}
	<p>Error: {showErrorMessage()}</p>
{/if}

<ListCategory tools={tools} gudangs={gudangs} onNew={newGudang} />
<p>Total: {countOfGudangs} item{countOfGudangs > 1 ? 's' : ''}</p>

{#if showNotification}
	<ToastNotification
		timeout={timeout}
		title="Error"
		subtitle={errorMessage}
		caption={new Date().toLocaleString()}
		on:close={() => {
			timeout = undefined;
		}}
	/>
{/if}

<FormGudang
	onSubmit={submit}
	bind:open={open}
	bind:gudang={gudang}
	isUpdating={isUpdating}
	isError={isError}
	errorMessage={errorMessage}
	employees={employees}
/>
