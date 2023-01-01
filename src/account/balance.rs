use opis::Integer;
use std::error::Error;
use super::Account;

impl Account {

    pub fn increase_balance(&mut self, amount: &Integer) {

        self.balance += amount;

    }

    pub fn decrease_balance(&mut self, amount:&Integer) -> Result<(), Box<dyn Error>> {

        if &self.balance >= amount {

            self.balance -= amount;

            Ok(())

        } else {

            Err("Not enough balance!")?

        }
    }

}
