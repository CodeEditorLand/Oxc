use std::path::{Path, PathBuf};

use oxc_allocator::Allocator;
use oxc_codegen::CodeGenerator;
use oxc_minifier::{CompressOptions, Compressor};
use oxc_parser::Parser;
use oxc_span::SourceType;

use crate::{
	babel::BabelCase,
	suite::{Case, TestResult},
	test262::{Test262Case, TestFlag},
};

pub struct MinifierTest262Case {
	base:Test262Case,
}

impl Case for MinifierTest262Case {
	fn new(path:PathBuf, code:String) -> Self { Self { base:Test262Case::new(path, code) } }

	fn code(&self) -> &str { self.base.code() }

	fn path(&self) -> &Path { self.base.path() }

	fn test_result(&self) -> &TestResult { self.base.test_result() }

	fn skip_test_case(&self) -> bool { self.base.should_fail() }

	fn run(&mut self) {
		let source_text = self.base.code();

		let is_module = self.base.meta().flags.contains(&TestFlag::Module);

		let source_type = SourceType::default().with_module(is_module);

		let result = get_result(source_text, source_type);

		self.base.set_result(result);
	}
}

pub struct MinifierBabelCase {
	base:BabelCase,
}

impl Case for MinifierBabelCase {
	fn new(path:PathBuf, code:String) -> Self { Self { base:BabelCase::new(path, code) } }

	fn code(&self) -> &str { self.base.code() }

	fn path(&self) -> &Path { self.base.path() }

	fn test_result(&self) -> &TestResult { self.base.test_result() }

	fn skip_test_case(&self) -> bool {
		self.base.skip_test_case()
			|| self.base.should_fail()
			|| self.base.source_type().is_typescript()
	}

	fn run(&mut self) {
		let source_text = self.base.code();

		let source_type = self.base.source_type();

		let result = get_result(source_text, source_type);

		self.base.set_result(result);
	}
}
// Test minification by minifying twice because it is a idempotent
fn get_result(source_text:&str, source_type:SourceType) -> TestResult {
	let source_text1 = minify(source_text, source_type);

	let source_text2 = minify(&source_text1, source_type);

	if source_text1 == source_text2 {
		TestResult::Passed
	} else {
		TestResult::ParseError(String::new(), false)
	}
}

fn minify(source_text:&str, source_type:SourceType) -> String {
	let allocator = Allocator::default();

	let ret = Parser::new(&allocator, source_text, source_type).parse();

	let program = allocator.alloc(ret.program);

	Compressor::new(&allocator, CompressOptions::all_true()).build(program);

	CodeGenerator::new().build(program).source_text
}
