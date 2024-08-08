<script lang="ts">
  import { formatNumber } from "$lib/components/NumberFormat";
  import type { iProduct } from "$lib/interfaces";
  import { Grid, Row, Column } from "carbon-components-svelte";
  export let product: iProduct;
</script>

<Grid noGutter class="font-small">
  <Row noGutter>
    <Column>id:</Column>
    <Column>{product.id}</Column>
    <Column>Stock:</Column>
    <Column><strong>{product.unitInStock} {product.unit}</strong></Column>
  </Row>
  <Row noGutter>
    <Column>Barcode:</Column>
    <Column>{product.barcode}</Column>
    <Column>HPP:</Column>
    <Column>{formatNumber(product.hpp)}</Column>
  </Row>
  <Row noGutter>
    <Column>Nama barang:</Column>
    <Column>{product.name}</Column>
    <Column>Margin:</Column>
    <Column>{formatNumber(product.margin, 4)}%</Column>
  </Row>
  <Row noGutter>
    <Column>Variant:</Column>
    <Column>{product.variantName ?? ""}</Column>
    <Column>Harga:</Column>
    <Column>{formatNumber(product.price)}</Column>
  </Row>
  <Row noGutter>
    <Column>Kategori:</Column>
    <Column>{product.categoryName}</Column>
    <Column>Berat:</Column>
    <Column>{product.heavy} kg</Column>
  </Row>
  <Row noGutter>
    <Column>Supplier:</Column>
    <Column>{product.supplierName}</Column>
    <Column>Deskripsi:</Column>
    <Column>{product.descriptions ?? "-"}</Column>
  </Row>
</Grid>

<style lang="scss">
  :global(.font-small .bx--col) {
    font-size: 12px;
  }
</style>
