use proc_macro2::{Delimiter, TokenTree};

use syn::{Field, Path, PathSegment};

const PARENS: Delimiter = Delimiter::Parenthesis;

/// checks to see if a field's type is `Option`.  This logic is necessary to find cases where fields contain nested
/// Options and avoid a `Some(None)` case.
pub fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
    let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.ident.to_string());
        acc.push('|');
        acc
    });
    vec!["Option|", "std|option|Option|", "core|option|Option|"]
        .into_iter()
        .find(|s| idents_of_path == *s)
        .and_then(|_| path.segments.last())
}

/// checks to see if the `should_ignore` attribute has been put before a field.
pub fn should_ignore(field: &Field) -> bool {
    check_attr_on_field(field, "should_ignore")
}

/// checks to see if the `into_from` attribute has been put before a field.
pub fn into_from(field: &Field) -> bool {
    check_attr_on_field(field, "into_from")
}

fn check_attr_on_field(field: &Field, name: &str) -> bool {
    let mut attr_exists = false;

    field.attrs.iter().for_each(|field| {
        let attr_seg: Vec<String> = field.path.segments.iter().map(|seg| format!("{}", seg.ident)).collect();

        let diff_attr = attr_seg == ["diff"];
        let arg_iter = field.tokens.clone().into_iter().next();

        let should_ignore = match arg_iter {
            Some(TokenTree::Group(gr)) if gr.delimiter() == PARENS => {
                let tokens: Vec<String> = gr.stream().into_iter().map(|tt| format!("{}", tt)).collect();

                tokens.contains(&name.into())
            }
            _ => false,
        };

        attr_exists = attr_exists || diff_attr && should_ignore
    });

    attr_exists
}
