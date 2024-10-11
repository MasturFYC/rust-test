<script lang="ts">
  import { browser } from "$app/environment";
  import { baseURL, credential_include, type iGudang } from "$lib/interfaces";
  import { useQuery } from "@sveltestack/svelte-query";
  import ListCategory from "./list.svelte";
  import { getRelationProp } from "$lib/fetchers";

  const url = `${baseURL}/gudangs`;

  type iResult = {
    count: number;
    data: iGudang[];
    status: string;
  };

  const initResult: iResult = {
    count: 0,
    data: [],
    status: "page Loading",
  };

  // let ready = false;

  // let cred: RequestCredentials = "same-origin";

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

  $: {
    query.setEnabled(browser);
    employeeQuery.setEnabled(browser);
  }
</script>

<svelte:head>
  <title>Gudang</title>
  <meta name="description" content="Gudang this app" />
</svelte:head>

{#if $query.isLoading || $employeeQuery.isLoading}
  <p>Loading...</p>
{:else if $query.isError}
  <p>Error: {showErrorMessage()}</p>
{:else if $query.isSuccess}
  <ListCategory
    gudangs={$query.data.data}
    employees={$employeeQuery.data?.data}
  />
  <p>Total: {$query.data.count} item{$query.data.count > 1 ? "s" : ""}</p>
{/if}

<style lang="scss">
  p {
    margin-top: 12px;
  }
</style>
