<script lang="ts">
  import Address from "$lib/components/Address.svelte";
  import type { iRelation, RelationTypeWIthID } from "$lib/interfaces";
  import {
    Button,
    ComboBox,
    DataTable,
    Toolbar,
    ToolbarContent,
    ToolbarSearch,
  } from "carbon-components-svelte";
  import { Edit, NewTab } from "carbon-icons-svelte";
  import { createEventDispatcher } from "svelte";
  import DeleteRelation from "./DeleteRelation.svelte";

  const dispatch = createEventDispatcher();

  export let data: iRelation[] = [];
  export let isUpdating = false;
  export let relationTypes: RelationTypeWIthID[] = [];

  let headers = [
    // { key: "id", value: "#ID", width: "10%" },
    { key: "name", value: "Nama", width: "25%" },
    { key: "street", value: "Alamat", width: "auto" },
    { key: "region", value: "Rayon", width: "80px" },
    { key: "relationType", value: "Tipe", width: "100px" },
    { key: "cmd", value: "", width: "120px" },
  ];

  function edit_relation(id: number) {
    dispatch("edit", id);
  }

  let txt: string | undefined = undefined;
</script>

<DataTable zebra size="short" {headers} rows={data}>
  <svelte:fragment slot="cell" let:row let:cell>
    {#if cell.key === "cmd"}
      <Button
        tooltipPosition="left"
        tooltipAlignment="end"
        size="small"
        kind="ghost"
        iconDescription="Edit"
        icon={Edit}
        on:click={() => edit_relation(row.id)}
      />
      <DeleteRelation idData={row.id} on:deleteData isDeleting={isUpdating} />
    {:else if cell.key === "street"}
      <Address street={row["street"]} city={row["city"]} phone={row["phone"]} />
    {:else if cell.key === "relationType"}
      {cell.value.join(", ")}
    {:else if cell.key === "region"}
      {cell.value ?? ""}
    {:else}
      {cell.value}
    {/if}
  </svelte:fragment>

  <Toolbar size="sm">
    <ToolbarContent>
      <ToolbarSearch
        on:change={() => dispatch("change_search", txt)}
        bind:value={txt}
        on:clear={() => dispatch("change_search", null)}
        placeholder={"nama, alamat, kota, no. telp"}
      />
      <!-- <ToolbarMenu>
				<ToolbarMenuItem primaryFocus>Restart all</ToolbarMenuItem>
				<ToolbarMenuItem href="https://cloud.ibm.com/docs/loadbalancer-service"
					>API documentation</ToolbarMenuItem
				>
				<ToolbarMenuItem hasDivider danger>Stop all</ToolbarMenuItem>
			</ToolbarMenu> -->
      <ComboBox
        class={"relation"}
        size="sm"
        autocomplete="off"
        type="inline"
        placeholder="Tipe relasi"
        items={relationTypes}
        on:select={(e) => dispatch("changeType", e.detail.selectedId)}
        on:clear={() => dispatch("changeType", null)}
      />
      <Button on:click={() => edit_relation(0)} icon={NewTab}>Buat baru</Button>
    </ToolbarContent>
  </Toolbar>
</DataTable>

<style lang="scss">
  // .cell-right {
  // 	text-align: right;
  // }
  // .code-pre {
  // 	font-size: small;
  // }
  /*
		 :global(table.bx--data-table th, table.bx--data-table--zebra) {
		background-color: #000;
		 }
		 */

  :global(.bx--list-box__field .bx--text-input.relation) {
    border-bottom: none;
    // background-color: var(--cds-link-01);
    // color: var(--cds-ui-01);
  }
  // :global(.bx--table-expand__button) {
  // 	width: auto;
  // 	min-height: 16px;
  // }
</style>
