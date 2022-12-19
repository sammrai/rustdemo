trait Fruits{
    //抽象クラス。継承先で具体化する
    fn price(&self) -> u32;
}

struct Apple;

impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

// トレイトにおけるデフォルト関数
trait Summary{
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

trait Message{
    fn message(&self ) -> String{
        String::from("Message")
    }
}


struct NewsArticle{
    headline:  String,
    location:  String,
    author:  String,
    content:  String,
}
impl  Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author,self.location)
    // }
}
impl Message for NewsArticle {
}


struct Tweet{
    username: String,
    content: String,
    repry: bool,
    retweet: bool,
}


impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run(){
//トレイト：複数の型タイプに対して共通の機能を持たせることができる 
    let apple = Apple{};
    let banana: Banana = Banana{};
    get_price(apple);
    get_price(banana);

    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already now, peaple."),
        repry: false,
        retweet: false,
    };
    println!("1 new teet: {}",tweet.summarize());
     
    let article = NewsArticle{
        headline:  String::from("some headline"),
        location:  String::from("some location"),
        author:  String::from("some author"),
        content:  String::from("some content"),
    };
    println!("{}",article.summarize());
    nortify(&article);

    // tweet はmessageを実装していないのでエラーになる
    // nortify_another(&tweet);
    nortify_another(&article);

}
fn get_price<T: Fruits>(fruis: T){
    println!("price is {}", fruis.price());
}

// トレイトSummary を実装しているデータ型であれば使用できる
fn nortify(item: &impl Summary){
    println!("Breakng news! {}", item.summarize());
}

// 引数で受け取れるデータ型に、Summary+Messageのどちらも実装しているデータ型のみに制限
fn nortify_another(item: &(impl Summary+Message)){
    println!("#Breakng news! {}", item.summarize());
    println!("#Message! {}", item.message());

}