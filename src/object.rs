use fides::{merkle_tree, hash::blake_3};

pub struct Object {
   leaf: bool,
   data: Vec<u8>
}

impl Object {

   pub fn hash(&self) -> [u8;32] {

      merkle_tree::root(
         blake_3,
         &[
            match self.leaf {
               true => &[1],
               false => &[0],
            },
            &self.data
         ]
      )

   }

   pub fn size(&self) -> usize {

      self.data.len()

   }

}
