pub struct FirstName {
    pub value: String,
}

//impl FirstName {
//    pub fn new(name: &str) -> Result<FirstName, String> {
//        if name.len() < 2 {
//            Err("Name should be atleast two characters".to_string())
//        } else {
//            Ok(FirstName {
//                value: name.to_string(),
//            })
//        }
//    }
//    pub fn get_value(&self) -> &String {
//        &self.value
//    }
//}

pub struct LastName {
    pub value: String,
}

macro_rules! generate_get_value {
    ($struct_type:ident, $return_type: ty) => {
        impl $struct_type {
            pub fn new(value: $return_type) -> Self {
                $struct_type { value }
            }
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    };
}
