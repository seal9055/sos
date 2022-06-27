use core::fmt;

/// Emits green color codes to print out a provided string in green
pub struct Green(pub &'static str);
impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x1B[32m")?;
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?;
        Ok(())
    }
}

/// Emits red color codes to print out a provided string in red
pub struct Red(pub &'static str);
impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x1B[31m")?; // prefix code
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?; // postfix code
        Ok(())
    }
}

