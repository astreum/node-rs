use crate::blocks::Block;
use opis::Int;

impl Block {

    pub fn next_solar_price(&self) -> Int {

        if self.solar_used > Int::from_decimal("900000000000") {
                                
            &self.solar_price + &Int::one()
        
        } else if self.solar_used < Int::from_decimal("100000000000") {
            
            if self.solar_price == Int::one() {
                
                self.solar_price.clone()
            
            } else {
                
                &self.solar_price - &Int::one()
            
            }

        } else {
            
            self.solar_price.clone()
        
        }
        
    }
    
}
