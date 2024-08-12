<script lang="ts">
  import LabelProperty from "$lib/components/LabelProperty.svelte";
  import { formatNumber } from "$lib/components/NumberFormat";
  import type { iProduct } from "$lib/interfaces";
  import { Grid, Row, Column } from "carbon-components-svelte";
  import { toNumber } from "./handler";
  export let product: iProduct;
  export let oldQty: number = 0;
  export let newQty: number = 0;
</script>

<!-- <div>
	{JSON.stringify(
		{
			stock: toNumber(product.unitInStock),
			oldQty: oldQty,
			newQty: newQty,
		},
		null,
		4,
	)}
</div> -->
<Grid noGutter>
  <Row noGutter>
    <Column>
      <LabelProperty sm>
        <svelte:fragment slot="label">ID</svelte:fragment>
        <svelte:fragment slot="value">{product.id}</svelte:fragment>
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Barcode</svelte:fragment>
        <svelte:fragment slot="value">{product.barcode}</svelte:fragment>
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Nama barang</svelte:fragment>
        <svelte:fragment slot="value">{product.name}</svelte:fragment>
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Variant</svelte:fragment>
        <svelte:fragment slot="value"
          >{product.variantName ?? ""}</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Supplier</svelte:fragment>
        <svelte:fragment slot="value">{product.supplierName}</svelte:fragment>
      </LabelProperty>
    </Column>
    <Column>
      <LabelProperty sm>
        <svelte:fragment slot="label">Stock</svelte:fragment>
        <svelte:fragment slot="value">
          <strong
            >{toNumber(product.unitInStock) - oldQty} + {newQty} = {toNumber(
              product.unitInStock,
            ) -
              oldQty +
              newQty}
            {product.unit}</strong
          >
        </svelte:fragment>
      </LabelProperty>
      <LabelProperty sm right>
        <svelte:fragment slot="label">HPP</svelte:fragment>
        <svelte:fragment slot="value"
          >{formatNumber(product.hpp)}</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty sm right>
        <svelte:fragment slot="label">Margin</svelte:fragment>
        <svelte:fragment slot="value"
          >{formatNumber(product.margin, 4)}%</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty sm right>
        <svelte:fragment slot="label">Harga</svelte:fragment>
        <svelte:fragment slot="value"
          >{formatNumber(product.price)}</svelte:fragment
        >
      </LabelProperty>
      <LabelProperty sm>
        <svelte:fragment slot="label">Berat</svelte:fragment>
        <svelte:fragment slot="value"
          >{formatNumber(product.heavy)} kg</svelte:fragment
        >
      </LabelProperty>
    </Column>
  </Row>
  <Row>
    <Column>
      <LabelProperty sm>
        <svelte:fragment slot="label">Deskripsi</svelte:fragment>
        <svelte:fragment slot="value">{product.descriptions}</svelte:fragment>
      </LabelProperty>
    </Column>
  </Row>
</Grid>

<style lang="scss">
  :global(.font-small .bx--col, .font-small .bx--col-md-1) {
    font-size: 12px;
  }
</style>
