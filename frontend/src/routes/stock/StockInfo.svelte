<script lang="ts">
import { formatNumber } from "$lib/components/NumberFormat";
import type { iStock } from "$lib/interfaces";
import { Grid, Row, Column } from "carbon-components-svelte";
import dayjs from "dayjs";
import { createEventDispatcher } from "svelte";
import LabelProperty from "$lib/components/LabelProperty.svelte";

const dispatch = createEventDispatcher();

export let data: iStock;
</script>

<!-- <Button icon={Close} size="small" on:click={() => dispatch("closeInfo", null)}
  >Close info</Button
> -->
<Grid noGutter>
  <Row>
    <Column sm md noGutter>
      <LabelProperty sm>
        <svelte:fragment slot="label">ID</svelte:fragment>
        <svelte:fragment slot="value">{data.id}</svelte:fragment>
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">No. Faktur</svelte:fragment>
        <svelte:fragment slot="value">{data.invoiceId}</svelte:fragment>
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Tgl. pembelian</svelte:fragment>
        <svelte:fragment slot="value"
          >{dayjs(data.createdAt).format("DD MMMM YYYY")}</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Jatuh tempo</svelte:fragment>
        <svelte:fragment slot="value"
          >{dayjs(data.dueAt).format("DD MMMM YYYY")}</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Supplier</svelte:fragment>
        <svelte:fragment slot="value">{data.supplierName}</svelte:fragment>
      </LabelProperty>
    </Column>
    <Column sm md noGutter>
      <LabelProperty sm>
        <svelte:fragment slot="label">Penjaga gudang</svelte:fragment>
        <svelte:fragment slot="value">{data.warehouseName}</svelte:fragment>
      </LabelProperty>
      <LabelProperty right sm>
        <svelte:fragment slot="label">Total</svelte:fragment>
        <svelte:fragment slot="value"
          >{formatNumber(data.total)}</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty right sm>
        <svelte:fragment slot="label">Cash / DP</svelte:fragment>
        <svelte:fragment slot="value">{formatNumber(data.dp)}</svelte:fragment>
      </LabelProperty>
      <LabelProperty right sm>
        <svelte:fragment slot="label">Angsuran</svelte:fragment>
        <svelte:fragment slot="value"
          >{formatNumber(data.payment)}</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty sm right>
        <svelte:fragment slot="label">Sisa bayar</svelte:fragment>
        <svelte:fragment slot="value"
          >{formatNumber(data.remain)}</svelte:fragment
        >
      </LabelProperty>
    </Column>
  </Row>
</Grid>
