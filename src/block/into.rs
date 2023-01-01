use opis::Integer;

use super::Block;

impl Into<Vec<u8>> for &Block {

    fn into(self) -> Vec<u8> {

        let chain_bytes: Vec<u8> = (&self.chain).into();

        let number_bytes: Vec<u8> = (&self.number).into();

        let solar_used_bytes: Vec<u8> = Integer::from(&self.solar_used).into();

        let time_bytes: Vec<u8> = Integer::from(&self.time).into();

        let tx_bytes: Vec<Vec<u8>> = self.transactions.iter().map(|x| x.into()).collect();

        astro_format::encode(&[
            &self.accounts_hash.to_vec(),
            &chain_bytes,
            &self.data,
            &self.delay_output,
            &number_bytes,
            &self.previous_block_hash.to_vec(),
            &self.receipts_hash.to_vec(),
            &self.signature.to_vec(),
            &solar_used_bytes,
            &time_bytes,
            &astro_format::encode(
                &(tx_bytes
                    .iter()
                    .map(|x| x.as_slice())
                    .collect::<Vec<_>>()
                )
            ),
            &self.transactions_hash.to_vec(),
            &self.validator.0.to_vec()
        ])

    }

}

impl Into<Vec<u8>> for Block {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}
