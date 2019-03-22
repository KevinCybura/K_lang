macro_rules! parse_try(
     ($function:ident, $tokens:ident, $settings:ident, $parsed_tokens:ident) => (
           parse_try!($function, $tokens, $settings, $parsed_tokens, )
         );

     ($function:ident, $tokens:ident, $settings:ident, $parsed_tokens:ident, $($arg:expr), *) => (
         match $function($tokens, $settings, $($arg),*) {
             Good(ast, toks) => {
                 $parsed_tokens.extend(toks.into_iter());
                 ast
             },
             NotComplete => {
                 $parsed_tokens.reverse();
                 $tokens.extend($parsed_tokens.into_iter());
                                return NotComplete;
             },
             Bad(message) => return Bad(message)
         }
         )

);


macro_rules! expected_token (
    ([ $($token:pat, $value:expr, $result:stmt);+ ] <= $tokens:ident, $parsed_tokens:ident, $error:expr) => (
        match $tokens.pop() {
            $(
                Some($token) => {
                    $parsed_tokens.push($value);
                    $result
                },
            )+
            None => {
                $parsed_tokens.reverse();
                $tokens.extent($parsed_tokens.into_iter());
                return NotComplete;
            },
            _ => return error($error)
        }
    );
)
