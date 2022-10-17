// Copyright (c) 2021-2022 Miguel Barreto and others
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand_action_list_directive(input: DeriveInput) -> TokenStream {
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let st_name = input.ident;
  let lifetimes = input.generics.lifetimes().last();

  quote! {
      #[automatically_derived]
      impl #impl_generics HasDirectiveData <'a> for  #st_name #ty_generics #where_clause{
        fn directive_data(&'a self) -> &'a DirectiveData {
          &self.data
        }
      }
        impl #impl_generics Directive <#lifetimes> for  #st_name #ty_generics #where_clause{
        fn build_action_list(
          &'a self,
          settings: &Settings,
          yaml: &StrictYaml,
        ) -> Result<Vec<Box<(dyn dotfiles_core::action::Action<'a> + 'a)>>, dotfiles_core::error::DotfilesError> {
          self.parse_action_list(settings, yaml).map(|vec| {
            let result: Vec<Box<(dyn Action<'a> + 'a)>> = vec
              .into_iter()
              .map(|action| {
                let boxed: Box<(dyn Action<'a> + 'a)> = Box::new(action);
                boxed
              })
              .collect();
            result
          })
        }
      }
  }
}
