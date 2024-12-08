import {
	baseURL,
	credential_include,
	type iGudang,
	type iOrder,
	type iOrderDetail
} from '$lib/interfaces';

export type iGudangResult = {
	count: number;
	data: iGudang[];
	status: string;
};

type OrderProps = {
	status: string;
	data: iOrder[];
	totalItems: number;
	totalPages: number;
	count: number;
	currentPage: number;
};

export async function getOrderById(
	id: number,
	defaultValue?: { order: iOrder; details: iOrderDetail[] }
): Promise<{ status: string; order: iOrder; details: iOrderDetail[] }> {
	if (id === 0) {
		return await Promise.resolve({ ...defaultValue!, status: 'success' });
	}
	const url = `${baseURL}/orders/details/${id}`;
	const options = {
		method: 'GET',
		credentials: credential_include
	};
	const request = new Request(url, options);
	const result = await fetch(request);
	let json = await result.json();
	// console.log(json);
	return json;
}

export async function getOrders(
	opt: number,
	page: number,
	limit: number,
	customerId: number,
	salesId: number,
	txt: string
): Promise<OrderProps> {
	const url = `${baseURL}/orders?opt=${opt}&page=${page}&limit=${limit}${opt === 1 ? `&txt=${txt}` : ''}${opt === 2 ? `&custid=${customerId}` : ''}${opt === 3 ? `&salesid=${salesId}` : ''}`;
	const options = {
		method: 'GET',
		credentials: credential_include
	};
	const request = new Request(url, options);
	const result = await fetch(request);
	const r = await result.json();
	return r;
}

export async function postCreateOrder(
	order: iOrder,
	details: iOrderDetail[]
): Promise<{ status: string; id: number; length: number }> {
	const url = `${baseURL}/orders`;
	const json = JSON.stringify({
		order: order,
		details: details
	});
	const options = {
		headers: {
			'content-type': 'application/json'
		},
		body: json,
		method: 'POST',
		credentials: credential_include
	};
	const request = new Request(url, options);
	const response = await fetch(request);
	const result = await response.json();
	return result;
}

export async function postUpdateOrder(
	id: number,
	order: iOrder,
	details: iOrderDetail[]
): Promise<{ status: string; id: number; length: number }> {
	const url = `${baseURL}/orders/${id}`;
	const json = JSON.stringify({
		order: order,
		details: details
	});
	const options = {
		headers: {
			'content-type': 'application/json'
		},
		body: json,
		method: 'PUT',
		credentials: credential_include
	};
	const request = new Request(url, options);
	const response = await fetch(request);
	const result = await response.json();
	return result;
}

export async function postUpdateOnlyOrder(
	id: number,
	order: iOrder
): Promise<{ status: string; id: number; length: number }> {
	const url = `${baseURL}/orders/update-only-order/${id}`;
	const json = JSON.stringify(order);
	const options = {
		headers: {
			'content-type': 'application/json'
		},
		body: json,
		method: 'PUT',
		credentials: credential_include
	};
	const request = new Request(url, options);
	const response = await fetch(request);
	const result = await response.json();
	return result;
}

export async function postDeleteOrder(
	ids: number[]
): Promise<{ status: string; data: number }> {
	const url = `${baseURL}/orders`;
	const json = JSON.stringify(ids);
	const options = {
		headers: {
			'content-type': 'application/json'
		},
		body: json,
		method: 'DELETE',
		credentials: credential_include
	};
	const request = new Request(url, options);
	const response = await fetch(request);
	const result = await response.json();
	return result;
}

export function toNumber(v: string | number | undefined): number {
	if (v == undefined || v === null) return 0;
	if (typeof v === 'string') {
		return +v;
	}
	return v;
}

export async function fetchGudangs(): Promise<iGudangResult> {
	const url = `${baseURL}/gudangs`;
	const options = {
		headers: {
			'content-type': 'application/json'
		},
		method: 'GET',
		credentials: credential_include
	};
	const request = new Request(url, options);
	let result = await fetch(request);

	return (await result.json()) as iGudangResult;
}
