#[macro_export]
macro_rules! token {
    ($thing:tt) => {
        #[allow(unused_imports)]
        us $crate::lexer::TokenVariety::*;
        use $crate::lexer::Token::from($thing)
    }
}