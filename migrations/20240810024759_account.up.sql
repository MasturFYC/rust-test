-- Add up migration script here
-- INSERT INTO accounts (id, name, normal, en_name)
--     VALUES (101, 'Kas', 1, 'Cash');
-- INSERT INTO accounts (id, name, normal, en_name)
--     VALUES (106, 'Persediaan barang', 1, 'Inventory');
-- INSERT INTO accounts (id, name, normal, en_name)
--     VALUES (111, 'Piutang penjualan', 1, 'Loan');
-- INSERT INTO accounts (id, name, normal, en_name)
--     VALUES (204, 'Utang dagang', -1, 'Account payable');
-- INSERT INTO accounts (id, name, normal, en_name)
--     VALUES (421, 'Penjualan barang', -1,  'Revenue');
-- INSERT INTO accounts (id, name, normal, en_name)
--     VALUES (521, 'Biaya beli barang', 1,  'Cost of goods sold');

ALTER TABLE ledger_details
    ALTER COLUMN amount TYPE NUMERIC(12,2);
