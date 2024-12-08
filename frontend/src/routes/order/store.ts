import type { iOrder, iOrderDetail } from '$lib/interfaces';
import { writable } from 'svelte/store';
import dayjs from 'dayjs';
import { browser } from '$app/environment';

const dueRange = 7;
const tgl = dayjs();
const next_tgl = dayjs().add(dueRange, 'day');

export const initOrder: iOrder = {
	id: 0,
	customerId: 0,
	salesId: 0,
	paymentType: '',
	updatedBy: '',
	total: 0,
	dp: 0,
	payment: 0,
	remain: 0,
	dueAt: next_tgl.format(),
	createdAt: tgl.format(),
	updatedAt: tgl.format(),
	dueRange: dueRange
};

let persistedOrder = browser && localStorage.getItem('persistedOrder');
let persistedDetails = browser && localStorage.getItem('persistedDetails');
// console.log('ORDER', persistedOrder);

export const order = writable<iOrder>(
	persistedOrder ? JSON.parse(persistedOrder) : { ...initOrder }
);
export const details = writable<iOrderDetail[]>(
	persistedDetails ? JSON.parse(persistedDetails) : []
);
export const isOrderUpdating = writable(false);
export const isOrderLoading = writable(false);

if (browser) {
	order.subscribe((o) =>
		localStorage.setItem('persistedOrder', JSON.stringify(o))
	);

	details.subscribe((o) =>
		localStorage.setItem('persistedDetails', JSON.stringify(o))
	);
}
