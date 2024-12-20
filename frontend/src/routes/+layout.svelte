<style global>
	.bx--side-nav {
		background-color: var(--cds-ui-background);
		color: var(--cds-text-02);
		border-right: 1px solid var(--cds-ui-02);
	}
	.bx--side-nav__divider {
		background-color: var(--cds-ui-03);
	}
	a.bx--side-nav__link > .bx--side-nav__link-text {
		color: var(--cds-text-02);
	}
	.bx--side-nav__submenu {
		color: var(--cds-text-02);
	}
	.bx--side-nav__menu a.bx--side-nav__link--current,
	.bx--side-nav__menu a.bx--side-nav__link[aria-current='page'],
	a.bx--side-nav__link--current {
		background-color: var(--cds-hover-ui);
	}

	.bx--side-nav__menu a.bx--side-nav__link--current > span,
	.bx--side-nav__menu a.bx--side-nav__link[aria-current='page'] > span,
	a.bx--side-nav__link--current > span {
		color: var(--cds-text-01);
	}

	.bx--side-nav__icon > svg {
		fill: var(--cds-text-02);
	}
	a.bx--side-nav__link[aria-current='page'],
	a.bx--side-nav__link--current {
		background-color: var(--cds-hover-ui);
	}
	a.bx--side-nav__link[aria-current='page'] .bx--side-nav__link-text,
	a.bx--side-nav__link--current .bx--side-nav__link-text {
		color: var(--cds-text-01);
	}
	.bx--side-nav__item:not(.bx--side-nav__item--active)
		> .bx--side-nav__link:hover,
	.bx--side-nav__menu
		a.bx--side-nav__link:not(.bx--side-nav__link--current):not(
			[aria-current='page']
		):hover {
		color: var(--cds-text-01);
		background-color: var(--cds-hover-ui);
	}
	.bx--side-nav__item:not(.bx--side-nav__item--active)
		> .bx--side-nav__link:hover
		> span,
	.bx--side-nav__item:not(.bx--side-nav__item--active)
		.bx--side-nav__menu-item
		> .bx--side-nav__link:hover
		> span {
		color: var(--cds-text-01);
	}
	.bx--side-nav__submenu:hover {
		color: var(--cds-text-01);
		background-color: var(--cds-hover-ui);
	}
</style>

<script lang="ts">
	import { Toggle } from 'carbon-components-svelte';
	import { QueryClient, QueryClientProvider } from '@sveltestack/svelte-query';
	import { page } from '$app/stores';
	import ToggleTheme from '$lib/components/ToggleTheme.svelte';
	import UserInfo from '$lib/components/user-info.svelte';
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
		Theme
	} from 'carbon-components-svelte';
	import 'carbon-components-svelte/css/all.css';
	import {
		Category,
		Dashboard,
		Autoscaling as Next,
		OrderDetails,
		Product,
		GroupAccount as Relation,
		SendToBack,
		IbmDb2Warehouse as Warehouse
	} from 'carbon-icons-svelte';
	import { expoIn } from 'svelte/easing';
	import dayjs from 'dayjs';
	import locale from 'dayjs/locale/id';
	import advanced from 'dayjs/plugin/advancedFormat';
	import timezone from 'dayjs/plugin/timezone';
	import utc from 'dayjs/plugin/utc';
	import { isLogginIn } from '$lib/store';
	import { baseURL, credential_include } from '$lib/interfaces';
	import { onMount } from 'svelte';

	dayjs.extend(timezone);
	dayjs.extend(utc);
	dayjs.extend(advanced);
	dayjs.locale(locale);

	const url = `${baseURL}/users/me`;

	// const staticUrl = import.meta.env.VITE_API_STATICURL as string;
	let { children } = $props();

	let isRail = $state(false);
	let client_width = $state(0);
	// let selected_side = $state(1);
	let isSideNavOpen = $state(false);
	let isOpen = $state(false);
	const selected = '0';

	const transitions = {
		'0': {
			text: 'Default (duration: 200ms)',
			value: { duration: 200 }
		},
		'1': {
			text: 'Custom (duration: 600ms, delay: 50ms, easing: expoIn)',
			value: { duration: 600, delay: 50, easing: expoIn }
		},
		'2': {
			text: 'Disabled',
			value: false
		}
	};

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				refetchOnWindowFocus: false,
				// refetchOnMount: false,
				// refetchOnReconnect: false,
				retry: false
				//        staleTime: 5 * 60 * 1000
			}
		}
	});

	const sideLink = [
		{
			id: 1,
			href: '/',
			title: 'Dashboard',
			icon: Dashboard
		},
		{
			id: 2,
			href: '/relation',
			title: 'Relasi',
			icon: Relation
		},
		{
			id: 3,
			href: '/category',
			title: 'Kategori barang',
			icon: Category
		},
		{
			id: 4,
			href: '/gudang',
			title: 'Gudang',
			icon: Warehouse
		},
		{
			id: 5,
			href: '/product',
			title: 'Data barang',
			icon: Product
		},
		{
			id: 6,
			href: '/stock',
			title: 'Stock',
			icon: SendToBack
		},
		{
			id: 7,
			href: '/order',
			title: 'Order',
			icon: OrderDetails
		}
	];

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
				if (result.status === 'success') {
					isLogginIn.update(() => true);
				}
			})
			.catch((e) => {
				console.log('Error:', e);
			});
	}

	onMount(async () => {
		load_profile();
	});
</script>

<Theme persist persistKey="__carbon-theme" />

<svelte:window bind:innerWidth={client_width} />

<Header bind:isSideNavOpen={isSideNavOpen}>
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
		<HeaderAction bind:isOpen={isOpen} transition={transitions[selected].value}>
			<HeaderPanelLinks>
				<HeaderPanelDivider><ToggleTheme /></HeaderPanelDivider>
				<UserInfo closePanel={(e) => (isOpen = e)} />
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

{#if $isLogginIn}
	<SideNav
		bind:isOpen={isSideNavOpen}
		rail={(client_width >= 720 && client_width <= 1080) || isRail}
		expansionBreakpoint={720}
		fixed
	>
		<SideNavItems>
			{#each sideLink as side (side.id)}
				<SideNavLink
					icon={side.icon}
					href={side.href}
					text={side.title}
					isSelected={$page.url.pathname == side.href}
				/>
				<!-- on:click={() => (selected_side = side.id)} -->
			{/each}
			<SideNavMenu icon={Next} text="Master">
				<SideNavMenuItem href="/relation" text="Relasi" />
				<SideNavMenuItem href="/category" text="Kategori Barang" />
				<SideNavMenuItem href="/product" text="Data Barang" />
			</SideNavMenu>
			<Toggle
				labelText="Sidenav"
				size="sm"
				style="margin: 9px;"
				on:toggle={() => (isRail = !isRail)}
			>
				<span slot="labelA" style="color: red">Hide</span>
				<span slot="labelB" style="color: green">Expand</span>
			</Toggle>
		</SideNavItems>
	</SideNav>

	<Content>
		<Grid condensed={client_width <= 640} noGutter={client_width <= 640}>
			<Row>
				<Column>
					<QueryClientProvider client={queryClient}>
						{@render children()}
					</QueryClientProvider>
				</Column>
			</Row>
		</Grid>
	</Content>
{:else}
	<Content>
		<QueryClientProvider client={queryClient}>
			{@render children()}
		</QueryClientProvider>
	</Content>
{/if}
