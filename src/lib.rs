pub fn remember<'a, T>(data: &'a T) -> &'static T {
    unsafe {
        std::mem::transmute(data)
    }
}

pub fn remember_mut<'a, 'b, T>(data: &'a mut T) -> &'b mut T {
    unsafe {
        std::mem::transmute(data)
    }
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
