<script lang="ts">
	import { goto } from "$app/navigation";
	import { credential_include, type iUserLogin } from "$lib/interfaces";
	import { Button, PasswordInput, TextInput } from "carbon-components-svelte";
	import { Login } from "carbon-icons-svelte";

	const baseURL = import.meta.env.VITE_API_URL;
	const url = `${baseURL}/auth/login`;

	let user: iUserLogin = { email: "", password: "" };
	let isLoading = false;
	let message = "";


	const submit = async () => {
		isLoading = true;
		const options = {
			headers: {
				"content-type": "application/json"
			},
			method: "POST",
			credentials: credential_include,
			body: JSON.stringify(user)
		}

		const request = new Request(url, options);
		const result = await fetch(request);
		
		if(result.ok) {
			await goto ("/", {replaceState: true})
		}

	};
</script>

<div class="div-form">
	<h1>Login</h1>
	<p>Sign in to have access</p>
	<form on:submit|preventDefault={submit}>
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
