import axios from "../axios-base";
import type { iRegion } from "../interfaces";

const endpoint = "/unit/list";

async function fetchRegions() {
  const { data } = await axios.get<iRegion[]>(endpoint);
  return data;
}

export const queryRegionsOptions = () => ({
  queryKey: ["region", "list"],
  queryFn: async () => await fetchRegions(),
});
