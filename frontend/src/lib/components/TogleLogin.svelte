<script lang="ts">
	import { goto } from "$app/navigation";
	import { baseURL, credential_include, type iCurrentUser } from "$lib/interfaces";
	import { HeaderPanelLink, LocalStorage } from "carbon-components-svelte";

	export let isOpen = true;

	let profile: iCurrentUser = {
		id: "",
		name: "",
		email: "",
		photo: "",
		role: "",
		verified: false,
		updatedAt: "",
		createdAt: "",
	};

	const url = `${baseURL}/auth/logout`;
	let storage: LocalStorage;

	async function log_out() {
		isOpen = false;
		const options = {
			headers: { accept: "application/json" },
			method: "GET",
			credentials: credential_include,
		};

		const request = new Request(url, options);
		let result = await fetch(request);

		if (result.ok) {
			storage.clearItem();
			await goto("/", { replaceState: true });
		}
	}
</script>

<LocalStorage key="__user_info" bind:this={storage} bind:value={profile} />

{#if profile.id != ""}
	<HeaderPanelLink on:click={() => log_out()}
		>Logout {profile.name}!</HeaderPanelLink
	>
{:else}
	<HeaderPanelLink on:click={() => (isOpen = false)} href="/login"
		>Login</HeaderPanelLink
	>
{/if}
