<script lang="ts">
	import LabelProperty from '$lib/components/LabelProperty.svelte';
	import {
		baseURL,
		credential_include,
		type iCurrentUser
	} from '$lib/interfaces';
	import { LocalStorage } from 'carbon-components-svelte';
	import { onMount } from 'svelte';

	type iResult = {
		status: string;
		data: iCurrentUser;
	};

	const url = `${baseURL}/users/me`;

	let profile: iCurrentUser = {
		id: '',
		name: '',
		email: '',
		photo: '',
		role: '',
		verified: false,
		updatedAt: '',
		createdAt: ''
	};

	async function load_profile() {
		const options = {
			headers: {
				'content-type': 'application/json',
				accept: 'application/json'
			},
			method: 'GET',
			credentials: credential_include
		};

		const request = new Request(url, options);
		const result = await fetch(request);

		if (result.ok) {
			const res = (await result.json()) as iResult;
			profile = res.data;
		}
	}

	onMount(async () => {
		load_profile();
	});
</script>

<svelte:head>
	<title>Profile</title>
	<meta name="description" content="Profile this app" />
</svelte:head>

<LocalStorage key="__user_info" bind:value={profile} />

<h1>Profile</h1>

<LabelProperty>
	<span slot="label">ID:</span>
	<span slot="value">{profile.id}</span>
</LabelProperty><LabelProperty>
	<span slot="label">Nama:</span>
	<span slot="value">{profile.name}</span>
</LabelProperty>
<LabelProperty>
	<span slot="label">email:</span>
	<span slot="value"><strong>{profile.email}</strong></span>
</LabelProperty><LabelProperty>
	<span slot="label">Verified:</span>
	<span slot="value">{profile.verified}</span>
</LabelProperty><LabelProperty>
	<span slot="label">Role:</span>
	<span slot="value">{profile.role}</span>
</LabelProperty><LabelProperty>
	<span slot="label">Registered at:</span>
	<span slot="value">{profile.createdAt}</span>
</LabelProperty>
