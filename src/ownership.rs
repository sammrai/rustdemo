pub fn run(){
    let s1 = String::from("hello");
    let s2 = s1;
    // データ所有している型は、代入すると所有権が移る(move) s1->s2
    // 所有権による二重解法エラーを回避するしくみ
    // println!("{} {}",s1,s2)
    println!("{}",s2);

    // 所有権が映らないコピーされる場合
    // 文字列スライスは、スタックに格納されるのでコピーされるだけ
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1,i2);
    println!("i1: {:p}",&i1);
    println!("i2: {:p}",&i2);

    // リテラル
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1,sl2);
    println!("sl1: {:p}",&i1);
    println!("sl2: {:p}",&sl2);

    // deep copy
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3: {:p}",&s3);
    println!("s4: {:p}",&s4);
    println!("heap memory address of s3: {:?}",s3.as_ptr());
    println!("heap memory address of s4: {:?}",s4.as_ptr());
    println!("{} {}",s3,s4);

    // 関数にs5の所有権は、関数の引数に渡したときに移る
    // take_owmershipの変数sにs5の所有権が移り、関数を抜けるときにドロップするので、s5が解法され、同時にヒープのhellpも消え参照できなくなる
    let s5 = String::from("hellp");
    take_ownership(s5);
    // println!("{}",s5);

    // 文字型
    let s5 = String::from("hello");
    println!("{:p} ({:?} {} {} {})",&s5, s5.as_ptr(),s5.len(),s5.capacity(),s5);
    take_ownership(s5);
    // println!("{:p} {:?} {} {} {} ",&s5, s5.as_ptr(),s5.len(),s5.capacity(),s5);

    // 関数の戻り値に引数をセットするので、ｓ7にs6の所有権が移る。
    let s6 = String::from("hello");
    println!("s6: {:p} ({:?} {} {} {})",&s6, s6.as_ptr(),s6.len(),s6.capacity(),s6);
    let s7 = take_giveback_ownership(s6);
    println!("s7: {:p} ({:?} {} {} {})",&s7, s7.as_ptr(),s7.len(),s7.capacity(),s7);

    // 値を渡すと消えてしまうのが不便な場合が多いので、参照渡しを利用できる
    let s8 = String::from("hello");
    let len = calc_length(&s8);
    println!("{}",len);
    println!("s8: {:p} ({:?} {} {} {})",&s8, s8.as_ptr(),s8.len(),s8.capacity(),s8);

    // ミュータブルな変数を参照で渡し、変更することができる
    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}",s9);

    // イミュータブルな参照は複数作ることができる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}",s10,r1,r2);

    // 実験:  ミュータブルは参照は複数作れない。複数の参照から同時に値を変更できなくするためのしくみ。
    // mutは変更できるという属性なので、すでにr1に参照が取られているので、新たにr2が参照をとることができない。
    let mut s10 = String::from("hello");
    // let r1 = &s10; 
    // let r2 = &mut s10;
    // println!("{} {} {}",s10,r1,r2);

    // 実験: ミュータブルなs11
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    // println!("{}",s11); // r1のミュータブルな参照の有効な期間は、所有者のs11であっても参照することもできない。
    println!("{}",r1);
    println!("{}",s11); // 順序を変えるとアクセスできる。

    // 実験: 
    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} {}",r1,r2);
    // 
    let r3 = &mut s12;
    *r3 = String::from("hello_update");
    println!("{}",s12);


}
fn take_ownership(s: String){
    println!("{:p} ({:?} {} {} {})",&s, s.as_ptr(),s.len(),s.capacity(),s);
    // println!("{}",s);
}

fn take_giveback_ownership(s: String) -> String{
    // セミコロンがないそれが戻り値になる
    s
}

fn calc_length(s: &String) -> usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str("_world");
}