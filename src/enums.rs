
enum OS{
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run(){
    let linux = OS::Linux(1992, String::from("linus"));
    print_os_info(linux);
    let windows = OS::Windows(1985, String::from("Microsoft"));
    print_os_info(windows);
    let mac = OS::Mac(2001, String::from("Apple"));
    print_os_info(mac);
    // println!("{:?}",linux);

}

fn print_os_info(os: OS){
    match os{
        OS::Windows(year,who ) =>{
            println!("Windows: {} {}",year,who);
        }
        OS::Mac(year,who ) =>{
            println!("Mac: {} {}",year,who);
        }
        OS::Linux(year,who ) =>{
            println!("Linux: {} {}",year,who);
        }
    }
}