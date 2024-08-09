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
  dueAt?: string;
  createdAt?: string;
  updatedAt?: string;
  supplierName?: string;
  warehouseName?: string;
  isModified?: boolean;
  isDetailChanged?: boolean;
  isPayed?: boolean;
};

export type iStockDetail = {
  stockId: number;
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
};
