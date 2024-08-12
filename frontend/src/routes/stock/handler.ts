import {
  baseURL,
  credential_include,
  type iStock,
  type iStockDetail,
} from "$lib/interfaces";
export async function getStockById(
  id: number,
  defaultValue?: { stock: iStock; details: iStockDetail[] },
): Promise<{ status: string; stock: iStock; details: iStockDetail[] }> {
  if (id === 0) {
    return await Promise.resolve({ ...defaultValue!, status: "success" });
  }

  const url = `${baseURL}/stocks/details/${id}`;
  const options = {
    method: "GET",
    credentials: credential_include,
  };
  const request = new Request(url, options);
  const result = await fetch(request);
  let json = await result.json();
  return json;
}

export async function getStocks(
  opt: number,
  page: number,
  limit: number,
  supplierId: number,
  warehouseId: number,
  txt: string,
): Promise<{
  status: string;
  stocks: iStock[];
  totalItems: number;
  totalPages: number;
  count: number;
  currentPage: number;
}> {
  const url = `${baseURL}/stocks?opt=${opt}&page=${page}&limit=${limit}${opt === 1 ? `&txt=${txt}` : ""}${opt === 2 ? `&supid=${supplierId}` : ""}${opt === 3 ? `&wareid=${warehouseId}` : ""}`;
  const options = {
    method: "GET",
    credentials: credential_include,
  };
  const request = new Request(url, options);
  const result = await fetch(request);
  return await result.json();
}

export async function postCreateStock(
  stock: iStock,
  details: iStockDetail[],
): Promise<{ status: string; id: number; length: number }> {
  const url = `${baseURL}/stocks`;
  const json = JSON.stringify({
    stock: stock,
    details: details,
  });

  const options = {
    headers: {
      "content-type": "application/json",
    },
    body: json,
    method: "POST",
    credentials: credential_include,
  };
  const request = new Request(url, options);
  const response = await fetch(request);
  const result = await response.json();
  return result;
}

export async function postUpdateStock(
  id: number,
  stock: iStock,
  details: iStockDetail[],
): Promise<{ status: string; id: number; length: number }> {
  // console.log(id, stock,details);
  const url = `${baseURL}/stocks/${id}`;
  const json = JSON.stringify({
    stock: stock,
    details: details,
  });

  const options = {
    headers: {
      "content-type": "application/json",
    },
    body: json,
    method: "PUT",
    credentials: credential_include,
  };
  const request = new Request(url, options);
  const response = await fetch(request);
  const result = await response.json();
  return result;
}

export async function postDeleteStock(
  ids: number[],
): Promise<{ status: string; data: number }> {
  // console.log(id, stock,details);
  const url = `${baseURL}/stocks`;
  const json = JSON.stringify(ids);

  const options = {
    headers: {
      "content-type": "application/json",
    },
    body: json,
    method: "DELETE",
    credentials: credential_include,
  };
  const request = new Request(url, options);
  const response = await fetch(request);
  const result = await response.json();
  return result;
}

export function toNumber(v: string | number | undefined): number {
  if (v == undefined || v === null) return 0;
  if (typeof v === "string") {
    return +v;
  }
  return v;
}
