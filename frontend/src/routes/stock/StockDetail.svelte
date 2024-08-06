<script lang="ts">
  import { formatRupiah } from "$lib/components/NumberFormat";
  import type { iStockDetail } from "$lib/interfaces";
  import { DataTable, TextInput } from "carbon-components-svelte";
  import type {
    DataTableCell,
    DataTableHeader,
    DataTableRow,
  } from "carbon-components-svelte/types/DataTable/DataTable.svelte";
  import { ArrowDown } from "carbon-icons-svelte";
  import { onDestroy, onMount } from "svelte";

  let reform: HTMLDivElement;

  let headers = [
    { key: "barcode", value: "Barcode", width: "20%" },
    { key: "name", value: "Nama barang", width: "auto" },
    { key: "qty", value: "Qty", width: "80px" },
    { key: "price", value: "Harga", width: "100px" },
    { key: "discount", value: "Discount", width: "100px" },
    { key: "subtotal", value: "Subtotal", width: "120px" },
  ];

  export let data: iStockDetail[] = [];
  let currentId = 0;
  let currentDetail: iStockDetail;
  let currentKey = "barcode";

  function onRowClick(e: CustomEvent<DataTableRow>) {
    e.preventDefault();
    let d = e.detail as iStockDetail;
    currentDetail = d;
    let setFocus = false;

    if (currentId != d.id) {
      setFocus = true;
    }

    currentId = d.id;

    if (setFocus) {
      const ctlId = "#" + currentKey + "-id";
      setTimeout(() => {
        const elem = document.querySelector(ctlId) as HTMLInputElement;
        if (elem) {
          elem.focus();
          elem.select();
        }
      }, 100);
    }
  }
  function onCellClik(e: CustomEvent<DataTableCell>): void {
    e.preventDefault();
    currentKey = e.detail.key;
  }

  function clickOutSize(event: any) {
    const withinBoundaries = event.composedPath().includes(reform);

    if (withinBoundaries) {
      // console.log("Click happened inside element");
    } else {
      // console.log("Click happened **OUTSIDE** element");
      currentId = 0;
    }
  }

  function setFocuse(ctlId: string) {
    const elem = document.querySelector(ctlId) as HTMLInputElement;
    if (elem) {
      elem.focus();
      elem.select();
    }
  }

  function barcodeOnKeyDown(e: KeyboardEvent, row: DataTableRow) {
    if (e.key === "Enter") {
      e.preventDefault();
      const ctlId = "#qty-id";
      setFocuse(ctlId);
    } else if (e.key === "Tab" && e.shiftKey) {
      const i = data.findIndex((f) => f.id === row.id);
      if (i > 0) {
        e.preventDefault();
        currentDetail = data[i - 1];
        currentKey = "price";
        currentId = currentDetail.id;
        setTimeout(() => {
          setFocuse("#" + currentKey + "-id");
        }, 100);
      }
    }
  }

  function priceOnKeyDown(e: KeyboardEvent, row: DataTableRow) {
    if ((e.key === "Tab" && !e.shiftKey) || e.key === "Enter") {
      e.preventDefault();
      let i = data.findIndex((f) => f.id === row.id);
      i++;
      currentKey = "barcode";
      if (i === data.length) {
        currentDetail = data[0];
        currentId = currentDetail.id;
        // e.preventDefault();
      } else {
        currentDetail = data[i];
        currentId = currentDetail.id;
      }

      setTimeout(() => {
        const ctlId = "#" + currentKey + "-id";
        setFocuse(ctlId);
      }, 100);
    }
  }

  function qtyOnKeyDown(e: KeyboardEvent, row: DataTableRow) {
    if (e.key === "Enter") {
      e.preventDefault();
      currentKey = "price";
      const ctlId = "#" + currentKey + "-id";
      setFocuse(ctlId);
    }
  }

  // function onTableClick(
  // 	e: CustomEvent<{
  // 		header?: DataTableHeader | undefined;
  // 		row?: DataTableRow | undefined;
  // 		cell?: DataTableCell | undefined;
  // 	}>,
  // ) {
  // 	if (e.detail.cell) {
  // 		currentDetail = e.detail.row as iStockDetail;
  // 		currentId = e.detail.row?.id;
  // 		currentKey = e.detail?.cell?.key;
  // 		console.log("TABLE", currentDetail);
  // 	}
  // }

  onDestroy(() => {
    document.removeEventListener("click", clickOutSize);
  });

  onMount(() => {
    document.addEventListener("click", clickOutSize);
  });
</script>

<div
  bind:this={reform}
  tabindex={1}
  role="row"
  aria-labelledby="tablw-detail"
  on:keydown={(e) => {
    if (e.key !== "ArrowUp" && e.key !== "ArrowDown") {
      return true;
    }
    e.preventDefault();

    const i = data.findIndex((f) => f.id === currentId);

    let x = 0;
    if (e.key === "ArrowDown") {
      x = i === data.length - 1 ? 0 : i + 1;
    } else if (e.key === "ArrowUp") {
      x = i === 0 ? data.length - 1 : i - 1;
    }

    if (i >= 0) {
      currentDetail = data[x];
      currentId = currentDetail.id;

      if (currentKey === "name" || currentKey === "hpp") {
        currentKey = "barcode";
      }
      setTimeout(() => {
        setFocuse("#" + currentKey + "-id");
      }, 100);
    }
  }}
>
  <DataTable
    rows={data}
    {headers}
    zebra
    size="short"
    on:click:row={onRowClick}
    on:click:cell={onCellClik}
  >
    <svelte:fragment slot="cell-header" let:header>
      {#if header.key === "price" || header.key === "hpp" || header.key === "qty" || header.key === "discount" || header.key === "subtotal"}
        <div class="cell-right">{header.value}</div>
      {:else}
        {header.value}
      {/if}
    </svelte:fragment>
    <svelte:fragment slot="cell" let:row let:cell>
      {#if currentId === row["id"]}
        <!-- <form
					on:submit={(e) => {
						e.preventDefault();
						const i = data.findIndex((f) => f.id === currentId);
						if (i >= 0) {
							data.splice(i, 1, currentDetail);
							data = [...data];
						}
					}}
				> -->
        {#if cell.key === "qty"}
          <TextInput
            bind:value={currentDetail.qty}
            size="sm"
            class="cell-edit input-number"
            id="qty-id"
            on:focus={(e) => (currentKey = "qty")}
            on:keydown={(e) => qtyOnKeyDown(e, row)}
          />
        {:else if cell.key === "discount"}
          <TextInput
            bind:value={currentDetail.discount}
            size="sm"
            class="cell-edit input-number"
            id="price-id"
            on:focus={(e) => (currentKey = "price")}
            on:keydown={(e) => priceOnKeyDown(e, row)}
          />
        {:else if cell.key === "barcode"}
          <TextInput
            list="barcode-list"
            bind:value={currentDetail.barcode}
            size={"sm"}
            class="cell-edit"
            id="barcode-id"
            on:focus={(e) => (currentKey = "barcode")}
            on:keydown={(e) => barcodeOnKeyDown(e, row)}
          />
        {:else if cell.key === "price" || cell.key === "hpp" || cell.key === "qty" || cell.key === "discount" || cell.key === "subtotal"}
          <div class="cell-right">{formatRupiah(cell.value)}</div>
        {:else}
          {cell.value}
        {/if}
        <!-- </form> -->
      {:else if cell.key === "price" || cell.key === "hpp" || cell.key === "qty" || cell.key === "discount" || cell.key === "subtotal"}
        <div class="cell-right">{formatRupiah(cell.value)}</div>
      {:else}
        {cell.value}
      {/if}
    </svelte:fragment>
  </DataTable>
</div>

<datalist id="barcode-list">
  <option value="SY12" />
  <option value="SY16" />
  <option value="GG12" />
</datalist>

<style lang="scss">
  .cell-right {
    text-align: right;
  }

  :global(.bx--list-box__field .bx--text-input.supplier) {
    border-bottom: none;
    // background-color: var(--cds-link-01);
    // color: var(--cds-ui-01);
  }
  :global(.bx--table-expand__button) {
    width: auto;
    min-height: 16px;
  }
  :global(.bx--text-input--sm.cell-edit) {
    margin: 0;
    padding: 0 3px;
    height: auto;
  }
  :global(.bx--text-input.input-number) {
    text-align: right;
  }
</style>
