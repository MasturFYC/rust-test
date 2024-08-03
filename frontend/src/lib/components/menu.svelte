<!-- <div class="mx-5">
	<div>
		<a href="/undangan-tahlil">Undangan Tahlil</a>
	</div>
	<div class="mt-2">
		<a href="/bilangan">Bilangan</a>
	</div>
	<div class="mt-2">
		<a href="/ketua-dprd">Ketua DPRD Indramayu</a>
	</div>
	<div class="mt-2">
		<p><a href="/rekamedia">Rekamedis</a></p>
	</div>
	<div class="mt-2">
		<a href="/label103">Label 103</a>
	</div>
	<div class="mt-2">
		<a href="/mdta">MDTA</a>
	</div>
	<div class="subtitle mb-2 mt-4">ID Card</div>
	<div class="ml-4 mt-2">
		<p><a href="/cgr">Cipto Gudang Rabat (CGR)</a></p>
	</div>
	<div class="ml-4 mt-2">
		<p><a href="/cipto">Toko Cipto</a></p>
	</div>
	<div class="ml-4 mt-2">
		<p><a href="/musatani">Musa Tani</a></p>
	</div>
	<div class="ml-4 mt-2">
		<p><a href="/cpo">CPO Fitness</a></p>
	</div>
	<div class="ml-4 mt-2">
		<a href="/pdam">Kartu PDAM</a>
	</div>
</div> -->

<script lang="ts">

	let currentMenu = 0;
	const generalMenu = [
		// {id: 1, name: "Undangan Tahlil", href: "/undangan-tahlil", isActive: 'true'},
		{id: 2, name: "Bilangan", href: "/bilangan", isActive: ''},
		// {id: 3, name: "Ketua DPRD Indramayu", href: "/ketua-dprd", isActive: ''},
		// {id: 4, name: "Pixel Logo", href: "/logo", isActive: ''},
		 {id: 5, name: "Label-103", href: "/label103", isActive: ''},
		{id: 6, name: "MDATA", href: "/mdta", isActive: ''},
		{id: 16, name: "Rekap MDTA", href: "/rekap-mdta", isActive: ''},
	]
	const cardMenu = [
		{id: 11, name: "PDAM", href: "/pdam", isActive: ''},
		{id: 7, name: "CGR", href: "/cgr", isActive: 'true'},
		{id: 8, name: "Toko Cipto", href: "/cipto", isActive: ''},
		{id: 10, name: "CPO Fitness", href: "/cpo", isActive: ''},
		{id: 9, name: "Musa Tani", href: "/musatani", isActive: ''},
	//	{id: 12, name: "OSIS SMA Tukdana", href: "/osis", isActive: ''},
		{id: 15, name: "Toko Sumiyati", href: "/toko-sumiyati", isActive: ''},
		{id: 14, name: "Toko Luna Jaya", href: "/toko-luna", isActive: ''},
		{id: 17, name: "RS Bhayangkara", href: "/rumkit", isActive: ''},
		{id: 13, name: "Raos", href: "/raos", isActive: ''},
	]</script>
<aside class="menu mx-5">
	<p class="menu-label mt-5">General</p>
	<ul class="menu-list">
		{#each generalMenu as m }
			<li><a href="{m.href}" class="{m.id === currentMenu ? 'is-active':''}" on:click={() => currentMenu = m.id}>{m.name}</a></li>
		{/each}
	</ul>
	<p class="menu-label">ID Card</p>
	<ul class="menu-list">
		{#each cardMenu as m }
			<li><a href="{m.href}" class="{m.id === currentMenu ? 'is-active':''}" on:click={() => currentMenu = m.id}>{m.name}</a></li>
		{/each}

	</ul>
	<!-- <ul class="menu-list">
		<li><a>Team Settings</a></li>
		<li>
			<a class="">Manage Your Team</a>
			<ul>
				<li><a>Members</a></li>
				<li><a>Plugins</a></li>
				<li><a>Add a member</a></li>
			</ul>
		</li>
		<li><a>Invitations</a></li>
		<li><a>Cloud Storage Environment Settings</a></li>
		<li><a>Authentication</a></li>
	</ul> -->
	<!-- <p class="menu-label">Transactions</p>
	<ul class="menu-list">
		<li><a>Payments</a></li>
		<li><a>Transfers</a></li>
		<li><a>Balance</a></li>
	</ul> -->
</aside>
