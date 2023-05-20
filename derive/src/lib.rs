use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Index};

#[proc_macro_derive(Bytes)]
pub fn derive_bytes(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let generics = input.generics;

    let mut gen_where = quote!();

    {
        let mut gen = Vec::new();
        for generig in generics.type_params() {
            gen.push(quote!(#generig: TBytes))
        }

        if !gen.is_empty() {
            gen_where = quote! {
                where #(#gen),*
            }
        }
    }

    match input.data {
        syn::Data::Struct(s) => {
            let mut names = Vec::with_capacity(s.fields.len());
            let mut types = Vec::with_capacity(s.fields.len());

            for field in s.fields {
                if let Some(ident) = field.ident {
                    names.push(ident);
                }

                types.push(field.ty);
            }

            if !names.is_empty() {
                let mut from_bytes_blocks = Vec::new();

                for (i, (name, type_)) in names.iter().zip(types.iter()).enumerate() {
                    let mut before_names = Vec::new();
                    for name in names[0..i].iter().rev() {
                        before_names.push(name);
                    }
                    from_bytes_blocks.push(quote! {
                        let #name = if let Some(value) = <#type_>::from_bytes(buffer) {value}else{
                            #(
                                let mut bytes = #before_names.to_bytes();
                                while let Some(byte) = bytes.pop(){
                                    buffer.insert(0, byte);
                                }
                            )*
                            return None;
                        };
                    });
                }

                quote! {
                    #[allow(clippy::question_mark)]
                    impl #generics TBytes for #ident #generics #gen_where{
                        fn size(&self) -> usize{
                            #(self.#names.size())+*
                        }

                        fn to_bytes(&self) -> Vec<u8>{
                            let mut buffer = Vec::with_capacity(self.size());

                            #(buffer.append(&mut self.#names.to_bytes());)*

                            buffer
                        }

                        fn from_bytes(buffer: &mut TBuffer) -> Option<Self>{
                            #(#from_bytes_blocks)*
                            Some(Self{
                                #(#names),*
                            })
                        }
                    }
                }
                .into()
            } else {
                let nums = (0usize..types.len())
                    .map(Index::from)
                    .collect::<Vec<Index>>();
                let names = nums
                    .iter()
                    .map(|n| format_ident!("v{}", n))
                    .collect::<Vec<proc_macro2::Ident>>();

                let mut from_bytes_blocks = Vec::new();

                for (i, (name, type_)) in names.iter().zip(types.iter()).enumerate() {
                    let mut before_names = Vec::new();
                    for name in names[0..i].iter().rev() {
                        before_names.push(name);
                    }
                    from_bytes_blocks.push(quote! {
                        let #name = if let Some(value) = <#type_>::from_bytes(buffer) {value}else{
                            #(
                                let mut bytes = #before_names.to_bytes();
                                while let Some(byte) = bytes.pop(){
                                    buffer.insert(0, byte);
                                }
                            )*
                            return None;
                        };
                    });
                }

                quote! {
                    #[allow(clippy::question_mark)]
                    impl #generics TBytes for #ident #generics #gen_where{
                        fn size(&self) -> usize{
                            #(self.#nums.size())+*
                        }

                        fn to_bytes(&self) -> Vec<u8>{
                            let mut buffer = Vec::with_capacity(self.size());

                            #(buffer.append(&mut self.#nums.to_bytes());)*

                            buffer
                        }

                        fn from_bytes(buffer: &mut TBuffer) -> Option<Self>{
                            #(#from_bytes_blocks)*
                            Some(Self(
                                #(#names),*
                            ))
                        }
                    }
                }
                .into()
            }
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
                idents.push(&variant.ident);
                fields.push(&variant.fields);
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
                           #ident : <#ty>::from_bytes(buffer)?
                        ));
                        variant_vars.push(quote!(
                            #ident
                        ));
                    } else {
                        variant_init_vars.push(quote!(
                           <#ty>::from_bytes(buffer)?
                        ));
                        let vii = format_ident!("v{}", ii);
                        variant_vars.push(quote!(#vii));
                    }
                }
                if !variant_vars.is_empty() {
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
                if !variant_init_vars.is_empty() {
                    let vars = if is_object {
                        fields
                            .iter()
                            .map(|f| f.ident.clone().unwrap())
                            .collect::<Vec<_>>()
                    } else {
                        (0..fields.len())
                            .map(|f| format_ident!("f{f}"))
                            .collect::<Vec<_>>()
                    };
                    let mut from_bytes_var = Vec::new();
                    for (i, field) in fields.iter().enumerate() {
                        let ty = &field.ty;
                        let before = vars[0..i].iter().rev().collect::<Vec<_>>();
                        from_bytes_var.push(quote! {
                            if let Some(value) = <#ty>::from_bytes(buffer) {value}else{
                                #(
                                    let mut bytes = #before.to_bytes();
                                    while let Some(byte) = bytes.pop(){
                                        buffer.insert(0, byte);
                                    }
                                )*
                                let mut bytes = id.to_bytes();
                                while let Some(byte) = bytes.pop(){
                                    buffer.insert(0, byte);
                                }

                                return None;
                            }
                        });
                    }
                    if is_object {
                        variant_from.push(quote! {
                            {
                                #(let #vars = #from_bytes_var;)*
                                return Some(Self::#ident{#(#vars),*})
                            }
                        });
                    } else {
                        variant_from.push(quote! {
                            {
                            #(let #vars = #from_bytes_var;)*
                            return Some(Self::#ident(#(#vars),*))
                            }
                        });
                    }
                } else {
                    variant_from.push(quote! {
                        return Some(Self::#ident)
                    });
                }
            }

            quote! {
                #[allow(clippy::question_mark)]
                impl #generics TBytes for #ident #generics #gen_where{
                    fn size(&self) -> usize{
                        (match self{
                            #(Self::#variants => #variant_size),*
                        } + 0usize.size())
                    }

                    fn to_bytes(&self) -> Vec<u8>{
                        let mut buffer = Vec::new();

                        match self{
                            #(Self::#variants => {buffer.append(&mut #idxs.to_bytes()); #variant_to}),*
                        };

                        buffer
                    }

                    fn from_bytes(buffer: &mut TBuffer) -> Option<Self>{
                        let id = usize::from_bytes(buffer)?;

                        match id{
                            #(#idxs => #variant_from,)*
                            _=> {
                                let mut bytes = id.to_bytes();
                                while let Some(byte) = bytes.pop(){
                                    buffer.insert(0, byte);
                                }
                                None
                            }
                        }
                    }
                }
            }
            .into()
        }
        syn::Data::Union(_) => todo!(),
    }
}
