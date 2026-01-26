<style lang="scss">
	.div-form {
		max-width: 320px;
		width: 100%;
		margin: 0 auto;
	}
	p {
		margin-top: 16px;
		margin-bottom: 6px;
	}
</style>

<script lang="ts">
	import { goto } from '$app/navigation';
  import { baseURL, credential_include, type iUserLogin } from '$lib/interfaces';
	import { Button, PasswordInput, TextInput } from 'carbon-components-svelte';
	import { Login } from 'carbon-icons-svelte';
	import { isLogginIn } from '$lib/store';
	// import ProductInfo from '../stock/ProductInfo.svelte';

	const url = `${baseURL}/auth/login`;

	let user: iUserLogin = $state({ email: '', password: '' });
	let isLoading = $state(false);
	let message = $state('');

	const submit = async (e: SubmitEvent) => {
		e.preventDefault();

		isLoading = true;
		const options = {
			headers: {
				'content-type': 'application/json'
			},
			method: 'POST',
			credentials: credential_include,
			body: JSON.stringify(user)
		};

		const request = new Request(url, options);
		const result = await fetch(request);

		if (result.ok) {
			isLogginIn.update(() => true);
			await goto('/', { replaceState: true });
		}
	};
</script>

<div class="div-form">
	<h1>Login</h1>
	<p>Sign in to have access</p>
	<form onsubmit={submit}>
		<TextInput
			autocomplete="email"
			type="email"
			bind:value={user.email}
			placeholder="email"
			disabled={isLoading}
		/>
		<br />
		<PasswordInput
			disabled={isLoading}
			autocomplete="current-password"
			bind:value={user.password}
			placeholder="Enter password"
		/>
		<br />
		<Button type="submit" icon={Login} skeleton={isLoading}>Login</Button>
	</form>
	<div class="has-text-danger mt-4 block">{message}</div>
</div>
