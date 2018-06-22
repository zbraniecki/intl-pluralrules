extern crate syn;
extern crate proc_macro2;

use quote::ToTokens;
use self::proc_macro2::{Ident, Span};

enum PluralCategory {
    ONE,
    OTHER,
}

fn convert_literal(num: usize) -> syn::NestedMeta {
    syn::NestedMeta::Literal(
        syn::Lit::Int(
            syn::LitInt::new(num as u64, syn::IntSuffix::None, Span::call_site())
        )
    )
}

fn convert_ident(left: &str) -> syn::Ident {
    Ident::new(left, Span::call_site())
}

fn create_eq_expression(left: &str, right: usize) -> syn::Expr {
    let l = convert_ident(left);
    let r = convert_literal(right);

    let eq_tokens = quote! {
        po.#l == #r
    };
    syn::parse2(eq_tokens).expect("Unable to parse tokens")
}

fn create_and_expression(left: syn::Expr, right: syn::Expr) -> syn::Expr {
    let and_tokens = quote! {
        #left && #right
    };
    syn::parse2(and_tokens).expect("Unable to parse tokens")
}

fn create_condition_block(cat: PluralCategory, condition: syn::Expr) -> syn::Expr {
    let cat_name = match cat {
        PluralCategory::ONE => "ONE",
        PluralCategory::OTHER => "OTHER",
    };
    let cat_ident = Ident::new(cat_name, Span::call_site());

    let block_tokens = quote! {
        if #condition { PluralCategory::#cat_ident }
    };
    syn::parse2(block_tokens).expect("Unable to parse tokens")
}

pub fn stringify(expr: syn::Expr) -> String {
    format!("{}", expr.into_token_stream().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_expression() {
        assert_eq!("po . i == 1", stringify(create_eq_expression("i", 1)));
        assert_eq!("po . v == 5", stringify(create_eq_expression("v", 5)));
    }

    #[test]
    fn test_condition_block() {
        let cond = create_eq_expression("i", 1);
        let block = create_condition_block(PluralCategory::ONE, cond);
        assert_eq!("if po . i == 1 { PluralCategory :: ONE }", stringify(block));
    }

    #[test]
    fn test_condition_block_with_and() {
        let eq1 = create_eq_expression("i", 1);
        let eq2 = create_eq_expression("v", 0);
        let and_cond = create_and_expression(eq1, eq2);
        let block = create_condition_block(PluralCategory::ONE, and_cond);
        assert_eq!("if po . i == 1 && po . v == 0 { PluralCategory :: ONE }", stringify(block));
    }
}
