use syn::{Expr, Ident, Lit, LitStr};

/// Extension methods for [`Expr`].
///
/// Provides methods for quickly unwrapping nested variants and fields
///  from [`Expr`], `panic()`-ing with a meaningful message whenever
/// unexpected patterns are encountered.
pub(crate) trait ExprExt {
    /// Unwraps a reference to the [`LitStr`] contained in `self`.
    fn as_lit_str(&self) -> &LitStr;

    /// Unwraps a reference to the single [`Ident`] of the [`Path`] contained in `self`.
    ///
    /// [`Path`]: syn::Path
    fn as_single_ident_path(&self) -> &Ident;
}

impl ExprExt for Expr {
    fn as_lit_str(&self) -> &LitStr {
        let Expr::Lit(expr_lit) =  self else {
            panic!("unexpected expression: {self:?}");
        };

        let Lit::Str(lit_str) = &expr_lit.lit else {
            panic!("unexpected literal: {expr_lit:?}");
        };

        lit_str
    }

    fn as_single_ident_path(&self) -> &Ident {
        let Expr::Path(expr_path) = self else {
            panic!("unexpected expression: {self:?}");
        };

        let path = &expr_path.path;

        if path.segments.len() > 1 {
            panic!("unexpected multi-segment path: {path:?}");
        }

        &path.segments[0].ident
    }
}
