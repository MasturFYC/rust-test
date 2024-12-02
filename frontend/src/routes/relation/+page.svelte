<script lang="ts">
	import { browser } from '$app/environment';
	import FormRelation from '$lib/components/FormRelation.svelte';
	import {
		baseURL,
		credential_include,
		type iRelation,
		type RelationTypeWIthID
	} from '$lib/interfaces';
	import {
		useIsFetching,
		useMutation,
		useQuery,
		useQueryClient
	} from '@sveltestack/svelte-query';
	import { Pagination } from 'carbon-components-svelte';
	import RelationList from './RelationList.svelte';
	import { GroupAccount } from 'carbon-icons-svelte';
	// import { untrack } from "svelte";

	const initData: iRelation = {
		id: 0,
		name: '',
		city: '',
		isActive: true,
		isSpecial: false,
		relationType: ['Customer'],
		createdAt: '',
		updatedAt: ''
	};

	type iResponse = {
		status: string;
		data: iRelation;
	};

	type iResult = {
		status: string;
		count: number;
		totalPages: number;
		totalItems: number;
		data: iRelation[];
	};

	const client = useQueryClient();
	const title = 'Relasi';
	const url = `${baseURL}/relations`;
	const pages = [3, 5, 10, 25, 50];
	const qKey = 'relations';
	const initResult: iResult = {
		status: 'page loading',
		count: 0,
		totalPages: 0,
		totalItems: 0,
		data: []
	};

	let pageSize = $state(10);
	let page = $state(1);
	let pageNext = $derived(page + 1);
	let opt = $state(0);
	let relationId: string | undefined = $state(undefined);
	let txt: string | undefined = $state(undefined);
	let isUpdating = $state(false);
	// let open = $state(false);
	// let isError = $state(false);
	let errorMessage = $state('');
	//    let allowFetch = false;
	let isEdit = $state(false);
	let data = $state<iRelation>({ ...initData });
	let q_key = $derived.by(() => {
		return [qKey, page, pageSize, relationId, txt];
	});
	let q_next = $derived.by(() => {
		return [qKey, pageNext, pageSize, relationId, txt];
	});
	let isFetching = $derived(useIsFetching(q_key));

	async function getRelationTypes() {
		const options = {
			method: 'GET',
			credentials: credential_include
		};

		const request = new Request(`${url}/list/types`, options);
		let result = await fetch(request);
		return (await result.json()) as RelationTypeWIthID[];
	}

	async function fetchData(p: number): Promise<iResult> {
		if (browser) {
			const options = {
				headers: {
					'content-type': 'application/json'
				},
				method: 'GET',
				credentials: credential_include
			};

			const request = new Request(
				`${url}?page=${p}&limit=${pageSize}&opt=${opt}${opt === 1 ? `&txt=${txt}` : ''}${opt === 2 ? `&reltype=${relationId}` : ''}`,
				options
			);
			let result = await fetch(request);
			return (await result.json()) as iResult;
		}
		return Promise.resolve(initResult);
	}

	const prefetchNextPage = (data: iResult) => {
		//console.log({"page": page, "page size": data.totalPages});
		if (page < data.totalPages) {
			client.prefetchQuery(q_next, () => fetchData(page + 1));
		}
	};

	const fetchCreateData = async (e: iRelation): Promise<iResponse> => {
		//const url = `${baseURL}/relations`;
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

	const fetchUpdateData = async (e: iRelation): Promise<iResponse> => {
		//const url = `${baseURL}/relations/${e.id}`;
		const request = new Request(`${url}/${e.id}`, {
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

	const fetchDeleteData = async (e: number) => {
		//const url = `${baseURL}/relations/${e}`;
		const request = new Request(`${url}/${e}`, {
			method: 'DELETE',
			credentials: credential_include
		});

		return await (await fetch(request)).json();
	};

	const createData = useMutation(fetchCreateData, {
		onMutate: async (_e: iRelation) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousData = client.getQueryData<iResult>(q_key);

			// Optimistically update to the new value
			if (previousData) {
				client.setQueryData<iResult>(q_key, previousData);
			}

			return previousData;
		},
		onSuccess: async (data: any, _variable: iRelation, _context) => {
			if (data.status === 'success') {
				// setTimeout(() => {
				isUpdating = false;
				// if (data.status !== "fail") {
				// open = false;
				isEdit = false;
			} else {
				// isError = true;
				errorMessage = data.message;
			}
			//}, 250);
			// }
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (_err: any, _variables: any, context: any) => {
			if (context?.previousData) {
				client.setQueryData<iResult>(q_key, context.previousData);
			}
		},
		// Always refetch after error or success:
		onSettled: async () => {
			await client.invalidateQueries(q_key);
			await client.invalidateQueries({
				queryKey: ['relation'],
				refetchInactive: true
			});
		}
	});

	const updateData = useMutation(fetchUpdateData, {
		onMutate: async (_e: iRelation) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousRelation = client.getQueryData<iResult>(q_key);

			// Optimistically update to the new value
			if (previousRelation) {
				client.setQueryData<iResult>(q_key, previousRelation);
			}

			return previousRelation;
		},
		onSuccess: async (d: any, _variable: iRelation, context) => {
			if (context) {
				//		setTimeout(() => {
				isUpdating = false;
				if (d.status !== 'fail') {
					// open = false;
					isEdit = false;
				} else {
					// isError = true;
					errorMessage = d.message;
				}
				//}, 1500);
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (_err: any, _variables: any, context: any) => {
			if (context?.previousRelation) {
				client.setQueryData<iResult>(q_key, context.previousRelation);
			}
		},
		onSettled: async () => {
			await client.invalidateQueries(q_key);
			await client.invalidateQueries({
				queryKey: ['relation'],
				refetchInactive: true
			});
		}
	});

	const deleteData = useMutation(fetchDeleteData, {
		onMutate: async (_e: number) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();
			// Snapshot the previous value
			const previousData = client.getQueryData<iResult>(q_key);

			if (previousData?.data) {
				client.setQueryData<iResult>(q_key, previousData);
			}
			return previousData;
		},

		// onSuccess: async () => {
		// },
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (_err: any, _variables: any, context: any) => {
			if (context?.previousData) {
				client.setQueryData<iResult[]>(q_key, context.previousData);
			}
		},

		onSettled: async (
			raw: any
			//			error: any,
			//			variables: string,
			//			context: iResult | undefined,
		) => {
			if (raw.status === 'fail') {
				// isError = true;
				errorMessage = raw.message;
				// timeout = 3_000;
			}
			setTimeout(async () => {
				isUpdating = false;
				await client.invalidateQueries(q_key);
				await client.invalidateQueries({
					queryKey: ['relation'],
					refetchInactive: true
				});
			}, 500);
		}
	});

	function editRelation(e: number): void {
		if (e === 0) {
			data = { ...initData };
			isEdit = true;
		} else {
			let test = $query.data?.data.filter((f) => f.id === e)[0];
			if (test) {
				data = { ...test };
				isEdit = true;
			}
		}
	}

	function submit(e: iRelation) {
		//  console.log(e);
		isUpdating = true;
		if (e.id === 0) {
			$createData.mutate(e);
		} else {
			$updateData.mutate(e);
		}
	}

	async function delete_data(e: number) {
		$deleteData.mutate(e);
	}

	let query = $derived.by(() => {
		// untrack(() => page);
		// untrack(() => q_key.includes(page));

		return useQuery<iResult, Error>({
			queryKey: q_key,
			keepPreviousData: true,
			enabled: browser,
			queryFn: async () => await fetchData(page),
			onSuccess: prefetchNextPage
		});
	});

	// function setQueryOption(
	//   // p: number,
	//   // l: number,
	//   c: string | undefined,
	//   t: string | undefined,
	// ) {
	//     query.updateOptions( {
	//         queryKey: [qKey, page, pageSize, c, t],
	//     // keepPreviousData: true,
	//     // queryFn: async () => await fetchData(page),
	//     // onSuccess: prefetchNextPage,
	//     // enabled: true,
	//   });
	//   // {
	//   // 	queryKey: [qKey, p, l],
	//   // 	keepPreviousData: true,
	//   // 	queryFn: async () => await fetchData(p),
	//   // 	onSuccess: prefetchNextPage,
	//   // });
	// }

	function change_type(e: string | undefined): void {
		if (e) {
			opt = 2;
		} else {
			opt = 0;
		}
		relationId = e;
	}

	function change_search(e: string | undefined): void {
		if (e) {
			opt = 1;
		} else {
			opt = 0;
		}
		txt = e;
	}

	const queryTypes = useQuery('relTypes', getRelationTypes, {
		enabled: browser
	});

	// // $: query.setEnabled(true);
	$effect.pre(() => {
		if (browser) {
			queryTypes.setEnabled(browser);
			//         query.setEnabled(browser);
			//         console.log("TEST");
		}
	});

	// $effect.pre(() => {
	//     setQueryOption(current_type, txt);
	// });

	let queryData = $derived.by(() => {
		if ($query.isSuccess && $query.data) {
			return $query.data.data;
		}
		return [];
	});

	let totalItems = $derived.by(() => {
		if ($query.isSuccess && $query.data) {
			return $query.data.totalItems;
		}
		return 0;
	});

	// $inspect($query);
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" content="Relation this app" />
</svelte:head>

{#snippet paginating(isLoading: boolean)}
	<Pagination
		totalItems={totalItems}
		pageSizes={pages}
		page={!isLoading ? page : 0}
		pageSize={pageSize}
		on:update={(e) => {
			e.preventDefault();
			if (!isLoading) {
				page = e.detail.page;
				pageSize = e.detail.pageSize;
			}
		}}
		on:click:button--next={(e) => {
			page = e.detail.page;
		}}
		on:click:button--previous={(e) => {
			page = e.detail.page;
		}}
	/>
{/snippet}

<h2><GroupAccount size={24} /> {title}</h2>

{#if isEdit}
	<FormRelation
		initdata={{ ...data }}
		errorMessage={errorMessage}
		bind:open={isEdit}
		saveRelation={submit}
		relationTypes={$queryTypes.data}
	/>
{/if}

{#if $query.isLoading}
	<span>Loading...</span>
{:else if $query.isError}
	<span>Error: {$query.error.message}</span>
	<!-- {:else if $query.isSuccess} -->
{/if}

<RelationList
	selectedId={relationId}
	searchText={txt}
	data={queryData}
	edit={editRelation}
	onDeleteData={delete_data}
	bind:isUpdating={isUpdating}
	relationTypes={$queryTypes.data}
	onChangeType={change_type}
	onChangeSearch={change_search}
/>

{@render paginating($isFetching !== 0)}
