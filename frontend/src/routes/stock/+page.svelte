<script lang="ts">
	import type { iCurrentUser, iStock, iStockDetail } from "$lib/interfaces";
	import { useQuery, useQueryClient } from "@sveltestack/svelte-query";
	import FormStock from "./FormStock.svelte";
	import dayjs from "dayjs";
	import { getBarcodes, getSupplierProp } from "$lib/fetchers";
	import { browser } from "$app/environment";
	import {
		Column,
		Grid,
		Loading,
		LocalStorage,
		Pagination,
		Row,
		ToastNotification,
	} from "carbon-components-svelte";
	import { onMount, tick } from "svelte";
	import StockDetail from "./StockDetail.svelte";
	import ProductNotFound from "./ProductNotFound.svelte";
	import { formatNumber } from "$lib/components/NumberFormat";
	import { numberToText } from "$lib/number-to-string";
	import StockInfo from "./StockInfo.svelte";
	import StockList from "./StockList.svelte";
	import {
		getStockById,
		getStocks,
		postCreateStock,
		postDeleteStock,
		postUpdateStock,
		toNumber,
	} from "./handler";
	import type { Unsubscriber } from "svelte/store";

	const client = useQueryClient();

	let txt = "";
	let page = 1;
	let limit = 5;
	let supplierId = 0;
	let warehouseId = 0;
	let opt = 0;
	let stockId = 0;
	const qKey = "stocks";

	let showInfo = false;
	let isEdit = false;
	let dueRange = 7;
	let tgl = dayjs();
	let next_tgl = dayjs().add(dueRange, "day");
	let profile: iCurrentUser = {
		id: "",
		name: "",
		email: "",
		photo: "",
		role: "",
		verified: false,
		updatedAt: "",
		createdAt: "",
	};

	const initData: iStock = {
		id: 0,
		supplierId: 0,
		warehouseId: 0,
		paymentType: "Cash",
		updatedBy: profile.name,
		total: 0,
		dp: 0,
		payment: 0,
		remain: 0,
		invoiceId: "",
		dueAt: next_tgl.format(),
		createdAt: tgl.format(),
		updatedAt: tgl.format(),
		dueRange: dueRange,
	};

	let details: iStockDetail[] = [];
	function onProductNotFound(e: CustomEvent<string>) {
		showNotification = false;
		setTimeout(() => {
			showNotification = true;
		}, 250);
		txt = e.detail;
		timeout = 6_000;
	}

	let stock = { ...initData };

	const supplierQuery = useQuery(
		"supProp",
		async () => await getSupplierProp(["Supplier"]),
		{
			enabled: browser,
		},
	);
	const employeeQuery = useQuery(
		"emplProp",
		async () => await getSupplierProp(["Employee"]),
		{
			enabled: browser,
		},
	);

	const queryBarcode = useQuery("barcodes", async () => await getBarcodes(), {
		enabled: browser,
	});

	export const prefetchNextPage = (data: {
		status: string;
		stocks: iStock[];
		totalItems: number;
		totalPages: number;
		count: number;
		currentPage: number;
	}) => {
		if (data.currentPage < data.totalPages) {
			client.prefetchQuery(
				[qKey, opt, data.currentPage + 1, limit, supplierId, warehouseId, txt],
				() =>
					getStocks(
						opt,
						data.currentPage + 1,
						limit,
						supplierId,
						warehouseId,
						txt,
					),
			);
		}
	};

	const queryStock = useQuery({
		queryKey: [qKey, { id: stockId }],
		queryFn: async () => getStockById(stockId),
		enabled: false,
	});

	const queryStocks = useQuery({
		queryKey: [qKey, opt, page, limit, supplierId, warehouseId, txt],
		queryFn: async () =>
			await getStocks(opt, page, limit, supplierId, warehouseId, txt),
		enabled: browser,
		onSuccess: prefetchNextPage,
		keepPreviousData: true,
	});

	function setQueryOption(
		p: number,
		l: number,
		o: number,
		s: number,
		w: number,
		t: string,
	) {
		queryStocks.setOptions({
			queryKey: [qKey, o, p, l, s, w, t],
			keepPreviousData: true,
			enabled: browser,
			queryFn: async () =>
				await getStocks(opt, page, limit, supplierId, warehouseId, txt),
			onSuccess: prefetchNextPage,
		});

		// console.log([qKey, p, l, o, t, r]);
		// console.log(q_key);
	}
	let innerWidth = 0;
	let timeout: number | undefined = undefined;

	function onStockChanged(e: CustomEvent<iStockDetail[]>): void {
		let total = e.detail.reduce((o, t) => o + toNumber(t.subtotal), 0);
		stock.total = total;
		stock.remain = total - (toNumber(stock.dp) + toNumber(stock.payment));
		stock.isModified = true;
		stock.isDetailChanged = true;
		// console.log(stock);
	}


	onMount(() => {
		stock.updatedBy = profile.name;
	});

	let subscribe: Unsubscriber;

	function createNewStock(e: CustomEvent<number>): void {
		// stock = { ...initData };
		// details = [];
		editStock(e)
	}

	async function editStock(e: CustomEvent<number>) {
		// if (e.detail === 0) {
		// 	stock = { ...initData };
		// 	details = [];
		// 	isEdit = true;
		// } else {
			// if(stockId === e.detail) {
			// 	subscribe();
			// }

			stockId = e.detail;
			isEdit = true;
			await tick();
			queryStock.setOptions({
				queryKey: [qKey, { id: stockId }],
				queryFn: async () => getStockById(stockId, {stock: {...initData}, details: []}),
				enabled: true,
			});

			if (!subscribe) {
				// console.log(subscribe)
				subscribe = queryStock.subscribe((o) => {
					stock = o.data?.stock ?? { ...initData };
					details = o.data?.details ?? [];
				});
			}
		// }
	}

	async function saveData(e: CustomEvent<iStockDetail[]>) {
		if (e.detail.length > 0) {
			const x = dayjs(stock.createdAt);
			const y = dayjs(stock.dueAt);
			const duration = y.diff(x, "days");
			details = e.detail.map((m) => ({ ...m, stockId: stock.id }));
			delete stock.isDetailChanged;
			delete stock.isModified;
			delete stock.isPayed;

			if (stock.id === 0) {
				const result = await postCreateStock(
					{ ...stock, updatedBy: profile.name, dueRange: duration },
					details,
				);
				if (result) {
					await client.invalidateQueries([qKey, {id: stockId}]);
					await client.invalidateQueries([
						qKey,
						opt,
						page,
						limit,
						supplierId,
						warehouseId,
						txt,
					]);
					isEdit = false;
				}
			}  else {
				const result = await postUpdateStock(
					stockId, { ...stock, updatedBy: profile.name, dueRange: duration },
					details,
				);
				if (result) {
					await client.invalidateQueries([qKey, {id: stockId}]);
					await client.invalidateQueries([
						qKey,
						opt,
						page,
						limit,
						supplierId,
						warehouseId,
						txt,
					]);
					isEdit = false;
				}
			}
		}
	}

	async function deleteStocks(e: CustomEvent<number[]>) {
		let log = await postDeleteStock(e.detail);
		if(log && log.data > 0) {
			client.invalidateQueries([
						qKey,
						opt,
						page,
						limit,
						supplierId,
						warehouseId,
						txt,
					])
		}
		// console.log(log);
	}
	$: {
		supplierQuery.setEnabled(browser);
		employeeQuery.setEnabled(browser);
		queryStocks.setEnabled(browser);
		queryStock.setEnabled(browser);
	}
	$: showNotification = timeout !== undefined;
	$: isDataValid =
		stock.supplierId > 0 &&
		stock.warehouseId > 0 &&
		stock.total > 0 &&
		stock.invoiceId.trim().length > 0;
	$: setQueryOption(page, limit, opt, supplierId, warehouseId, txt);


</script>

<svelte:window bind:innerWidth />

<svelte:head>
	<title>Stock</title>
	<meta name="description" content="Stock this app" />
</svelte:head>

<LocalStorage key="__user_info" bind:value={profile} />
{#if $supplierQuery.isLoading || $employeeQuery.isLoading || $queryBarcode.isLoading || $queryStocks.isLoading}
	<Loading withOverlay small />
{:else if isEdit}
	<Grid noGutter={innerWidth > 720}>
		<Row>
			<Column noGutterRight><h1>Stock</h1></Column>
			<!-- <Column>{stock.dp}</Column> -->
			<Column noGutterLeft style={"text-align: right;"}>
				<h1><strong>{formatNumber(stock.total)}</strong></h1>
				<div class="text-number">
					{numberToText(stock.total.toString(10))}
				</div></Column
			>
		</Row>
	</Grid>

	{#if showInfo}
		<StockInfo data={stock} on:closeInfo={() => (showInfo = false)} />
	{:else}
		<FormStock
			bind:data={stock}
			suppliers={$supplierQuery.data?.data}
			employees={$employeeQuery.data?.data}
			bind:innerWidth
		/>
		<StockDetail
			data={details}
			barcodes={$queryBarcode.data?.data}
			bind:isStockValid={isDataValid}
			on:productNotFound={onProductNotFound}
			on:stockChanged={onStockChanged}
			on:createNewStock={createNewStock}
			on:showInfo={(e) => {
				details = e.detail;
				showInfo = true;
			}}
			on:close={() => (isEdit = false)}
			on:save={saveData}
		/>
	{/if}
{:else}
	<StockList
		data={$queryStocks.data?.stocks}
		suppliers={$supplierQuery.data?.data}
		employees={$employeeQuery.data?.data}
		on:supplierChange={(e) => {
			opt = e.detail === 0 ? 0 : 2;
			supplierId = e.detail;
		}}
		on:warehouseChange={(e) => {
			opt = e.detail === 0 ? 0 : 3;
			warehouseId = e.detail;
		}}
		on:search={(e) => {
			opt = e.detail === null ? 0 : 1;
			txt = e.detail ?? "";
		}}
		on:edit={editStock}
		on:deleteStocks={deleteStocks}
	/>
	<Pagination
		totalItems={$queryStocks.data?.totalItems}
		pageSizes={[3, 5, 10, 20, 50]}
		pageSize={limit}
		{page}
		on:update={(e) => {
			limit = e.detail.pageSize;
			page = e.detail.page;
		}}
		on:click:button--next={(e) => (page = e.detail.page)}
		on:click:button--previous={(e) => (page = e.detail.page)}
	/>
{/if}

{#if showNotification}
	<ToastNotification
		style={"margin-top: 24px; width: 100%"}
		on:click={() => (timeout = 12_000)}
		fullWidth
		{timeout}
		kind="warning-alt"
		on:close={(e) => {
			timeout = undefined;
		}}
	>
		<strong slot="subtitle">Produk yang anda cari tidak ditemukan</strong>
		<div>
			<ProductNotFound productName={txt} />
		</div>
	</ToastNotification>
{/if}

<!-- <div><code><pre>{JSON.strngify(stock, null, 4)}</pre></code></div> -->
