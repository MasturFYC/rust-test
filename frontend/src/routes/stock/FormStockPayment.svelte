<script lang="ts">
	import {
		Column,
		FluidForm,
		Grid,
		Modal,
		Row
	} from 'carbon-components-svelte';
	import { Save } from 'carbon-icons-svelte';
	// import { createEventDispatcher, onMount } from "svelte";

	import { formatNumber, getNumber } from '$lib/components/NumberFormat';
	import InputNumber from '$lib/components/NumberInput.svelte';
	import { tick } from 'svelte';
	import { toNumber } from './handler';

	interface Props {
		total: number;
		dp: number;
		payment: number;
		open: boolean;
		onupdate: (e: number) => void;
	}

	let {
		open = $bindable(false),
		total = 0,
		dp = 0,
		payment = 0,
		onupdate
	}: Props = $props();
	let localDp = $state(dp);

	let strRemain = $derived.by(() => {
		return formatNumber(
			toNumber(total) - (toNumber(payment) + toNumber(localDp))
		);
	});

	async function submit() {
		onupdate(localDp);
		await tick();
		open = false;
	}

	function on_dp_change(e: CustomEvent<string | number | null>): void {
		if (typeof e.detail === 'string') {
			localDp = getNumber(e.detail);
		}
	}
</script>

<Modal
	bind:open={open}
	hasForm
	alert
	size="sm"
	modalHeading={'Pembayaran'}
	preventCloseOnClickOutside
	primaryButtonText="OK"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={'#stock-dp'}
	on:click:button--secondary={() => (open = false)}
	on:click:button--primary={submit}
>
	<!--   primaryButtonDisabled={isUpdating} -->
	<FluidForm>
		<Grid noGutter>
			<Row>
				<Column sm md noGutterRight>
					<InputNumber
						value={formatNumber(total)}
						labelText="Total"
						size="sm"
						classes={'align-left'}
						readonly
					/>
					<InputNumber
						value={formatNumber(localDp)}
						labelText="Jumlah bayar"
						id="stock-dp"
						size="sm"
						classes={'align-left'}
						on:input={on_dp_change}
					/>
				</Column>
				<Column noGutterLeft>
					<InputNumber
						value={formatNumber(payment)}
						labelText="Angsuran"
						readonly
						classes={'align-left'}
						size="sm"
					/>
					<InputNumber
						value={strRemain}
						labelText="Sisa bayar"
						readonly
						classes={'align-left'}
						size="sm"
					/>
				</Column>
			</Row>
		</Grid>
		<!-- <ComboBox
            size="sm"
            titleText="Kategori"
            placeholder="Pilih kategori"
            selectedId={data.categoryId}
            warn={category_invalid}
            items={categories}
            {shouldFilterItem}
            on:select={(e) => (data.categoryId = e.detail.selectedId)}
            on:clear={() => (data.categoryId = 0)}
          /> -->
	</FluidForm>

	<!-- {#if isUpdating}
    <InlineLoading
      status="active"
      description={data.id === 0 ? "Posting data..." : "Updating data..."}
    />
  {/if} -->

	<!-- {#if isError}
    <InlineLoading description={errorMessage} status="error" />
  {/if} -->
</Modal>
