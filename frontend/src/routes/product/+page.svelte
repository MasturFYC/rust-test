<style lang="css">
:global(.bx--label) {
  margin-bottom: 3px;
  margin-top: 9px;
}

:global(.bx--list-box__menu-item, .bx--list-box__menu-item__option) {
  height: auto;
}
</style>

<script lang="ts">
import { browser } from "$app/environment";
import FormProduct from "$lib/components/FormProduct.svelte";
import { baseURL, credential_include, type iProduct } from "$lib/interfaces";
import { Product } from "carbon-icons-svelte";
import {
  getCategoryProp,
  getRelationProp,
  type iPropertyWithID,
} from "$lib/fetchers";
import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@sveltestack/svelte-query";
import { Loading, Pagination } from "carbon-components-svelte";
import dayjs from "dayjs";
import ProductList from "./ProductList.svelte";

type ResultBase = {
  status: string;
  count: number;
  totalPages: number;
  totalItems: number;
};

type iResult = ResultBase & {
  data: iProduct[];
};

type iResponse = {
  status: string;
  data: iProduct;
};

const title = "Data Barang";

const client = useQueryClient();
const url = `${baseURL}/products`;
const qKey = "products";

const initResult: iResult = {
  status: "page loading",
  count: 0,
  totalPages: 0,
  totalItems: 0,
  data: [],
};

const initData: iProduct = {
  id: 0,
  supplierId: 0,
  name: "",
  barcode: "",
  unit: "",
  content: 0.0,
  hpp: 0.0,
  margin: 11,
  price: 0.0,
  ppn: 0.0,
  heavy: 0.0,
  isActive: true,
  variantName: "",
  descriptions: "",
  categoryId: 0,
  createdAt: dayjs().toISOString(),
  updatedAt: dayjs().toISOString(),
  stocks: [],
};

let pageSize = 5;
let page = 1;
let pages = [3, 5, 10, 25, 50];
let isUpdating = false;
let isLoading = true;
let open = false;
let isError = false;
let errorMessage = "";
let innerWidth = 720;
let txt: string | undefined = undefined;
let rel_id: string | undefined = undefined;
let opt = 0;
let cat_id = 0;

const search = (e: CustomEvent<string | undefined>) => {
  if (e.detail === null) {
    opt = 0;
    txt = undefined;
  } else {
    opt = 1;
    txt = e.detail;
  }
};

async function fetchData(p: number): Promise<iResult> {
  //console.log(q_key);
  if (browser) {
    const options = {
      headers: {
        "content-type": "application/json",
      },
      method: "GET",
      credentials: credential_include,
    };

    const request = new Request(
      `${url}?opt=${opt}&page=${p}&limit=${pageSize}${opt === 1 ? `&txt=${txt}` : ""}${opt === 2 ? `&relid=${rel_id}` : ""}${opt === 3 ? `&catid=${cat_id}` : ""}`,
      options,
    );

    let result = await fetch(request);
    return (await result.json()) as iResult;
  }

  return Promise.resolve(initResult);
}

const prefetchNextPage = (data: iResult) => {
  if (page < data.totalPages) {
    client.prefetchQuery([qKey, page + 1, pageSize], () => fetchData(page + 1));
  }
};

let data: iProduct = { ...initData };

const query = useQuery<iResult, Error>({
  queryKey: [qKey, page, pageSize],
  queryFn: async () => await fetchData(page),
  onSuccess: prefetchNextPage,
  keepPreviousData: true,
  enabled: browser,
});

function setQueryOption(
  p: number,
  l: number,
  o: number,
  t: string | undefined,
  r: string | undefined,
  c: number,
) {
  query.setOptions({
    queryKey: [qKey, p, l, o, t, r, c],
    keepPreviousData: true,
    queryFn: async () => await fetchData(page),
    onSuccess: prefetchNextPage,
  });

  // console.log([qKey, p, l, o, t, r]);
  // console.log(q_key);
}

const fetchCreateData = async (e: iProduct): Promise<iResponse> => {
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

const fetchUpdateData = async (e: iProduct): Promise<iResponse> => {
  const request = new Request(`${url}/${e.id}`, {
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(e),
    method: "PUT",
    credentials: credential_include,
  });

  const result = await fetch(request);
  // console.log(result);
  return await result.json();
};

const fetchDeleteData = async (e: string) => {
  const request = new Request(`${url}/${e}`, {
    method: "DELETE",
    credentials: credential_include,
  });

  return await (await fetch(request)).json();
};

const createData = useMutation(fetchCreateData, {
  onMutate: async (e: iProduct) => {
    // Cancel any outgoing refetches (so they don't overwrite our optimistic update)
    await client.cancelQueries();

    // Snapshot the previous value
    const previousData = client.getQueryData<iResult>(q_key);

    // Optimistically update to the new value
    if (previousData) {
      client.setQueryData<iResult>(q_key, previousData);
    }

    return previousData;
  },
  onSuccess: async (data: any, _variable: iProduct, _context) => {
    // setTimeout(() => {
    if (data.status === "success") {
      open = false;
      isUpdating = false;
      isError = false;
    } else {
      isError = true;
      errorMessage = data.message;
    }
    //}, 250);
  },
  // If the mutation fails, use the context returned from onMutate to roll back
  onError: (_err: any, _variables, context: any) => {
    // console.log(err);
    isUpdating = false;
    if (context?.previousData) {
      client.setQueryData<iResult>(q_key, context.previousData);
    }
  },
  // Always refetch after error or success:
  onSettled: async () => {
    await client.invalidateQueries([qKey]);
  },
});

const updateData = useMutation(fetchUpdateData, {
  onMutate: async (_e: iProduct) => {
    // Cancel any outgoing refetches (so they don't overwrite our optimistic update)
    await client.cancelQueries();

    // Snapshot the previous value
    const previousData = client.getQueryData<iResult>(q_key);

    // Optimistically update to the new value
    if (previousData) {
      client.setQueryData<iResult>(q_key, previousData);
    }

    return previousData;
  },
  onSuccess: async (data: any, _variable: iProduct, _context) => {
    if (data.status === "failed") {
      isError = true;
      errorMessage = data.message;
    }
  },
  // If the mutation fails, use the context returned from onMutate to roll back
  onError: (_err: any, _variables: any, context: any) => {
    isUpdating = false;
    if (context?.previousData) {
      client.setQueryData<iResult>(q_key, context.previousData);
    }
  },
  onSettled: async (data: any) => {
    if (data.status === "success") {
      // console.log("SUCCESS")
      setTimeout(() => {
        isUpdating = false;
        open = false;
        errorMessage = "";
      }, 500);
    }
    await client.invalidateQueries([qKey]);
  },
});

const deleteData = useMutation(fetchDeleteData, {
  onMutate: async (_e: string) => {
    // Cancel any outgoing refetches (so they don't overwrite our optimistic update)
    await client.cancelQueries();

    // Snapshot the previous value
    const previousData = client.getQueryData<iResult>(q_key);

    // Optimistically update to the new value
    if (previousData) {
      client.setQueryData<iResult>(q_key, previousData);
    }

    return previousData;
  },
  onSuccess: async () => {
    //setTimeout(() => {
    isUpdating = false;
    //}, 1500);
  },
  // If the mutation fails, use the context returned from onMutate to roll back
  onError: (err: any, variables: any, context: any) => {
    if (context?.previousData) {
      client.setQueryData<iResult>(q_key, context.previousData);
    }
  },
  onSettled: async (
    raw: any,
    //			error: any,
    //			variables: string,
    //			context: iResult | undefined,
  ) => {
    if (raw.status === "fail") {
      isError = true;
      errorMessage = raw.message;
      // timeout = 3_000;
    }
    await client.invalidateQueries([qKey]);
  },
});

function edit_product(e: CustomEvent<number | undefined>): void {
  const id = e.detail;

  if (id) {
    const test = $query.data?.data.filter((f) => f.id === id)[0];
    if (test) {
      data = { ...test };
      open = true;
    }
  } else {
    data = { ...initData };
    open = true;
  }
}

function delete_data(e: CustomEvent<string>): void {
  const id = e.detail;
  $deleteData.mutate(id);
}

function submit(e: CustomEvent<iProduct>) {
  const id = e.detail.id;
  if (id === 0) {
    $createData.mutate(e.detail);
  } else {
    $updateData.mutate(e.detail);
  }
  //open = false;
}

function supplier_change(e: CustomEvent<string | undefined>): void {
  rel_id = e.detail;
  if (e.detail === null) {
    opt = 0;
  } else {
    opt = 2;
  }
}

// onMount(async () => {
// 	query.setEnabled(true);
// });

function category_change(e: CustomEvent<number>): void {
  cat_id = e.detail;
  opt = cat_id === 0 ? 0 : 3;
}

$: q_key = [qKey, page, pageSize, opt, txt, rel_id, cat_id];

const categoryQuery = useQuery<iPropertyWithID, Error>(
  "catProp",
  getCategoryProp,
  {
    enabled: browser,
  },
);

const supplierQuery = useQuery(
  "supProp",
  async () => await getRelationProp(["Supplier"]),
  {
    enabled: browser,
  },
);

$: {
  categoryQuery.setEnabled(browser);
  supplierQuery.setEnabled(browser);
  query.setEnabled(browser);
  isLoading = false;
}

$: setQueryOption(page, pageSize, opt, txt, rel_id, cat_id);
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content="Product this app" />
</svelte:head>

<svelte:window bind:innerWidth={innerWidth} />

<h2><Product size={24} /> {title}</h2>
<!-- <subtitle>Tabel data barang / produk</subtitle> -->
{#if $categoryQuery.isLoading || $supplierQuery.isLoading || $query.isLoading || isLoading}
  <Loading withOverlay={false} />
{/if}

{#if open}
  <FormProduct
    suppliers={$supplierQuery.data?.data}
    categories={$categoryQuery.data?.data}
    data={data}
    bind:open={open}
    bind:isError={isError}
    bind:errorMessage={errorMessage}
    bind:isUpdating={isUpdating}
    on:submit={submit}
    bind:innerWidth={innerWidth}
  />
{/if}

{#if $query.isLoading}
  <span>Loading...</span>
{:else if $query.isError}
  <span>Error: {$query.error.message}</span>
{:else if $query.isSuccess}
  <ProductList
    data={$query.data.data}
    suppliers={$supplierQuery.data?.data}
    categories={$categoryQuery.data?.data}
    on:edit={edit_product}
    on:deleteData={delete_data}
    bind:innerWidth={innerWidth}
    on:search={search}
    on:categoryChange={category_change}
    on:supplierChange={supplier_change}
  />

  <Pagination
    totalItems={$query.data.totalItems}
    pageSizes={pages}
    pageSize={pageSize}
    page={page}
    on:update={(e) => {
      pageSize = e.detail.pageSize;
      page = e.detail.page;
    }}
    on:click:button--next={(e) => (page = e.detail.page)}
    on:click:button--previous={(e) => (page = e.detail.page)}
  />
{/if}
