<script lang="ts">
	import { formatRupiah } from "$lib/components/NumberFormat";
	import { Column, Grid, Row } from "carbon-components-svelte";
	import type { DataTableRow } from "carbon-components-svelte/types/DataTable/DataTable.svelte";
	import DeleteProduct from "./DeleteProduct.svelte";
   

   export let row: DataTableRow;
   export let innerWidth = 720;

</script>
<Grid noGutter>
	<Row>
		{#if (innerWidth < 720) }
		<Column md>
		<code class="code-pre">	<pre>barcode:    {row["barcode"]}
HPP:        {formatRupiah(row["hpp"])}
Margin:     {formatRupiah(row["margin"], 2)}%

</pre></code></Column>
	{/if}
<Column><code class="code-pre"><pre>Variant:    {row["variantName"]}
Berat:      {formatRupiah(row["heavy"],2)} kg
Aktif:      {row["isActive"] ? "Ya" : "Tidak"}
Kategori:   {row["categoryName"]}
Supplier:   {row["supplierName"]}
Deskripsi:  {row["descriptions"]}</pre></code></Column>
<Column>		
	<DeleteProduct productId={row.id} on:deleteData />
</Column>
</Row>
</Grid>

<style lang="scss">
	.code-pre {
		font-size: small;
	}
</style>