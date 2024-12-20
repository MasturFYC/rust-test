<script lang="ts">
	import { formatNumber, getNumber } from "$lib/components/NumberFormat";
	import NumberPercent from "$lib/components/NumberPercent.svelte";
    import { baseURL, credential_include, type iProduct } from "$lib/interfaces";
	import { Button, Column, Row } from "carbon-components-svelte";
	import { Add } from "carbon-icons-svelte";

    interface Props {
        onadd: (p: iProduct, qty: number) => void,
        product: iProduct
    }
    let {
        product,
        onadd
    }: Props = $props();
	
    async function findProduct(e: string) {

		const url = `${baseURL}/products/barcode/${e}`;

		const options = {
			headers: {
				'content-type': 'application/json'
			},
			method: 'GET',
			credentials: credential_include
		};

		const request = new Request(url, options);
		let result = await fetch(request);

        if (result.ok) {
			let json = await result.json();
            console.log(json);
			let p = json.data;
               onadd(p, getNumber(qty));
		}
	}

    let qty = $state('1');
</script>
<Row>
    <Column noGutterRight md={2}><div class="top-16">{product.barcode}</div></Column>
    <Column><div class="top-16">{product.name}</div></Column>
    <Column as><div class="top-16">{formatNumber(product.price)}</div></Column>
    <Column as><div class="min-60 margin-16 top-16">{product.stocks[0].qty} {product.unit}</div></Column>
    <Column as>
        <div class="margin-16" title="Qty">
        <NumberPercent                
            bind:value={qty}
            on:change={(e) => {
                const el = e.currentTarget as HTMLInputElement;
                if(el) {
                    qty = el.value;
                }
            }}
        />
        </div>
    </Column>
    <Column noGutter as>
        <div class="right-16">
        <Button onclick={() => findProduct(product.barcode)} size="small" icon={Add}>Tambahkan</Button>
        </div>
    </Column>
</Row>

<style lang="scss">
.margin-16 {
    margin-left: 24px;
    width: 64px;
}
.top-16 {
    margin-top: 8px;
}
.right-16 {
    margin-right: 16px;
}
.min-60 {
    min-width: 90px;
}
</style>
