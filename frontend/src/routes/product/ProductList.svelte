<style lang="scss">
.cell-right {
  text-align: right;
}
/*
		 :global(table.bx--data-table th, table.bx--data-table--zebra) {
		background-color: #000;
		 }
		 */
</style>

<script lang="ts">
import { formatNumber } from "$lib/components/NumberFormat";
import type {
  iProduct,
  iProductStock,
  iPropertyID,
  iRelationProp,
} from "$lib/interfaces";
import {
  Button,
  ComboBox,
  DataTable,
  Toolbar,
  ToolbarContent,
  ToolbarSearch,
} from "carbon-components-svelte";
import { Edit, NewTab } from "carbon-icons-svelte";
import { createEventDispatcher } from "svelte";
import ExapandedList from "./ExapandedList.svelte";

export let data: iProduct[] = [];
export let innerWidth = 720;
export let suppliers: iRelationProp[] = [];
export let categories: iPropertyID[] = [];

const dispatch = createEventDispatcher();
const headers = [
  { key: "name", value: "Nama Barang", width: "auto" },
  { key: "barcode", value: "Barcode", width: "100px" },
  { key: "stocks", value: "Stock", width: "90px" },
  { key: "hpp", value: "HPP", width: "80px" },
  { key: "margin", value: "Margin", width: "80px" },
  { key: "price", value: "Harga", width: "90px" },
  { key: "cmd", value: "", width: "60px" },
];
const headers2 = [
  { key: "name", value: "Nama Barang", width: "auto" },
  { key: "stocks", value: "Stock", width: "90px" },
  { key: "price", value: "Harga", width: "90px" },
  { key: "cmd", value: "", width: "60px" },
];

function get_headers() {
  if (innerWidth < 720) {
    return headers2;
  }
  return headers;
}

function edit_product(id: number | undefined) {
  dispatch("edit", id);
}

let txt = "";

function submit_search(e: Event): void {
  dispatch("search", txt);
}

function search_clear(e: any): void {
  dispatch("search", undefined);
}

// function get_suppliers() {
// 	return suppliers.map((m) => ({ id: m.id, text: m.text }));
// }
const defaultStock = (stocks: iProductStock[]) => {
  const d = stocks.filter((f) => f.gudangId === 1)[0];
  if (d) {
    return d.qty;
  }
  return 0;
};

let sup_light = false;
let cat_light = false;
// $: 	console.log(suppliers)
</script>

<DataTable expandable size="medium" headers={get_headers()} rows={data}>
  <svelte:fragment slot="cell-header" let:header>
    {#if header.key === "price" || header.key === "hpp" || header.key === "stocks" || header.key === "margin"}
      <div class="cell-right">{header.value}</div>
    {:else}
      {header.value}
    {/if}
  </svelte:fragment>

  <svelte:fragment slot="cell" let:row let:cell>
    {#if cell.key === "cmd"}
      <Button
        tooltipPosition="left"
        tooltipAlignment="end"
        size="small"
        kind="ghost"
        iconDescription="Edit"
        icon={Edit}
        on:click={() => edit_product(row.id)}
      />
      <!-- <div></div> -->
    {:else if cell.key === "relationType"}
      <div style="cell-right">{cell.value.join(", ")}</div>
    {:else if cell.key === "hpp" || cell.key === "price"}
      <div class="cell-right">{formatNumber(cell.value)}</div>
    {:else if cell.key === "margin"}
      <div class="cell-right">{formatNumber(cell.value, 2)}%</div>
    {:else if cell.key === "stocks"}
      <div class="cell-right">
        {formatNumber(defaultStock(cell.value))}
        {row["unit"]}
      </div>
    {:else}
      {cell.value}
    {/if}
  </svelte:fragment>
  <svelte:fragment slot="expanded-row" let:row>
    <ExapandedList row={row} bind:innerWidth={innerWidth} on:deleteData />
  </svelte:fragment>

  <Toolbar size="sm">
    <ToolbarContent>
      <ToolbarSearch
        on:change={submit_search}
        bind:value={txt}
        on:clear={search_clear}
      />
      <ComboBox
        type="inline"
        light={cat_light}
        size="sm"
        style="border-bottom: none"
        class={"supplier"}
        placeholder="filter by category"
        items={categories}
        on:select={(e) => {
          dispatch("categoryChange", e.detail.selectedId);
          cat_light = true;
        }}
        on:clear={() => {
          cat_light = false;
          dispatch("categoryChange", 0);
        }}
      />
      <ComboBox
        type="inline"
        light={sup_light}
        style="border-bottom: none"
        class={"supplier"}
        size="sm"
        placeholder="filter by supplier"
        items={suppliers.map((m) => ({ id: m.id, text: m.text }))}
        on:select={(e) => {
          dispatch("supplierChange", e.detail.selectedId);
          sup_light = true;
        }}
        on:clear={() => {
          dispatch("supplierChange", undefined);
          sup_light = false;
        }}
      />
      <!-- <ToolbarMenu>
				<ToolbarMenuItem primaryFocus>Restart all</ToolbarMenuItem>
				<ToolbarMenuItem href="https://cloud.ibm.com/docs/loadbalancer-service"
					>API documentation</ToolbarMenuItem
				>
				<ToolbarMenuItem hasDivider danger>Stop all</ToolbarMenuItem>
			</ToolbarMenu> -->
      <Button on:click={() => edit_product(undefined)} icon={NewTab}
        >Buat baru</Button
      >
    </ToolbarContent>
  </Toolbar>
</DataTable>
