<script lang="ts">
  import {
    baseURL,
    credential_include,
    type iGudang,
    type iRelationProp,
  } from "$lib/interfaces";
  import { useMutation, useQueryClient } from "@sveltestack/svelte-query";
  import {
    Button,
    DataTable,
    FluidForm,
    InlineLoading,
    Modal,
    TextInput,
    ToastNotification,
    Toolbar,
    ToolbarContent,
    ToolbarMenu,
    ComboBox,
    ToolbarMenuItem,
  } from "carbon-components-svelte";
  import { Edit, NewTab, Save } from "carbon-icons-svelte";
  // import type { ComboBoxItem } from "carbon-components-svelte/types/ComboBox/ComboBox.svelte";
  import DeleteGudang from "./DeleteGudang.svelte";
//  import type { ComboBoxItem } from "carbon-components-svelte/src/ComboBox/ComboBox.svelte";
	import {IbmDb2Warehouse as Warehouse} from 'carbon-icons-svelte';

  export let employees: iRelationProp[] = [];

  const client = useQueryClient();
  let isUpdating = false;
  let isError = false;
  let err_msg = "";

  export let gudangs: iGudang[] = [];

  type iResult = {
    count: number;
    data: iGudang[];
    status: string;
  };

  // let data: iResult = {
  // 	count: 0,
  // 	data: [],
  // 	status: ""
  // };

  const fetchUpdateData = async (e: iGudang): Promise<iGudang> => {
    const url = `${baseURL}/gudangs/${gudang.id}`;
    const request = new Request(url, {
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(gudang),
      method: "PUT",
      credentials: credential_include,
    });

    const result = await fetch(request);
    return await result.json();
  };

  const fetchCreateData = async (e: iGudang): Promise<iGudang> => {
    const url = `${baseURL}/gudangs`;
    const request = new Request(url, {
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(gudang),
      method: "POST",
      credentials: credential_include,
    });

    return await (await fetch(request)).json();
  };

  const fetchDeleteData = async (e: number) => {
    const url = `${baseURL}/gudangs/${e}`;
	  //	console.log(url);
    const request = new Request(url, {
      method: "DELETE",
      credentials: credential_include,
    });

    return await (await fetch(request)).json();
  };

  const createData = useMutation(fetchCreateData, {
    onMutate: async (e: iGudang) => {
      // Cancel any outgoing refetches (so they don't overwrite our optimistic update)
      await client.cancelQueries();

      // Snapshot the previous value
      const previousData = client.getQueryData<iResult>(["gudang", "list"]);

      // Optimistically update to the new value
      if (previousData) {
        client.setQueryData<iResult>(["category", "list"], previousData);
      }

      return previousData;
    },
    onSuccess: async (data: any, variable: iGudang, context) => {
      if (context) {
        setTimeout(() => {
          isUpdating = false;
          if (data.status !== "fail") {
            open = false;
          } else {
            isError = true;
            err_msg = data.message;
          }
        }, 1000);
        //await client.invalidateQueries(["category", "list"]);
        //client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
      }
    },
    // If the mutation fails, use the context returned from onMutate to roll back
    onError: (err: any, variables: any, context: any) => {
      console.log(err);
      if (context?.previousData) {
        client.setQueryData<iResult>(["gudang", "list"], context.previousData);
      }
      //      selectedCategoryId.set($category.id)
      // errorMesage.set(`Nama kategori '${$category.name}'' sudah ada!`);
    },
    // Always refetch after error or success:
    onSettled: async () => {
      await client.invalidateQueries(["gudang", "list"]);
    },
  });

  const updateData = useMutation(fetchUpdateData, {
    onMutate: async (e: iGudang) => {
      // Cancel any outgoing refetches (so they don't overwrite our optimistic update)
      await client.cancelQueries();

      // Snapshot the previous value
      const previousGudang = client.getQueryData<iResult>(["gudang", "list"]);

      // Optimistically update to the new value
      if (previousGudang) {
        client.setQueryData<iResult>(["gudang", "list"], previousGudang);
      }

      return previousGudang;
    },
    onSuccess: async (data: any, variable: iGudang, context) => {
      if (context) {
        setTimeout(() => {
          isUpdating = false;
          if (data.status !== "fail") {
            open = false;
          } else {
            isError = true;
            err_msg = data.message;
          }
        }, 1000);
        //        await client.invalidateQueries(["category", "list"]);
        //client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
      }
    },
    // If the mutation fails, use the context returned from onMutate to roll back
    onError: (err: any, variables: any, context: any) => {
      if (context?.previousGudang) {
        client.setQueryData<iResult>(
          ["Gudang", "list"],
          context.previousGudang,
        );
        //        selectedCategoryId.set($category.id)
      }
      // errorMesage.set(`Nama kategori '${$category.name}' sudah ada!`);
    },
    onSettled: async (
      data: any,
      error: any,
      variables: iGudang,
      context: iResult | undefined,
    ) => {
      await client.invalidateQueries(["gudang", "list"]);
    },
  });

  const deleteData = useMutation(fetchDeleteData, {
    onMutate: async (e: number) => {
      // Cancel any outgoing refetches (so they don't overwrite our optimistic update)
      await client.cancelQueries();

      // Snapshot the previous value
      const previousGudang = client.getQueryData<iResult>(["gudang", "list"]);

      // Optimistically update to the new value
      if (previousGudang) {
        client.setQueryData<iResult>(["gudang", "list"], previousGudang);
      }

      return previousGudang;
    },
    onSuccess: async () => {
      setTimeout(() => {
        isUpdating = false;
      }, 1000);

      gudang = {
        id: 0,
        name: "",
        employeeId: 0,
        employeeName: "",
      };
    },
    // If the mutation fails, use the context returned from onMutate to roll back
    onError: (err: any, variables: any, context: any) => {
      if (context?.previousGudang) {
        client.setQueryData<iResult>(
          ["gudang", "list"],
          context.previousGudang,
        );
      }
    },
    onSettled: async (
      data: any,
      error: any,
      variables: number,
      context: iResult | undefined,
    ) => {
      if (data.status === "fail") {
        showNotification = true;
        err_msg = data.message;
        timeout = 3_000;
      }
      await client.invalidateQueries(["gudang", "list"]);
      isUpdating = false;
    },
  });

  function delete_gudang(e: CustomEvent<number>) {
    isUpdating = true;
    $deleteData.mutate(e.detail);
  }

  function get_employees() {
    return employees.map((m) => ({ id: m.id, text: m.text }));
  }

  let open = false;
  let gudang: iGudang = {
    id: 0,
    name: "",
    employeeId: 0,
    employeeName: "",
    locate: "",
  };

  function edit_gudang(id: number) {
    isError = false;
    err_msg = "";
    // timeout = undefined;

    let test = gudangs.filter((f) => f.id == id);
    if (test.length > 0) {
      gudang = { ...test[0] };
      open = true;
    }
  }

  let headers = [
    { key: "id", value: "#ID", width: "10%" },
    { key: "name", value: "Nama", width: "auto" },
    { key: "employeeName", value: "Penjaga Gudang", width: "auto" },
    { key: "locate", value: "Lokasi Gudang", width: "auto" },
    { key: "cmd", value: "", width: "150px" },
  ];

  function submit() {
    isError = false;
    isUpdating = true;
    if (gudang.id > 0) {
      $updateData.mutate(gudang);
    } else {
      $createData.mutate(gudang);
    }
  }

  function new_gudang() {
    isError = false;
    err_msg = "";
    // timeout = undefined;

    gudang = {
      id: 0,
      name: "",
      employeeId: 0,
      employeeName: "",
      locate: "",
    };
    open = true;
  }

  // 	const descriptionMap = [
  // 		"Submitting...",
  // 		"Success",
  // 		"Cancelling...",
  // ]	;

  // 	const stateMap = [
  // 		"finished",
  // 		"dormant",
  // 		"dormant",
  // 	];

  // 	let timeout: NodeJS.Timeout;
  // 	let state = 1;

  // 	function reset(incomingState: number) {
  // 		if (incomingState > 2) {
  // 			clearTimeout(timeout);
  // 		}

  // 		if (incomingState) {
  // 			timeout = setTimeout(() => {
  // 				state = incomingState;
  // 			}, 2000);
  // 		}
  // 	}

  // 	onDestroy(() => reset(4));

  // 	$: reset(3);

  let selectedRowIds = [gudangs.length > 0 ? gudangs[0].id : 0];

  let client_width = 0;
  let timeout: undefined | number = undefined;
  let showNotification = false;

  // function shouldFilterItem(item: ComboBoxItem, value: string) {
  //   if (!value) return true;
  //   return item.text.toLowerCase().includes(value.toLowerCase());
  // }

  $: showNotification = timeout !== undefined;
  $: employee_invalid = gudang.employeeId === 0;

  // $: console.log("selectedRowIds", selectedRowIds);
</script>

<svelte:window bind:innerWidth={client_width} />

<Modal
  bind:open
  hasForm
  id="gud-modal"
  preventCloseOnClickOutside
  modalHeading="Gudang"
  primaryButtonText="Simpan"
  primaryButtonIcon={Save}
  secondaryButtonText="Batal"
  selectorPrimaryFocus={"#gud-name"}
  on:click:button--secondary={() => (open = false)}
  on:click:button--primary={submit}
  size="xs"
  primaryButtonDisabled={isUpdating || employee_invalid}
>
  <form>
    <TextInput
      id="gud-name"
      bind:value={gudang.name}
      labelText="Nama"
      placeholder="e.g. Junaedi"
    />
    <ComboBox
      id="gud-ware"
      titleText="Penjaga gudang"
      placeholder="Pilih penjaga gudang"
      selectedId={gudang.employeeId}
      warn={employee_invalid}
      items={get_employees()}
      on:select={(e) => {
        if (e.detail.selectedId > 0) {
          gudang.employeeId = e.detail.selectedId;
        }
      }}
      on:clear={() => (gudang.employeeId = 0)}
    />
    <TextInput
      id="gud-locate"
      bind:value={gudang.locate}
      labelText="Lokasi"
      placeholder="Jl. Merdeka Barat"
    />
  </form>

  {#if isUpdating}
    <InlineLoading
      status="active"
      description={gudang.id === 0 ? "Posting data..." : "Updating data..."}
    />
  {/if}

  {#if isError}
    <InlineLoading description={err_msg} status="error" />
  {/if}
</Modal>

<DataTable
  useStaticWidth={client_width > 640}
  zebra
  size="short"
  description="Tabel daftar gudang"
  {headers}
  rows={gudangs}
  bind:selectedRowIds
>
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
      <DeleteGudang gudangId={row.id} on:deleteGudang={delete_gudang} />
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

	<strong slot="title"><Warehouse size={24} /> Gudang Barang</strong>

</DataTable>

{#if showNotification}
  <ToastNotification
    {timeout}
    title="Error"
    subtitle={err_msg}
    caption={new Date().toLocaleString()}
    on:close={(e) => {
      timeout = undefined;
    }}
  />
{/if}

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
  :global(#gud-modal .bx--modal-container.bx--modal-container--xs) {
    max-height: 100%;
    /* height: 640px; */
  }
</style>
