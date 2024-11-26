<script lang="ts">
export let showModal = false;
</script>

<div class="modal" class:is-active={showModal}>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div class="modal-background" on:click={() => (showModal = false)}></div>
	<div class="modal-card">
		<header class="modal-card-head">
			<p class="modal-card-title subtitle mb-0"><slot name="header" /></p>
			<button class="delete" aria-label="close" on:click={() => (showModal = false)}></button>
		</header>
		<section class="modal-card-body">
			<slot name="content" />
		</section>
		<footer class="modal-card-foot">
			<slot name="footer" />
		</footer>
	</div>
</div>
