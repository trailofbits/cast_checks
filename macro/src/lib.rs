// #![doc = include_str!("../../README.md")]

use proc_macro::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse, parse_macro_input,
    spanned::Spanned,
    visit_mut::{visit_expr_mut, VisitMut},
    Expr, ExprCast, Item, Type,
};

#[cfg(not(procmacro2_semver_exempt))]
static WARN: std::sync::Once = std::sync::Once::new();

/// A procedural macro to check for invalid casts
///
/// # Example
///
/// ```ignored
/// #[cast_checks::enable]
/// fn as_u16(x: u64) -> u16 {
///     x as u16
///}
/// ```
///
/// For additional details, see the [repository documentation].
///
/// [repository documentation]: https://github.com/trailofbits/cast_checks
#[proc_macro_attribute]
pub fn enable(args: TokenStream, item: TokenStream) -> TokenStream {
    assert!(args.is_empty());
    let mut item = parse_macro_input!(item as Item);
    Visitor.visit_item_mut(&mut item);
    item.into_token_stream().into()
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_expr_mut(self, expr);

        if cfg!(not(any(debug_assertions, feature = "release"))) {
            return;
        }

        let Expr::Cast(ExprCast { expr: ref operand, ref ty, ..}) = expr else {
            return;
        };

        if matches!(**ty, Type::Infer(_)) {
            return;
        }

        let span = expr.span();

        #[cfg(procmacro2_semver_exempt)]
        let msg = {
            let mut tokens = proc_macro2::TokenStream::new();
            expr.to_tokens(&mut tokens);
            let text = format!(
                "`{}` at {}:{}:{}",
                tokens,
                span.source_file().path().display(),
                span.start().line,
                span.start().column,
            );
            if enabled("CAST_CHECKS_LOG") {
                println!("cast_checks rewriting {text}");
            }
            format!("invalid cast in {text}")
        };

        #[cfg(not(procmacro2_semver_exempt))]
        let msg = {
            if enabled("CAST_CHECKS_LOG") {
                WARN.call_once(|| {
                    println!();
                    println!(
                        "WARNING: `CAST_CHECKS_LOG` is enabled, but this option requires \
                         `--cfg procmacro2_semver_exempt` to be passed to rustc"
                    );
                    println!();
                });
            }
            String::from("invalid cast")
        };

        let tokens = quote_spanned! { span =>
            {
                #[allow(unused_imports)]
                use cast_checks::MaybeTryIntoFallback;

                if let Some(result) = cast_checks::MaybeTryInto::<_, #ty >::new( #operand ).maybe_try_into() {
                    result.expect( #msg )
                } else {
                    #operand as #ty
                }
            }
        };

        *expr = parse(tokens.into()).unwrap();
    }
}

fn enabled(key: &str) -> bool {
    std::env::var(key).map_or(false, |value| value != "0")
}
