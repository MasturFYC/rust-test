<script lang="ts">
  import { browser } from "$app/environment";
  import { formatNumber } from "$lib/components/NumberFormat";
  import { getBarcodes, getSupplierProp } from "$lib/fetchers";
  import type { iCurrentUser, iStock, iStockDetail } from "$lib/interfaces";
  import { numberToText } from "$lib/number-to-string";
  import { useQuery, useQueryClient } from "@sveltestack/svelte-query";
  import {
  	Column,
  	Grid,
  	Loading,
  	LocalStorage,
  	Pagination,
  	Row,
  	ToastNotification,
  } from "carbon-components-svelte";
  import dayjs from "dayjs";
  import { onDestroy, tick } from "svelte";
  import FormStock from "./FormStock.svelte";
  import FormStockPayment from "./FormStockPayment.svelte";
  import ProductNotFound from "./ProductNotFound.svelte";
  import StockDetail from "./StockDetail.svelte";
  import StockList from "./StockList.svelte";
  import {
  	getStockById,
  	getStocks,
  	postCreateStock,
  	postDeleteStock,
  	postUpdateOnlyStock,
  	postUpdateStock,
  } from "./handler";
  import {
  	details,
  	initStock,
  	isStockLoading,
  	isStockUpdating,
  	stock,
  } from "./store";

  const client = useQueryClient();

  let txt = "";
  let page = 1;
  let limit = 5;
  let supplierId = 0;
  let warehouseId = 0;
  let opt = 0;
  let stockId = 0;
  let open = false;
  const qKey = "stocks";

  let isEdit = false;

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

  function onProductNotFound(e: CustomEvent<string>) {
    showNotification = false;
    setTimeout(() => {
      showNotification = true;
    }, 250);
    txt = e.detail;
    timeout = 6_000;
  }

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

  function stockClose(e: CustomEvent<any>): void {
    changeStockSession(0, false);
  }

  function createNewStock(e: CustomEvent<number>): void {
    isEdit = false;
    changeStockSession(0, true);
  }

  function editStock(e: CustomEvent<number>) {
    changeStockSession(e.detail, true);
  }

  async function changeStockSession(id: number, mode: boolean) {
    stockId = id;
    await tick();
    queryStock.setOptions({
      queryKey: [qKey, { id: stockId }],
      queryFn: async () =>
        getStockById(stockId, { stock: { ...initStock }, details: [] }),
      enabled: true,
    });

    // setTimeout(() => {
    isStockLoading.update((o) => (o = false));
    // }, 250);
    isEdit = mode;
  }
  async function updateOnlyStock() {
    const x = dayjs($stock.createdAt);
    const y = dayjs($stock.dueAt);
    const duration = y.diff(x, "days");

    const savedStock: iStock = {
      ...$stock,
      updatedBy: profile.name,
      dueRange: duration,
    };
    delete savedStock.isDetailChanged;
    delete savedStock.isModified;
    delete savedStock.isPayed;

    // console.log(savedStock);

    const result = await postUpdateOnlyStock(stockId, savedStock);
    if (result) {
      await client.invalidateQueries([qKey, { id: stockId }]);
      await client.invalidateQueries([
        qKey,
        opt,
        page,
        limit,
        supplierId,
        warehouseId,
        txt,
      ]);
      isStockUpdating.update((o) => (o = false));
      isEdit = false;
    }

    changeStockSession(0, false);
  }

  async function saveStock(e: CustomEvent<iStockDetail[]>) {
    if ($stock.isDetailChanged) {
      console.log("UDPATE-WITH-DETAILS");
      if (e.detail.length > 0) {
        const x = dayjs($stock.createdAt);
        const y = dayjs($stock.dueAt);
        const duration = y.diff(x, "days");

        const savedStock: iStock = {
          ...$stock,
          paymentType: "Cash",
          updatedBy: profile.name,
          dueRange: duration,
        };
        delete savedStock.isDetailChanged;
        delete savedStock.isModified;
        delete savedStock.isPayed;

        // console.log(savedStock);

        if ($stock.id === 0) {
          const result = await postCreateStock(savedStock, e.detail);
          if (result) {
            await client.invalidateQueries([qKey, { id: stockId }]);
            await client.invalidateQueries([
              qKey,
              opt,
              page,
              limit,
              supplierId,
              warehouseId,
              txt,
            ]);
            // setTimeout(() => {
            isStockUpdating.update((o) => (o = false));
            isEdit = false;
            // }, 250);
          }
        } else {
          const result = await postUpdateStock(stockId, savedStock, e.detail);
          if (result) {
            await client.invalidateQueries([qKey, { id: stockId }]);
            await client.invalidateQueries([
              qKey,
              opt,
              page,
              limit,
              supplierId,
              warehouseId,
              txt,
            ]);
            // setTimeout(() => {
            isStockUpdating.update((o) => (o = false));
            isEdit = false;
            // }, 250);
          }
        }
        changeStockSession(0, false);
      }
    } else {
      console.log("UDPATE-ONLY-STOCK");
      updateOnlyStock();
    }
  }

  async function deleteStocks(e: CustomEvent<number[]>) {
    let log = await postDeleteStock(e.detail);
    if (log && log.data > 0) {
      client.invalidateQueries([
        qKey,
        opt,
        page,
        limit,
        supplierId,
        warehouseId,
        txt,
      ]);
    }

    setTimeout(() => {
      isStockUpdating.set(false);
    }, 250);
    // console.log(log);
  }

  const unsubscribe = queryStock.subscribe((o) => {
    stock.set(o.data?.stock ?? { ...initStock });
    details.set(o.data?.details ?? []);
  });

  onDestroy(unsubscribe);

  $: {
    supplierQuery.setEnabled(browser);
    employeeQuery.setEnabled(browser);
    queryStocks.setEnabled(browser);
    queryStock.setEnabled(browser);
  }
  $: showNotification = timeout !== undefined;
  $: setQueryOption(page, limit, opt, supplierId, warehouseId, txt);
</script>

<svelte:window bind:innerWidth />

<svelte:head>
  <title>Stock</title>
  <meta name="description" content="Stock this app" />
</svelte:head>

<LocalStorage key="__user_info" bind:value={profile} />

<FormStockPayment bind:open />

{#if $supplierQuery.isLoading || $employeeQuery.isLoading || $queryBarcode.isLoading || $queryStocks.isLoading}
  <Loading withOverlay small />
{:else if isEdit}
  <Grid noGutter={innerWidth > 720}>
    <Row>
      <Column noGutterRight><h1>Stock</h1></Column>
      <!-- <Column>{stock.dp}</Column> -->
      <Column noGutterLeft style={"text-align: right;"}>
        <h1><strong>{formatNumber($stock.total)}</strong></h1>
        <div class="text-number">
          {numberToText($stock.total.toString(10))}
        </div></Column
      >
    </Row>
  </Grid>

  <FormStock
    suppliers={$supplierQuery.data?.data}
    employees={$employeeQuery.data?.data}
    bind:innerWidth
  />
  <StockDetail
    barcodes={$queryBarcode.data?.data}
    on:productNotFound={onProductNotFound}
    on:createNewStock={createNewStock}
    on:close={stockClose}
    on:save={saveStock}
    on:addDp={() => (open = true)}
  />
{:else}
  <StockList
    data={$queryStocks.data?.stocks}
    suppliers={$supplierQuery.data?.data}
    employees={$employeeQuery.data?.data}
    {txt}
    seletedSupplierId={supplierId}
    selectedWarehouseId={warehouseId}
    on:supplierChange={(e) => {
      opt = e.detail === 0 ? 0 : 2;
      supplierId = e.detail;
    }}
    on:warehouseChange={(e) => {
      opt = e.detail === 0 ? 0 : 3;
      warehouseId = e.detail;
    }}
    on:search={(e) => {
      opt = e.detail === null || e.detail === "" ? 0 : 1;
      txt = e.detail ?? "";
    }}
    on:edit={editStock}
    on:deleteStocks={deleteStocks}
  />
  <Pagination
    totalItems={$queryStocks.data?.totalItems}
    pageSizes={[3, 5, 10, 20, 50]}
    pageSize={limit}
    style="margin-top: 1px;"
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

<!-- <div><code><pre>{JSON.stringify($stock, null, 4)}</pre></code></div> -->
