import axios from "../axios-base";
import type { iBebanInfo } from "../interfaces";

const searchendpoint = "/beban/find";

async function fetchBebans(ids: number[], m: number, y: number) {
  const { data } = await axios.post<iBebanInfo[]>(
    `${searchendpoint}/${m}/${y}`,
    { ids: ids },
    { headers: { "Content-Type": "application/json" } },
  );
  return data;
}

export const queryBebanOptions = (ids: number[], m: number, y: number) => ({
  queryKey: ["beban", "list", { ids: ids, m: m, y: y }],
  queryFn: async () => await fetchBebans(ids, m, y),
});
