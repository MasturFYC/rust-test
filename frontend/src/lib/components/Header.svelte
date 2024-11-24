<script lang="ts">
import { goto } from "$app/navigation";
import {
  accessToken,
  currentRole,
  currentUser,
  isLogginIn,
  refreshToken,
  tokenType,
} from "$lib/store";
import { onMount } from "svelte";

const staticUrl = import.meta.env.VITE_API_STATICURL as string;
const bearer = $tokenType + " " + $accessToken;

export let showMeLoginForm = false;

let height;
let isActive = false;

const signin = async () => {
  showMeLoginForm = true;
  isActive = false;
  await goto("/login");
};

const signout = async () => {
  isActive = false;
  isLogginIn.set(false);
  currentRole.set([]);
  accessToken.set("");
  refreshToken.set("");
  tokenType.set("");
  await goto("/");

  // await fetchApi
  // 	.headers({
  // 		Accept: 'application/json'
  // 	})
  // 	.url('/auth/signout')
  // 	// .auth($tokenType + ' ' + $accessToken)
  // 	// .options({ credential: 'include'})
  // 	.post()
  // 	//.unauthorized(renewToken)
  // 	.json(async (json) => {
  // 		console.log(json.message);
  // 		isLogginIn.set(false);
  // 		currentUser.set('');
  // 		currentRole.set([]);
  // 		accessToken.set('');
  // 		tokenType.set('');
  // 		refreshToken.set('');
  // 		await goto('/');
  // 	});
};

const loadCurrentUser = async () => {
  // const result = await fetchApi
  // 	.auth(bearer)
  // 	.get("/auth/current-user")
  // 	.unauthorized(renewToken)
  // 	// .error(401, () => {
  // 	//  	console.log("Unauthorized")
  // 	//  	// throw new Error("Unauthorized")
  // 	// })
  // 	.res(async (res) => {
  // 		return await res.json()
  // 	})
  // 	.then(json => {
  // 		return json as iCurrentUser;
  // 	})
  // 	.catch(err => {
  // 		console.log(err)
  // 		return null;
  // 	})
  // return result;
};

// const loadUser = async () => {
// 	// console.log($refreshToken);

// 	if ($refreshToken) {
// 		fetchApi
// 			// .headers({
// 			// 	Accept: "application/json",
// 			// })
// 			.post({ refresh_token: $refreshToken }, "/auth/refresh")
// 			.json(async (data: iUserLogin) => {
// 				// console.log(data);
// 				isLogginIn.set(true);
// 				//						currentUser.set(data.username);
// 				//						currentRole.set(data.roles);
// 				refreshToken.set(data.refreshToken);
// 				accessToken.set(data.accessToken);
// 				tokenType.set(data.tokenType);
// 			})
// 			.catch((err) => {
// 				console.log(err);
// 			});
// 	}
// };

onMount(async () => {
  const user = await loadCurrentUser();

  // if(user) {
  // 	// isLogginIn.set(true)
  // 	currentRole.set(user.roles)
  // }
  // loadUser();
});
</script>

<!-- svelte-ignore a11y-no-redundant-roles -->
<nav
  class="navbar is-light"
  role="navigation"
  aria-label="main navigation"
  bind:clientHeight={height}
>
  <div class="navbar-brand">
    <a
      href="/#"
      role="button"
      class="navbar-burger{isActive ? ' is-active' : ''}"
      aria-label="menu"
      aria-expanded="false"
      data-target="navbarBasicExample"
      on:click={() => (isActive = !isActive)}
    >
      <span aria-hidden="true" />
      <span aria-hidden="true" />
      <span aria-hidden="true" />
    </a>
  </div>

  <div
    id="navbarBasicExample"
    class="navbar-menu{isActive ? ' is-active' : ''}"
  >
    <div class="navbar-start">
      <!-- <a class="navbar-item" href="/"> Home </a>

				<a class="navbar-item"> Documentation </a> -->

      <!-- <div class="navbar-item has-dropdown is-hoverable{isActive ? ' is-active' : ''}">
					<a class="navbar-link"> More </a>

					<div class="navbar-dropdown">
						<a class="navbar-item" href="/undangan-tahlil"> About </a>
						<a class="navbar-item"> Jobs </a>
						<a class="navbar-item"> Contact </a>
						<hr class="navbar-divider" />
						<a class="navbar-item"> Report an issue </a>
					</div>
				</div> -->
    </div>

    <div class="navbar-end">
      <!-- <div class="navbar-item"> -->
      <!-- <div class="buttons"> -->
      {#if $isLogginIn}
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a class="navbar-item is-warning" href="#" on:click={signout}>
          Sign out {$currentUser}!
        </a>
      {:else}
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a class="navbar-item is-warning" href="#" on:click={signin}>Sign in</a>
      {/if}
    </div>
    <!-- </div> -->
    <!-- </div> -->
  </div>
</nav>
