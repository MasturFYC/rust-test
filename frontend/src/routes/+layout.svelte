<script lang="ts">
//@ts-nocheck
//import "../index.scss";

	import {
		QueryClient,
		QueryClientProvider,
	} from "@sveltestack/svelte-query";

	import { page } from "$app/stores";
	import ToggleTheme from "$lib/components/ToggleTheme.svelte";
	import TogleLogin from "$lib/components/TogleLogin.svelte";
	import UserInfo from "$lib/components/user-info.svelte";
	import {
		Column,
		Content,
		Grid,
		Header,
		HeaderAction,
		HeaderPanelDivider,
		HeaderPanelLink,
		HeaderPanelLinks,
		HeaderUtilities,
		Row,
		SideNav,
		SideNavItems,
		SideNavLink,
		SideNavMenu,
		SideNavMenuItem,
		SkipToContent,
		Theme,
	} from "carbon-components-svelte";
	import "carbon-components-svelte/css/all.css";
	import {
		Category,
		Dashboard,
		Autoscaling as Next,
		Product,
		GroupAccount as Relation,
	} from "carbon-icons-svelte";
	import { expoIn } from "svelte/easing";

	//	export const ssr = false;
	//	export const prerender = false;

	let isSideNavOpen = false;
	let isOpen = false;
	let selected = "0";
	let transitions = {
		"0": {
			text: "Default (duration: 200ms)",
			value: { duration: 200 },
		},
		"1": {
			text: "Custom (duration: 600ms, delay: 50ms, easing: expoIn)",
			value: { duration: 600, delay: 50, easing: expoIn },
		},
		"2": {
			text: "Disabled",
			value: false,
		},
	};
	let client_width = 0;
	const staticUrl = import.meta.env.VITE_API_STATICURL as string;

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				refetchOnWindowFocus: false,
				// refetchOnMount: false,
				// refetchOnReconnect: false,
				retry: false,
				//        staleTime: 5 * 60 * 1000
			},
		},
	});

	let selected_side = 1;
	let sideLink = [
		{
			id: 1,
			href: "/",
			title: "Dashboard",
			icon: Dashboard,
		},
		{
			id: 2,
			href: "/relation",
			title: "Relasi",
			icon: Relation,
		},
		{
			id: 3,
			href: "/category",
			title: "Kategori barang",
			icon: Category,
		},
		{
			id: 4,
			href: "/product",
			title: "Data barang",
			icon: Product,
		},
	];
</script>

<Theme persist persistKey="__carbon-theme" />

<svelte:window bind:innerWidth={client_width} />

<Header bind:isSideNavOpen>
	<span slot="company"
		><img
			src="https://static.sapulidi.site/pixel.svg?raw=true"
			height="32"
			alt="Logo"
		/></span
	>
	<svelte:fragment slot="skip-to-content">
		<SkipToContent />
	</svelte:fragment>
	<HeaderUtilities>
		<HeaderAction bind:isOpen transition={transitions[selected].value}>
			<HeaderPanelLinks>
				<HeaderPanelDivider><ToggleTheme /></HeaderPanelDivider>
				<TogleLogin bind:isOpen />
				<UserInfo bind:isOpen />
				<HeaderPanelDivider>Switcher subject 2</HeaderPanelDivider>
				<HeaderPanelLink>Switcher item 1</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 2</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 3</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 4</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 5</HeaderPanelLink>
			</HeaderPanelLinks>
		</HeaderAction>
	</HeaderUtilities>
</Header>
<SideNav
	bind:isOpen={isSideNavOpen}
	rail={client_width >= 720 && client_width <= 1080}
	expansionBreakpoint={720}
	fixed
>
	<SideNavItems>
		{#each sideLink as side}
			<SideNavLink
				icon={side.icon}
				href={side.href}
				text={side.title}
				isSelected={$page.url.pathname == side.href}
				on:click={() => (selected_side = side.id)}
			/>
		{/each}
		<SideNavMenu icon={Next} text="Master">
			<SideNavMenuItem href="/relation" text="Relasi" />
			<SideNavMenuItem href="/category" text="Kategori Barang" />
			<SideNavMenuItem href="/product" text="Data Barang" />
		</SideNavMenu>
	</SideNavItems>
</SideNav>

<Content>
	<Grid condensed={client_width <= 640} noGutter={client_width <= 640}>
		<Row>
			<Column>
				<QueryClientProvider client={queryClient}>
					<slot />
				</QueryClientProvider>
			</Column>
		</Row>
	</Grid>
</Content>
