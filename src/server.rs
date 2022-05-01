#[derive(Debug)]
pub struct Request {
    pub body: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>
}

impl Request {

    pub fn from(buffer: [u8; 1024]) -> Self {
        
        let request_text = String::from_utf8_lossy(&buffer[..]);

        let mut request: Request = Request {
            body: None,
            method: None,
            path: None,
            version: None
        };
        
        let lines: Vec<&str> = request_text.lines().collect::<Vec<_>>();
        
        let first_line: Vec<&str> = lines[0].split(" ").collect::<Vec<_>>();

        if first_line.len() == 3 {

            request.method = Some(first_line[0].to_string());

            request.path = Some(first_line[1].to_string());

            request.version = Some(first_line[2].to_string());
            
        }

        request

    }

}

#[derive(Debug)]
pub struct Response {
    pub body: String,
    pub status: String
}

impl Response {

    pub fn new() -> Self {
        Response {
            body: "{\"success\":\"true\"}".to_string(),
            status: "200 OK".to_string()
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {

        let response = format!(
            "HTTP/1.1 {}\nAccess-Control-Allow-Origin: *\nContent-Type: application/json\nContent-Length: {}\n\n{}",
            self.status,
            self.body.len(),
            self.body
        );

        println!("{}", response);

        response.into_bytes()

    }

}
