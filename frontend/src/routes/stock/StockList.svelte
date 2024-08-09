<script lang="ts">
  import { formatNumber } from "$lib/components/NumberFormat";
  import type { iStock, iPropertyID, iRelationProp } from "$lib/interfaces";
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
  import { createEventDispatcher } from "svelte";

  export let data: iStock[] = [];
  // export let innerWidth = 720;
  export let suppliers: iRelationProp[] = [];
  export let employees: iRelationProp[] = [];

  let txt = "";
  let sup_light = false;
  let ware_light = false;
  let selectedRowIds: number[] = [];

  const dispatch = createEventDispatcher();
  const headers = [
    { key: "id", value: "ID#", width: "80px" },
    { key: "createdAt", value: "Tanggal", width: "120px" },
    { key: "invoiceId", value: "No. Faktur", width: "auto" },
    { key: "supplierName", value: "Supplier", width: "auto" },
    { key: "warehouseName", value: "Penjaga gudang", width: "auto" },
    { key: "total", value: "Total", width: "110px" },
  ];

  function editStock(id: number) {
    dispatch("edit", id);
  }

  function searchStock(e: Event): void {
    dispatch("search", txt);
  }

  function searchClear(e: any): void {
    dispatch("search", undefined);
  }

  function deleteItems(e: MouseEvent): void {
    throw new Error("Function not implemented.");
  }
</script>

<DataTable
  batchSelection
  batchExpansion
  size="short"
  title="Stock"
  description="Tabel data pembelian"
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
    {:else}
      {cell.value}
    {/if}
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
        light={sup_light}
        size="sm"
        style="width: 165px; border-bottom: none;"
        class={"supplier"}
        placeholder="supplier"
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
        light={ware_light}
        class={"supplier"}
        size="sm"
        style="width: 165px; border-bottom: none;"
        placeholder="penjaga gudang"
        items={employees.map((m) => ({ id: m.id, text: m.text }))}
        on:select={(e) => {
          dispatch("warehouseChange", e.detail.selectedId);
          ware_light = true;
        }}
        on:clear={() => {
          dispatch("warehouseChange", undefined);
          ware_light = false;
        }}
      />
      <Button
        kind="danger"
        size="small"
        disabled={selectedRowIds.length == 0}
        icon={Delete}
        on:click={deleteItems}>Hapus stock</Button
      >
      <Button on:click={() => editStock(0)} icon={NewTab}>Buat baru</Button>
    </ToolbarContent>
  </Toolbar>
</DataTable>
