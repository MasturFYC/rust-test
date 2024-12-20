<script lang="ts">
	import { browser } from '$app/environment';
	import { baseURL, credential_include, type iProduct } from '$lib/interfaces';
	import { Grid, Search } from 'carbon-components-svelte';
	import { useQuery } from '@sveltestack/svelte-query';
	import Product from '$lib/components/Product.svelte';

	type ResultBase = {
		status: string;
		count: number;
		totalPages: number;
		totalItems: number;
	};

	type iResult = ResultBase & {
		data: iProduct[];
	};

	interface Props {
		value: string;
    showHpp?: boolean,
		onselect: (p: iProduct, q: number) => void;
		onclear: () => void;
	}

	const initResult: iResult = {
		status: 'page loading',
		count: 0,
		totalPages: 0,
		totalItems: 0,
		data: []
	};

	const url = `${baseURL}/products`;

	async function fetchData(txt: string): Promise<iResult> {
		if (browser) {
			const options = {
				headers: {
					'content-type': 'application/json'
				},
				method: 'GET',
				credentials: credential_include
			};

			const request = new Request(`${url}?opt=1&txt=${txt}`, options);
			let result = await fetch(request);
			return (await result.json()) as iResult;
		}
		return Promise.resolve(initResult);
	}

	let { value = $bindable(''), onselect, onclear, showHpp = false }: Props = $props();

	const query = $derived.by(() => {
		return useQuery<iResult, Error>({
			queryKey: ['products', value],
			queryFn: async () => await fetchData(value),
			enabled: browser
		});
	});

	let products: iProduct[] = $derived.by(() => {
		return $query.isSuccess && $query.data ? $query.data.data : [];
	});

	let expanded = $state(true);
</script>

<div style="margin-top: 24px">
	<Search
		value={value}
		expandable
		bind:expanded={expanded}
		size="sm"
		on:clear={onclear}
		onchange={(e) => {
			const el = e.currentTarget as HTMLInputElement;
			if (el) {
				value = el.value;
			}
		}}
	/>
</div>

<Grid>
	{#each products as p}
		<Product product={p} onadd={(p, q) => onselect(p, q)} {showHpp} />
	{/each}
</Grid>
