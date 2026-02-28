use anyhow::Result;
use template::my_try;

fn main() -> Result<()> {
    // let result = f1("hello")?;
    let result = my_try!(f1("hello"));
    println!("result: {:?}", result);
    Ok(())
}

fn f1(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1:{}", s.as_ref()))
}
