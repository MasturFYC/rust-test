<script lang="ts">
  import type { iCurrentUser, iStock, iStockDetail } from "$lib/interfaces";
  import { useQuery } from "@sveltestack/svelte-query";
  import FormStock from "./FormStock.svelte";
  import dayjs from "dayjs";
  import { getSupplierProp } from "$lib/fetchers";
  import { browser } from "$app/environment";
  import { Loading, LocalStorage } from "carbon-components-svelte";
  import { onMount } from "svelte";
  import StockDetail from "./StockDetail.svelte";

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

  let details: iStockDetail[] = [
    {
      orderId: 0,
      id: 1,
      productId: 1,
      barcode: "3434",
      name: "Jarum Super 12",
      qty: 10,
      direction: 1,
      unit: "bks",
      hpp: 20000,
      price: 25000,
      discount: 0,
      subtotal: 250000,
    },
    {
      orderId: 0,
      id: 2,
      productId: 2,
      barcode: "SY16",
      name: "Gudang garam surya",
      qty: 1,
      direction: 1,
      unit: "bks",
      hpp: 33000,
      price: 36000,
      discount: 0,
      subtotal: 36000,
    }
  ];

  let data = { ...initData };
	let orderId = 0;

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

	let innerWidth = 0;

  onMount(() => {
    data.updatedBy = profile.name;
  });

  $: {
    supplierQuery.setEnabled(browser);
    employeeQuery.setEnabled(browser);
  }


</script>

<svelte:window bind:innerWidth />

<svelte:head>
  <title>Stock</title>
  <meta name="description" content="Stock this app" />
</svelte:head>

<LocalStorage key="__user_info" bind:value={profile} />

<h1>Stock</h1>

{#if $supplierQuery.isLoading || $employeeQuery.isLoading}
	<Loading />
{:else}
<FormStock
  {data}
  suppliers={$supplierQuery.data?.data}
  employees={$employeeQuery.data?.data}
  bind:innerWidth
/>
<StockDetail data={details} />
{/if}
