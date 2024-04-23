use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, ItemFn};

struct NsisFn {
    func: ItemFn,
}

impl Parse for NsisFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let func: ItemFn = input.parse()?;
        Ok(Self { func })
    }
}

#[proc_macro_attribute]
pub fn nsis_fn(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(tokens as NsisFn);
    let NsisFn { func } = tokens;

    let ident = func.sig.ident;
    let block = func.block;
    let attrs = func.attrs;

    quote! {
        #(#attrs)*
        #[no_mangle]
        #[allow(non_standard_style)]
        pub unsafe extern "C" fn #ident(
            hwnd_parent: ::windows_sys::Win32::Foundation::HWND,
            string_size: core::ffi::c_int,
            variables: *mut ::nsis_plugin_api::wchar_t,
            stacktop: *mut *mut ::nsis_plugin_api::stack_t,
        ) {
            ::nsis_plugin_api::exdll_init(string_size, variables, stacktop);
            #block
        }
    }
    .into()
}
