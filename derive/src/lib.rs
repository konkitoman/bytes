use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

#[proc_macro_derive(Bytes)]
pub fn derive_bytes(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    match input.data {
        syn::Data::Struct(s) => {
            let mut names = Vec::with_capacity(s.fields.len());
            let mut types = Vec::with_capacity(s.fields.len());

            for field in s.fields {
                names.push(field.ident);
                types.push(field.ty);
            }
            quote! {
                impl TBytes for #ident{
                    fn size(&self) -> usize{
                        #(self.#names.size())+*
                    }

                    fn to_bytes(&self) -> Vec<u8>{
                        let mut buffer = Vec::with_capacity(self.size());

                        #(buffer.append(&mut self.#names.to_bytes());)*

                        buffer
                    }

                    fn from_bytes(bytes: &mut Vec<u8>) -> Option<Self>{
                        #(let #names = <#types>::from_bytes(bytes)?;)*
                        Some(Self{
                            #(#names),*
                        })
                    }
                }
            }
            .into()
        }
        syn::Data::Enum(e) => {
            // [0, 1, 2]
            // how many variants enum has
            let mut idxs = Vec::new();
            // [Auth, Id]
            // variants
            let mut idents = Vec::new();
            // what fields variant has
            let mut fields = Vec::new();

            for (i, variant) in e.variants.iter().enumerate() {
                idxs.push(i);
                idents.push(variant.ident.clone());
                fields.push(variant.fields.clone());
            }

            // [Self::Auth { id: <u16>::from_bytes(bytes)?] } or
            // [Self::Auth(<u16>::from_bytes(bytes)? )]
            let mut variant_from = Vec::new();

            // [self.id.to_bytes()] or [self.0.to_bytes()]
            let mut variant_to = Vec::new();
            // [Self::Auth{id}, Self::AuthRes{accepted}] or {Self::Auth(v0), Self::AuthRes(v0)}
            let mut variants = Vec::new();
            // [id.size()] or [v0.size()]
            // [id.size() + response.size()] or [v0.size() + v1.size()]
            let mut variant_size = Vec::new();

            for i in 0..idents.len() {
                let ident = idents[i].clone();
                let fields = fields[i].clone();

                // [id : <u16>::from_bytes()?] or [<u16>::from_bytes()?]
                let mut variant_init_vars = Vec::new();
                //  [id] or [v0]
                let mut variant_vars = Vec::new();

                let mut is_object = false;
                for (ii, field) in fields.iter().enumerate() {
                    let ty = field.ty.clone();
                    if let Some(ident) = field.ident.clone() {
                        is_object = true;
                        variant_init_vars.push(quote!(
                           #ident : <#ty>::from_bytes(bytes)?
                        ));
                        variant_vars.push(quote!(
                            #ident
                        ));
                    } else {
                        variant_init_vars.push(quote!(
                           <#ty>::from_bytes(bytes)?
                        ));
                        let vii = format_ident!("v{}", ii);
                        variant_vars.push(quote!(#vii));
                    }
                }
                if variant_vars.len() > 0 {
                    if is_object {
                        variants.push(quote!(#ident{#(#variant_vars),*}));
                    } else {
                        variants.push(quote!(#ident(#(#variant_vars),*)));
                    }
                    variant_to.push(quote! {
                        #(buffer.append(&mut #variant_vars.to_bytes());)*
                    });
                    variant_size.push(quote! {
                        #(#variant_vars.size())+*
                    });
                } else {
                    variants.push(quote! {
                        #ident
                    });
                    variant_to.push(quote! {});
                    variant_size.push(quote! {0});
                }
                if variant_init_vars.len() > 0 {
                    if is_object {
                        variant_from.push(quote! {
                            #ident{#(#variant_init_vars),*}
                        });
                    } else {
                        variant_from.push(quote! {
                            #ident(#(#variant_init_vars),*)
                        });
                    }
                } else {
                    variant_from.push(quote! {
                        #ident
                    });
                }
            }

            quote! {
                impl TBytes for #ident{
                    fn size(&self) -> usize{
                        match self{
                            #(Self::#variants => #variant_size),*
                        }
                    }

                    fn to_bytes(&self) -> Vec<u8>{
                        let mut buffer = Vec::new();

                        match self{
                            #(Self::#variants => {buffer.append(&mut #idxs.to_bytes()); #variant_to}),*
                        };

                        buffer
                    }

                    fn from_bytes(bytes: &mut Vec<u8>) -> Option<Self>{
                        let id = usize::from_bytes(bytes)?;

                        match id{
                            #(#idxs => Some(Self::#variant_from),)*
                            _=> None
                        }
                    }
                }
            }
            .into()
        }
        syn::Data::Union(_) => todo!(),
    }
}
