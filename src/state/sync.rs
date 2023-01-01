use super::State;

impl State {

    pub fn sync(&self) {

        println!("syncing ...");

        self.messages();

        self.update();

    }
}