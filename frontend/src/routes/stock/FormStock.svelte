<style lang="css">
:global(label.bx--label) {
  margin-bottom: 3px;
  margin-top: 9px;
}
:global(.bx--checkbox-wrapper) {
  margin-top: 12px;
}
:global(.bx--text-input.input-number) {
  text-align: right;
}
:global(.bx--label--inline--sm) {
  min-width: 4.5rem;
  padding-right: 0;
  margin-right: 0;
}
:global(.bx--list-box__menu-item, .bx--list-box__menu-item__option) {
  height: auto;
}

.supplier-info {
  height: auto;
  font-size: x-small;
  margin-top: 3px;
  margin-bottom: 0;
  white-space: wrap;
}
</style>

<script lang="ts">
import type { iRelationProp } from "$lib/interfaces";
import {
  Column,
  ComboBox,
  DatePicker,
  DatePickerInput,
  Form,
  Grid,
  Row,
  TextInput,
} from "carbon-components-svelte";
import dayjs from "dayjs";
import { formatNumber, getNumber } from "$lib/components/NumberFormat";
import { stock } from "./store";
import { toNumber } from "./handler";
import type { ComboBoxItem } from "carbon-components-svelte/src/ComboBox/ComboBox.svelte";

// const dispatch = createEventDispatcher();
export let suppliers: iRelationProp[] = [];
export let employees: iRelationProp[] = [];
export let innerWidth = 0;

type DatePict =
  | string
  | {
      selectedDates: [dateFrom: Date, dateTo?: Date];
      dateStr:
        | string
        | {
            from: string;
            to: string;
          };
    };

let ref_invoice: HTMLInputElement;

function get_suppliers() {
  return suppliers.map((m) => ({ id: m.id, text: m.text }));
}
function get_employees() {
  return employees.map((m) => ({ id: m.id, text: m.text }));
}

function shouldFilterItem(item: ComboBoxItem, value: string) {
  if (!value) return true;
  return item.text.toLowerCase().includes(value.toLowerCase());
}
function get_employee_info(id: number): string {
  let item = suppliers.filter((f) => f.id === id)[0];

  if (item) {
    let info: string;

    if (item.street) {
      info = item.street;
      info = info + " - " + item.city;
    } else {
      info = item.city;
    }
    if (item.phone) {
      info = info + ", " + item.phone;
    }

    return info;
  }
  return "-";
}

function get_supplier_info(id: number): string {
  let item = suppliers.filter((f) => f.id === id)[0];

  if (item) {
    let info: string;

    if (item.street) {
      info = item.street;
      info = info + " - " + item.city;
    } else {
      info = item.city;
    }
    if (item.phone) {
      info = info + ", " + item.phone;
    }

    return info;
  }
  return "-";
}
function on_employee_changed(
  e: CustomEvent<{ selectedId: any; selectedItem: ComboBoxItem }>,
): void {
  const empl = employees.filter((f) => f.id === e.detail.selectedId)[0];
  if (empl) {
    stock.update((s) => ({
      ...s,
      warehouseId: empl.id,
      warehouseName: empl.text,
      isModified: true,
    }));
  }
}

function on_supplier_changed(
  e: CustomEvent<{ selectedId: any; selectedItem: ComboBoxItem }>,
): void {
  const empl = suppliers.filter((f) => f.id === e.detail.selectedId)[0];
  if (empl) {
    stock.update((s) => ({
      ...s,
      supplierId: empl.id,
      supplierName: empl.text,
      isModified: true,
    }));
  }
}
function on_employee_clear(e: any): void {
  stock.update((s) => ({ ...s, warehouseId: 0, warehouseName: undefined }));
}

function on_supplier_clear(e: any): void {
  stock.update((s) => ({ ...s, supplierId: 0, supplierName: undefined }));
}

function onDateChange(e: CustomEvent<DatePict>) {
  e.preventDefault();
  if (typeof e.detail === "string") {
  } else {
    let d = e.detail.selectedDates[0];
    let date = dayjs();
    date = date.set("date", d.getDate());
    date = date.set("month", d.getMonth());
    date = date.set("year", d.getFullYear());
    stock.update((s) => ({
      ...s,
      createdAt: date.format(),
      isModified: true,
    }));
  }
}

let strDate = dayjs($stock.createdAt).format("DD-MM-YYYY");
let strDp = formatNumber(toNumber($stock.dp));

$: if (ref_invoice) {
  ref_invoice.focus();
}

function updateDp(str: string) {
  const dp = getNumber(str);
  stock.update((s) => ({
    ...s,
    dp: dp,
    total: toNumber(s.total) - (toNumber(s.payment) + dp),
  }));
}
$: updateDp(strDp);
</script>

<Form on:submit style="margin: 24px 0 0 0;">
  <Grid noGutter={innerWidth > 720} fullWidth>
    <Row>
      <Column noGutterRight sm={2} md>
        <DatePicker
          datePickerType="single"
          bind:value={strDate}
          dateFormat="d-m-Y"
          on:change={onDateChange}
        >
          <DatePickerInput
            accesskey="t"
            style="max-width: 100%;min-width:150px"
            labelText="Tanggal pembelian"
            placeholder="mm/dd/yyyy"
          />
        </DatePicker>
      </Column>
      <Column noGutter sm={2} md lg>
        <TextInput
          accesskey="n"
          bind:ref={ref_invoice}
          id="invoice-id"
          labelText="No. faktur"
          on:change={() => stock.update((s) => ({ ...s, isModified: true }))}
          bind:value={$stock.invoiceId}
        />
      </Column>
      <Column noGutter md={2} sm={2}>
        <ComboBox
          accesskey="s"
          id="supplier-id"
          titleText="Supplier"
          selectedId={$stock.supplierId}
          placeholder="Pilih supplier"
          items={get_suppliers()}
          shouldFilterItem={shouldFilterItem}
          on:select={on_supplier_changed}
          on:clear={on_supplier_clear}
          let:item
        >
          <div><strong>{item.text}</strong></div>
          <div class="supplier-info">
            {get_supplier_info(item.id)}
          </div>
        </ComboBox>
      </Column>
      <Column noGutterLeft md={2} sm={2}>
        <ComboBox
          accesskey="g"
          id="warehouse-id"
          titleText="Checker"
          selectedId={$stock.warehouseId}
          placeholder="Nama pengechek"
          items={get_employees()}
          shouldFilterItem={shouldFilterItem}
          on:select={on_employee_changed}
          on:clear={on_employee_clear}
          let:item
        >
          <div><strong>{item.text}</strong></div>
          <div class="supplier-info">
            {get_employee_info(item.id)}
          </div>
        </ComboBox>
      </Column>
    </Row>
  </Grid>
</Form>
