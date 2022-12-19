
#[derive(Debug)] // でバグ用の出力を自動で実装する
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct  Rectangle{
    width :u32,
    height : u32,
}
impl Rectangle {
    // selfがない場合は::で呼び出す
    fn create(width: u32, height: u32) -> Self{
        Self {width, height}
    }
    // &self とすると インスタンスメソッドとして呼び出すことができる
    // selfとすると、areaに所有権が写ってしまい使えなくなるので参照で渡す
    fn area(&self){
        println!("{}",self.width*self.height)
    }
}

pub fn run(){
     let user1 = User{
        email: String::from ("somepne@example.com"),
        username: String::from ("someusername123"),
        active: true,
        sign_in_count: 1,
     };
    //  let user2 = user1;
    //  println!("{:?}",user1);
    let mut user1 = User{
        email: String::from ("somepne@example.com"),
        username: String::from ("someusername123"),
        active: true,
        sign_in_count: 1,
     };
     user1.email = String::from("anotheremail@example.com");
     println!("{:#?}", user1);

     let user2 = build_user(String::from("email@example.com"), String::from("username"));
     println!("{:#?}", user2);

     let rect = Rectangle::create(20,20);
     println!("{:#?}", rect);
     rect.area();
     println!("{:#?}", rect);


}

// 構造体からインスタンスを生成する関数
fn build_user(email: String, username: String) -> User{
    User { username: username,
           email: email,
           sign_in_count: 1,
           active: true }
}