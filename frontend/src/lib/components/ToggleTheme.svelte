<script lang="ts">
	import { Theme } from 'carbon-components-svelte';
	import type { CarbonTheme } from 'carbon-components-svelte/types/Theme/Theme.svelte';
	let theme: CarbonTheme = 'g80';
</script>

<Theme
	bind:theme={theme}
	persist
	persistKey="__carbon-theme"
	render="toggle"
	toggle={{
		themes: ['white', 'g90'],
		labelA: 'Dark',
		labelB: 'Light',
		hideLabel: true,
		size: 'sm'
	}}
/>
