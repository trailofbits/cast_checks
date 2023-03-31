// #![doc = include_str!("../../README.md")]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse, parse_quote_spanned,
    spanned::Spanned,
    visit_mut::{visit_expr_mut, visit_item_mod_mut, VisitMut},
    Error, Expr, ExprCast, File, Item, ItemMod, Type,
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
pub fn enable(args: TokenStream, tokens: TokenStream) -> TokenStream {
    assert!(args.is_empty());
    match parse::<Item>(tokens.clone()) {
        Ok(mut item) => {
            Visitor.visit_item_mut(&mut item);
            item.into_token_stream()
        }
        Err(error) => {
            let error = if let Ok(file) = parse::<File>(tokens) {
                Error::new(
                    file.span(),
                    "applying `cast_checks::enable` at the crate root is not currently supported",
                )
            } else {
                Error::new(error.span(), format!("failed to parse item: {error}"))
            };
            error.to_compile_error()
        }
    }
    .into()
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_item_mod_mut(&mut self, item_mod: &mut ItemMod) {
        visit_item_mod_mut(self, item_mod);

        #[cfg(procmacro2_semver_exempt)]
        if item_mod.content.is_none() && enabled("CAST_CHECKS_LOG") {
            println!("cast_checks not descending into {}", tokens_at(item_mod));
        }
    }

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

        #[cfg(procmacro2_semver_exempt)]
        let msg = {
            let text = tokens_at(expr);
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

        *expr = parse_quote_spanned! { expr.span() =>
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
    }
}

#[cfg(procmacro2_semver_exempt)]
fn tokens_at<T>(tokens: &T) -> String
where
    T: Spanned + ToTokens,
{
    let span = tokens.span();
    format!(
        "`{}` at {}:{}:{}",
        tokens.to_token_stream(),
        span.source_file().path().display(),
        span.start().line,
        span.start().column,
    )
}

fn enabled(key: &str) -> bool {
    std::env::var(key).map_or(false, |value| value != "0")
}
