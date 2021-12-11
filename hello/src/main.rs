fn main() {
    println!("Hello, world!");
    println!("hell!!!");
    hoge();
}

fn hoge() {
    println!("hoge");
    let southern = "Southern";
    let japan = "ハローワールド！！！";
    let regions = [southern, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
