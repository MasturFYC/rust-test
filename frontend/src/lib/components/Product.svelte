<style lang="scss">
	.margin-16 {
		margin-left: 24px;
		width: 64px;
	}
	.top-16 {
		margin-top: 8px;
	}
	:global(div.row-odd.bx--row) {
		border-bottom: 1px solid var(--cds-ui-03);
		margin: 0 -32px;
		padding: 0;
	}
	.text-right {
		text-align: right;
		min-width: 90px;
	}
</style>

<script lang="ts">
	import { formatNumber, getNumber } from '$lib/components/NumberFormat';
	import NumberPercent from '$lib/components/NumberPercent.svelte';
	import type { iProduct } from '$lib/interfaces';
	import { Button, Column, Row } from 'carbon-components-svelte';
	import { Add } from 'carbon-icons-svelte';

	interface Props {
		showHpp?: boolean;
		onadd: (qty: number) => void;
		product: iProduct;
	}

	let { product, onadd, showHpp }: Props = $props();
	let qty = $state('1');

	// $inspect(isCustomerSpecial);
</script>

<Row class="row-odd">
	<Column noGutter md={2}><div class="top-16">{product.barcode}</div></Column>
	<Column><div class="top-16">{product.name}</div></Column>
	{#if showHpp}
		<Column as><div class="top-16">{formatNumber(product.hpp)}</div></Column>
	{:else}
		<Column as><div class="top-16">{formatNumber(product.price)}</div></Column>
	{/if}
	<Column as
		><div class="top-16 text-right">
			{product.stocks[0].qty}
			{product.unit}
		</div></Column
	>
	<Column as>
		<div class="margin-16" title="Qty">
			<NumberPercent
				bind:value={qty}
				size={'sm'}
				on:change={(e) => {
					const el = e.currentTarget as HTMLInputElement;
					if (el) {
						qty = el.value;
					}
				}}
			/>
		</div>
	</Column>
	<Column noGutter as>
		<div>
			<Button
				onclick={() => onadd(getNumber(qty))}
				size="small"
				icon={Add}
				kind="secondary">Tambahkan</Button
			>
		</div>
	</Column>
</Row>
