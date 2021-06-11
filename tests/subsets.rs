use serde::{Deserialize, Serialize};
use subset_struct::subset;

struct Extant {}

#[subset(
	/// This documentation will appear on `Base`, as will the attribute.
	#[derive(Deserialize)]
	default,

	/// Attributes, including documentation, added here will appear on `Filtered`, each conversion method and the `From` impl block.
	{
		// A `From` implementation is also generated,
		// but a more specific method is easier to use flexibly.
		// Use `&self` to clone each field instead.

		/// Document specific conversion methods like this.
		into_forwarded(self),
	} ->
		/// This will appear only on `Filtered`.
		#[derive(Serialize)]
		Filtered,

	// Add a new lifetime (which will appear before any others)
	// to generate a proxy structure instead.
	// `mut` is optional.
	{
		// Conversions, including `From`, must happen from the matching reference.
		as_proxy(&mut self),
	} -> mut Proxy<'a>,

	// Specifying custom conversion methods or additional attributes is optional:

	/// This documentation will appear only on `Empty`, but not the `From` implementation.
	Empty,

	// You can use `ref` to skip generating the subset struct definition, which will only generate conversions.
	// Use `ref mut` if needed.
	// Additional attributes for the struct definition are not allowed here,
	// but you can still specify some for the `From` impl if you write `{} -> ref Extant` instead and put them above that.
	ref Extant,
)]
pub struct Base {
	pub secret: String,
	#[subset(
		Filtered (
			// You could specify set of subset-specific attributes here, as above.
		),
		Proxy,
	)]
	pub public: String,
}
