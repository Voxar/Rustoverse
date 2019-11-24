mod allo;
use allo::{ Intent };

fn main() {
    println!("Hello, world!");
    
    let identity = allo::Identity { };
    let avatar = allo::Avatar { };
    
    let mut client = allo::Client::new(
        "allospace://nevyn.nu".to_string(),
        identity, 
        avatar
    );


    client.poll();
    client.poll();
    client.poll();
    client.poll();
    client.poll();
    client.poll();
    client.poll();
    client.poll();
}