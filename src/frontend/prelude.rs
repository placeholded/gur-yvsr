pub mod exec {
    struct Acc {
        val: Option<isize>
    }
    impl Acc {
        pub fn get_value(&self) -> Result<isize, ()> {
            if let Some(n) = self.val {
                return Ok(n)
            }
            Err(())
        }
        pub(crate) fn clear(&mut self) {
            self.val = None
        }
        pub(crate) fn get_details(&self) -> String {
            if let Some(n) = self.val {
                return format!("accumulator value: {n}")
            }
            "accumulator was empty".to_string()
        }
    }
    
    
}