import axios from '../axios-base';
import type { iTrx } from '../interfaces';

const endpoint = '/trx/list';

async function fetchTrxs() {
	const { data } = await axios.get<iTrx[]>(endpoint);
	return data;
}

export const queryTrxsOptions = () => ({
	queryKey: ['trx', 'list'],
	queryFn: async () => await fetchTrxs()
});

const searchendpoint = '/trx/find';

async function fetchFindTrxs(txt: string, m: number, y: number) {
	if (!txt) return [];
	const { data } = await axios.get<iTrx[]>(`${searchendpoint}/${txt}/${m}/${y}`);
	return data;
}

export const queryFindTrxsOptions = (txt: string, m: number, y: number) => ({
	queryKey: ['trx', { name: txt, m: m, y: y }],
	queryFn: async () => await fetchFindTrxs(txt, m, y)
});
