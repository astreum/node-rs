use astro_format::arrays;

use super::Block;

impl Block {

    pub fn to_bytes(&self) -> Vec<u8> {

        let transactions: Vec<Vec<u8>> = self.transactions.iter().map(|x| x.to_bytes()).collect();

        let mut transactions_refs = Vec::new();

        for (i,_) in transactions.iter().enumerate() {
            transactions_refs.push(&transactions[i][..])
        }

        arrays::encode(&[
            &self.accounts_hash,
            &self.chain.to_bytes(),
            &self.number.to_bytes(),
            &self.previous_block_hash,
            &self.receipts_hash,
            &self.signature,
            &self.solar_price.to_bytes(),
            &self.solar_used.to_bytes(),
            &self.time.to_bytes(),
            &arrays::encode(&transactions_refs),
            &self.validator
        ])

    }

}
