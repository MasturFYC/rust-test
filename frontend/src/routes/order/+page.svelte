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
	import type { iCurrentUser, iOrder, iOrderDetail, iProduct, iProductStock  } from '$lib/interfaces';
	import { OrderDetails as SendToBack } from 'carbon-icons-svelte';
    import ProductSearch from './ProductSearch.svelte';
	import { numberToText } from '$lib/number-to-string';
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
	import FormOrder from './FormOrder.svelte';
	import FormOrderPayment from './FormOrderPayment.svelte';
	import ProductNotFound from './ProductNotFound.svelte';
	import OrderDetail from './OrderDetail.svelte';
	import OrderList from './OrderList.svelte';
	import type { iGudangResult } from './handler';

	import {
		fetchGudangs,
		getOrderById,
		getOrders,
		postCreateOrder,
		postDeleteOrder,
		postUpdateOnlyOrder,
		postUpdateOrder,
		toNumber
	} from './handler';
	import {
		details,
		initOrder,
		isOrderLoading,
		isOrderUpdating,
		order
	} from './store';

	const title = 'Order';
	const qKey = 'orders';
	const client = useQueryClient();
	const pages = [3, 5, 10, 25, 50];
	let txt = $state('');
	let page = $state(1);
	let pageSize = $state(5);
	let customerId = $state(0);
	let salesId = $state(0);
	let opt = $state(0);
	let orderId = $state($order.id);
	let open = $state(false);
	let innerWidth = $state(0);
	let timeout: number | undefined = $state(undefined);
	let showNotification = $state(false);
	let isOpen = $state(false);
    let isSearch = $state(false);
    let strNotFound = $state('');
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
	let query_key = $derived([qKey, page, pageSize, customerId, salesId, txt]);
	const query_next = $derived([
		qKey,
		pageNext,
		pageSize,
		customerId,
		salesId,
		txt
	]);
	let isFetching = $derived(useIsFetching(query_key));

	function onProductNotFound(e: string) {
		// showNotification = false;
		// setTimeout(() => {
			// showNotification = true;
		// }, 250);
		// txt = e;
		// timeout = 6_000;
        strNotFound = e;
        isSearch = true;
	}

	const customerQuery = useQuery(
		['relation', 'customer'],
		async () => await getRelationProp(['Customer']),
		{
			enabled: browser
		}
	);

	const salesQuery = useQuery(
		['relation', 'sales'],
		async () => await getRelationProp(['Sales']),
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
		data: iOrder[];
		totalItems: number;
		totalPages: number;
		count: number;
		currentPage: number;
	}) => {
		//	console.log(data.currentPage, data.totalPages);

		if (data.currentPage < data.totalPages) {
			client.prefetchQuery(query_next, () => loadorder(data.currentPage + 1));
		}
	};

	const loadorder = async (p: number) => {
		return await getOrders(opt, p, pageSize, customerId, salesId, txt);
	};

	const queryOrder = $derived.by(() => {
		return useQuery({
			queryKey: [qKey, { id: orderId }],
			queryFn: async () => getOrderById(orderId),
			enabled: false
		});
	});

	const queryOrders = $derived(
		useQuery({
			queryKey: query_key,
			queryFn: async () => loadorder(page),
			enabled: browser,
			onSuccess: prefetchNextPage,
			keepPreviousData: true
		})
	);

	// function setQueryOption(
	// 	p: number,
	// 	l: number,
	// 	o: number,
	// 	s: number,
	// 	w: number,
	// 	t: string
	// ) {
	// 	queryOrders.setOptions({
	// 		queryKey: [qKey, o, p, l, s, w, t],
	// 		keepPreviousData: true,
	// 		enabled: browser,
	// 		queryFn: async () =>
	// 			await getOrders(opt, page, pageSize, customerId, salesId, txt),
	// 		onSuccess: prefetchNextPage
	// 	});
	// 	// console.log([qKey, p, l, o, t, r]);
	// 	// console.log(q_key);
	// }

	async function openOrder(_e: any) {
		isOrderLoading.update(() => false);
		await tick();
		isOpen = true;
		//changeOrderSession(0, true);
	}

	function createNewOrder(_e: number): void {
		isOpen = false;
		orderId = 0;
		order.set({ ...initOrder });
		details.set([]);
		//changeOrderSession(0, false);
	}

	function editOrder(e: number) {
		if (e === 0) {
			isOpen = false;
			return;
		}
		changeOrderSession(e, false);
	}

	async function changeOrderSession(id: number, mode: boolean) {
		if (id !== orderId) {
			orderId = id;
			await tick();
			queryOrder.setOptions({
				queryKey: [qKey, { id: orderId }],
				queryFn: async () =>
					getOrderById(orderId, { order: { ...initOrder }, details: [] }),
				enabled: true
			});

			isOrderLoading.update(() => false);
		}
		// }, 250);
		isOpen = mode;
	}
	async function updateOnlyOrder() {
		const x = dayjs($order.createdAt);
		const y = dayjs($order.dueAt);
		const duration = y.diff(x, 'days');

		const savedOrder: iOrder = {
			...$order,
			updatedBy: profile.name,
			dueRange: duration
		};
		delete savedOrder.isDetailChanged;
		delete savedOrder.isModified;
		delete savedOrder.isPayed;

		// console.log(savedOrder);

		const result = await postUpdateOnlyOrder(orderId, savedOrder);
		if (result) {
			await client.invalidateQueries([qKey, { id: orderId }]);
			await client.invalidateQueries(query_key);
			isOrderUpdating.update(() => false);
			isOpen = false;
		}

		changeOrderSession(0, false);
	}

	async function saveOrder(e: iOrderDetail[]) {
		if ($order.isDetailChanged) {
			if (e.length > 0) {
				const x = dayjs($order.createdAt);
				const y = dayjs($order.dueAt);
				const duration = y.diff(x, 'days');

				const savedOrder: iOrder = {
					...$order,
					isProtected: true,
					paymentType: 'Cash',
					updatedBy: profile.name,
					dueRange: duration
				};

				// console.log(savedOrder);

				delete savedOrder.isDetailChanged;
				delete savedOrder.isModified;
				delete savedOrder.isPayed;
				let updateSuccess = false;

				if (orderId === 0) {
					const result = await postCreateOrder(savedOrder, e);
					if (result) {
						updateSuccess = true;
					}
				} else {
					const result = await postUpdateOrder(orderId, savedOrder, e);
					if (result) {
						updateSuccess = true;
					}
				}
				// editOrder(0);
				if (updateSuccess) {
					await client.invalidateQueries([qKey, { id: orderId }]);
					await client.invalidateQueries(query_key);
					isOrderUpdating.update(() => false);
					isOpen = false;
					await tick();
					createNewOrder(0);
				}
				//changeOrderSession(0, false);
			}
		} else {
			// console.log("UDPATE-ONLY-STOCK");
			updateOnlyOrder();
		}
	}

	async function deleteOrders(e: number[]) {
		let i = e.findIndex((f) => f === orderId);

		let log = await postDeleteOrder(e);
		if (log && log.data > 0) {
			client.invalidateQueries(query_key);
		}
		setTimeout(() => {
			isOrderUpdating.set(false);
		}, 250);

		if (orderId > 0 && i >= 0) {
			await tick();
            orderId = 0;
			order.set({ ...initOrder });
			details.set([]);
		}
		// console.log(log);
	}

    function createNewId(): number {
        console.log($details.length);
        if($details.length >= 1) {
	    	let test = $details.reduce((prev, cur) =>
		    	prev.id > cur.id ? prev : cur
		    ).id;
		    return test + 1;
        }
        return 1;
	}

    function selectProduct(p: iProduct, q: number) {
        const i = $details.findIndex(f => f.productId === p.id);
        const n = p.stocks.findIndex(f => f.gudangId === 1);
        const stock: iProductStock = p.stocks[n];
       
        if(i >= 0) {
            const slices = [...$details];
            const d = slices[i];
            d.qty = toNumber(d.qty) + q;
            d.subtotal = (toNumber(d.price) - toNumber(d.discount)) * d.qty
            slices.splice(i, 1, d);
            details.update(() => [...slices]);
        } else {
            const d: iOrderDetail = {
                orderId: $order.id,
 			    id: createNewId(),
				price: toNumber(p.price),
                qty: q,
				unit: p.unit,
				productId: p.id,
				direction: -1,
                discount: 0,
                subtotal: toNumber(p.price) * q,
				name: p.name,
				barcode: p.barcode,
				hpp: toNumber(p.hpp),
				oldQty: 0, // stock.qty,
                oldGudangId: stock.gudangId,
                gudangName: stock.name,
                gudangId: stock.gudangId 
            }
            details.update((o) => ([...o, d]));
        }

        const total = $details.reduce((o,t) => o + toNumber(t.subtotal), 0);
 		order.update(
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

	const subsribe = () => {
		order.set($queryOrder.data?.order ?? { ...initOrder });
		details.set($queryOrder.data?.details ?? []);
	};

	let isOrderAvailabel = $derived.by(() => {
		return (
			$queryOrder.isSuccess &&
			$queryOrder.data &&
			$queryOrder.status === 'success' &&
			$queryOrder.isFetchedAfterMount
		);
	});

	let customers = $derived.by(() => {
		if ($customerQuery.isSuccess && $customerQuery.data) {
			return $customerQuery.data.data;
		}
		return [];
	});

	let sales = $derived.by(() => {
		if ($salesQuery.isSuccess && $salesQuery.data) {
			return $salesQuery.data.data;
		}
		return [];
	});

	let orders = $derived.by(() => {
		if ($queryOrders.isSuccess && $queryOrders.data) {
			return $queryOrders.data.data;
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
		if ($queryOrders.isSuccess && $queryOrders.data) {
			return $queryOrders.data.totalItems;
		}
		return 0;
	});

	$effect.pre(() => {
		if (isOrderAvailabel) {
			subsribe();
		}
	});

	// $effect.root(()=>  {
	// 	customerQuery.setEnabled(browser);
	// 	salesQuery.setEnabled(browser);
	// 	gudangQuery.setEnabled(browser);
	// 	queryOrders.setEnabled(browser);
	// 	queryOrder.setEnabled(browser);
	// });

	$effect(() => {
		showNotification = timeout !== undefined;
	});

	// $inspect($order);
	// $: setQueryOption(page, pageSize, opt, customerId, salesId, txt);
</script>

<svelte:window bind:innerWidth={innerWidth} />

<svelte:head>
	<title>{title}</title>
	<meta name="description" content="Order this app" />
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

{#snippet formOrder()}
	<Grid noGutter={innerWidth > 720}>
		<Row>
			<Column noGutterRight md={2}>
				<Grid noGutter>
					<Row>
						<Column>Bayar:</Column>
						<Column>{formatNumber($order.dp)}</Column>
					</Row>
					<Row>
						<Column>Angsuran:</Column>
						<Column>{formatNumber($order.payment)}</Column>
					</Row>
					<Row>
						<Column>Sisa bayar:</Column>
						<Column>{formatNumber($order.remain)}</Column>
					</Row>
				</Grid>
			</Column>
			<Column noGutterLeft>
				<div class="col-parent">
					<div class="col-child">
						<h1 class="num-right">
							<strong>{formatNumber($order.total)}</strong>
						</h1>
						<div class="text-number">
							{numberToText($order.total.toString(10))}
						</div>
					</div>
				</div>
			</Column>
		</Row>
	</Grid>
{/snippet}
{#snippet orderDetail()}
	<FormOrder customers={customers} sales={sales} bind:innerWidth={innerWidth} />
	<OrderDetail
		barcodes={barcodes}
		gudangs={gudangs}
		onnotfound={onProductNotFound}
		onnew={createNewOrder}
		onopen={openOrder}
		onsave={saveOrder}
		onadddp={() => (open = true)}
	/>
{/snippet}

{#snippet orderList()}
	<OrderList
		data={orders}
		customers={customers}
		sales={sales}
		txt={txt}
		selectedCustomerId={customerId}
		selectedSalesId={salesId}
		oncustomerchange={(e) => {
			opt = e === 0 ? 0 : 2;
			customerId = e;
		}}
		onsaleschange={(e) => {
			opt = e === 0 ? 0 : 3;
			salesId = e;
		}}
		onsearch={(e) => {
			opt = e === null || e === '' ? 0 : 1;
			txt = e ?? '';
		}}
		onedit={editOrder}
		ondelete={deleteOrders}
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
			<h2>
				<SendToBack size={24} />
				{title}{isOpen && orderId === 0 ? '' : ` #${orderId}`}
			</h2>
		</Column>
		<Column
			>{#if $customerQuery.isLoading || $salesQuery.isLoading || $queryBarcode.isLoading || $queryOrders.isLoading || $gudangQuery.isLoading}
				<Loading withOverlay={false} small={true} />
			{/if}</Column
		>
	</Row>
</Grid>

{#if open}
	<FormOrderPayment
		bind:open={open}
		total={$order.total}
		dp={$order.dp}
		payment={$order.payment}
		onupdate={(e) => {
			order.update((o) => ({
				...o,
				dp: e,
				remain: toNumber(o.total) - (e + toNumber(o.payment))
			}));
		}}
	/>
{/if}

{@render formOrder()}
{#if isOpen}
	{@render orderList()}
	{@render paginating($isFetching !== 0)}
{:else}
	{@render orderDetail()}
{/if}
{#if showNotification}
	{@render toas()}
{/if}

{#if isSearch && !isOpen}
    <ProductSearch value={strNotFound} onselect={(p, q) => selectProduct(p, q)} onclear={() => {isSearch=false;}} />
{/if}
