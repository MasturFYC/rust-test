<script lang="ts">
  import { formatNumber } from "$lib/components/NumberFormat";
  import type { iProduct } from "$lib/interfaces";
  import { Grid, Row, Column } from "carbon-components-svelte";
  export let product: iProduct;
</script>

<Grid class="font-small">
  <Row noGutter>
    <Column md={1}>id:</Column>
    <Column>{product.id}</Column>
    <Column md={1}>Stock:</Column>
    <Column><strong>{product.unitInStock} {product.unit}</strong></Column>
  </Row>
  <Row noGutter>
    <Column md={1}>Barcode:</Column>
    <Column>{product.barcode}</Column>
    <Column md={1}>HPP:</Column>
    <Column>{formatNumber(product.hpp)}</Column>
  </Row>
  <Row noGutter>
    <Column md={1}>Nama barang:</Column>
    <Column>{product.name}</Column>
    <Column md={1}>Margin:</Column>
    <Column>{formatNumber(product.margin, 4)}%</Column>
  </Row>
  <Row noGutter>
    <Column md={1}>Variant:</Column>
    <Column>{product.variantName ?? ""}</Column>
    <Column md={1}>Harga:</Column>
    <Column>{formatNumber(product.price)}</Column>
  </Row>
  <Row noGutter>
    <Column md={1}>Kategori:</Column>
    <Column>{product.categoryName}</Column>
    <Column md={1}>Berat:</Column>
    <Column>{product.heavy} kg</Column>
  </Row>
  <Row noGutter>
    <Column md={1}>Supplier:</Column>
    <Column>{product.supplierName}</Column>
    <Column md={1}>Deskripsi:</Column>
    <Column>{product.descriptions ?? "-"}</Column>
  </Row>
</Grid>

<style lang="scss">
  :global(.font-small .bx--col, .font-small .bx--col-md-1) {
    font-size: 12px;
  }
</style>
