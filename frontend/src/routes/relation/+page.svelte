<script lang="ts">
	import { browser } from "$app/environment";
	import FormRelation from "$lib/components/FormRelation.svelte";
	import {
		baseURL,
		credential_include,
		type iRelation,
		type RelationTypeWIthID,
	} from "$lib/interfaces";
	import {
		useMutation,
		useQuery,
		useQueryClient,
	} from "@sveltestack/svelte-query";
	import { Pagination } from "carbon-components-svelte";
	import RelationList from "./RelationList.svelte";

	const url = `${baseURL}/relations`;
	let qKey = "relations";
	let pageSize = 5;
	let page = 1;
	let pages = [3, 5, 10, 25, 50];
	let opt = 0;
	let current_type: string | undefined = undefined;
	let txt: string | undefined = undefined;

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

	const initResult: iResult = {
		status: "page loading",
		count: 0,
		totalPages: 0,
		totalItems: 0,
		data: [],
	};

	const client = useQueryClient();
	let isUpdating = false;
	let open = false;
	let isError = false;
	let errorMessage = "";
	//    let allowFetch = false;

	async function getRelationTypes() {
		const options = {
				method: "GET",
				credentials: credential_include,
			};

			const request = new Request(
				`${url}/list/types`,
				options,
			);

			let result = await fetch(request);

			return (await result.json()) as RelationTypeWIthID[];

	}

	async function fetchData(p: number): Promise<iResult> {
		if (browser) {
			const options = {
				headers: {
					"content-type": "application/json",
				},
				method: "GET",
				credentials: credential_include,
			};

			const request = new Request(
				`${url}?page=${p}&limit=${pageSize}&opt=${opt}${opt===1?`&txt=${txt}`:""}${opt===2?`&reltype=${current_type}`:""}`,
				options,
			);

			let result = await fetch(request);

			return (await result.json()) as iResult;
		}

		return Promise.resolve(initResult);
	}

	const prefetchNextPage = (data: iResult) => {
		//console.log({"page": page, "page size": data.totalPages});
		if (page < data.totalPages) {
			client.prefetchQuery([qKey, page + 1, pageSize], () =>
				fetchData(page + 1),
			);
		}
	};

	let isEdit = false;
	let initData: iRelation = {
		id: 0,
		name: "",
		city: "",
		isActive: true,
		isSpecial: false,
		relationType: ["Customer"],
		createdAt: "",
		updatedAt: "",
	};

	const fetchCreateData = async (e: iRelation): Promise<iResponse> => {
		//const url = `${baseURL}/relations`;
		const request = new Request(url, {
			headers: {
				"content-type": "application/json",
			},
			body: JSON.stringify(e),
			method: "POST",
			credentials: credential_include,
		});

		return await (await fetch(request)).json();
	};

	const fetchUpdateData = async (e: iRelation): Promise<iResponse> => {
		//const url = `${baseURL}/relations/${e.id}`;
		const request = new Request(`${url}/${e.id}`, {
			headers: {
				"content-type": "application/json",
			},
			body: JSON.stringify(e),
			method: "PUT",
			credentials: credential_include,
		});

		const result = await fetch(request);
		return await result.json();
	};

	const fetchDeleteData = async (e: string) => {
		//const url = `${baseURL}/relations/${e}`;
		const request = new Request(`${url}/${e}`, {
			method: "DELETE",
			credentials: credential_include,
		});

		return await (await fetch(request)).json();
	};

	const createData = useMutation(fetchCreateData, {
		onMutate: async (e: iRelation) => {
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
		onSuccess: async (data: any, variable: iRelation, context) => {
			if (context) {
				// setTimeout(() => {
				isUpdating = false;
				if (data.status !== "fail") {
					open = false;
					isEdit = false;
				} else {
					isError = true;
					errorMessage = data.message;
				}
				//}, 250);
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, variables: any, context: any) => {
			console.log(err);
			if (context?.previousData) {
				client.setQueryData<iResult>(q_key, context.previousData);
			}
		},
		// Always refetch after error or success:
		onSettled: async () => {
			await client.invalidateQueries(q_key);
		},
	});

	const updateData = useMutation(fetchUpdateData, {
		onMutate: async (e: iRelation) => {
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
		onSuccess: async (data: any, variable: iRelation, context) => {
			if (context) {
				//		setTimeout(() => {
				isUpdating = false;
				if (data.status !== "fail") {
					open = false;
					isEdit = false;
				} else {
					isError = true;
					errorMessage = data.message;
				}
				//}, 1500);
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, variables: any, context: any) => {
			if (context?.previousRelation) {
				client.setQueryData<iResult>(q_key, context.previousRelation);
			}
		},
		onSettled: async () => {
			await client.invalidateQueries(q_key);
		},
	});

	const deleteData = useMutation(fetchDeleteData, {
		onMutate: async (e: string) => {
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
		onSuccess: async () => {
			//setTimeout(() => {
			isUpdating = false;
			//}, 1500);
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, variables: any, context: any) => {
			if (context?.previousRelation) {
				client.setQueryData<iResult>(q_key, context.previousRelation);
			}
		},
		onSettled: async (
			raw: any,
			//			error: any,
			//			variables: string,
			//			context: iResult | undefined,
		) => {
			if (raw.status === "fail") {
				isError = true;
				errorMessage = raw.message;
				// timeout = 3_000;
			}
			await client.invalidateQueries(q_key);
			isUpdating = false;
		},
	});

	let data: iRelation = { ...initData };

	function editRelation(e: CustomEvent<number>): void {
		if (e.detail === 0) {
			data = { ...initData };
			isEdit = true;
		} else {
			let test = $query.data?.data.filter((f) => f.id === e.detail)[0];
			if (test) {
				data = {...test};
				isEdit = true;
			}
		}
	}

	function submit(e: CustomEvent<iRelation>) {
		isUpdating = true;
		if (e.detail.id === 0) {
			$createData.mutate(e.detail);
		} else {
			$updateData.mutate(e.detail);
		}
	}

	function delete_data(e: CustomEvent<string>): void {
		$deleteData.mutate(e.detail);
	}

	const query = useQuery<iResult, Error>({
		queryKey: [qKey, page, pageSize],
		keepPreviousData: true,
		enabled: browser,
		queryFn: async () => await fetchData(page),
		onSuccess: prefetchNextPage,
	});

	function setQueryOption(p: number, l: number, c: string|undefined, t: string|undefined) {
		query.setOptions({
			queryKey: [qKey, p, l, c, t],
			keepPreviousData: true,
			queryFn: async () => await fetchData(page),
			onSuccess: prefetchNextPage,
			enabled: browser
		});
		// {
		// 	queryKey: [qKey, p, l],
		// 	keepPreviousData: true,
		// 	queryFn: async () => await fetchData(p),
		// 	onSuccess: prefetchNextPage,
		// });
	}

	function change_type(e: CustomEvent<string|undefined>): void {
		current_type = e.detail;
		if(e.detail) {
			opt = 2;
		} else {
			opt = 0;
		}
	}

	function change_search(e: CustomEvent<string|undefined>): void {
		txt = e.detail;
		if(e.detail) {
			opt = 1;
		} else {
			opt = 0;
		}
	}

	const queryTypes = useQuery("relTypes", getRelationTypes, {enabled: browser});
	$: q_key = [qKey, page, pageSize, current_type, txt];

	// $: query.setEnabled(true);
	$: {
		queryTypes.setEnabled(browser);
		query.setEnabled(browser);
	}
	$: setQueryOption(page, pageSize, current_type, txt);


</script>

<svelte:head>
	<title>Relasi</title>
	<meta name="description" content="Relation this app" />
</svelte:head>

{#if isEdit}
	<FormRelation {data} {errorMessage} bind:open={isEdit} on:submit={submit} relationTypes={$queryTypes.data} />
{/if}

{#if $query.isLoading}
	<span>Loading...</span>
{:else if $query.isError}
	<span>Error: {$query.error.message}</span>
{:else if $query.isSuccess}
	<RelationList
		data={$query.data.data}
		on:edit={editRelation}
		on:deleteData={delete_data}
		bind:isUpdating
		relationTypes={$queryTypes.data}
		on:changeType={change_type}
		on:change_search={change_search}
	/>
	<Pagination
		totalItems={$query.data.totalItems}
		pageSizes={pages}
		{pageSize}
		{page}
		on:update={(e) => {
			pageSize = e.detail.pageSize;
			page = e.detail.page;
		}}
		on:click:button--next={(e) => {
			page = e.detail.page;
		}}
		on:click:button--previous={(e) => {
			page = e.detail.page;
		}}
	/>
{/if}
