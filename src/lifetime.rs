pub fn run(){
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}",res1);
    // 大きい方を返す関数を作成数ｒ

    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}",res2);
    }
    
}

// ライフタイムアノテーション 'a
// 帰り値のライフタイムには短い方を適用するという意味になる
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}