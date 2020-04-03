use crate::type_tree::TypeTree;
use crate::write_ident;

use proc_macro2::TokenStream;
use quote::quote;

use std::collections::*;
use std::iter::FromIterator;

#[derive(Default)]
pub struct TypeNamespaces(pub BTreeMap<String, TypeTree>);

impl TypeNamespaces {
    pub fn to_stream(&self) -> TokenStream {
        let mut tokens = Vec::new();

        for (name, tree) in self.0.iter() {
            let name = write_ident(name);
            let tree = tree.to_stream();

            tokens.push(quote! {
                pub mod #name {
                    #tree
                }
            });
        }

        TokenStream::from_iter(tokens)
    }
}