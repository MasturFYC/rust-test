use bigdecimal::BigDecimal;
use builder::mod_struct::Coa;
use uuid::Uuid;

use super::{
    // builder::{Coa, LedgerDetailBuilder},
    LedgerDetail,
};

// pub struct LedgerUtil {}

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
    #[allow(dead_code)]
    pub fn from_order(
        total: &BigDecimal,
        dp: &BigDecimal,
        hpp: &BigDecimal,
        ref_id: Uuid,
        ledger_id: Uuid,
    ) -> (Vec<LedgerDetail>, usize) {
        let mut details: Vec<LedgerDetail> = Vec::new();
        let mut i: i16 = 1;
        let remain = total - dp;
        let pass = bigdecimal::BigDecimal::from(0);

        let detail = builder::LedgerUtil::LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Revenue)
            .with_direction(-1)
            .with_amount(total.to_owned())
            .with_descriptions("Penjualan barang")
            .build();

        details.push(detail);

        if remain.le(&pass) {
            i += 1;

            let detail = LedgerDetailBuilder::default()
                .with_ref_id(ref_id)
                .with_ledger_id(ledger_id)
                .with_id(i)
                .with_account_id(Coa::Cash)
                .with_direction(1)
                .with_amount(total.to_owned())
                .with_descriptions("Cash payment")
                .build();

            details.push(detail);
        } else {
            // sisa pembayaran
            i += 1;
            let detail = LedgerDetailBuilder::default()
                .with_ref_id(ref_id)
                .with_ledger_id(ledger_id)
                .with_id(i)
                .with_account_id(Coa::Loan)
                .with_direction(1)
                .with_amount(remain)
                .with_descriptions("Piutang barang")
                .build();

            details.push(detail);

            // jika ada pembayaran
            if dp.gt(&pass) {
                i += 1;
                let detail = LedgerDetailBuilder::default()
                    .with_ref_id(ref_id)
                    .with_ledger_id(ledger_id)
                    .with_id(i)
                    .with_account_id(Coa::Cash)
                    .with_direction(1)
                    .with_amount(dp.to_owned())
                    .with_descriptions("Cash DP")
                    .build();

                details.push(detail);
            }
        }

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Inventory)
            .with_direction(-1)
            .with_amount(hpp.to_owned())
            .with_descriptions("Persediaan barang")
            .build();

        details.push(detail);

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::GoodCost)
            .with_direction(1)
            .with_amount(hpp.to_owned())
            .with_descriptions("Biaya Beli Barang")
            .build();

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
    #[allow(dead_code)]
    pub fn from_order_payment(
        amount: &BigDecimal,
        ref_id: Uuid,
        ledger_id: Uuid,
    ) -> (Vec<LedgerDetail>, usize) {
        let mut details: Vec<LedgerDetail> = Vec::new();
        let mut i: i16 = 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Cash)
            .with_direction(1)
            .with_amount(amount.to_owned())
            .with_descriptions("Titip bayar")
            .build();

        details.push(detail);

        i += 1;

        let detail = LedgerDetailBuilder::default()
            .with_ref_id(ref_id)
            .with_ledger_id(ledger_id)
            .with_id(i)
            .with_account_id(Coa::Loan)
            .with_direction(-1)
            .with_amount(amount.to_owned())
            .with_descriptions("Piutang penjualan")
            .build();

        details.push(detail);

        (details, i as usize)
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;
    use builder;

    #[allow(dead_code)]
    #[test]
    fn test_create_schema_builder() {
        let amount = bigdecimal::BigDecimal::from(25_000);
        let (data, _) = builder::LedgerUtil::from_order_payment(&amount, Uuid::new_v4(), Uuid::new_v4());

        for (_, d) in data.iter().enumerate() {
            println!("{:#?}", d);
        }
    }

    #[allow(dead_code)]
    #[test]
    fn test_test_library() {
        let amount = bigdecimal::BigDecimal::from(25_000);
        let (data, _) = builder::LedgerUtil::from_order_payment(&amount, Uuid::new_v4(), Uuid::new_v4());
        for (_, d) in data.iter().enumerate() {
            println!("{:#?}", d);
        }
    }
}
