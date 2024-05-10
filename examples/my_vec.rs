use anyhow::Result;

fn main() -> Result<()> {
    let v = my_vec!(1, 2, 3);
    println!("{:?}", v);

    let v = my_vec![1;4];
    println!("{:?}", v);

    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
    ];

    println!("{:?}", v);

    Ok(())
}

// my_vec! = my_vec! { 1, 2, 3 }; // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    // 空 vec
    () => { Vec::new() };
    // 支持指定数字，[1;4] => [1,1,1,1]
    ($elem:expr; $n:expr) => { std::vec::from_elem($elem, $n) };
    // 一个或多个, $(,)?是支持结尾可以有一个逗号
    ($($x:expr),+ $(,)?) => {
        {
            // 一个个push效率不高
            // let mut temp_vec = Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec

            // 这种方式效率高
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}
