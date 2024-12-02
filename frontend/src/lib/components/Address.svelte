<script lang="ts">
	export let street = '';
	export let city: string;
	export let phone = '';
</script>

<span
	>{#if street !== ''}{street} -
	{/if}{city}{#if phone !== ''}, {phone}{/if}</span
>
