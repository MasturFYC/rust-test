import axios from "../axios-base";
import type { iPaket } from "../interfaces";

const endpoint = "/paket/list";

async function fetchPakets() {
  const { data } = await axios.get<iPaket[]>(endpoint);
  return data;
}

export const queryPaketOptions = () => ({
  queryKey: ["paket", "list"],
  queryFn: async () => await fetchPakets(),
});
