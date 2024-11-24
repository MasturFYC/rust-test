import axios from "../axios-base";
import type { iCustomer, iMember, iTrx } from "../interfaces";

const endpoint = "/customer/list";

async function fetchCustomers(txt: string) {
  if (!txt) return [];
  const { data } = await axios.get<iCustomer[]>(endpoint + "/" + txt);
  return data;
}

export const queryCustomersOptions = (txt: string) => ({
  queryKey: ["customer", "list", { name: txt }],
  queryFn: async () => await fetchCustomers(txt),
});

async function fetchIndexCustomers() {
  const { data } = await axios.get<iCustomer[]>("/customer/index");
  return data;
}

export const queryIndexCustomersOptions = () => ({
  queryKey: ["customer", "list"],
  queryFn: async () => await fetchIndexCustomers(),
});

async function fetchMembers(type_id: number) {
  const end_point = `/member/list/${type_id}`;
  const { data } = await axios.get<iMember[]>(end_point);
  return data;
}

export const queryMembersOptions = (type_id: number) => ({
  queryKey: ["member", "list", { type: type_id }],
  queryFn: async () => await fetchMembers(type_id),
});

async function fetchMemberNonCustomers() {
  const end_point = "/member/list/non-customer";
  const { data } = await axios.get<iMember[]>(end_point);
  return data;
}

export const queryMemberNonCustomerOptions = () => ({
  queryKey: ["member", "list", { type: 0 }],
  queryFn: async () => await fetchMemberNonCustomers(),
});

async function fetchCustomerTrx(cust_id: number) {
  const { data } = await axios.get<iTrx[]>(`/customer/trx/${cust_id}`);
  return data;
}

export const queryCustomerTrxOptions = (cust_id: number) => ({
  queryKey: ["member", "trx", cust_id],
  queryFn: async () => await fetchCustomerTrx(cust_id),
});
