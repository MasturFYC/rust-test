<script lang="ts">
import { useIsFetching } from '@sveltestack/svelte-query';
import { createEventDispatcher } from 'svelte';
import Icon from './Icon.svelte';

const isFetching = useIsFetching();
const dispatch = createEventDispatcher();

export let size = 10;
export let page = 0;
export let totalPages = 0;
export let totalElements = 0;

let initSize = size;
export let rows = [3, 5, 10, 25, 50, 100, 250, 500, 1000];

$: if (initSize != size) {
	initSize = size;
	dispatch('changeSize', size);
}

$: isLoading = $isFetching > 0;
</script>

<!--
	<nav class="pagination is-mall" role="navigation" aria-label="pagination">
		<a class="pagination-previous">
			<span class="icon is-small"
							><i class="fa-solid fa-chevron-left"></i></span
						>
		</a>
		<a class="pagination-next">
			<span class="icon is-small"
							><i class="fa-solid fa-chevron-right"></i></span
						>
		</a>
		<ul class="pagination-list">
			<li><span class="pagination-ellipsis">&hellip;</span></li>
			{#each Array(totalPages) as _, i}
			<li><a class="pagination-link {page === i ? 'is-link':'is-light'}" aria-label="Goto page {i}" on:click={() => dispatch("setPage", i)}>{i + 1}</a></li>
			{/each}
			<li><span class="pagination-ellipsis">&hellip;</span></li>
		</ul>
	</nav> -->

<div class="columns is-mobile">
	<div class="column is-3">
		<div class="columns">
			<div class="column is-narrow">
				<div class="select is-size-7">
					<select bind:value={size}>
						{#each rows as c}
							<option value={c}>{c}</option>
						{/each}
					</select>
				</div>
			</div>
			<div class="column is-narrow is-size-7">
				{isLoading ? 'Fetching...' : ''}
			</div>
		</div>
	</div>
	<!-- <div class="column tags is-hidden-mobile">
			{#each Array(totalPages) as _, i} -->
	<!-- svelte-ignore a11y-missing-attribute -->
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<!-- <a class="tag {page === i ? 'is-link':'is-light'}" on:click={() => dispatch("setPage", i)}>{i + 1}</a>				 -->
	<!-- {/each}
		</div> -->
	<div class="column"></div>
	<div class="column is-narrow">
		<div class="columns is-gapless is-mobile">
			<div class="column is-narrow">
				<button
					title="first page"
					disabled={page === 0}
					class="button is-size-7"
					on:click={() => dispatch('setPage', 0)}
				>
					<Icon iconName="chevrons-left" />
				</button>
			</div>
			<div class="column is-narrow mx-2">
				<button
					title="previous"
					disabled={page === 0}
					class="button is-size-7"
					on:click={() => dispatch('setPage', Math.max(page - 1, 0))}
				>
					<Icon iconName="chevron-left" />
				</button>
			</div>
			<div class="column is-size-7 has-text-centered">
				<div class="py-1 has-text-weight-bold mx-4">
					{page + 1} / {totalPages} of ({totalElements})
				</div>
			</div>
			<div class="column is-narrow mx-2">
				<button
					title="next"
					disabled={page === totalPages - 1 || totalPages === 0}
					class="button is-size-7"
					on:click={() => dispatch('setPage', page + 1)}
				>
					<Icon iconName="chevron-right" />
				</button>
				<!-- Math.max(page < (totalPages-1) ? page + 1 : page)) -->
			</div>
			<div class="column is-narrow">
				<button
					title="last page"
					disabled={page === (totalPages === 0 ? 1 : totalPages) - 1}
					class="button is-size-7"
					on:click={() => dispatch('setPage', totalPages - 1)}
				>
					<Icon iconName="chevrons-right" size={16} />
				</button>
				<!-- Math.max(page < (totalPages-1) ? page + 1 : page)) -->
			</div>
		</div>
	</div>
</div>
