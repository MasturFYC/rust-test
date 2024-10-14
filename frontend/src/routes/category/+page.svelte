<script lang="ts">
  import { browser } from "$app/environment";
  import { baseURL, credential_include, type iCategory } from "$lib/interfaces";
  import { useQuery } from "@sveltestack/svelte-query";
  import ListCategory from "./list.svelte";
  import { Category } from "carbon-icons-svelte";

  const title = "Kategori Barang";
  const url = `${baseURL}/categories`;

  type iResult = {
    count: number;
    data: iCategory[];
    status: string;
  };

  const initResult: iResult = {
    count: 0,
    data: [],
    status: "page Loading",
  };

  // let ready = false;

  // let cred: RequestCredentials = "same-origin";

  async function fetchCategories(): Promise<iResult> {
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

  const queryCategoryOptions = () => ({
    queryKey: ["category", "list"],
    queryFn: async () => await fetchCategories(),
    enabled: browser,
  });

  const query = useQuery<iResult, Error>(queryCategoryOptions());

  function showErrorMessage() {
    if ($query.error instanceof Error) {
      return $query.error.message;
    }
    return "Cannot load category list.";
  }

  $: query.setEnabled(browser);
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content="Category this app" />
</svelte:head>

<h2><Category size={24} /> {title}</h2>

{#if $query.isLoading}
  <p>Loading...</p>
{:else if $query.isError}
  <p>Error: {showErrorMessage()}</p>
{:else if $query.isSuccess}
  <ListCategory categories={$query.data.data} />
  <p>Total: {$query.data.count} item{$query.data.count > 1 ? "s" : ""}</p>
{/if}

<style lang="scss">
  p {
    margin-top: 12px;
  }
</style>
