// リストの構造体を作ってみる
enum List{
    Node(i32, Box<List>),
    Nil
}


pub fn run(){
    // スタックオーバフローを発生させる
    // 7MBのデータを作成
    let a1: [u8; 7_000_000] = [1; 7_000_000];
    // スタックの容量上限は8MBなので、これを超える例えば9MBを指定するとスタックオーバフローが発生する
    // let a1: [u8; 9_000_000] = [1; 9_000_000];

    // ベクタ型は動的に要素数を変更することができる
    // 構造はstring型と同じだがcapには配列の長さが入る
    let mut  v1 = vec![1,2,3,4]; //24byte
    let      v2 = vec![5,6,7,8];
    let mut  v3 = vec![9,10];

    println!("stack address of v1 is {:p}",&v1);
    println!("stack address of v2 is {:p}",&v2);
    println!("ヒープ内の実データの先頭アドレス v1 is {:?}",v1.as_ptr());
    println!("len v1 is {:?}",v1.len());
    println!("cap v1 is {:?}",v1.capacity());

    v1.insert(1, 10);
    println!("{:?}",v1);

    v1.remove(0);
    println!("{:?}",v1);

    // 配列の結合 v3の先頭アドレスを &mutで渡す
    v1.append(&mut v3);
    println!("{:?}",v1);
    println!("{:?}",v3);
    
    // ボックスポインタ
    let t1: (i64, String) = (10, String::from("hello"));
    println!("stack address of tuple data is {:p}",&t1);
    println!("heap memory address of t1.1: {:?}",t1.1.as_ptr());
    println!("len of t1.1: {:?}",t1.1.len());
    println!("cap of t1.1: {:?}",t1.1.capacity());

    // ボックスポインタを作る
    let mut b1 = Box::new(t1);
    (*b1).1 += "world";
    println!("{} {}",b1.0,b1.1);
    println!("stack address of box pointer is {:p}",&b1);//番地
    println!("heap address of box pointer is {:p}",b1); // 中身

}