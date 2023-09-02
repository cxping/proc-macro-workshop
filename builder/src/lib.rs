extern crate proc_macro;

// 导入所需的 syn、quote 和 proc_macro2 库


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Builder)]
pub fn generate_struct_and_builder(input: TokenStream) -> TokenStream {
    // 解析输入为 DeriveInput
    let input = parse_macro_input!(input as DeriveInput);
    
    // 获取结构体名称
    let struct_name = &input.ident;
    
    // 生成 Builder 结构体名称
    let builder_name = format!("{}Builder", struct_name);
    let builder_ident = Ident::new(&builder_name, struct_name.span());

    // 生成 Builder 结构体字段及其类型
    let builder_fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_name = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    quote! {
                        #field_name: #field_type
                    }
                });
                quote! {
                   #(#field_name),*
                }
            },
            _ => {
                return TokenStream::from(quote! {
                    compile_error!("Builder macro only supports named fields in the struct.");
                });
            }
        },
        _ => {
            return TokenStream::from(quote! {
                compile_error!("Builder macro only supports struct types.");
            });
        }
    };


    // 生成 Builder 方法
    let builder_methods = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let methods = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_ty = &field.ty;
                    quote! {
                        pub fn #field_name(&mut self, value: #field_ty) -> &mut Self {
                            self.#field_name = value;
                            self
                        }
                    }
                });
                quote! {
                    #(#methods)*
                }
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    //构建build函数
    let build_expand = quote!{
        impl #struct_name{
            pub fn build()->#builder_ident{
                #builder_ident::default()
            }
        }
    };

    let quote_span = quote!{
      
      #[derive(Default,Debug)]
        pub struct #builder_ident{
            #builder_fields
        }
        impl #builder_ident{
            #builder_methods
        }
        #build_expand //生成一个默认的函数
    };
    println!("{quote_span}");
    TokenStream::from(quote_span)

//     // 生成 build 方法
//     let build_method = quote! {
//         pub fn build(&self) -> #struct_name {
//             #struct_name {
//                 #(#builder_fields),*
//             }
//         }
//     };

//     // 生成 Builder 结构体及其方法
//     let generated_code = quote! {
//         pub struct #builder_ident {
//             #(#builder_fields)*
//         }

//         impl #builder_ident {
//             pub fn new() -> Self {
//                 #builder_ident {
//                     #(#builder_fields)*
//                 }
//             }

//             #builder_methods
//             #build_method
//         }
//     };

//     generated_code.into()
// }


// #[proc_macro_derive(Builder)]
// pub fn derive_build(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     // 获取结构体名称
//     let struct_name = &input.ident;
//     //组合构建一个新建的名称
//     let builder_name = format!("{}Builder", struct_name);
//     let builder_ident = syn::Ident::new(&builder_name, struct_name.span());
//     //获取结构体参数
//     let fields = match &input.data {
//         syn::Data::Struct(data) => match &data.fields {
//             syn::Fields::Named(named) => &named.named,
//             _ => {
//                 return TokenStream::from(quote! {
//                     compile_error!("Builder macro only supports named fields in the struct.");
//                 });
//             }
//         },
//         _ => {
//             return TokenStream::from(quote! {
//                 compile_error!("Builder macro only supports struct types.");
//             });
//         }
//     };

//     //生成结构体字段的Bulder方法
//     let builder_fields = fields.iter().map(|field| {
//         let field_name = field.ident.as_ref().expect("Field must have a name");
//         let field_ty = &field.ty;
//         quote! {
//             #field_name: Option<#field_ty>,
//         }
//     });

    
//     // 生成代码
//     // let expand = quote! {
//     //     impl #struct_name {
//     //         pub fn builder() -> #builder_ident {
//     //             #builder_ident {
//     //                 executable: String::new(),
//     //                 args: Vec::new(),
//     //                 env: Vec::new(),
//     //                 current_dir: String::new(),
//     //             }
//     //         }
//     //     }

//     //     pub struct #builder_ident {
//     //         executable: String,
//     //         args: Vec<String>,
//     //         env: Vec<String>,
//     //         current_dir: String,
//     //     }

//     //     impl #builder_ident {
//     //         pub fn executable(mut self, value: String) -> Self {
//     //             self.executable = value;
//     //             self
//     //         }
//     //         pub fn args(mut self, value: String) -> Self {
//     //             self.args.push(value);
//     //             self
//     //         }
//     //         pub fn env(mut self, value: String) -> Self {
//     //             self.env.push(value);
//     //             self
//     //         }
//     //         pub fn current_dir(mut self, value: String) -> Self {
//     //             self.current_dir = value;
//     //             self
//     //         }
//     //         pub fn build(self) -> #struct_name {
//     //             #struct_name {
//     //                 executable: self.executable,
//     //                 args: self.args,
//     //                 env: self.env,
//     //                 current_dir: self.current_dir,
//     //             }
//     //         }
//     //     }
//     // };
//     let builder_ident_clone = builder_ident.clone();
//     println!("{:?}",builder_ident_clone);
//     // 生成Builder模式的代码
//     let expanded = quote! {
//         pub struct #builder_ident {
//             #($(#builder_fields)* ) *
//         }

//         // impl #builder_ident {
//         //     pub fn builder() -> #builder_ident {
//         //         #builder_ident {
//         //             #(#builder_ident_clone)*
//         //         }
//         //     }
//         // }
//     };

//     // 将生成的代码转换为TokenStream并返回
//     expanded.into()
}

