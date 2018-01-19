#[macro_export]
macro_rules! c_enum {
    (typedef enum {
        $( $field_name:ident ),+
    } $name:ident ;) => {
        
        #[allow(non_camel_case_types)]
        #[derive(Debug, Eq, PartialEq)]
        #[repr(C)]
        pub enum $name {
            $(
                $field_name,
            )+
        }
    }
}

#[macro_export]
macro_rules! c_struct {
    (typedef struct {
        $( $field_type:ty : $field_name:ident ; )+
    } $name:ident ;) => {
        
        #[allow(non_snake_case)]
        #[repr(C)]
        pub struct $name {
            $(
                /// TODO documentation
                pub $field_name: $field_type,
            )+
        }
    }
}

