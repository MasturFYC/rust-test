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
};
