const Angka: string[] = [
	'', // 0
	'Satu', // 1
	'Dua', // 2
	'Tiga', // 3
	'Empat', // 4
	'Lima', // 5
	'Enam', // 6
	'Tujuh', // 7
	'Delapan', // 8
	'Sembilan', // 9
	'Sepuluh', // 10
	'Sebelas', // 11
	'Seratus', // 12
	'Seribu', // 13
	'Puluh', // 14
	'Belas', // 15
	'Ratus', // 16
	'Ribu', // 17
	'Juta', // 18
	'Milyar', // 19
	'Trilyun' // 20
];

function numberToText(e: string): string | undefined {
	if (e === '0' || e === null || isNaN(parseInt(e, 10))) {
		return satuan(0);
	}

	//const l = e.length;

	//return [fractionText(e).length, fractionText(e)].join(".");
	const a = fractionText(e);
	const l = a.length;

	if (l === 1) {
		const s = '' + a.at(0);
		if (s.length === 1) return satuan(parseInt(s, 10));
		if (s.length === 2) return puluhan(parseInt(s, 10));
		return ratusan(parseInt(s, 10));
	}
	if (l === 2) return ribuan(a);
	if (l === 3) return jutaan(a);
	if (l === 4) return milyaran(a);
	if (l === 5) return trilyunan(a);
	if (l === 6) return ngibul(a);
	if (e.length > 18) return 'Number to long.';
	return satuan(0);

	//if (l === 1) return satuan(parseInt(e, 10));
	// if (l === 2) return puluhan(parseInt(e, 10));
	// if (l === 3) return ratusan(parseInt(e,10));
	// if (l >= 4 && l <= 6) return ribuan(e, l);
	// if (l >= 7 && l <= 9) return jutaan(e, l);
	// if (l >= 10 && l <= 12) return milyaran(e, l);
	// if (l >= 13 && l <= 15) return trilyunan(e, l);
	// if (l >= 16 && l <= 18) return ngibul(e, l);
	// if (l > 18) return "Number to long.";
	// return satuan(0);
}

function satuan(e: number): string {
	return '' + Angka.at(e);
}

function puluhan(e: number): string {
	const t: string = '' + e;
	if (t.length <= 1) return satuan(e);

	const a = parseInt(t.substring(0, 1), 10);
	const b = parseInt(t.substring(1, 2), 10);

	if (a === 0) return satuan(b);

	if (a === 1) {
		switch (b) {
			case 0:
				return satuan(10);
			case 1:
				return satuan(11);
			default:
				return [satuan(b), satuan(15)].join(' ');
		}
	}

	if (b === 0) {
		return [satuan(a), satuan(14)].join(' ');
	}
	return [satuan(a), satuan(14), satuan(b)].join(' ');
}

function ratusan(e: number): string {
	const t = '' + e;

	if (t.length <= 2) return puluhan(e);

	const a = parseInt(t.substring(0, 1), 10);
	const puluh = parseInt(t.substring(1, 3), 10);

	// if (a === 0) return puluhan(puluh);

	if (a === 1) {
		return [satuan(12), puluhan(puluh)].join(' ');
	}
	return [satuan(a), satuan(16), puluhan(puluh)].join(' ');
}

function ribuan(arr: string[]): string {
	const a = parseInt('' + arr.at(0), 10);
	const b = parseInt('' + arr.at(1), 10);
	return [ribu(a), ratusan(b)].join(' ');
}

function jutaan(arr: string[]): string {
	const a = parseInt('' + arr.at(0), 10);
	const b = parseInt('' + arr.at(1), 10);
	const c = parseInt('' + arr.at(2), 10);
	return [ratusan(a), a === 0 ? satuan(0) : satuan(18), ribu(b), ratusan(c)].join(' ');
}

function milyaran(arr: string[]): string {
	const a = parseInt('' + arr.at(0), 10);
	const b = parseInt('' + arr.at(1), 10);
	const c = parseInt('' + arr.at(2), 10);
	const d = parseInt('' + arr.at(3), 10);
	return [
		ratusan(a),
		a === 0 ? satuan(0) : satuan(19),
		ratusan(b),
		b === 0 ? satuan(0) : satuan(18),
		ribu(c),
		ratusan(d)
	].join(' ');
}

function trilyunan(arr: string[]): string {
	const a = parseInt('' + arr.at(0), 10);
	const b = parseInt('' + arr.at(1), 10);
	const c = parseInt('' + arr.at(2), 10);
	const d = parseInt('' + arr.at(3), 10);
	const x = parseInt('' + arr.at(4), 10);
	return [
		ratusan(a),
		a === 0 ? satuan(0) : satuan(20),
		ratusan(b),
		b === 0 ? satuan(0) : satuan(19),
		ratusan(c),
		c === 0 ? satuan(0) : satuan(18),
		ribu(d),
		ratusan(x)
	].join(' ');
}

function ngibul(arr: string[]): string {
	// const a = e.substring(0, l - 12);
	// const b = e.substring(l - 12, l);
	// return "---"
	return [
		ribuan(['' + arr.at(0), '' + arr.at(1)]),
		satuan(20),
		milyaran(['' + arr.at(2), '' + arr.at(3), '' + arr.at(4), '' + arr.at(5), '' + arr.at(6)])
	].join(' ');
}

function fractionText(t: string): string[] {
	const l = t.length;
	let arr: string[] = [];
	arr.push(
		t.substring(l - 18, l - 15),
		t.substring(l - 15, l - 12),
		t.substring(l - 12, l - 9),
		t.substring(l - 9, l - 6),
		t.substring(l - 6, l - 3),
		t.substring(l - 3, l)
	);
	//const reg = new RegExp(/\.*/, "i");
	return arr.filter((e) => e);
}

function ribu(e: number): string {
	return e === 0 ? satuan(0) : e === 1 ? satuan(13) : [ratusan(e), satuan(17)].join(' ');
}

export { numberToText, fractionText };
