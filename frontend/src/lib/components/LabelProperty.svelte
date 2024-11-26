<style lang="scss">
.sm {
	font-size: 95% !important;
}
.medium {
	font-size: 100% !important;
}
.columns {
	display: flex;
	flex-direction: row;
	align-items: first baseline;
	// margin-bottom: 10px;
	// margin-top: 16px;
}
.column-label::after {
	content: ':';
}
.column-label {
	flex: 5;
	text-align: right;
	margin-right: 16px;
	display: flexbox;
}
.column-value {
	flex: 7;
	display: inline-flex;
	margin-right: 16px;
	border-bottom: 1px solid var(--cds-ui-03);
	align-items: center;
	height: 26px;
}
.right {
	justify-content: end;
}
</style>

<script lang="ts">
export let right = false;
export let medium = false;
export let sm = false;
</script>

<div class="columns" class:sm={sm} class:medium={medium}>
	<div class="column-label"><slot name="label" /></div>
	<div class="column-value" class:right={right}><slot name="value" /></div>
</div>
