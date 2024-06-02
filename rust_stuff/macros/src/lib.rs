#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($( $e: expr ),+ $(,)?) => {{
        const C: usize = $crate::count![@COUNT; $($e),*];
        let mut v = Vec::with_capacity(C);
        $(v.push($e);)+
        v
    }};
    ($e: expr; $c: expr) => {{
        let count = $c;
        let mut v = Vec::with_capacity(count);
        for _ in 0..$c {
            v.push($e);
        }
        v
    }};
}

#[macro_export]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}
