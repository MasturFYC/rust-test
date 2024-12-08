export type iOrder = {
	id: number;
	customerId: number;
	salesId: number;
	paymentType: string;
	updatedBy: string;
	total: number;
	dp: number;
	payment: number;
	remain: number;
	dueRange: number;
	dueAt: string;
	isProtected?: boolean;
	createdAt: string;
	updatedAt: string;
	customerName?: string;
	salesName?: string;
	isModified?: boolean;
	isDetailChanged?: boolean;
	isPayed?: boolean;

};

export type iOrderDetail = {
	orderId: number;
	id: number;
	productId: number;
	barcode: string;
	name: string;
	qty: number;
	direction: number;
	unit: string;
	hpp: number;
	price: number;
	discount: number;
	subtotal: number;
	oldQty: number;
	gudangId: number;
	gudangName: string;
	oldGudangId: number;
};
