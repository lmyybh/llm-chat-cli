use core::fmt::{Display, Formatter, Result};  

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Assistant,
}

impl Display for Role {  
    fn fmt(&self, f: &mut Formatter) -> Result {  
        match self {  
            Role::User => write!(f, "User"),  
			Role::Assistant => write!(f, "Assistant"),
		}  
    }  
}