use subset_struct::subset;

#[subset {
	default(aa aa,,,),
	default(),
	{+} -> Hello, // The `+` is invalid there.
}]
pub struct Set {
	pub a: (),
	pub b: (),
	pub c: (),
}
