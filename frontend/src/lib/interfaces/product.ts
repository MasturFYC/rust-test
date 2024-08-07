export type iCategory = {
  id: number;
  name: string;
};

export type iProduct = {
  id: number;
  supplierId: number;
  categoryId: number;
  name: string;
  barcode: string;
  unit: string;
  unitInStock: number;
  content: number;
  hpp: number;
  margin: number;
  price: number;
  ppn: number;
  heavy: number;
  isActive: boolean;
  variantName?: string;
  descriptions?: string;
  createdAt: string;
  updatedAt: string;
  categoryName?: string;
  supplierName?: string;
};
