import axios from "../axios-base";
import type { iProduct } from "../interfaces/product";

const endpoint = "/product/list";
const endpoint2 = "/product/list-by-category";

async function fetchProducts(page: number, limit: number) {
  const { data } = await axios.get<iProduct[]>(`${endpoint}/${page}/${limit}`);
  return data;
}

export const queryProductOptions = (page: number, limit: number) => ({
  queryKey: ["product", "list"],
  queryFn: async () => await fetchProducts(page, limit),
});
