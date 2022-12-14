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
                        #(let #names = #types::from_bytes(bytes)?;)*
                        Some(Self{
                            #(#names),*
                        })
                    }
                }
            }
            .into()
        }
        syn::Data::Enum(e) => {
            let mut idxs = Vec::new();
            let mut idents = Vec::new();
            let mut fields = Vec::new();

            for (i, variant) in e.variants.iter().enumerate() {
                idxs.push(i);
                idents.push(variant.ident.clone());
                fields.push(variant.fields.clone());
            }

            let mut froms_fb = Vec::new();
            let mut froms_tb = Vec::new();
            let mut matchs_tb = Vec::new();

            for i in 0..idents.len() {
                let ident = idents[i].clone();
                let fields = fields[i].clone();

                let mut f = Vec::new();
                let mut t = Vec::new();
                let mut tt = Vec::new();

                let mut braket = false;
                for (ii, field) in fields.iter().enumerate() {
                    let ty = field.ty.clone();
                    if let Some(ident) = field.ident.clone() {
                        braket = true;
                        f.push(quote!(
                           #ident : #ty::from_bytes(bytes)?
                        ));
                        t.push(quote!(
                            #ident.to_bytes()
                        ));
                        tt.push(quote!(#ident))
                    } else {
                        f.push(quote!(
                           #ty::from_bytes(bytes)?
                        ));
                        let vii = format_ident!("v{}", ii);
                        t.push(quote!(#vii.to_bytes()));
                        tt.push(quote!(#vii))
                    }
                }
                if t.len() > 0 {
                    if braket {
                        matchs_tb.push(quote!(#ident{#(#tt),*}));
                    } else {
                        matchs_tb.push(quote!(#ident(#(#tt),*)));
                    }
                    froms_tb.push(quote! {
                        #(buffer.append(&mut #t);)*
                    });
                } else {
                    matchs_tb.push(quote! {
                        #ident
                    });
                    froms_tb.push(quote! {});
                }
                if f.len() > 0 {
                    if braket {
                        froms_fb.push(quote! {
                            #ident{#(#f),*}
                        });
                    } else {
                        froms_fb.push(quote! {
                            #ident(#(#f),*)
                        });
                    }
                } else {
                    froms_fb.push(quote! {
                        #ident
                    });
                }
            }

            quote! {
                impl TBytes for #ident{
                    fn size(&self) -> usize{
                        match self{
                            #(#idents => 0),*
                        }
                    }

                    fn to_bytes(&self) -> Vec<u8>{
                        let mut buffer = Vec::new();

                        match self{
                            #(Self::#matchs_tb => {buffer.append(&mut #idxs.to_bytes()); #froms_tb}),*
                        };

                        buffer
                    }

                    fn from_bytes(bytes: &mut Vec<u8>) -> Option<Self>{
                        let id = usize::from_bytes(bytes)?;

                        match id{
                            #(#idxs => Some(Self::#froms_fb),)*
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
