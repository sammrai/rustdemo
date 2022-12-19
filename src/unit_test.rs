struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn compare_area(&self, other: &Rectangle) -> bool{
        self.width * self.height > other.width * other.width
    }
}


fn double_value(a: i32) -> i32{
    a * 2
}

fn greeting(name: &str) -> String{
    format!("hello {} san", name)
}

// cargotestというコマンドを使ってテストをする時に使用する。
// cargotestの時だけビルドされる。通常のコンパイルからは除外される
#[cfg(test)]
// mod はモジュールを意味し、サブファイルと等価。
// unit_test/tests.rs
mod tests{
    // 1つ上の藻ジュルにアクセスする。(*はすべてのモジュールを意味する)
    use super::*;
    #[test]
    fn test_a_is_lager(){
        let a = Rectangle{
            width: 5,
            height: 3,
        };
        let b = Rectangle{
            width: 3,
            height: 3,
        };
        assert!(a.compare_area(&b));
        // cargo testで実行できる
    }
    #[test]
    fn test_a_is_smaller(){
        let a = Rectangle{
            width: 3,
            height: 3,
        };
        let b = Rectangle{
            width: 5,
            height: 5,
        };
        assert!(!(a.compare_area(&b)));
    }
    #[test]
    fn test_double(){
        assert_eq!(6,double_value(3));
    }
    
    #[test]
    fn test_contains_name(){
        let res = greeting("rust");
        assert!(res.contains("rust"))
    }
}