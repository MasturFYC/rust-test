import type { iOrder, iOrderDetail } from '$lib/interfaces';
import { writable } from 'svelte/store';
import dayjs from 'dayjs';

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

export const order = writable<iOrder>({ ...initOrder });
export const details = writable<iOrderDetail[]>([]);
export const isOrderUpdating = writable(false);
export const isOrderLoading = writable(false);
