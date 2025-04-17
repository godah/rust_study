
fn main(){

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    route(&home);
    route(&loopback);


    println!("Coin: {}",value_in_cents(Coin::Penny));
    println!("Coin: {}",value_in_cents(Coin::Nickel));
    println!("Coin: {}",value_in_cents(Coin::Dime));
    println!("Coin: {}",value_in_cents(Coin::Quarter));


   
    

}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}



fn route(ip_kind: &IpAddr) {
    println!("Routing IP: {}, kind: {:?}", ip_kind.address, ip_kind.kind);
}



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


