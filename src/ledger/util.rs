use bigdecimal::BigDecimal;
use uuid::Uuid;

use super::{LedgerDetail, LedgerDetailBuilder};

// /// 106 - persediaan barang
// const ACC_INVENTORY: i16 = 0x6A;
// /// 521 - biaya beli barang
// const ACC_GOODS_COST: i16 = 0x209;
// /// 421 - penjualan barang
// // const ACC_REVENUE: i16 = 0x1A5;
// /// 111 - piutang penjualan
// const ACC_PIUTANG: i16 = 0x6F;
// /// 111 - kas
// const ACC_KAS: i16 = 0x65;

#[derive(Copy, Clone)]
pub enum Coa {
    /// 106 - persediaan barang
    Inventory = 0x6A,
    /// 521 - biaya beli barang
    GoodCost = 0x209,
    /// 421 - penjualan barang
    Revenue = 0x1A5,
    /// 111 - piutang penjualan
    Loan = 0x6F,
    /// 111 - kas
    Cash = 0x65,
}

impl Into<i16> for Coa {
    fn into(self) -> i16 {
        self as i16
    }
}

pub struct LedgerUtil {}

impl LedgerUtil {
    /// ## Ledger untuk transaksi order
    /// `(-) 106 - Persediaan barang` -> **hpp**
    ///
    /// `(+) 521 - Biaya beli barang` -> **hpp**
    ///
    /// `(-) 421 - Penjualan barang` -> **total**
    ///
    /// Tidak digunakan jika tidak ada pembayaran sama sekali dan diganti dengan
    /// *`111 - Piutang barang`*.
    ///
    /// `(+) 101 - Kas` -> **dp**
    ///
    /// Jika pembayaran tidak lunas atau tidak ada pembayaran sama sekali.
    /// Catat sisa pembayaran ke dalam piutang,
    /// data diambil dari total order dikurangi dp.
    /// Tidak digunakan jika *`dp`* lebih besar atau sama dengan *`total`*
    ///
    /// `(+) 111 - Piutang barang` -> (**total - dp**)
    ///
    /// **`ref_id`** order id
    ///
    /// **`ledger_id`** biasanya sama dengan order id
    ///
    pub fn from_order(
        total: &BigDecimal,
        dp: &BigDecimal,
        hpp: &BigDecimal,
        ref_id: Option<Uuid>,
        ledger_id: Uuid,
    ) -> (Vec<LedgerDetail>, usize) {
        let mut details: Vec<LedgerDetail> = Vec::new();
        let mut i: i16 = 1;
        let remain = total - dp;
        let pass = bigdecimal::BigDecimal::from(0);

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Revenue)
            .with_direction(-1_i16)
            .with_amount(total.to_owned())
            .with_descriptions("Penjualan barang")
            .build()
            .unwrap();

        details.push(detail);

        if remain.le(&pass) {
            i += 1;

            let detail = LedgerDetailBuilder::default()
                .with_ref_id(ref_id)
                .with_ledger_id(ledger_id)
                .with_id(i)
                .with_account_id(Coa::Cash)
                .with_direction(1_i16)
                .with_amount(total.to_owned())
                .with_descriptions("Cash payment")
                .build()
                .unwrap();
            details.push(detail);
        } else {
            // sisa pembayaran
            i += 1;
            let detail = LedgerDetailBuilder::default()
                .with_ref_id(ref_id)
                .with_ledger_id(ledger_id)
                .with_id(i)
                .with_account_id(Coa::Loan)
                .with_direction(1_i16)
                .with_amount(remain)
                .with_descriptions("Piutang barang")
                .build()
                .unwrap();
            details.push(detail);

            // jika ada pembayaran
            if dp.gt(&pass) {
                i += 1;
                let detail = LedgerDetailBuilder::default()
                    .with_ref_id(ref_id)
                    .with_ledger_id(ledger_id)
                    .with_id(i)
                    .with_account_id(Coa::Cash)
                    .with_direction(1_i16)
                    .with_amount(dp.to_owned())
                    .with_descriptions("Cash DP")
                    .build()
                    .unwrap();
                details.push(detail);
            }
        }

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Inventory)
            .with_direction(-1_i16)
            .with_amount(hpp.to_owned())
            .with_descriptions("Persediaan barang")
            .build()
            .unwrap();

        details.push(detail);

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::GoodCost)
            .with_direction(1_i16)
            .with_amount(hpp.to_owned())
            .with_descriptions("Biaya Beli Barang")
            .build()
            .unwrap();

        details.push(detail);

        (details, i as usize)
    }

    /// ## Ledger untuk transaksi pembayaran piutang
    ///
    /// `(+) 101 - Kas` -> **amount**
    ///
    /// `(-) 111 - Piutang barang` -> **amount**
    ///
    /// **`ref_id`** payment id
    ///
    /// **`ledger_id`** ledger id biasanya sama dengan payment id
    ///
    pub fn from_order_payment(
        amount: &BigDecimal,
        ref_id: Option<Uuid>,
        ledger_id: Uuid,
    ) -> (Vec<LedgerDetail>, usize) {
        let mut details: Vec<LedgerDetail> = Vec::new();
        let mut i: i16 = 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Cash)
            .with_direction(1_i16)
            .with_amount(amount.to_owned())
            .with_descriptions("Titip bayar")
            .build()
            .unwrap();

        details.push(detail);

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Loan)
            .with_direction(-1_i16)
            .with_amount(amount.to_owned())
            .with_descriptions("Piutang penjualan")
            .build()
            .unwrap();

        details.push(detail);

        (details, i as usize)
    }
}
