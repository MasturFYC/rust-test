import type { iCategory, iPropertyID, iRelationProp } from '../interfaces';
import { baseURL, credential_include } from '$lib/interfaces';

const url = `${baseURL}/categories'`


export type iPropertyWithID = {
	status: string;
	data: iPropertyID[];
	count: number;
};

async function fetchCategories(): Promise<iCategory[]> {
	const options = {
		headers: {
			"content-type": "application/json"
		},
		method: "GET",
		credentials: credential_include,
	}

	const request = new Request(url, options);
	let result = await fetch(request);
	let data = await result.json() as iCategory[];
	console.log(data);

	return data;

}

export async function getCategoryProp(): Promise<iPropertyWithID> {
		const options = {
			headers: {
				accept: "application/json",
			},
			method: "GET",
			credentials: credential_include,
		};

		const request = new Request(
			`${baseURL}/categories/property/list`,
			options,
		);

		let result = await fetch(request);
		return (await result.json()) as iPropertyWithID;
}

export const queryCategoryOptions = () => ({
	queryKey: ['category', 'list'],
	queryFn: async () => await fetchCategories()
});

// export const queryCategoryProp = {
// 		queryKey: 'catProp',
// 		queryFn: async () => await getCategoryProp()
// }

type iRelationResult = {
	status: string;
	data: iRelationProp[];
};

export async function getSupplierProp(p: string[]): Promise<iRelationResult> {
		const url = `${baseURL}/relations/property`;
		const options = {
			headers: {
				"content-type": "application/json",
				accept: "application/json",
			},
			method: "POST",
			credentials: credential_include,
			body: JSON.stringify(p),
		};
		const request = new Request(url, options);
		const result = await fetch(request);

		return (await result.json()) as iRelationResult;
}

export async function getBarcodes(): Promise<{status: string, data: {barcode: string}[]}> {
	const url = `${baseURL}/products/barcodes/list`;
	const options = {
		headers: {
			"content-type": "application/json",
			accept: "application/json",
		},
		method: "GET",
		credentials: credential_include,
	};
	const request = new Request(url, options);
	const result = await fetch(request);

	return (await result.json());
}
