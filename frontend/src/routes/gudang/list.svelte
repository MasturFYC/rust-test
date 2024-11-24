<style lang="css">
:global(#menu-gud-ware.bx--list-box__menu) {
  position: fixed;
  z-index: 1;
  margin-left: 17px;
  margin-right: 17px;
  width: auto;
}
:global(#gud-ware) {
  margin-top: 6px;
  margin-bottom: 6px;
}
</style>

<script lang="ts">
import { type iGudang } from "$lib/interfaces";
import {
  Button,
  DataTable,
  Toolbar,
  ToolbarContent,
  ToolbarMenu,
  ToolbarMenuItem,
} from "carbon-components-svelte";
import { Edit, NewTab } from "carbon-icons-svelte";
import DeleteGudang from "./DeleteGudang.svelte";
import { createEventDispatcher } from "svelte";

export let gudangs: iGudang[] = [];
const dispatch = createEventDispatcher();
// let selectedRowIds = [gudangs.length > 0 ? gudangs[0].id : 0];
let client_width = 0;

function edit_gudang(id: number) {
  dispatch("edit", id);
}

let headers = [
  { key: "id", value: "#ID", width: "10%" },
  { key: "name", value: "Nama", width: "auto" },
  { key: "employeeName", value: "Penjaga Gudang", width: "auto" },
  { key: "locate", value: "Lokasi Gudang", width: "auto" },
  { key: "cmd", value: "", width: "150px" },
];

function new_gudang() {
  const newData: iGudang = {
    id: 0,
    name: "",
    employeeId: 0,
    employeeName: "",
  };
  dispatch("newGudang", newData);
}
</script>

<svelte:window bind:innerWidth={client_width} />
<DataTable zebra headers={headers} rows={gudangs}>
  <svelte:fragment slot="cell" let:row let:cell>
    {#if cell.key === "cmd"}
      <Button
        tooltipPosition="left"
        tooltipAlignment="end"
        size="small"
        kind="ghost"
        iconDescription="Edit"
        icon={Edit}
        on:click={() => edit_gudang(row.id)}
      />
      <DeleteGudang gudangId={row.id} on:deleteGudang />
    {:else if cell.key === "locate"}
      {cell.value ?? "-"}
    {:else}
      {cell.value}
    {/if}
  </svelte:fragment>

  <Toolbar size="sm">
    <ToolbarContent>
      <!-- <ToolbarSearch /> -->
      <ToolbarMenu>
        <ToolbarMenuItem primaryFocus>Restart all</ToolbarMenuItem>
        <ToolbarMenuItem href="https://cloud.ibm.com/docs/loadbalancer-service"
          >API documentation</ToolbarMenuItem
        >
        <ToolbarMenuItem hasDivider danger>Stop all</ToolbarMenuItem>
      </ToolbarMenu>
      <Button on:click={new_gudang} icon={NewTab}>Buat baru</Button>
    </ToolbarContent>
  </Toolbar>
</DataTable>
