<script lang="ts">
	import { browser } from '$app/environment';
	import { useQuery } from '@sveltestack/svelte-query';
	import { baseURL, credential_include, type iProduct } from '$lib/interfaces';
	import { InlineLoading } from 'carbon-components-svelte';
	// import { formatNumber } from "$lib/components/NumberFormat";
	import ProductInfo from './ProductInfo.svelte';

	type iResult = {
		status: string;
		data: iProduct;
	};

	interface Props {
		productId: number;
		oldQty: number;
		newQty: number;
		selectedGudangId: number;
		oldGudangId: number;
	}

	let {
		productId = 0,
		oldQty = 0,
		newQty = 0,
		selectedGudangId = 0,
		oldGudangId = 0
	}: Props = $props();

	async function fetchProduct() {
		const url = `${baseURL}/products/${productId}`;
		const options = {
			headers: {
				'content-type': 'application/json'
			},
			method: 'GET',
			credentials: credential_include
		};
		const request = new Request(url, options);
		let result = await fetch(url, request);
		return await result.json();
	}

	let query = useQuery<iResult, Error>(['product', productId], fetchProduct, {
		enabled: false
	});

	$effect(() => {
		query.setEnabled(browser);
	});
</script>

{#if $query.isLoading}
	<InlineLoading />
{:else if $query.isError}
	<code><pre>{$query.error.message}</pre></code>
{:else if $query.isSuccess && $query.data.data}
	<ProductInfo
		product={$query.data.data}
		oldQty={oldQty}
		newQty={newQty}
		selectedGudangId={selectedGudangId}
		oldGudangId={oldGudangId}
	/>
{/if}
