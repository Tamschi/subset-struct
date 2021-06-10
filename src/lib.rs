#![doc(html_root_url = "https://docs.rs/subset-struct/0.0.1")]
#![warn(clippy::pedantic)]

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use std::fmt::Display;
use syn::{
	parse::{Parse, ParseStream, Peek},
	parse_macro_input,
	punctuated::Punctuated,
	spanned::Spanned,
	token::{Brace, CustomToken, Paren},
	Error, Ident, NestedMeta, Result, Token,
};
use tap::Pipe;

#[proc_macro_attribute]
pub fn subset(attr_params: TokenStream1, item: TokenStream1) -> TokenStream1 {
	let attr_args = parse_macro_input!(attr_args as TokenStream);
	let item = parse_macro_input!(item as TokenStream);

	item.into()
}

enum AttributeParameter {
	Default((Token![default], Paren, Punctuated<TokenStream, Token![,]>)),
	Subset(
		(
			Option<(
				Brace,
				Punctuated<
					(
						Ident,
						Paren,
						Option<Token![&]>,
						Option<Token![mut]>,
						Token![self],
					),
					Token![,],
				>,
				Token![->],
			)>,
			Ident,
			Paren,
			Punctuated<TokenStream, Token![,]>,
		),
	),
}
impl AttributeParameter {
	pub fn parse_punctuated(
		input: ParseStream,
		errors: &mut Vec<syn::Error>,
	) -> Punctuated<Self, Token![,]> {
		let mut result = Punctuated::new();
		while !input.is_empty() {
			result.push_value(Self::parse(input, errors));
			if let Some(comma) = input.parse().unwrap() {
				result.push_punct(comma)
			} else if !input.is_empty() {
				errors.push(Error::new(input.span(), "Expected `,`"))
			}
		}
		result
	}

	pub fn parse(input: ParseStream, errors: &mut Vec<syn::Error>) -> Self {
		let params;
		if let Some(r#default) = input.parse() {
			Self::Default((
				r#default,
				{
					let mut group = peek_parse_or_error_fallback(
						input,
						Paren,
						errors,
						r#default.span(),
						"Expected `(…)`",
						|span| TokenTree::Group(Group::new(Paren(span), TokenStream::new())),
					)
					.unwrap();
					loop {
						match group {
							TokenTree::Group(group) => match group.delimiter() {
								Delimiter::Parenthesis => {
									params = group.stream();
									break Paren(group.span());
								}
								Delimiter::None => group = gr,
								Delimiter::Brace => todo!(),
								Delimiter::Bracket => todo!(),
							},
							_ => unreachable!(),
						}
					}
				},
				{ todo!() },
			))
		} else {
			todo!()
		}
	}
}

fn peek_parse_or_error_fallback<P: Peek, T: Parse>(
	input: ParseStream,
	peek: P,
	errors: &mut Vec<Error>,
	fallback_span: Span,
	message: impl Display,
	fallback: impl FnOnce(Span) -> T,
) -> Result<T> {
	if input.peek(peek) {
		return input.parse();
	}
	// flattened `else`
	let span = if input.is_empty() {
		fallback_span
	} else {
		input.span()
	};
	errors.push(Error::new(span, message));
	fallback(span).pipe(Ok)
}
