use subset_struct::subset;

#[subset {
	default(aa aa,,,),
	default(),
	{} -> Hello,
}]
pub struct Set {
	pub a: (),
	pub b: (),
	pub c: (),
}
