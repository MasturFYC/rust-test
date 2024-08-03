<script lang="ts">
	import { baseURL, credential_include, type iCategory } from "$lib/interfaces";
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
		ToolbarMenuItem
	} from "carbon-components-svelte";
	import { Edit, NewTab, Save } from "carbon-icons-svelte";
	import DeleteCategory from "./DeleteCategory.svelte";

	const client = useQueryClient();
	let isUpdating = false;
	let isError = false;
	let err_msg = "";

	export let categories: iCategory[] = [];

	type iResult = {
		count: number;
		data: iCategory[];
		status: string;
	};

	// let data: iResult = {
	// 	count: 0,
	// 	data: [],
	// 	status: ""
	// };

	const fetchUpdateData = async (e: iCategory): Promise<iCategory> => {
		const url = `${baseURL}/categories/${category.id}`;
		const request = new Request(url, {
			headers: {
				"content-type": "application/json",
			},
			body: JSON.stringify(category),
			method: "PUT",
			credentials: credential_include,
		});

		const result = await fetch(request);
		return await result.json();
	};

	const fetchCreateData = async (e: iCategory): Promise<iCategory> => {
		const url = `${baseURL}/categories`;
		const request = new Request(url, {
			headers: {
				"content-type": "application/json",
			},
			body: JSON.stringify(category),
			method: "POST",
			credentials: credential_include,
		});

		return await (await fetch(request)).json();
	};

	const fetchDeleteData = async (e: number) => {
		const url = `${baseURL}/categories/${e}`;
		const request = new Request(url, {
			method: "DELETE",
			credentials: credential_include,
		});

		return await (await fetch(request)).json();
	};

	const createData = useMutation(fetchCreateData, {
		onMutate: async (e: iCategory) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousData = client.getQueryData<iResult>(["category", "list"]);

			// Optimistically update to the new value
			if (previousData) {
				client.setQueryData<iResult>(["category", "list"], previousData);
			}

			return previousData;
		},
		onSuccess: async (data: any, variable: iCategory, context) => {
			if (context) {
				setTimeout(() => {
					isUpdating = false;
					if (data.status !== "fail") {
						open = false;
					} else {
						isError = true;
						err_msg = data.message;						
					}
				}, 250);
				//await client.invalidateQueries(["category", "list"]);
				//client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, variables: any, context: any) => {
			console.log(err);
			if (context?.previousData) {
				client.setQueryData<iResult>(
					["category", "list"],
					context.previousData,
				);
			}
			//      selectedCategoryId.set($category.id)
			// errorMesage.set(`Nama kategori '${$category.name}'' sudah ada!`);
		},
		// Always refetch after error or success:
		onSettled: async () => {
			await client.invalidateQueries(["category", "list"]);
		},
	});

	const updateData = useMutation(fetchUpdateData, {
		onMutate: async (e: iCategory) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousCategories = client.getQueryData<iResult>([
				"category",
				"list",
			]);

			// Optimistically update to the new value
			if (previousCategories) {
				client.setQueryData<iResult>(["category", "list"], previousCategories);
			}

			return previousCategories;
		},
		onSuccess: async (data: any, variable: iCategory, context) => {
			if (context) {
				setTimeout(() => {
					isUpdating = false;
					if (data.status !== "fail") {
						open = false;
					} else {
						isError = true;						
						err_msg = data.message;
					}
				}, 1500);
				//        await client.invalidateQueries(["category", "list"]);
				//client.setQueryData<iCategory[]>(["category", "list"], [...context, data.data]);
			}
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, variables: any, context: any) => {
			if (context?.previousCategories) {
				client.setQueryData<iResult>(
					["category", "list"],
					context.previousCategories,
				);
				//        selectedCategoryId.set($category.id)
			}
			// errorMesage.set(`Nama kategori '${$category.name}' sudah ada!`);
		},
		onSettled: async (
			data: any,
			error: any,
			variables: iCategory,
			context: iResult | undefined,
		) => {
			await client.invalidateQueries(["category", "list"]);
		},
	});

	const deleteData = useMutation(fetchDeleteData, {
		onMutate: async (e: number) => {
			// Cancel any outgoing refetches (so they don't overwrite our optimistic update)
			await client.cancelQueries();

			// Snapshot the previous value
			const previousCategories = client.getQueryData<iResult>([
				"category",
				"list",
			]);

			// Optimistically update to the new value
			if (previousCategories) {
				client.setQueryData<iResult>(["category", "list"], previousCategories);
			}

			return previousCategories;
		},
		onSuccess: async () => {
			setTimeout(() => {
				isUpdating = false;
			}, 1500);

			category = {
				id: 0,
				name: "",
			};
		},
		// If the mutation fails, use the context returned from onMutate to roll back
		onError: (err: any, variables: any, context: any) => {
			
			if (context?.previousCategories) {
				client.setQueryData<iResult>(
					["category", "list"],
					context.previousCategories,
				);
			}
		},
		onSettled: async (data: any, error: any, variables: number, context: iResult | undefined) => {
			if(data.status === "fail") {
				showNotification = true;
				err_msg = data.message;
				timeout = 3_000;
			}
			await client.invalidateQueries(["category", "list"]);
			isUpdating = false;
		},
	});

	function delete_category(e: CustomEvent<number>) {
		isUpdating = true;
		$deleteData.mutate(e.detail);
	}

	let open = false;
	let category: iCategory = {
		id: 0,
		name: "",
	};

	function edit_category(id: number) {
		isError = false;
		err_msg = "";
		// timeout = undefined;

		let test = categories.filter((f) => f.id == id);
		if (test.length > 0) {
			category = test[0];
			open = true;
		}
	}

	let headers = [
		{ key: "id", value: "#ID", width: "10%" },
		{ key: "name", value: "Nama", width: "auto" },
		{ key: "cmd", value: "", width: "150px" },
	];

	function submit() {
		isError = false;
		isUpdating = true;
		if (category.id > 0) {
			$updateData.mutate(category);
		} else {
			$createData.mutate(category);
		}
	}

	function new_category() {
		isError = false;
		err_msg = "";
		// timeout = undefined;

		category = {
			id: 0,
			name: "",
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

	let selectedRowIds = [categories.length > 0 ? categories[0].id : 0];

	let client_width = 0;
	let timeout: undefined | number = undefined;
	let showNotification = false;
	$: showNotification = timeout !== undefined;

	// $: console.log("selectedRowIds", selectedRowIds);
</script>

<svelte:window bind:innerWidth={client_width} />

<Modal
	bind:open
	hasForm	
	preventCloseOnClickOutside
	modalHeading="Kategori"
	primaryButtonText="Simpan"
	primaryButtonIcon={Save}
	secondaryButtonText="Batal"
	selectorPrimaryFocus={"#cat-name"}
	on:click:button--secondary={() => (open = false)}
	on:click:button--primary={submit}
	size="xs"
	primaryButtonDisabled={isUpdating}
>
	<FluidForm>
		<TextInput id="cat-name" bind:value={category.name} labelText="Nama" />
	</FluidForm>

	{#if isUpdating}
		<InlineLoading
			status="active"
			description={category.id === 0 ? "Posting data..." : "Updating data..."}
		/>
	{/if}

	{#if isError}
		<InlineLoading description={err_msg} status="error" />
	{/if}
</Modal>

<DataTable
	useStaticWidth={client_width>640}
	zebra
	size="short"
	title="Kategori Barang"
	description="Tabel daftar katagori barang"
	{headers}
	rows={categories}
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
				on:click={() => edit_category(row.id)}
			/>
			<DeleteCategory categoryId={row.id} on:deleteCategory={delete_category} />
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
			<Button on:click={new_category} icon={NewTab}>Buat baru</Button>
		</ToolbarContent>
	</Toolbar>
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