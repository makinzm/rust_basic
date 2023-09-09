/**
    Hello, world!
    s_origin is hello
    s is hellohellod
*/
fn main() {
    println!("Hello, world!");

    let s_origin: &str = "hello";
    // ここで所有権は移らない.
    let mut s1: String = String::from(s_origin);
    println!("s_origin is {}", s_origin);

    s1.push_str(s_origin);
    s1.push('d');
    println!("s is {}", s1);

    let chars: Vec<char> = s_origin.chars().collect();

    println!("{}", chars[0..2].iter().collect::<String>());

    let u16_value: u16 = 42;
    let i32_value: i32 = -10;

    example_generic(u16_value);
    example_generic(i32_value);

    let vec: Vec<i32> = vec![2000,3,4,5,6,7,1000];
    println!("{}", get_largest(&vec));

    let r;
    {
        let x = 5;
        // 参照を奪ってしまえば life time は考えなくて良い.
        r = x;
    }
    println!("{}", r);

    let s: Tmp;
    let t: Tmp;
    {
        let name = "hello";
        s = Tmp{ name};
        println!("s: {}",s.name);
        let name = String::from(name);
        t = Tmp{name: &name};
        println!("t: {}",t.name);
    }
    println!("s in outer: {}",s.name);
    // borrowed value does not live long enough
    // println!("{}",t.name);
}

//  expected named lifetime parameter と怒られる('a がないと)
struct Tmp<'a>{
    name: &'a str,
}

fn get_largest<T: std::cmp::PartialOrd+Copy>(lst: &[T]) -> T{
    let mut res = &lst[0];
    for i in lst {
        if *i > *res {
            res = i;
        }
    }
    let final_res = *res;
    // if there is no `+Copy`
    // move occurs because `*res` has type `T`, which does not implement the `Copy` trait
    final_res
}

fn example_generic<T: Into<i32>>(value: T) -> impl Into<i32> {
    let int_value: i32 = value.into() as i32;
    println!("{}", int_value);
    int_value
}
