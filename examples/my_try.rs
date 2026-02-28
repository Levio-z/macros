use anyhow::Result;

macro_rules! my_try {
    ($e:expr) => {{
        let e: Result<_, anyhow::Error> = $e;
        match e {
            Ok(v) => v,
            Err(e) => panic!("{:?}", e),
        }
    }};
}

fn main() -> Result<()> {
    // let result = f1("hello")?;
    let result = my_try!(f1("hello"));
    println!("result: {:?}", result);
    Ok(())
}

fn f1(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1:{}", s.as_ref()))
}
