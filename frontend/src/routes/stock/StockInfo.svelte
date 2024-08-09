<script lang="ts">
  import { formatNumber } from "$lib/components/NumberFormat";
  import type { iStock } from "$lib/interfaces";
  import { Grid, Row, Column, Button } from "carbon-components-svelte";
  import { Close } from "carbon-icons-svelte";
  import dayjs from "dayjs";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let data: iStock;
</script>

<Button icon={Close} size="small" on:click={() => dispatch("closeInfo", null)}
  >Close info</Button
>

<Grid style={"margin: 16px 0"}>
  <Row>
    <Column sm={4} md class="box" noGutter>
      <Grid>
        <Row>
          <Column>ID</Column>
          <Column>{data.id}</Column>
        </Row>
        <Row>
          <Column>No. faktur</Column>
          <Column>{data.invoiceId}</Column>
        </Row>
        <Row>
          <Column>Tanggal pembelian</Column>
          <Column>{dayjs(data.createdAt).format("DD MMMM YYYY")}</Column>
        </Row>
        <Row>
          <Column>Jatuh tempo</Column>
          <Column>{dayjs(data.dueAt).format("DD MMMM YYYY")}</Column>
        </Row>
        <Row>
          <Column>Supplier</Column>
          <Column>{data.supplierName ?? ""}</Column>
        </Row>
      </Grid>
    </Column>
    <Column md={1} sm={0} />
    <Column sm={4} md class="box" noGutter>
      <Grid>
        <Row>
          <Column>Penjaga gudang</Column>
          <Column>{data.warehouseName ?? ""}</Column>
        </Row>
        <Row>
          <Column>Total</Column>
          <Column
            ><div class:cell-right={true}>
              {formatNumber(data.total)}
            </div></Column
          >
        </Row>
        <Row>
          <Column>Cash / DP</Column>
          <Column
            ><div class:cell-right={true}>{formatNumber(data.dp)}</div></Column
          >
        </Row>
        <Row>
          <Column>Angsuran</Column>
          <Column
            ><div class:cell-right={true}>
              {formatNumber(data.payment)}
            </div></Column
          >
        </Row>
        <Row>
          <Column>Sisa bayar</Column>
          <Column
            ><div class:cell-right={true}>
              {formatNumber(data.remain)}
            </div></Column
          >
        </Row>
      </Grid>
    </Column>
  </Row>
</Grid>
