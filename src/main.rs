fn main() {
    println!("Hello, world!");
}

pub trait FromTxt {
    fn from_txt(txt: &str) -> Option<Self> where Self: Sized;
}
