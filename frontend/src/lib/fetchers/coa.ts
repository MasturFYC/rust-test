import axios from "../axios-base";
import type { iAccount } from "../interfaces";

const endpoint = "/coa/list";

async function fetchCoas() {
  const { data } = await axios.get<iAccount[]>(endpoint);
  return data;
}

export const queryCoasOptions = () => ({
  queryKey: ["coa", "list"],
  queryFn: async () => await fetchCoas(),
});

const paymentendpoint = "/coa/payment";
async function fetchCoaPayments() {
  const { data } = await axios.get<iAccount[]>(paymentendpoint);
  return data;
}

export const queryCoaPaymentOptions = () => ({
  queryKey: ["coa", "payment"],
  queryFn: async () => await fetchCoaPayments(),
});
