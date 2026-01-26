<style lang="css">
	:global(.bx--label) {
		margin-bottom: 3px;
		margin-top: 9px;
	}

	:global(.bx--list-box__menu-item, .bx--list-box__menu-item__option) {
		height: auto;
	}
</style>

<script lang="ts">
	import { browser } from '$app/environment';
	import FormProduct from '$lib/components/FormProduct.svelte';
	import { baseURL, credential_include, type iProduct } from '$lib/interfaces';
	import { Product } from 'carbon-icons-svelte';
	import {
		getCategoryProp,
		getRelationProp,
		type iPropertyWithID
	} from '$lib/fetchers';
	import {
		useIsFetching,
		useMutation,
		useQuery,
		useQueryClient
	} from '@sveltestack/svelte-query';
	import {
		Column,
		Grid,
		Loading,
		Pagination,
		Row
	} from 'carbon-components-svelte';
	import dayjs from 'dayjs';
	import ProductList from './ProductList.svelte';
	import DeleteProduct from '$lib/components/DeleteButton.svelte';
	import { tick } from 'svelte';

	type ResultBase = {
		status: string;
		count: number;
		totalPages: number;
		totalItems: number;
	};

	type iResult = ResultBase & {
		data: iProduct[];
	};

	type iResponse = {
		status: string;
		data: iProduct;
	};

	const title = 'Data Barang';
	const client = useQueryClient();
	const url = `${baseURL}/products`;
	const qKey = 'products';

	const initResult: iResult = {
		status: 'page loading',
		count: 0,
		totalPages: 0,
		totalItems: 0,
		data: []
	};

	const initData: iProduct = {
		id: 0,
		supplierId: 0,
		name: '',
		barcode: '',
		unit: '',
		content: 0.0,
		hpp: 0.0,
		margin: 11,
		price: 0.0,
		ppn: 0.0,
		heavy: 0.0,
		isActive: true,
		variantName: '',
		descriptions: '',
		categoryId: 0,
		createdAt: dayjs().toISOString(),
		updatedAt: dayjs().toISOString(),
		stocks: []
	};

	const pages = [3, 5, 10, 25, 50];

	let pageSize = $state(5);
	let page = $state(1);
	let isUpdating = $state(false);
	let isLoading = $state(true);
	let open = $state(false);
	let isError = $state(false);
	let errorMessage = $state('');
	let innerWidth = $state(720);
	let txt: string | undefined = $state(undefined);
	let rel_id: number = $state(0);
	let opt = $state(0);
	let cat_id = $state(0);

	let q_key = $derived.by(() => {
		return [qKey, page, pageSize, txt, rel_id, cat_id];
	});

	let pageNext = $derived(page + 1);
	let q_next = $derived.by(() => {
		return [qKey, pageNext, pageSize, txt, rel_id, cat_id];
	});

	let isFetching = $derived(useIsFetching(q_key));

	const search = (e: string | undefined) => {
		opt = e ? 1 : 0;

		txt = e;
	};

	async function fetchData(p: number): Promise<iResult> {
		//console.log(q_key);
		if (browser) {
			const options = {
				headers: {
					'content-type': 'application/json'
				},
				method: 'GET',
				credentials: credential_include
			};

			const request = new Request(
				`${url}?opt=${opt}&page=${p}&limit=${pageSize}${opt === 1 ? `&txt=${txt}` : ''}${opt === 2 ? `&relid=${rel_id}` : ''}${opt === 3 ? `&catid=${cat_id}` : ''}`,
				options
			);

			let result = await fetch(request);
			return (await result.json()) as iResult;
		}

		return Promise.resolve(initResult);
	}

	const prefetchNextPage = (data: iResult) => {
		if (page < data.totalPages) {
			client.prefetchQuery(q_next, () => fetchData(pageNext));
		}
	};

	let data: iProduct = $state({ ...initData });

	const query = $derived.by(() => {
		return useQuery<iResult, Error>({
			queryKey: q_key,
			queryFn: async () => await fetchData(page),
			onSuccess: prefetchNextPage,
			keepPreviousData: true,
			enabled: browser
		});
	});

	// function setQueryOption(
	// 	p: number,
	// 	l: number,
	// 	o: number,
	// 	t: string | undefined,
	// 	r: number,
	// 	c: number
	// ) {
	// 	query.setOptions({
	// 		queryKey: [qKey, p, l, o, t, r, c],
	// 		keepPreviousData: true,
	// 		queryFn: async () => await fetchData(page),
	// 		onSuccess: prefetchNextPage
	// 	});
	// 	// console.log([qKey, p, l, o, t, r]);
	// 	// console.log(q_key);
	// }

  const fetchCreateData = async (e: iProduct): Promise<iResponse> => {
    const variable = JSON.stringify(e);
    console.log(variable);
    const request = {
      method: 'POST',
      mode: 'cors',
      headers: {
	'Content-Type': 'application/json',
	'Accept': 'application/json',
      },
      body: variable,
      credentials: credential_include
    };

    console.log(request.body);

    const data = await fetch(url, request);

    return data.json();
  };

	const fetchUpdateData = async (e: iProduct): Promise<iResponse> => {
		const request = new Request(`${url}/${e.id}`, {
			headers: {
				'content-type': 'application/json'
			},
			body: JSON.stringify(e),
			method: 'PUT',
			credentials: credential_include
		});

		const result = await fetch(request);
		// console.log(result);
		return await result.json();
	};

	const fetchDeleteData = async (e: number) => {
		const request = new Request(`${url}/${e}`, {
			method: 'DELETE',
			credentials: credential_include
		});

		return await (await fetch(request)).json();
	};

  const createData = useMutation(
    fetchCreateData,
    {
		onMutate: async (_e: iProduct) => {
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
		onSuccess: async (data: any, _variable: iProduct, _context) => {
		  // setTimeout(() => {
		  console.log(data);
			  setTimeout(() => {
			    errorMessage = data.message;
			    open = false;
			    isUpdating = false;
			    isError = false;
			  }, 250);
			//}, 250);
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, _variables, context: any) => {
			// console.log(err);
		  console.log(err);
			isUpdating = false;
			if (context?.previousData) {
				client.setQueryData<iResult>(q_key, context.previousData);
			}
		},
		// Always refetch after error or success:
		onSettled: async () => {
			await client.invalidateQueries([qKey]);
		}
	});

	const updateData = useMutation(fetchUpdateData, {
		onMutate: async (_e: iProduct) => {
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
		onSuccess: async (data: any, _variable: iProduct, _context) => {
			// console.log(data);
			if (data.status === 'error') {
				isError = true;
				setTimeout(() => {
					isUpdating = false;
				}, 250);
				errorMessage = data.message;
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (_err: any, _variables: any, context: any) => {
			isUpdating = false;
			if (context?.previousData) {
				client.setQueryData<iResult>(q_key, context.previousData);
			}
		},
		onSettled: async (data: any) => {
			if (data.status === 'success') {
				// console.log("SUCCESS")
				setTimeout(() => {
					isUpdating = false;
					open = false;
					errorMessage = '';
				}, 500);
			}
			await client.invalidateQueries([qKey]);
		}
	});

	const deleteData = useMutation(fetchDeleteData, {
		onMutate: async (_e: number) => {
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
		onSuccess: async () => {
			//setTimeout(() => {
			isUpdating = false;
			//}, 1500);
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (_err: any, _variables: any, context: any) => {
			if (context?.previousData) {
				client.setQueryData<iResult>(q_key, context.previousData);
			}
		},
		onSettled: async (
			raw: any
			//			error: any,
			//			variables: string,
			//			context: iResult | undefined,
		) => {
			if (raw.status === 'fail') {
				isError = true;
				errorMessage = raw.message;
				// timeout = 3_000;
			}
			await client.invalidateQueries([qKey]);
		}
	});

	// let kunci = $state({ count: 0 });
	// setContext("kunci", kunci);

	async function editProduct(e: number) {
		const id = e;

		if (id > 0) {
			const test = $query.data?.data.filter((f) => f.id === id)[0];
			if (test) {
				data = { ...test };
				await tick().then(() => {
					open = true;
				});
			}
		} else {
			data = { ...initData };
			await tick().then(() => {
				open = true;
			});
		}
		// kunci.count =  kunci.count + 500;
	}

	function deleteProduct(e: number): void {
		$deleteData.mutate(e);
	}

	function submit(e: iProduct) {
	  const id = e.id;
		if (id === 0) {
			$createData.mutate(e);
		} else {
			$updateData.mutate(e);
		}
		//open = false;
	}

	function supplierChange(e: number): void {
		opt = e === 0 ? 0 : 2;
		rel_id = e;
	}

	function categoryChange(e: number): void {
		opt = e === 0 ? 0 : 3;
		cat_id = e;
	}

	const categoryQuery = useQuery<iPropertyWithID, Error>(
		'catProp',
		getCategoryProp,
		{
			enabled: browser
		}
	);

	const supplierQuery = useQuery(
		'supProp',
		async () => await getRelationProp(['Supplier']),
		{
			enabled: browser
		}
	);

	let categories = $derived.by(() => {
		return $categoryQuery.isSuccess && $categoryQuery.data
			? $categoryQuery.data.data
			: [];
	});

	let suppliers = $derived.by(() => {
		return $supplierQuery.isSuccess && $supplierQuery.data
			? $supplierQuery.data.data
			: [];
	});

	let products = $derived.by(() => {
		return $query.isSuccess && $query.data ? $query.data.data : [];
	});

	let totalItems = $derived.by(() => {
		return $query.isSuccess && $query.data ? $query.data.totalItems : 0;
	});

	$effect(() => {
		categoryQuery.setEnabled(browser);
		supplierQuery.setEnabled(browser);
	});

	$effect.pre(() => {
		isLoading = false;
	});

	function reset() {
		open = false;
		isUpdating = false;
		isError = false;
		errorMessage = '';
	}

	let isProductLoading = $derived.by(() => $query.isLoading);

	$inspect(isLoading);

	// $effect(() => {
	//     setQueryOption(page, pageSize, opt, txt, rel_id, cat_id);
	//  });
</script>

{#snippet deleteTool(id: number)}
	<DeleteProduct idData={id} onDeleteData={deleteProduct} />
{/snippet}

{#snippet paginating(isLoading: boolean)}
	<Pagination
		style={'margin-top:1px'}
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

<svelte:head>
	<title>{title}</title>
	<meta name="description" content="Product this app" />
</svelte:head>

<svelte:window bind:innerWidth={innerWidth} />

<Grid noGutter>
	<Row>
		<Column><h2><Product size={24} /> {title}</h2></Column>
		<Column>
			{#if $categoryQuery.isLoading || $supplierQuery.isLoading || isLoading || isProductLoading}
				<Loading withOverlay={false} small={true} />
			{/if}
		</Column>
	</Row>
</Grid>
<!-- <subtitle>Tabel data barang / produk</subtitle> -->

<!-- {#if open} -->
<FormProduct
	suppliers={suppliers}
	categories={categories}
	bind:data={data}
	bind:open={open}
	bind:isError={isError}
	bind:errorMessage={errorMessage}
	bind:isUpdating={isUpdating}
	onSubmit={submit}
	bind:innerWidth={innerWidth}
	onReset={() => reset()}
/>
<!-- {/if} -->

{#if $query.isError}
	<span>Error: {$query.error.message}</span>
{/if}

<ProductList
	products={products}
	suppliers={suppliers}
	categories={categories}
	bind:innerWidth={innerWidth}
	onSearch={search}
	onEdit={editProduct}
	onCategoryChange={categoryChange}
	onSupplierChange={supplierChange}
	isProductLoading={isProductLoading}
	deleteTool={deleteTool}
/>

{@render paginating($isFetching !== 0)}
