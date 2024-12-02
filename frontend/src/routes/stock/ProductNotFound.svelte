<script lang="ts">
	import { browser } from '$app/environment';
	import { formatNumber } from '$lib/components/NumberFormat';
	import { baseURL, credential_include, type iProduct } from '$lib/interfaces';
	import { useQuery } from '@sveltestack/svelte-query';
	import { CodeSnippet, Loading, Tag } from 'carbon-components-svelte';

	export let productName = '';
	async function fetchProductName(name: string) {
		const url = `${baseURL}/products/names/list?txt=${name}&limit=5`;
		const options = {
			method: 'GET',
			credentials: credential_include
		};
		const request = new Request(url, options);
		const result = await fetch(request);

		console.log(result);

		return await result.json();
	}

	let query = useQuery<{ status: string; data: iProduct[] }, Error>(
		['productList', productName],
		async () => await fetchProductName(productName),
		{ enabled: browser }
	);

	$: {
		query.setEnabled(browser);
	}
</script>

{#if $query.isLoading}
	<Loading withOverlay small />
{:else if $query.isError}
	<strong>Error:</strong> {$query.error.message}
{:else if $query.isSuccess}
	<div>Apakah maksudnya produk seperti di bawah ini?</div>
	<div style="margin: 16px 0;">
		{#each $query.data.data as p, i}
			<p>
				<CodeSnippet code={p.barcode} type="inline" />
				<span>{p.name}, {p.variantName}</span>
				<Tag
					size="sm"
					type={i === 1
						? 'purple'
						: i === 2
							? 'blue'
							: i === 3
								? 'teal'
								: i === 4
									? 'green'
									: 'magenta'}><strong>{formatNumber(p.price)}</strong></Tag
				>
			</p>
		{/each}
	</div>
{/if}
