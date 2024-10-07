<script lang="ts">
import { formatNumber } from '$lib/components/NumberFormat';
import type {iProductStock} from '$lib/interfaces';
export let stocks: iProductStock[];
export let unit: string;
</script>

{#each stocks as s}
<span>{#if s.gudangId === 1}<strong>{s.name}</strong>{:else}{s.name}{/if}:
		<strong>{formatNumber(s.qty)}</strong> {unit}</span><br />
{/each}
