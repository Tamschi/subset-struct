#![doc(html_root_url = "https://docs.rs/subset-struct/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(
	clippy::non_ascii_literal,
	clippy::too_many_lines,
	clippy::type_complexity
)]

use call2_for_syn::call2_strict;
use debugless_unwrap::DebuglessUnwrap;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::quote_spanned;
use std::{
	fmt::Display,
	iter,
	sync::{atomic::AtomicBool, Once},
};
use syn::{
	braced, parenthesized,
	parse::{Parse, ParseBuffer, ParseStream, Parser, Peek},
	parse_macro_input,
	punctuated::Punctuated,
	spanned::Spanned,
	token::{Brace, CustomToken, Paren},
	Error, Ident, NestedMeta, Result, Token,
};
use tap::Pipe;

static HOOK_PANIC: Once = Once::new();

#[proc_macro_attribute]
pub fn subset(attr_params: TokenStream1, item: TokenStream1) -> TokenStream1 {
	HOOK_PANIC.call_once(|| {
		std::panic::set_hook(Box::new(|panic_info| {
			eprintln!("Macro panic: {}", panic_info)
		}))
	});

	let attr_args = parse_macro_input!(attr_params as TokenStream);
	let item = parse_macro_input!(item as TokenStream);

	let mut errors = vec![];
	let attr_args = call2_strict(attr_args, |attr_args: ParseStream| {
		AttributeParameter::parse_punctuated(attr_args, &mut errors)
	})
	.debugless_unwrap();

	//TODO below
	let mut output = TokenStream::new();
	for error in errors {
		output.extend(error.to_compile_error())
	}
	output.extend(item);
	output.into()
}

enum AttributeParameter {
	Default {
		default_: Token![default],
		attr_paren: Paren,
		attrs: Punctuated<TokenStream, Token![,]>,
	},
	Subset {
		conv_brace: Brace,
		conversions: Punctuated<
			(
				Ident,
				Paren,
				Option<Token![&]>,
				Option<Token![mut]>,
				Token![self],
			),
			Token![,],
		>,
		r_arrow: Token![->],
		name: Ident,
		attr_paren: Paren,
		attrs: Punctuated<TokenStream, Token![,]>,
		end_span: Span,
	},
	Invalid(TokenTree),
}
impl AttributeParameter {
	fn end_span(&self) -> Span {
		match self {
			AttributeParameter::Default {
				attr_paren: Paren { span },
				..
			} => *span,
			AttributeParameter::Subset { end_span, .. } => *end_span,
			AttributeParameter::Invalid(tt) => tt.span(),
		}
	}
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
				let span = result.last().unwrap().end_span();
				errors.push(Error::new(span, "Expected `,` after this"));
				result.push_punct(Token![,](span))
			}
		}
		result
	}

	pub fn parse(input: ParseStream, errors: &mut Vec<syn::Error>) -> Self {
		if let Some(default_) = input.parse().unwrap() {
			let mut attrs: Option<TokenStream> = None;
			Self::Default {
				default_,
				attr_paren: {
					if input.peek(Paren) {
						(|| -> Result<_> {
							let content;
							let paren = parenthesized!(content in input);
							attrs = (&content).parse::<TokenStream>().unwrap().pipe(Some);
							Ok(paren)
						})()
						.unwrap()
					} else {
						errors.push(Error::new(default_.span, "Expected `(…)` after this"));
						Parser::parse2(
							|fake: ParseStream| {
								let content;
								let paren = parenthesized!(content in fake);
								attrs = content.parse::<TokenStream>().unwrap().pipe(Some);
								Ok(paren)
							},
							quote_spanned!(default_.span=> ()),
						)
						.unwrap()
					}
				},
				attrs: {
					let mut punctuated = Punctuated::new();
					for token in attrs.unwrap() {
						if punctuated.empty_or_trailing() {
							punctuated.push_value(TokenStream::new())
						}
						match token {
							TokenTree::Punct(p) if p.as_char() == ',' => {
								punctuated.push_punct(Token![,](p.span()))
							}
							other => punctuated.last_mut().unwrap().extend(iter::once(other)),
						}
					}
					punctuated
				},
			}
		} else if input.peek(Brace) || input.peek(Ident) {
			let mut real_brace_span = None;
			let mut conversions: Option<TokenStream> = None;
			let name_span;
			let mut attrs: Option<TokenStream> = None;
			let mut end_span = input.span();
			Self::Subset {
				conv_brace: if input.peek(Brace) {
					(|| -> Result<_> {
						let content;
						let brace = braced!(content in input);
						conversions = (&content).parse::<TokenStream>().unwrap().pipe(Some);
						real_brace_span = Some(brace.span);
						end_span = brace.span;
						Ok(brace)
					})()
					.unwrap()
				} else {
					Parser::parse2(
						|fake: ParseStream| {
							let content;
							let brace = braced!(content in fake);
							conversions = content.parse::<TokenStream>().unwrap().pipe(Some);
							Ok(brace)
						},
						quote_spanned!(input.span()=> {}),
					)
					.unwrap()
				},
				conversions: {
					let mut punctuated = Punctuated::new();
					for token in conversions.unwrap() {
						todo!("conversions")
					}
					punctuated
				},
				r_arrow: if let Some(r_arrow) = input.parse().unwrap() {
					let r_arrow: Token![->] = r_arrow;
					end_span = r_arrow.span();
					r_arrow
				} else {
					if real_brace_span.is_some() {
						errors.push(Error::new(input.span(), "Expected `->` before this"));
					}
					Token![->](input.span())
				},
				name: if input.peek(Ident) {
					let name: Ident = input.parse().unwrap();
					name_span = name.span();
					end_span = name_span.span();
					name
				} else {
					errors.push(Error::new(input.span(), "Expected identifier"));
					name_span = real_brace_span.unwrap_or_else(|| input.span());
					Ident::new("__", name_span)
				},
				attr_paren: {
					if input.peek(Paren) {
						(|| -> Result<_> {
							let content;
							let paren = parenthesized!(content in input);
							attrs = (&content).parse::<TokenStream>().unwrap().pipe(Some);
							end_span = paren.span;
							Ok(paren)
						})()
						.unwrap()
					} else {
						Parser::parse2(
							|fake: ParseStream| {
								let content;
								let paren = parenthesized!(content in fake);
								attrs = content.parse::<TokenStream>().unwrap().pipe(Some);
								Ok(paren)
							},
							quote_spanned!(name_span=> ()),
						)
						.unwrap()
					}
				},
				attrs: {
					let mut punctuated = Punctuated::new();
					for token in attrs.unwrap() {
						if punctuated.empty_or_trailing() {
							punctuated.push_value(TokenStream::new())
						}
						match token {
							TokenTree::Punct(p) if p.as_char() == ',' => {
								punctuated.push_punct(Token![,](p.span()))
							}
							other => punctuated.last_mut().unwrap().extend(iter::once(other)),
						}
					}
					punctuated
				},
				end_span,
			}
		} else {
			errors.push(Error::new(
				input.span(),
				"Expected `default`, `{…} -> Ident` or identifier",
			));
			Self::Invalid(input.parse().unwrap())
		}
	}
}
