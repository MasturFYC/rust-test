use bigdecimal::BigDecimal;
use uuid::Uuid;

use super::{LedgerDetail, LedgerDetailBuilder};

pub struct LedgerUtil {}

impl LedgerUtil {
    /// <b>dp</b> = Pembayaran awal waktu transaksi<br/>
    /// <b>modal</b> = total dari penjumlahan hpp detail order<br/>
    /// <b>ref_id</b> = order id<br/>
    /// <b>ledger_id</b> = <id>ledger id, biasanya sama dengan order id
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
            .with_account_id(421_i16)
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
                .with_account_id(101_i16)
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
                .with_account_id(111_i16)
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
                    .with_account_id(101_i16)
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
            .with_account_id(106_i16)
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
            .with_account_id(521_i16)
            .with_direction(1_i16)
            .with_amount(hpp.to_owned())
            .with_descriptions("Biaya Beli Barang")
            .build()
            .unwrap();

        details.push(detail);

        (details, i as usize)
    }
}
