pub mod sub_a;
pub mod sub_b;

// 定数は大文字で定義
const MAX_POINTS: u32 = 100_000;

// ここでは変数を定義できない
// let x = 1;

pub fn run(){
    println!("here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("the value of x is {}",x);
    x = 6;
    println!("the value of x is {}",x);

    let i1 = 1;
    let f1 = 0.1;

    // PCに適したビット数を取得
    println!("{}", usize::BITS);

    // MAX_POINTSが格納されている
    println!("memory address of const is {:p}", &MAX_POINTS);

    // 8バイトのデータサイズが作成され、スタックに格納される i2 2バイトしかない。。
    let i2: i64 = 1;
    // let i4: i64 = 1;
    let i3: i64 = 2;
    println!("Stack adress of i2 is {:p}", &i2);
    println!("Stack adress of i3 is {:p}", &i3);

    // シャドーイング
    let  y= 5;
    println!("Stack adress of y is {:p}", &y);
    // 再度バインドすることで元のyは上書きされる
    let y = y+1;
    println!("Stack adress of y is {:p}", &y);
    let y = y*2;
    println!("Stack adress of y is {:p}", &y);
    println!("Stack value of y is {}", y);
    
    // 別のスコープで定義するとyはその中だけで有効
    {
        let y = 0;
        println!("Stack value of y is {}", y);
    }
    println!("Stack value of y is {}", y);

    // タプル型
    let t1 = (500, 6.4 ,"dummy");
    let (x,y,z) = t1;
    println!("the value of t1 is {} {} {}", t1.0, t1.1, t1.2);

    // 入れ子タプル
    // ref でポインタとして代入することができる
    // *をつけることでポインタの指す具体を扱うことができる
    let mut t2 = ((0,1),(2,3));
    // 0のポインタはref mut x1_ptr で取得できる。 _ は何も取得しないを意味する。
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    // 複雑な構造体はブラケットで?を書く。
    // int,float,str型はプリミティブ型と呼ばれ、{}だけで問題ない
    println!("{:?}", t2);

    // リスト
    // 配列はスタックにつまれる
    let a1 = [1,2,3,4,5];
    let a2 = [0;10];

    println!("{:?} {:?} {} {}", a1,a2, a1[2],a2[2]);

    // 文字列
    // hello 5
    // こんにちは挨拶 3x7 = 21
    let s1 = "helloこんにちは挨拶"; //26byte
    let s2 = "hello"; //5byte
    // s1 s2には　ポインタが入っているので、文字列の長さに応じていない？
    // 文字列は静的領域にスタックされている
    println!("stack address of s1 is {:p}", &s1);
    println!("stack address of s2 is {:p}", &s2);
    println!("static memory address of s1: {:?}",s1.as_ptr());
    println!("static memory address of s2: {:?}",s2.as_ptr());
    println!(" len of s1: {:?}",s1.len());
    println!(" len of s2: {:?}",s2.len());
    
    // 動的に文字列の長さを増やすことができないので、string型を使用する
    // mut を使って後でつかできるようにしておく
    // string型はheapに格納されている
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("s1 is {:p}", &s1);
    println!("s2 is {:p}", &s2);
    println!("static memory address of s1: {:?}",s1.as_ptr());
    println!("static memory address of s2: {:?}",s2.as_ptr());
    println!(" len of s1: {:?}",s1.len());
    println!(" len of s2: {:?}",s2.len());
    println!(" cap of s1: {:?}",s1.capacity());
    println!(" cap of s2: {:?}",s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!(" len of s1: {:?}",s1.len());
    println!(" len of s2: {:?}",s2.len());
    println!(" cap of s1: {:?}",s1.capacity());
    println!(" cap of s2: {:?}",s2.capacity());
    
}