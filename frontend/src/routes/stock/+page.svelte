<style lang="scss">
	.col-parent {
		display: flex;
		flex-direction: column;
	}
	.col-child {
		display: inline-block;
		align-self: flex-end;
	}
	.num-right {
		text-align: right;
	}
</style>

<script lang="ts">
	import { browser } from '$app/environment';
	import { formatNumber } from '$lib/components/NumberFormat';
	import { getBarcodes, getRelationProp } from '$lib/fetchers';
	import type { iCurrentUser, iProduct, iProductStock, iStock, iStockDetail } from '$lib/interfaces';
	import { SendToBack } from 'carbon-icons-svelte';
	import { numberToText } from '$lib/number-to-string';
  import ProductSearch from '$lib/components/ProductSearch.svelte';
	import {
		useQuery,
		useQueryClient,
		useIsFetching
	} from '@sveltestack/svelte-query';
	import {
		Column,
		Grid,
		Loading,
		LocalStorage,
		Pagination,
		Row,
		ToastNotification
	} from 'carbon-components-svelte';
	import dayjs from 'dayjs';
	import { tick } from 'svelte';
	import FormStock from './FormStock.svelte';
	import FormStockPayment from './FormStockPayment.svelte';
	import ProductNotFound from './ProductNotFound.svelte';
	import StockDetail from './StockDetail.svelte';
	import StockList from './StockList.svelte';
	import {
		type iGudangResult,
		fetchGudangs,
		getStockById,
		getStocks,
		postCreateStock,
		postDeleteStock,
		postUpdateOnlyStock,
		postUpdateStock,
		toNumber
	} from './handler';
	import {
		details,
		initStock,
		isStockLoading,
		isStockUpdating,
		stock
	} from './store';

	const title = 'Stock';
	const qKey = 'stocks';
	const client = useQueryClient();
	const pages = [3, 5, 10, 25, 50];
	let txt = $state('');
	let page = $state(1);
	let pageSize = $state(5);
	let supplierId = $state(0);
	let warehouseId = $state(0);
	let opt = $state(0);
	let stockId = $state(0);
	let open = $state(false);
	let innerWidth = $state(0);
	let timeout: number | undefined = $state(undefined);
	let showNotification = $state(false);
	let isEdit = $state(false);
  let strNotFound = $state('');
  let isSearch = $state(false);
	let profile: iCurrentUser = $state({
		id: '',
		name: '',
		email: '',
		photo: '',
		role: '',
		verified: false,
		updatedAt: '',
		createdAt: ''
	});

	let pageNext = $derived(page + 1);
	let query_key = $derived([
		qKey,
		page,
		pageSize,
		supplierId,
		warehouseId,
		txt
	]);
	const query_next = $derived([
		qKey,
		pageNext,
		pageSize,
		supplierId,
		warehouseId,
		txt
	]);
	let isFetching = $derived(useIsFetching(query_key));

	function onProductNotFound(e: string) {
		// showNotification = false;
		
  //   setTimeout(() => {
		// 	showNotification = true;
		// }, 250);
		// txt = e;
		// timeout = 6_000;

    isSearch = true;
    strNotFound = e;
	}

  function createNewId(): number {
		console.log($details.length);
		if ($details.length >= 1) {
			let test = $details.reduce((prev, cur) =>
				prev.id > cur.id ? prev : cur
			).id;
			return test + 1;
		}
		return 1;
	}

	function selectProduct(p: iProduct, q: number) {
		const i = $details.findIndex((f) => f.productId === p.id);
		const n = p.stocks.findIndex((f) => f.gudangId === 1);
		const stc: iProductStock = p.stocks[n];

		if (i >= 0) {
			const slices = [...$details];
			const d = slices[i];
			d.qty = toNumber(d.qty) + q;
			d.subtotal = (toNumber(d.price) - toNumber(d.discount)) * d.qty;
			slices.splice(i, 1, d);
			details.update(() => [...slices]);
		} else {
			const d: iStockDetail = {
				stockId: $stock.id,
				id: createNewId(),
				price: toNumber(p.hpp),
				qty: q,
				unit: p.unit,
				productId: p.id,
				direction: -1,
				discount: 0,
				subtotal: toNumber(p.hpp) * q,
				name: p.name,
				barcode: p.barcode,
				hpp: toNumber(p.hpp),
				oldQty: 0, // stock.qty,
				oldGudangId: stc.gudangId,
				gudangName: stc.name,
				gudangId: stc.gudangId
			};
			details.update((o) => [...o, d]);
		}

		const total = $details.reduce((o, t) => o + toNumber(t.subtotal), 0);
		stock.update(
			(s) =>
				(s = {
					...s,
					total: total,
					remain: total - (toNumber(s.dp) + toNumber(s.payment)),
					isModified: true,
					isDetailChanged: true
				})
		);
	}

	const supplierQuery = useQuery(
		['relation', 'supplier'],
		async () => await getRelationProp(['Supplier']),
		{
			enabled: browser
		}
	);

	const employeeQuery = useQuery(
		['relation', 'employee'],
		async () => await getRelationProp(['Employee']),
		{
			enabled: browser
		}
	);

	const queryGudangOptions = () => ({
		queryKey: ['gudang', 'list'],
		queryFn: async () => await fetchGudangs(),
		enabled: browser
	});

	const queryBarcodeOptions = () => ({
		queryKey: ['barcodes'],
		queryFn: async () => await getBarcodes(),
		enabled: browser
	});

	const gudangQuery = useQuery<iGudangResult, Error>(queryGudangOptions());
	const queryBarcode = useQuery(queryBarcodeOptions());

	const prefetchNextPage = (data: {
		status: string;
		stocks: iStock[];
		totalItems: number;
		totalPages: number;
		count: number;
		currentPage: number;
	}) => {
		//	console.log(data.currentPage, data.totalPages);

		if (data.currentPage < data.totalPages) {
			client.prefetchQuery(query_next, () => loadstock(data.currentPage + 1));
		}
	};

	const queryStock = $derived.by(() => {
		return useQuery({
			queryKey: [qKey, { id: stockId }],
			queryFn: async () => getStockById(stockId),
			enabled: false
		});
	});

	const loadstock = async (p: number) => {
		return await getStocks(opt, p, pageSize, supplierId, warehouseId, txt);
	};

	const queryStocks = $derived.by(() => {
		return useQuery({
			queryKey: query_key,
			queryFn: async () => loadstock(page),
			enabled: browser,
			onSuccess: prefetchNextPage,
			keepPreviousData: true
		});
	});

	// function setQueryOption(
	// 	p: number,
	// 	l: number,
	// 	o: number,
	// 	s: number,
	// 	w: number,
	// 	t: string
	// ) {
	// 	queryStocks.setOptions({
	// 		queryKey: [qKey, o, p, l, s, w, t],
	// 		keepPreviousData: true,
	// 		enabled: browser,
	// 		queryFn: async () =>
	// 			await getStocks(opt, page, pageSize, supplierId, warehouseId, txt),
	// 		onSuccess: prefetchNextPage
	// 	});
	// 	// console.log([qKey, p, l, o, t, r]);
	// 	// console.log(q_key);
	// }

	function stockClose(_e: any): void {
		changeStockSession(0, false);
	}

	function createNewStock(_e: number): void {
		isEdit = false;
		changeStockSession(0, true);
	}

	function editStock(e: number) {
		changeStockSession(e, true);
	}

	async function changeStockSession(id: number, mode: boolean) {
		stockId = id;
		await tick();
		queryStock.setOptions({
			queryKey: [qKey, { id: stockId }],
			queryFn: async () =>
				getStockById(stockId, { stock: { ...initStock }, details: [] }),
			enabled: true
		});

		// setTimeout(() => {
		isStockLoading.update(() => false);
		// }, 250);
		isEdit = mode;
	}
	async function updateOnlyStock() {
		const x = dayjs($stock.createdAt);
		const y = dayjs($stock.dueAt);
		const duration = y.diff(x, 'days');

		const savedStock: iStock = {
			...$stock,
			updatedBy: profile.name,
			dueRange: duration
		};
		delete savedStock.isDetailChanged;
		delete savedStock.isModified;
		delete savedStock.isPayed;

		// console.log(savedStock);

		const result = await postUpdateOnlyStock(stockId, savedStock);
		if (result) {
			await client.invalidateQueries([qKey, { id: stockId }]);
			await client.invalidateQueries(query_key);
			isStockUpdating.update(() => false);
			isEdit = false;
		}

		changeStockSession(0, false);
	}

	async function saveStock(e: iStockDetail[]) {
		if ($stock.isDetailChanged) {
			// console.log("UDPATE-WITH-DETAILS");
			if (e.length > 0) {
				const x = dayjs($stock.createdAt);
				const y = dayjs($stock.dueAt);
				const duration = y.diff(x, 'days');

				const savedStock: iStock = {
					...$stock,
					paymentType: 'Cash',
					updatedBy: profile.name,
					dueRange: duration
				};
				delete savedStock.isDetailChanged;
				delete savedStock.isModified;
				delete savedStock.isPayed;

				// console.log(savedStock);

				if ($stock.id === 0) {
					const result = await postCreateStock(savedStock, e);
					if (result) {
						await client.invalidateQueries([qKey, { id: stockId }]);
						await client.invalidateQueries(query_key);
						// setTimeout(() => {
						isStockUpdating.update(() => false);
						isEdit = false;
						// }, 250);
					}
				} else {
					//					console.log(e.detail);
					const result = await postUpdateStock(stockId, savedStock, e);
					if (result) {
						await client.invalidateQueries([qKey, { id: stockId }]);
						await client.invalidateQueries(query_key);
						// setTimeout(() => {
						isStockUpdating.update(() => false);
						isEdit = false;
						// }, 250);
					}
				}
				changeStockSession(0, false);
			}
		} else {
			// console.log("UDPATE-ONLY-STOCK");
			updateOnlyStock();
		}
	}

	async function deleteStocks(e: number[]) {
		let log = await postDeleteStock(e);
		if (log && log.data > 0) {
			client.invalidateQueries(query_key);
		}
		setTimeout(() => {
			isStockUpdating.set(false);
		}, 250);
		// console.log(log);
	}

	const subsribe = () => {
		queryStock.subscribe((o) => {
			stock.set(o.data?.stock ?? { ...initStock });
			details.set(o.data?.details ?? []);
		});
	};

	let isStockAvailabel = $derived.by(() => {
		return $queryStock.isSuccess && $queryStock.data;
	});

	let suppliers = $derived.by(() => {
		if ($supplierQuery.isSuccess && $supplierQuery.data) {
			return $supplierQuery.data.data;
		}
		return [];
	});

	let employees = $derived.by(() => {
		if ($employeeQuery.isSuccess && $employeeQuery.data) {
			return $employeeQuery.data.data;
		}
		return [];
	});

	let stocks = $derived.by(() => {
		if ($queryStocks.isSuccess && $queryStocks.data) {
			return $queryStocks.data.stocks;
		}
		return [];
	});

	let gudangs = $derived.by(() => {
		if ($gudangQuery.isSuccess && $gudangQuery.data) {
			return $gudangQuery.data.data;
		}
		return [];
	});

	let barcodes = $derived.by(() => {
		if ($queryBarcode.isSuccess && $queryBarcode.data.data) {
			return $queryBarcode.data.data;
		}
		return [];
	});

	let totalItems = $derived.by(() => {
		if ($queryStocks.isSuccess && $queryStocks.data) {
			return $queryStocks.data.totalItems;
		}
		return 0;
	});

	$effect.pre(() => {
		if (isStockAvailabel) {
			subsribe();
		}
	});

	// $effect.root(()=>  {
	// 	supplierQuery.setEnabled(browser);
	// 	employeeQuery.setEnabled(browser);
	// 	gudangQuery.setEnabled(browser);
	// 	queryStocks.setEnabled(browser);
	// 	queryStock.setEnabled(browser);
	// });

	$effect(() => {
		showNotification = timeout !== undefined;
	});
	// $: setQueryOption(page, pageSize, opt, supplierId, warehouseId, txt);
</script>

<svelte:window bind:innerWidth={innerWidth} />

<svelte:head>
	<title>{title}</title>
	<meta name="description" content="Stock this app" />
</svelte:head>

{#snippet paginating(isLoading: boolean)}
	<Pagination
		totalItems={totalItems}
		pageSizes={pages}
		pageSize={pageSize}
		style="margin-top: 1px;"
		page={!isLoading ? page : 0}
		on:update={(e) => {
			e.preventDefault();
			if (!isLoading) {
				pageSize = e.detail.pageSize;
				page = e.detail.page;
			}
		}}
		on:click:button--next={(e) => (page = e.detail.page)}
		on:click:button--previous={(e) => (page = e.detail.page)}
	/>
{/snippet}

{#snippet formStock()}
	<Grid noGutter={innerWidth > 720}>
		<Row>
			<Column noGutterRight>
				<h4>Nota #{$stock.id} / {$stock.invoiceId}</h4>
				<Grid noGutter>
					<Row>
						<Column>Bayar:</Column>
						<Column>{formatNumber($stock.dp)}</Column>
					</Row>
					<Row>
						<Column>Angsuran:</Column>
						<Column>{formatNumber($stock.payment)}</Column>
					</Row>
					<Row>
						<Column>Sisa bayar:</Column>
						<Column>{formatNumber($stock.remain)}</Column>
					</Row>
				</Grid>
			</Column>
			<Column noGutterLeft>
				<div class="col-parent">
					<div class="col-child">
						<h1 class="num-right">
							<strong>{formatNumber($stock.total)}</strong>
						</h1>
						<div class="text-number">
							{numberToText($stock.total.toString(10))}
						</div>
					</div>
				</div>
			</Column>
		</Row>
	</Grid>

	<FormStock
		suppliers={suppliers}
		employees={employees}
		bind:innerWidth={innerWidth}
	/>
	<StockDetail
		barcodes={barcodes}
		gudangs={gudangs}
		onnotfound={onProductNotFound}
		onnew={createNewStock}
		onclose={stockClose}
		onsave={saveStock}
		onadddp={() => (open = true)}
	/>
{/snippet}

{#snippet stockList()}
	<StockList
		data={stocks}
		suppliers={suppliers}
		employees={employees}
		txt={txt}
		selectedSupplierId={supplierId}
		selectedWarehouseId={warehouseId}
		onsupplierchange={(e) => {
			opt = e === 0 ? 0 : 2;
			supplierId = e;
		}}
		onwarehousechange={(e) => {
			opt = e === 0 ? 0 : 3;
			warehouseId = e;
		}}
		onsearch={(e) => {
			opt = e === null || e === '' ? 0 : 1;
			txt = e ?? '';
		}}
		onedit={editStock}
		ondelete={deleteStocks}
	/>
{/snippet}

{#snippet toas()}
	<ToastNotification
		style={'margin-top: 24px; width: 100%'}
		on:click={() => (timeout = 12_000)}
		fullWidth
		timeout={timeout}
		kind="warning-alt"
		on:close={(_e) => {
			timeout = undefined;
		}}
	>
		<strong slot="subtitle">Produk yang anda cari tidak ditemukan</strong>
		<div>
			<ProductNotFound productName={txt} />
		</div>
	</ToastNotification>
{/snippet}

<LocalStorage key="__user_info" bind:value={profile} />
<Grid noGutter>
	<Row>
		<Column>
			<h2><SendToBack size={24} /> {title}</h2>
		</Column>
		<Column
			>{#if $supplierQuery.isLoading || $employeeQuery.isLoading || $queryBarcode.isLoading || $queryStocks.isLoading || $gudangQuery.isLoading}
				<Loading withOverlay={false} small={true} />
			{/if}</Column
		>
	</Row>
</Grid>

{#if open}
	<FormStockPayment
		bind:open={open}
		total={$stock.total}
		dp={$stock.dp}
		payment={$stock.payment}
		onupdate={(e) => {
			stock.update((o) => ({
				...o,
				dp: e,
				remain: toNumber(o.total) - (e + toNumber(o.payment))
			}));
		}}
	/>
{/if}

{#if isEdit}
	{@render formStock()}
{:else}
	{@render stockList()}
	{@render paginating($isFetching !== 0)}
{/if}

{#if showNotification}
	{@render toas()}
{/if}
{#if isSearch && isEdit}
	<ProductSearch
    showHpp
		value={strNotFound}
		onselect={(p, q) => selectProduct(p, q)}
		onclear={() => {
			isSearch = false;
		}}
	/>
{/if}
