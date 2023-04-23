#![feature(test)]


pub mod asm;
pub mod diagn;
pub mod expr;
pub mod syntax;
pub mod util;
pub mod driver;


pub mod webasm;


#[cfg(test)]
pub mod test;


/// Convenience function to assemble a given string.
/// 
/// The code cannot access external files through
/// `#include` or `incbin()`.
/// 
/// Returns the generated bytes of the assembled binary
/// if successful, together with the report containing
/// any errors or warnings that were encountered.
pub fn assemble_str_to_binary(
	src: &str)
	-> (Option<Vec<u8>>, diagn::Report)
{
	let virtual_filename = "str";

	let mut report = diagn::Report::new();
	let mut fileserver = util::FileServerMock::new();
	fileserver.add(virtual_filename, src.clone());

	let opts = asm::AssemblyOptions::new();

	let assembly = asm::assemble(
		&mut report,
		&opts,
		&mut fileserver,
		&[virtual_filename]);
	
	(assembly.output.map(|o| o.format_binary()), report)
}