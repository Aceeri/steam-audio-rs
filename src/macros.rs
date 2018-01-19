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

#[macro_export]
macro_rules! vector3 {
    ($x:expr, $y:expr, $z:expr) => {{
        IPLVector3 { x: $x, y: $y, z: $z }
    }};
    ($x:expr) => {
        vector3!($x, $x, $x)
    };
    () => { vector3!(0.0) };
}

#[macro_export]
macro_rules! try_create_handle {
    () => {};
    ($type:tt ( $output:ident ), $call:expr) => {{

        use types::IPLerror::*;

        match $call {
            IPL_STATUS_FAILURE => Err(Error::Failure),
            IPL_STATUS_OUTOFMEMORY => Err(Error::OutOfMemory),
            IPL_STATUS_INITIALIZATION => Err(Error::Initialization),
            IPL_STATUS_SUCCESS => Ok($type($output)),
        }
    }};
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use types::*;

    #[cfg(test)]
    macro_rules! assert_vec3 {
        ($left:ident, $right: expr) => {
            assert_eq!($left.x, $right.x);
            assert_eq!($left.y, $right.y);
            assert_eq!($left.z, $right.z);
        }
    }

    #[test]
    fn vector3_macro_test() {
        let foo = vector3!(1.0, 2.0, 3.0);
        assert_vec3!(
            foo,
            IPLVector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );

        let foo = vector3!(4.0);
        assert_vec3!(
            foo,
            IPLVector3 {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            }
        );

        let foo = vector3!();
        assert_vec3!(
            foo,
            IPLVector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        );
    }
}
