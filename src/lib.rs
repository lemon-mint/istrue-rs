//! IsTrue
//! 
//! A simple library for checking if a value is true.

pub trait IsTrue {
    fn is_true(&self) -> bool;
}

impl IsTrue for bool {
    /// Check if the boolean is true
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate 
    /// ;
    /// use is_true_rs::IsTrue;
    /// 
    /// let x = true;
    /// assert_eq!(x.is_true(), true);
    /// ```
    fn is_true(&self) -> bool {
        *self == true
    }
}

#[cfg(test)]
mod tests {
    use super::IsTrue;

    #[test]
    fn is_true_eq_true() {
        assert_eq!(true.is_true(), true);
    }

    #[test]
    fn is_true_eq_false() {
        assert_eq!(false.is_true(), false);
    }
}
