

struct  Point<T>{
    x: T,
    y: T,
}

struct  PointAnother<T, U>{
    x: T,
    y: U,
}
//メソッドを実装
// self:<T,U>
// oher:<V,W>
// retn:<T,W>
impl<T,U> PointAnother<T,U>{
    // 
    fn mixup<V,W>(self, other: PointAnother<V,W>) -> PointAnother<T,W>{
        PointAnother { 
            x: self.x, // ジェネリクスのT 
            y: other.y  // ジェネリクスのW
        }
    }
}


pub fn run(){
    let number_list = vec![34, 50, 25, 100, 65];

    // 最大値を検索する関数をジェネリクスを使って作成する
    // println!("{}",largest_i32(number_list));

    let char_list = vec!['a','b', 'c', 'd'];
    println!("{}",largest(number_list));
    println!("{}",largest(char_list));

    // 構造体のジェネリクス
    let p1 = Point{ x:1, y:2};
    let p2 = Point{ x:1.0, y:2.0};
    let p3 = PointAnother{ x:1.0, y:2};
    let p4 = PointAnother{ x: "Rust", y:"a"};

    // 構造体の関数
    // 
    let p5 = p3.mixup(p4);
    println!("{} {}",p5.x ,p5.y)    
}

fn largest_i32(list: Vec<i32>) -> i32{
    let mut largest = list[0];
    for number in list{
        if number > largest {
            largest = number;
        }
    }
    largest
}

// Tは何らかの任意のデータ型を表す。
// PartialOrd + Copy : 大小の比較が可能なトレイト境界にTを限定する
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T{
    let mut largest = list[0];
    for number in list{
        if number > largest {
            largest = number;
        }
    }
    largest
}