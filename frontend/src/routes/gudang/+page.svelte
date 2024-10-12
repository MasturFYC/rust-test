<script lang="ts">
import { browser } from "$app/environment";
import { baseURL, credential_include, type iGudang } from "$lib/interfaces";
import { useQuery } from "@sveltestack/svelte-query";
import ListCategory from "./list.svelte";
import { getRelationProp } from "$lib/fetchers";
import {IbmDb2Warehouse as Warehouse} from 'carbon-icons-svelte';
import { useMutation, useQueryClient } from "@sveltestack/svelte-query";
import FormGudang from './form.svelte';
import { ToastNotification } from "carbon-components-svelte";

type iResult = {
	count: number;
	data: iGudang[];
	status: string;
};


const client = useQueryClient();
const url = `${baseURL}/gudangs`;

let open = false;
let isError = false;
let gudang: iGudang = {
	id: 0,
	name: "",
	employeeId: 0,
	employeeName: "",
	locate: "",
};
let timeout: undefined | number = undefined;
let showNotification = false;
let isUpdating = false;
let errorMessage = "";

/*
const initResult: iResult = {
  count: 0,
  data: [],
  status: "page Loading",
};
*/

const fetchUpdateData = async (e: iGudang): Promise<iGudang> => {
    const url = `${baseURL}/gudangs/${e.id}`;
    const request = new Request(url, {
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(e),
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
      body: JSON.stringify(e),
      method: "POST",
      credentials: credential_include,
    });

    return await (await fetch(request)).json();
  };

  const fetchDeleteData = async (e: number) => {
    const url = `${baseURL}/gudangs/${e}`;
    const request = new Request(url, {
      method: "DELETE",
      credentials: credential_include,
    });

    return await (await fetch(request)).json();
  };

  const createData = useMutation(fetchCreateData, {
    onMutate: async(_: iGudang) => {
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
    onSuccess: async (data: any, _variable: iGudang, context) => {
      if (context) {
        setTimeout(() => {
          isUpdating = false;
          if (data.status !== "fail") {
            open = false;
          } else {
            isError = true;
            errorMessage = data.message;
          }
        }, 1000);
        //await client.invalidateQueries(["category", "list"]);
        //client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
      }
    },
    // If the mutation fails, use the context returned from onMutate to roll back
    onError: (
			err: any,
			_variables: any,
			context: any
		) => {
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
    onMutate: async (
		 _: iGudang
		) => {
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
    onSuccess: async (
			data: any,
			_variables: iGudang,
			context
		) => {
      if (context) {
        setTimeout(() => {
          isUpdating = false;
          if (data.status !== "fail") {
            open = false;
          } else {
            isError = true;
            errorMessage = data.message;
          }
        }, 1000);
        //        await client.invalidateQueries(["category", "list"]);
        //client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
      }
    },
    // If the mutation fails, use the context returned from onMutate to roll back
    onError: (_err: any, _variables: any, context: any) => {
      if (context?.previousGudang) {
        client.setQueryData<iResult>(
          ["Gudang", "list"],
          context.previousGudang,
        );
        //        selectedCategoryId.set($category.id)
      }
      // errorMesage.set(`Nama kategori '${$category.name}' sudah ada!`);
    },
    onSettled: async (_data: any, _error: any, _variables: iGudang, _context: iResult | undefined,
    ) => {
      await client.invalidateQueries(["gudang", "list"]);
    },
  });

  const deleteData = useMutation(fetchDeleteData, {
    onMutate: async (_e: number) => {
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
    onError: (
			_err: any,
			_variables: any,
			context: any
		) => {
      if (context?.previousGudang) {
        client.setQueryData<iResult>(
          ["gudang", "list"],
          context.previousGudang,
        );
      }
    },
    onSettled: async (data: any, _error: any, _variables: number, _context: iResult | undefined,
    ) => {
      if (data.status === "fail") {
        showNotification = true;
        errorMessage = data.message;
        timeout = 3_000;
      }
      await client.invalidateQueries(["gudang", "list"]);
      isUpdating = false;
    },
  });

  async function fetchGudangs(): Promise<iResult> {
    const options = {
      headers: {
        "content-type": "application/json",
      },
      method: "GET",
      credentials: credential_include,
    };

    const request = new Request(url, options);
    let result = await fetch(request);

    return (await result.json()) as iResult;
  }

  const queryGudangOptions = () => ({
    queryKey: ["gudang", "list"],
    queryFn: async () => await fetchGudangs(),
    enabled: browser,
  });

  const query = useQuery<iResult, Error>(queryGudangOptions());

  function showErrorMessage() {
    if ($query.error instanceof Error) {
      return $query.error.message;
    }
    return "Cannot load gudang.";
  }

  const employeeQuery = useQuery(
    "empProp",
    async () => await getRelationProp(["Employee"]),
    {
      enabled: browser,
    },
  );

function submit(e: CustomEvent<iGudang>) {
	isError = false;
	isUpdating = true;
	if (e.detail.id > 0) {
		$updateData.mutate(e.detail);
	} else {
		$createData.mutate(e.detail);
	}
}

function deleteGudang(e: CustomEvent<number>) {
	isUpdating = true;
	$deleteData.mutate(e.detail);
}

function newGudang(e: CustomEvent<iGudang>) {
	isError = false;
	errorMessage = "";
	gudang = {...e.detail};
	open = true;
}

function editGudang(e: CustomEvent<number>) {
	isError = false;
	errorMessage = "";
	const id = e.detail;
	// timeout = undefined;

	if ($query.data) {
		let test = $query.data.data.filter(f => f.id === id);
		if (test.length > 0) {
			gudang = { ...test[0] };
			open = true;
		}
	}
}

$: showNotification = timeout !== undefined;
$: {
	query.setEnabled(browser);
	employeeQuery.setEnabled(browser);
}


</script>

<svelte:head>
  <title>Gudang</title>
  <meta name="description" content="Gudang this app" />
</svelte:head>

<h2><Warehouse size={24} /> Gudang Barang</h2>

{#if $query.isLoading || $employeeQuery.isLoading}
  <p>Loading...</p>
{:else if $query.isError}
  <p>Error: {showErrorMessage()}</p>
{:else if $query.isSuccess}
  <ListCategory
    gudangs={$query.data.data}
		on:edit={editGudang}
		on:newGudang={newGudang}
		on:deleteGudang={deleteGudang}
  />
  <p>Total: {$query.data.count} item{$query.data.count > 1 ? "s" : ""}</p>
{/if}

{#if showNotification}
  <ToastNotification
    {timeout}
    title="Error"
    subtitle={errorMessage}
    caption={new Date().toLocaleString()}
    on:close={() => {
      timeout = undefined;
    }}
  />
{/if}

<FormGudang
	on:submit={submit}
	bind:open={open}
	gudang={gudang}
	bind:isUpdating={isUpdating}
	bind:isError={isError}
	bind:errorMessage={errorMessage}
  employees={$employeeQuery.data?.data}
/>

<style lang="scss">
  p {
    margin-top: 12px;
  }
</style>
