
pub struct Interface {
    pub header: String,
    pub lines: Vec<String>
}

impl Interface {

    pub fn new() -> Self {
        Interface {
            header: String::new(),
            lines: Vec::new()
        }
    }

    pub fn push(&mut self, line: &str) {
        self.lines.push(line.to_string())
    }

    pub fn udpate(&mut self, lines: &Vec<&str>) {

        self.lines.clear();

        for line in lines {
            self.lines.push(line.to_string())
        }

    }

    pub fn refresh(&self) {

        print!("\x1Bc");

        println!("{}", self.header);
        
        for line in &self.lines {
            println!("{}", line)
        }

    	println!("- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -")

    }

}