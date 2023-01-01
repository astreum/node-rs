use super::Transaction;

impl Into<Vec<u8>> for &Transaction {

    fn into(self) -> Vec<u8> {

        let chain_bytes: Vec<u8> = (&self.chain).into();

        let counter_bytes: Vec<u8> = (&self.counter).into();

        let value_bytes: Vec<u8> = (&self.value).into();

        astro_format::encode(&[
            &chain_bytes,
            &counter_bytes,
            &self.data,
            &self.recipient.0,
            &self.sender.0,
            &value_bytes,
        ])

    }

}

impl Into<Vec<u8>> for Transaction {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}
