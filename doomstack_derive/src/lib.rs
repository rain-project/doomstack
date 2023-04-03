// Modules

mod doom;

// Interface

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_derive(Doom, attributes(doom))]
#[proc_macro_error]
pub fn doom_derive(input: TokenStream) -> TokenStream {
    doom::doom(input)
}
