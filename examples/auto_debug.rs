use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
struct RespBulkString {
    inner: String,

    #[debug(skip)]
    nothing: (),

    hello: String,
}
fn main() {
    let bulk_string = RespBulkString {
        inner: "one".to_string(),
        nothing: (),
        hello: "hello world".to_string(),
    };

    println!("{:#?}", bulk_string);
}
