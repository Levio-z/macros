// my_vec! = my_vec!{1 , 2 , 3} // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {{
        std::vec::from_elem($elem, $n)
    }};
    ($($x:expr),+ $(,)?) => {{
       <[_]>::into_vec(
            // Using the intrinsic produces a dramatic improvement in stack usage for
            // unoptimized programs using this code path to construct large Vecs.
           std::boxed::Box::new([$($x),+])
        )
    }};
}

#[macro_export]
macro_rules! my_try {
    ($e:expr) => {{
        let e: Result<_, anyhow::Error> = $e;
        match e {
            Ok(v) => v,
            Err(e) => panic!("{:?}", e),
        }
    }};
}
#[macro_export]
macro_rules! my_ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
