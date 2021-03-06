#![forbid(unsafe_code)]

fn lifetime_helper<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T { v }

fn lifetime_helper_mut<'a, 'b, T>(_: &'a &'b (), v: &'b mut T) -> &'a mut T { v }

/// create a static refrence
/// using https://github.com/rust-lang/rust/issues/25860
pub fn remember<'a, T>(data: &'a T) -> &'static T {
    let f: fn(_, &'a T) -> &'static T = lifetime_helper;
    f(&&(), data)
}

/// change a mutable refrence's lifetime
/// using https://github.com/rust-lang/rust/issues/25860
pub fn remember_mut<'a, 'b, T>(data: &'a mut T) -> &'b mut T {
    let f: fn(_, &'a mut T) -> &'b mut T = lifetime_helper_mut;
    f(&&(), data)
}


#[cfg(test)]
mod tests {
    use crate::remember;
    #[test]
    fn test_remember() {
        let mut data = 1;
        let refer = &data;
        let refer = remember(refer);
        let mut_refer = &mut data;
        assert_eq!(*refer, 1);
        *mut_refer = 2;
        assert_eq!(*refer, 2);
    }
    #[test]
    fn example() {
        fn printit(s :String) {
            println!("{}",s);
        }

        let s = String::new();
        let refer = remember(&s);
        printit(s);
        println!("refer is {}",refer); 
    }
}
