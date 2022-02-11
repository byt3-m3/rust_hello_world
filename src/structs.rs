// Traditional Struct 
struct Router {
    hostname: String,
    mgmt_ip: String

}

// Method declaration 
impl Router {
    // Constructor
    fn new(hostname: &str, mgmt_ip: &str) -> Router {
        Router {
            hostname: hostname.to_string(),
            mgmt_ip: mgmt_ip.to_string()
        }
      
    }

    // mutates state
    fn update_hostname(&mut self, hostname: &str) {
        self.hostname = hostname.to_string()
    }

    fn get_hostname(&self) -> String{
        format!("{}", self.hostname)
    }

    fn to_tuple(&self) -> (String, String){
        (format!("{}", self.hostname), format!("{}", self.mgmt_ip)) 

    }
}

// Tupel Struct
struct Firewall(String, String);

pub fn run() {
    let r1 = Router {
        hostname: String::from("R1"),
        mgmt_ip: String::from("192.168.1.6")
    };

    let mut r2 = Router::new("R2", "192.168.1.10");
    r2.update_hostname("R2-ORD");

    let filewall: Firewall = Firewall(String::from("FW"), String::from("192.1681.7"));

    println!("{} {}", r1.hostname, r1.mgmt_ip);
    println!("{} {}", r2.hostname, r2.mgmt_ip);

    println!("{} {}", filewall.0, filewall.1);
    println!("{:?}", r2.to_tuple());
    println!("{}", r2.get_hostname())

}