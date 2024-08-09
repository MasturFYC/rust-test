<script lang="ts">
	import type { iCurrentUser, iStock, iStockDetail } from "$lib/interfaces";
	import { useQuery } from "@sveltestack/svelte-query";
	import FormStock from "./FormStock.svelte";
	import dayjs from "dayjs";
	import { getBarcodes, getSupplierProp } from "$lib/fetchers";
	import { browser } from "$app/environment";
	import {
		Column,
		CopyButton,
		Grid,
		Loading,
		LocalStorage,
		Row,
		ToastNotification,
	} from "carbon-components-svelte";
	import { onMount } from "svelte";
	import StockDetail from "./StockDetail.svelte";
	import { fade } from "svelte/transition";
	import ProductNotFound from "./ProductNotFound.svelte";
	import { formatNumber, getNumber } from "$lib/components/NumberFormat";
	import { numberToText } from "$lib/number-to-string";
	// const client = useQueryClient();

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
		paymentType: "",
		updatedBy: profile.name,
		total: 0,
		dp: 0,
		payment: 0,
		remain: 0,
		invoiceId: "",
		dueAt: next_tgl.format(),
		createdAt: tgl.format(),
		updatedAt: tgl.format(),
	};

	let details: iStockDetail[] = [];
	//   [
	//   {
	//     orderId: 0,
	//     id: 1,
	//     productId: 1,
	//     barcode: "3434",
	//     name: "Jarum Super 12",
	//     qty: 10,
	//     direction: 1,
	//     unit: "bks",
	//     hpp: 20000,
	//     price: 25000,
	//     discount: 0,
	//     subtotal: 250000,
	//   },
	//   {
	//     orderId: 0,
	//     id: 2,
	//     productId: 2,
	//     barcode: "SY16",
	//     name: "Gudang garam surya",
	//     qty: 1,
	//     direction: 1,
	//     unit: "bks",
	//     hpp: 33000,
	//     price: 36000,
	//     discount: 0,
	//     subtotal: 36000,
	//   },
	// ];

	let txt = "";

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

	let innerWidth = 0;
	let timeout: number | undefined = undefined;

	function onStockChanged(e: CustomEvent<iStockDetail[]>): void {
		let total = e.detail.reduce((o, t) => o + t.subtotal, 0);
		stock.total = total;
		stock.remain = total - (stock.dp + stock.payment);
		stock.isModified = true;
		stock.isDetailChanged = true;
	}

	// function onDpChange(e: CustomEvent<string | number | null>) {
	//   if (typeof e.detail === "string") {
	//     const dp = getNumber(e.detail);
	//     stock.dp = dp;
	//     stock.remain = stock.total - (dp + stock.payment);
	//   }
	// }
	onMount(() => {
		stock.updatedBy = profile.name;
	});

	$: {
		supplierQuery.setEnabled(browser);
		employeeQuery.setEnabled(browser);
	}
	$: showNotification = timeout !== undefined;
	$: isDataValid =
		stock.supplierId > 0 &&
		stock.warehouseId > 0 &&
		stock.total > 0 &&
		stock.invoiceId.trim().length > 0;

	function createNewStock(e: CustomEvent<any>): void {
		stock = { ...initData };
		details = [];
	}
</script>

<svelte:window bind:innerWidth />

<svelte:head>
	<title>Stock</title>
	<meta name="description" content="Stock this app" />
</svelte:head>

<LocalStorage key="__user_info" bind:value={profile} />
<Grid noGutter={innerWidth > 720}>
	<Row>
		<Column noGutterRight><h1>Stock</h1></Column>
		<!-- <Column>{stock.dp}</Column> -->
		<Column noGutterLeft style={"text-align: right;"}>
			<h1><strong>{formatNumber(stock.total)}</strong></h1>
			<div class="text-number">
				{numberToText(stock.total.toFixed(0))}
			</div></Column
		>
	</Row>
</Grid>

{#if $supplierQuery.isLoading || $employeeQuery.isLoading || $queryBarcode.isLoading}
	<Loading withOverlay small />
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
