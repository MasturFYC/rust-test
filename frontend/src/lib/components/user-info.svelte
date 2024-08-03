<script lang="ts">
	import {
		baseURL,
		credential_include,
		type iCurrentUser,
	} from "$lib/interfaces";
	import { HeaderPanelLink, LocalStorage } from "carbon-components-svelte";
	import { onMount } from "svelte";

	type iResult = {
		status: string;
		data: iCurrentUser;
	};
	export let isOpen = true;
	let storage: LocalStorage;
	const url = `${baseURL}/users/me`;

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

	async function load_profile() {
		const options = {
			headers: {accept: "application/json"},
			method: "GET",
			credentials: credential_include,
		};

		const request = new Request(url, options);

		await fetch(request)
			.then(async (response) => response)
			.then(async (data) => {
				const result = await data.json();
				profile = result.data as iCurrentUser;
			})
			.catch((e) => {
				console.log("Error:", e)
				storage.clearItem();
			});
	}

	onMount(async () => {
		load_profile();
	});
</script>

<LocalStorage key="__user_info" bind:value={profile} bind:this={storage} />

{#if profile.id != ""}
	<HeaderPanelLink on:click={() => (isOpen = false)} href="/profile"
		>Profile {profile.name}</HeaderPanelLink
	>
{:else}
	<HeaderPanelLink on:click={() => (isOpen = false)}
		>Please login</HeaderPanelLink
	>
{/if}
