import type { iStockDetail, iStock } from '$lib/interfaces';
import { writable } from 'svelte/store';
import dayjs from 'dayjs';

const dueRange = 7;
const tgl = dayjs();
const next_tgl = dayjs().add(dueRange, 'day');

export const initStock: iStock = {
	id: 0,
	supplierId: 0,
	warehouseId: 0,
	paymentType: '',
	updatedBy: '',
	total: 0,
	dp: 0,
	payment: 0,
	remain: 0,
	invoiceId: '',
	dueAt: next_tgl.format(),
	createdAt: tgl.format(),
	updatedAt: tgl.format(),
	dueRange: dueRange
};

export const stock = writable<iStock>({ ...initStock });
export const details = writable<iStockDetail[]>([]);
export const isStockUpdating = writable(false);
export const isStockLoading = writable(false);
