import { SvelteComponent } from "svelte";

declare module "svelte-qrcode" {
  // Define the types for svelte-qrcode here
  // For example:
  export interface QRCodeProps {
    value: string;
    size?: number;
    background?: string;
    color?: string;
    errorCorrection?: string;
    padding?: number;
  }

  export default class QRCode extends SvelteComponent<QRCodeProps> {}
}
