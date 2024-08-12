<script lang="ts">
  import type { iStock } from "$lib/interfaces";
  import { ComboBox, Form, Modal } from "carbon-components-svelte";
  import { Save } from "carbon-icons-svelte";
  // import { createEventDispatcher, onMount } from "svelte";

  import type { ComboBoxItem } from "carbon-components-svelte/types/ComboBox/ComboBox.svelte";
  import {
    formatNumber,
    getNumber,
    getPercent,
  } from "$lib/components/NumberFormat";
  import InputNumber from "$lib/components/NumberInput.svelte";
  import { readonly } from "svelte/store";
  import { toNumber } from "./handler";
  import { stock } from "./store";
  import { tick } from "svelte";

  // export let innerWidth = 720;
  export let open = false;
  // export let isError = false;
  // export let isUpdating = false;
  // export let errorMessage = "";

  // const dispatch = createEventDispatcher();
  $: strRemain = formatNumber(
    toNumber($stock.total) - (toNumber($stock.payment) + toNumber($stock.dp)),
  );
  $: strDp = formatNumber(toNumber($stock.dp));

  async function submit() {
    // isUpdating = true;
    const newDp = getNumber(strDp);
    // console.log(dp);
    stock.update((s) => ({
      ...s,
      dp: newDp,
      // remain: toNumber(s.total) - (newDp + toNumber(s.payment)),
    }));
    // console.log($stock);
    await tick();
    open = false;
  }

  function on_dp_change(e: CustomEvent<string | number | null>): void {
    if (typeof e.detail === "string") {
      strDp = e.detail;
      const dp = getNumber(e.detail);
      const total = toNumber($stock.total);
      const payment = toNumber($stock.payment);
      const remain = total - (dp + payment);
      strRemain = formatNumber(remain);
    }
  }

  // function on_price_change(e: CustomEvent<string | number | null>): void {
  // }

  // function on_percent_change(e: CustomEvent<string | number | null>): void {
  // }

  // function shouldFilterItem(item: ComboBoxItem, value: string) {
  //   if (!value) return true;
  //   return item.text.toLowerCase().includes(value.toLowerCase());
  // }

  // function get_supplier_info(id: number): string {
  //   let item = suppliers.filter((f) => f.id === id)[0];

  //   if (item) {
  //     let info: string;

  //     if (item.street) {
  //       info = item.street;
  //       info = info + " - " + item.city;
  //     } else {
  //       info = item.city;
  //     }
  //     if (item.phone) {
  //       info = info + ", " + item.phone;
  //     }

  //     return info;
  //   }
  //   return "-";
  // }

  // function get_suppliers() {
  //   return suppliers.map((m) => ({ id: m.id, text: m.text }));
  // }

  // let str_price = formatNumber(data.price);
  // let str_hpp = formatNumber(data.hpp);
  // let str_percent = formatNumber(data.margin, 4);
  // let str_heavy = formatNumber(data.heavy, 2);
  //	let str_stock = cardNumber(data.unitInStock.toString());

  // $:	console.log(str_price, str_hpp, str_percent)

  //$: data.categoryId = +cat_id;
</script>

<Modal
  bind:open
  hasForm
  alert
  size="xs"
  modalHeading={"Pembayaran"}
  primaryButtonText="OK"
  primaryButtonIcon={Save}
  secondaryButtonText="Batal"
  selectorPrimaryFocus={"#stock-dp"}
  on:click:button--secondary={() => (open = false)}
  on:click:button--primary={submit}
>
  <!--   primaryButtonDisabled={isUpdating} -->
  <Form>
    <InputNumber
      value={formatNumber($stock.total)}
      labelText="Total"
      size="sm"
      readonly
      inline
    />
    <InputNumber
      inline
      value={formatNumber($stock.dp)}
      labelText="Jml bayar"
      id="stock-dp"
      size="sm"
      on:change={on_dp_change}
    />
    <InputNumber
      inline
      value={formatNumber($stock.payment)}
      labelText="Angsuran"
      readonly
      size="sm"
    />
    <InputNumber
      inline
      value={strRemain}
      labelText="Sisa bayar"
      readonly
      size="sm"
    />
    <!-- <ComboBox
            size="sm"
            titleText="Kategori"
            placeholder="Pilih kategori"
            selectedId={data.categoryId}
            warn={category_invalid}
            items={categories}
            {shouldFilterItem}
            on:select={(e) => (data.categoryId = e.detail.selectedId)}
            on:clear={() => (data.categoryId = 0)}
          /> -->
  </Form>

  <!-- {#if isUpdating}
    <InlineLoading
      status="active"
      description={$stock.id === 0 ? "Posting data..." : "Updating data..."}
    />
  {/if} -->

  <!-- {#if isError}
    <InlineLoading description={errorMessage} status="error" />
  {/if} -->
</Modal>
