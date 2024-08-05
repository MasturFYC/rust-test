export type iStock = {
	id: number;
	supplierId: number;
	warehouseId: number;
	paymentType: string;
	updatedBy: string;
	total: number;
	dp: number;
	payment: number;
	remain: number;
	invoiceId: string;
	dueRange: number;
	dueAt: string;
	createdAt: string;
	updatedAt: string;
	supplierName?: string;
	warehouseName?: string;
}
