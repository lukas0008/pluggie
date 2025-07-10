// use proc_macro::TokenStream;
// use quote::quote;

// #[proc_macro]
// pub fn describe_plugin(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(item.clone()).unwrap();
//     let name = &ast.ident;

//     let input: proc_macro2::TokenStream = input.into();
//     let item: proc_macro2::TokenStream = item.into();

//     let code = quote! {
//         #[unsafe(no_mangle)]
//         #[cfg(feature = "init")]
//         pub extern "C" fn pluggie_init(
//             ctx: abi_stable::std_types::RArc<
//                 abi_stable::external_types::RMutex<pluggie::internal_pluggie_context::InternalPluggieCtx>,
//             >,
//         ) {
//             let ctx = PluggieCtx::new(ctx);
//             init(ctx);
//         }
//     };

//     code.into()
// }
