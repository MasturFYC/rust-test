<script lang="ts">
	import {
		baseURL,
		credential_include,
		type iCurrentUser
	} from '$lib/interfaces';
	import { HeaderPanelLink, LocalStorage } from 'carbon-components-svelte';
	import { isLogginIn } from '$lib/store';
	import { goto } from '$app/navigation';

	// type iResult = {
	// 	status: string;
	// 	data: iCurrentUser;
	// };

	let { closePanel }: { closePanel: (e: boolean) => void } = $props();
	let storage: LocalStorage;
	const url = `${baseURL}/users/me`;

	let profile: iCurrentUser = $state({
		id: '',
		name: '',
		email: '',
		photo: '',
		role: '',
		verified: false,
		updatedAt: '',
		createdAt: ''
	});

	async function load_profile() {
		const options = {
			headers: { accept: 'application/json' },
			method: 'GET',
			credentials: credential_include
		};

		const request = new Request(url, options);

		await fetch(request)
			.then(async (response) => response)
			.then(async (data) => {
				const result = await data.json();
				profile = result.data as iCurrentUser;
				isLogginIn.update(() => true);
			})
			.catch((e) => {
				console.log('Error:', e);
				storage.clearItem();
			});
	}

	$effect(() => {
		if ($isLogginIn) {
			load_profile();
		}
	});

	async function log_out() {
		const options = {
			headers: { accept: 'application/json' },
			method: 'GET',
			credentials: credential_include
		};

		const logouturl = `${baseURL}/auth/logout`;

		const request = new Request(logouturl, options);
		let result = await fetch(request);

		if (result.ok) {
			isLogginIn.update(() => false);
			storage.clearItem();
			closePanel(false);
			document.execCommand('ClearAuthenticationCache');
			await goto('/', { replaceState: true });
		}
	}
</script>

<LocalStorage key="__user_info" bind:value={profile} bind:this={storage} />

{#if $isLogginIn}
	<HeaderPanelLink on:click={() => log_out()}
		>Logout {profile.name}!</HeaderPanelLink
	>

	<HeaderPanelLink on:click={() => closePanel(false)} href="/profile"
		>Profile {profile.name}</HeaderPanelLink
	>
{:else}
	<HeaderPanelLink on:click={() => closePanel(false)} href="/login"
		>Login</HeaderPanelLink
	>
	<HeaderPanelLink on:click={() => closePanel(false)}
		>Please login</HeaderPanelLink
	>
{/if}
