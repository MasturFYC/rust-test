<script lang="ts">
  import { formatNumber } from "$lib/components/NumberFormat";
  import type { iRelationProp, iStock } from "$lib/interfaces";
  import {
    Button,
    ComboBox,
    DataTable,
    Toolbar,
    ToolbarContent,
    ToolbarSearch,
  } from "carbon-components-svelte";
  import { Delete, Edit, NewTab } from "carbon-icons-svelte";
  import dayjs from "dayjs";
  import { createEventDispatcher, tick } from "svelte";
  import StockInfo from "./StockInfo.svelte";
  import { isStockLoading, isStockUpdating } from "./store";

  export let data: iStock[] = [];
  // export let innerWidth = 720;
  export let suppliers: iRelationProp[] = [];
  export let employees: iRelationProp[] = [];

  export let txt = "";
  export let selectedSupplierId = 0;
  export let selectedWarehouseId = 0;
  let sup_light = false;
  let ware_light = false;
  let selectedRowIds: number[] = [];

  const dispatch = createEventDispatcher();
  const headers = [
    { key: "id", value: "ID#", width: "12%" },
    { key: "createdAt", value: "Tanggal", width: "120px" },
    { key: "invoiceId", value: "No. Faktur", width: "auto" },
    { key: "supplierName", value: "Supplier", width: "auto" },
    { key: "warehouseName", value: "Checker", width: "auto" },
    { key: "total", value: "Total", width: "110px" },
    { key: "cmd", value: "", width: "60px" },
  ];

  async function editStock(id: number) {
    isStockLoading.set(true);
    await tick();

    dispatch("edit", id);
  }

  function searchStock(e: Event): void {
    dispatch("search", txt);
  }

  function searchClear(e: any): void {
    dispatch("search", undefined);
  }

  async function deleteItems(e: MouseEvent) {
    isStockUpdating.set(true);
    // await tick();
    dispatch("deleteStocks", selectedRowIds);
    await tick();
    selectedRowIds = [];
  }
</script>

<DataTable
  batchSelection
  batchExpansion
  size="medium"
  {headers}
  rows={data}
  bind:selectedRowIds
>
  <svelte:fragment slot="cell-header" let:header>
    {#if header.key === "total"}
      <div class="cell-right">{header.value}</div>
    {:else}
      {header.value}
    {/if}
  </svelte:fragment>

  <svelte:fragment slot="cell" let:row let:cell>
    {#if cell.key === "total"}
      <div class="cell-right">{formatNumber(cell.value)}</div>
    {:else if cell.key === "createdAt"}
      <div>{dayjs(cell.value).format("DD-MM-YYYY")}</div>
    {:else if cell.key === "cmd"}
      <Button
        tooltipPosition="left"
        tooltipAlignment="end"
        size="small"
        kind="ghost"
        iconDescription="Edit"
        disabled={$isStockUpdating || $isStockLoading}
        skeleton={$isStockLoading}
        icon={Edit}
        on:click={() => editStock(row.id)}
      />
    {:else}
      {cell.value}
    {/if}
  </svelte:fragment>
  <svelte:fragment slot="expanded-row" let:row>
    <StockInfo data={data.filter((f) => f.id === row.id)[0]} />
  </svelte:fragment>

  <Toolbar size="sm">
    <ToolbarContent>
      <ToolbarSearch
        on:change={searchStock}
        bind:value={txt}
        on:clear={searchClear}
      />
      <ComboBox
        type="inline"
        light={sup_light || selectedSupplierId > 0}
        size="sm"
        style="width: 165px; border-bottom: none;"
        class={"supplier"}
        placeholder="supplier"
        selectedId={selectedSupplierId}
        items={suppliers.map((m) => ({ id: m.id, text: m.text }))}
        on:select={(e) => {
          dispatch("supplierChange", e.detail.selectedId);
          sup_light = true;
        }}
        on:clear={() => {
          sup_light = false;
          dispatch("supplierChange", 0);
        }}
      />
      <ComboBox
        type="inline"
        light={ware_light || selectedWarehouseId > 0}
        class={"supplier"}
        size="sm"
        selectedId={selectedWarehouseId}
        style="width: 165px; border-bottom: none;"
        placeholder="penjaga gudang"
        items={employees.map((m) => ({ id: m.id, text: m.text }))}
        on:select={(e) => {
          dispatch("warehouseChange", e.detail.selectedId);
          ware_light = true;
        }}
        on:clear={() => {
          ware_light = false;
          dispatch("warehouseChange", 0);
        }}
      />
      <Button
        kind="danger"
        size="small"
        disabled={selectedRowIds.length == 0 || $isStockLoading}
        skeleton={$isStockUpdating}
        icon={Delete}
        on:click={deleteItems}>Hapus stock</Button
      >
      <Button
        size="small"
        disabled={$isStockUpdating || $isStockLoading}
        on:click={() => editStock(0)}
        icon={NewTab}>Buat baru</Button
      >
    </ToolbarContent>
  </Toolbar>
</DataTable>
