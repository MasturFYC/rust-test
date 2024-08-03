export const format = (node: HTMLInputElement, formatFunction: (value: string) => string) => {
  function updateValue(e: Event) {
    node.value = formatFunction(node.value)
  }

  node.addEventListener('input', updateValue)
  node.addEventListener('paste', updateValue)

  // Format on intial hydration
  node.value = formatFunction(node.value)

  return {
    destroy() {
      node.removeEventListener('input', updateValue)
      node.removeEventListener('paste', updateValue)
    }
  }

}

export const cardNumber = (value: string | number) => {
  let test = "0";

  if(typeof value === "number") {
		test = value.toLocaleString("id-ID");
  } else {
	  test = value;
  }

  const regex = /^(\d{0,3})(\d{0,3})(\d{0,3})(\d{0,3})(\d{0,3})(\d{0,3})$/g
  const onlyNumbers = test.replace(/[^\d]/g, '')
  const len = onlyNumbers.length % 3
  let s = onlyNumbers.substring(0, len)
  return onlyNumbers.substring(len).replace(regex, (regex, $1, $2, $3, $4, $5, $6) =>
    [s, $1, $2, $3, $4, $5, $6].filter(group => !!group).join('.')
  )
}

export const cardPercent = (value: string | number) => {
  let test2 = "0";

  if(typeof value === "number") {
		test2 = value.toLocaleString("id-ID");
  } else {
	  test2 = parseFloat(value).toLocaleString("id-ID");
  }

	const onlyNumbers = test2.replace(/[^\d^\,^\.]/g, '')
	let foundComa = false;
	let test = "";
	let stop = 0;
	for(let i = 0; i < onlyNumbers.length; i++) {
		if(stop === 4) break;
		if(foundComa) {
			stop++;
		}
		if(onlyNumbers[i] === "." || onlyNumbers[i] === ",") {
			if(!foundComa) {
				test = test + ",";
				foundComa = true;
			}
		} else {
			test = test + onlyNumbers[i];
		}
	}

	return test;
}

export const getPercent = (value: string) => {
  // const onlyNumbers = value.replace(/[^\d^\,]/g, '');
  return +(value.split(",").join("."));
}


export const cardNumberNoDecimal = (value: string) => {
  return value.replace(/[^\d]/g, '')
}
export const getNumber = (value: string): number => {
  //const regex = /^(\d{0,3})(\d{0,3})(\d{0,3})(\d{0,3})$/g
  const onlyNumbers = value.replace(/[^\d]/g, '')
  return +onlyNumbers
}
