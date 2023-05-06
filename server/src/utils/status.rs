use std::num::NonZeroU16;

#[derive(Copy, Clone, Debug)]
pub struct HttpStatus(NonZeroU16);

#[macro_use]
pub mod http_status {
    #[macro_export]
    macro_rules! http_status {
        (
            $(
                $(#[$docs:meta])*
                ($constants:ident, $code:expr, $message:expr);
            )+
        ) => {
            impl HttpStatus {
                $(
                    $(#[$docs])*
                    pub const $constants: HttpStatus = HttpStatus(unsafe {
                        NonZeroU16::new_unchecked($code)
                    });
                )+

                pub fn get_code(&self) -> u16 {
                    self.0.get()
                }

                pub fn get_message(&self) -> &'static str {
                    match self.0.get() {
                        $(
                            $code => $message,
                        )+
                        _ => panic!("Invalid HTTP status code"),
                    }
                }
            }
        };
    }
}

http_status! {
    (OK, 200, "OK");
    (CREATED, 201, "Created");
    (NO_CONTENT, 204, "No Content");
}
