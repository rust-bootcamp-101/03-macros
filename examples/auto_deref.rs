use macros::AutoDeref;

#[derive(Debug)]
#[allow(unused)]
#[derive(AutoDeref)]
#[deref(mutable = true, field = "inner")]
struct RespBulkString {
    inner: String,

    // #[debug(skip)]
    nothing: (),
}
fn main() {
    let bulk_string = RespBulkString {
        inner: "one".to_string(),
        nothing: (),
    };

    println!("{:#?}", bulk_string);

    parse_inner(&bulk_string);
}

fn parse_inner(s: &String) {
    println!("inner: {}", s)
}
