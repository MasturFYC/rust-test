<style lang="scss">
.code-pre {
	font-size: small;
}
</style>

<script lang="ts">
import { formatNumber } from '$lib/components/NumberFormat';
import { Column, Grid, Row } from 'carbon-components-svelte';
import Stocks from './stock.svelte';
import type { DataTableRow } from 'carbon-components-svelte/src/DataTable/DataTable.svelte';
import type {Snippet} from 'svelte';

interface Props {
    row: DataTableRow,
    innerWidth: number,
     deleteTool: (id: number) => ReturnType<Snippet>
}

let { row, innerWidth = $bindable(720), deleteTool }: Props = $props();

</script>

<Grid noGutter>
	<Row noGutter>
		{#if innerWidth < 720}
			<Column md>
				<code class="code-pre">
					<pre>barcode:    {row['barcode']}
HPP:        {formatNumber(row['hpp'])}
Margin:     {formatNumber(row['margin'], 2)}%

</pre></code
				></Column
			>
		{/if}
		<Column
			><code class="code-pre"
				><pre>Variant:    {row['variantName']}
Berat:      {formatNumber(row['heavy'], 2)} kg
Aktif:      {row['isActive'] ? 'Ya' : 'Tidak'}
Kategori:   {row['categoryName']}
Supplier:   {row['supplierName']}
Deskripsi:  {row['descriptions']}</pre></code
			></Column
		>
		<Column
			><code class="code-pre">
				Stock Gudang:<br />
				<Stocks stocks={row['stocks']} unit={row['unit']} />
			</code>
		</Column>
		<Column>
            {@render deleteTool(row.id)}
		</Column>
	</Row>
</Grid>
