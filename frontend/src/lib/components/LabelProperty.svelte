<div class="columns">
	<div class="column-label"><slot name="label" /></div>
	<div class="column-value"><slot name="value" /></div>
</div>

<style lang="scss">
	.columns {
		display: flex;
		flex-direction: row;
		margin-bottom: 10px;
		margin-top: 16px;
	}
	.column-label {
		flex: 3;
		text-align: right;
		margin-right: 10px;
		display: flexbox;
	}
	.column-value {
		flex: 9;
		text-align: left;
		display: inline-flex;
		margin-right: 10px;
	}
</style>

