// Auto generated by `tasks/compat_data/src/lib.rs`.
#![allow(clippy::enum_glob_use, clippy::match_same_arms)]

use std::sync::OnceLock;

use browserslist::Version;
use rustc_hash::FxHashMap;

use super::{Engine, EngineTargets};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ESFeature {
	ES5MemberExpressionLiterals,
	ES5PropertyLiterals,
	ES5ReservedWords,
	ES2015Parameters,
	ES2015TemplateLiterals,
	ES2015Literals,
	ES2015FunctionName,
	ES2015ArrowFunctions,
	ES2015BlockScopedFunctions,
	ES2015Classes,
	ES2015ObjectSuper,
	ES2015ShorthandProperties,
	ES2015DuplicateKeys,
	ES2015ComputedProperties,
	ES2015ForOf,
	ES2015StickyRegex,
	ES2015UnicodeEscapes,
	ES2015UnicodeRegex,
	ES2015Spread,
	ES2015Destructuring,
	ES2015BlockScoping,
	ES2015TypeofSymbol,
	ES2015NewTarget,
	ES2015Regenerator,
	ES2016ExponentiationOperator,
	ES2017AsyncToGenerator,
	ES2018AsyncGeneratorFunctions,
	ES2018ObjectRestSpread,
	ES2018DotallRegex,
	ES2018UnicodePropertyRegex,
	ES2018NamedCapturingGroupsRegex,
	ES2018LookbehindRegex,
	ES2019JsonStrings,
	ES2019OptionalCatchBinding,
	ES2020NullishCoalescingOperator,
	ES2020OptionalChaining,
	ES2020BigInt,
	ES2021NumericSeparator,
	ES2021LogicalAssignmentOperators,
	ES2022ClassStaticBlock,
	ES2022PrivatePropertyInObject,
	ES2022ClassProperties,
	ES2022PrivateMethods,
	ES2022MatchIndicesRegex,
	ES2024UnicodeSetsRegex,
	ES2025DuplicateNamedCapturingGroupsRegex,
	ES2025RegexpModifiers,
}
pub fn features() -> &'static FxHashMap<ESFeature, EngineTargets> {
	use ESFeature::*;
	use Engine::*;

	static FEATURES:OnceLock<FxHashMap<ESFeature, EngineTargets>> = OnceLock::new();

	FEATURES.get_or_init(|| {
		FxHashMap::from_iter([
			(
				ES5MemberExpressionLiterals,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(7u32, 0u32, 0u32)),
					(Safari, Version(5u32, 1u32, 0u32)),
					(OperaMobile, Version(12u32, 0u32, 0u32)),
					(Samsung, Version(1u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 13u32)),
					(Node, Version(0u32, 4u32, 0u32)),
					(Ie, Version(9u32, 0u32, 0u32)),
					(Firefox, Version(2u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Android, Version(4u32, 0u32, 0u32)),
					(Electron, Version(0u32, 20u32, 0u32)),
					(Opera, Version(12u32, 0u32, 0u32)),
					(Ios, Version(6u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(5u32, 0, 0)),
				])),
			),
			(
				ES5PropertyLiterals,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(7u32, 0u32, 0u32)),
					(Safari, Version(5u32, 1u32, 0u32)),
					(OperaMobile, Version(12u32, 0u32, 0u32)),
					(Samsung, Version(1u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 13u32)),
					(Node, Version(0u32, 4u32, 0u32)),
					(Ie, Version(9u32, 0u32, 0u32)),
					(Firefox, Version(2u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Android, Version(4u32, 0u32, 0u32)),
					(Electron, Version(0u32, 20u32, 0u32)),
					(Opera, Version(12u32, 0u32, 0u32)),
					(Ios, Version(6u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(5u32, 0, 0)),
				])),
			),
			(
				ES5ReservedWords,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(13u32, 0u32, 0u32)),
					(Safari, Version(3u32, 1u32, 0u32)),
					(OperaMobile, Version(10u32, 1u32, 0u32)),
					(Samsung, Version(1u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 13u32)),
					(Node, Version(0u32, 6u32, 0u32)),
					(Ie, Version(9u32, 0u32, 0u32)),
					(Firefox, Version(2u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Android, Version(4u32, 4u32, 0u32)),
					(Electron, Version(0u32, 20u32, 0u32)),
					(Opera, Version(10u32, 50u32, 0u32)),
					(Ios, Version(6u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(5u32, 0, 0)),
				])),
			),
			(
				ES2015Parameters,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(49u32, 0u32, 0u32)),
					(Safari, Version(16u32, 3u32, 0u32)),
					(OperaMobile, Version(36u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 0u32, 0u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 37u32, 0u32)),
					(Opera, Version(36u32, 0u32, 0u32)),
					(Ios, Version(16u32, 3u32, 0u32)),
					(Edge, Version(18u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015TemplateLiterals,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(41u32, 0u32, 0u32)),
					(Safari, Version(13u32, 0u32, 0u32)),
					(OperaMobile, Version(28u32, 0u32, 0u32)),
					(Samsung, Version(3u32, 4u32, 0u32)),
					(Node, Version(4u32, 0u32, 0u32)),
					(Firefox, Version(34u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 21u32, 0u32)),
					(Opera, Version(28u32, 0u32, 0u32)),
					(Ios, Version(13u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015Literals,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(44u32, 0u32, 0u32)),
					(Safari, Version(9u32, 0u32, 0u32)),
					(OperaMobile, Version(32u32, 0u32, 0u32)),
					(Samsung, Version(4u32, 0u32, 0u32)),
					(Node, Version(4u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 15u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 30u32, 0u32)),
					(Opera, Version(31u32, 0u32, 0u32)),
					(Ios, Version(9u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015FunctionName,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(51u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(41u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 5u32, 0u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 2u32, 0u32)),
					(Opera, Version(38u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015ArrowFunctions,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(47u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(34u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 13u32)),
					(Firefox, Version(43u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 36u32, 0u32)),
					(Opera, Version(34u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015BlockScopedFunctions,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(41u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(28u32, 0u32, 0u32)),
					(Samsung, Version(3u32, 4u32, 0u32)),
					(Node, Version(4u32, 0u32, 0u32)),
					(Ie, Version(11u32, 0u32, 0u32)),
					(Firefox, Version(46u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 21u32, 0u32)),
					(Opera, Version(28u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015Classes,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(46u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(33u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(5u32, 0u32, 0u32)),
					(Firefox, Version(45u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 36u32, 0u32)),
					(Opera, Version(33u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015ObjectSuper,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(46u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(33u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(5u32, 0u32, 0u32)),
					(Firefox, Version(45u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 36u32, 0u32)),
					(Opera, Version(33u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015ShorthandProperties,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(43u32, 0u32, 0u32)),
					(Safari, Version(9u32, 0u32, 0u32)),
					(OperaMobile, Version(30u32, 0u32, 0u32)),
					(Samsung, Version(4u32, 0u32, 0u32)),
					(Node, Version(4u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 14u32)),
					(Firefox, Version(33u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 27u32, 0u32)),
					(Opera, Version(30u32, 0u32, 0u32)),
					(Ios, Version(9u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015DuplicateKeys,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(42u32, 0u32, 0u32)),
					(Safari, Version(9u32, 0u32, 0u32)),
					(OperaMobile, Version(29u32, 0u32, 0u32)),
					(Samsung, Version(3u32, 4u32, 0u32)),
					(Node, Version(4u32, 0u32, 0u32)),
					(Firefox, Version(34u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 25u32, 0u32)),
					(Opera, Version(29u32, 0u32, 0u32)),
					(Ios, Version(9u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015ComputedProperties,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(44u32, 0u32, 0u32)),
					(Safari, Version(7u32, 1u32, 0u32)),
					(OperaMobile, Version(32u32, 0u32, 0u32)),
					(Samsung, Version(4u32, 0u32, 0u32)),
					(Node, Version(4u32, 0u32, 0u32)),
					(Firefox, Version(34u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 30u32, 0u32)),
					(Opera, Version(31u32, 0u32, 0u32)),
					(Ios, Version(8u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015ForOf,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(51u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(41u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 5u32, 0u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 2u32, 0u32)),
					(Opera, Version(38u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(15u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015StickyRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(49u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(36u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 15u32)),
					(Firefox, Version(3u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 37u32, 0u32)),
					(Opera, Version(36u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015UnicodeEscapes,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(44u32, 0u32, 0u32)),
					(Safari, Version(9u32, 0u32, 0u32)),
					(OperaMobile, Version(32u32, 0u32, 0u32)),
					(Samsung, Version(4u32, 0u32, 0u32)),
					(Node, Version(4u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 15u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 30u32, 0u32)),
					(Opera, Version(31u32, 0u32, 0u32)),
					(Ios, Version(9u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015UnicodeRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(50u32, 0u32, 0u32)),
					(Safari, Version(12u32, 0u32, 0u32)),
					(OperaMobile, Version(37u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 0u32, 0u32)),
					(Firefox, Version(46u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 1u32, 0u32)),
					(Opera, Version(37u32, 0u32, 0u32)),
					(Ios, Version(12u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015Spread,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(46u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(33u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(5u32, 0u32, 0u32)),
					(Firefox, Version(45u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 36u32, 0u32)),
					(Opera, Version(33u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015Destructuring,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(51u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(41u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 5u32, 0u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 2u32, 0u32)),
					(Opera, Version(38u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(15u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015BlockScoping,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(50u32, 0u32, 0u32)),
					(Safari, Version(11u32, 0u32, 0u32)),
					(OperaMobile, Version(37u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 0u32, 0u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 1u32, 0u32)),
					(Opera, Version(37u32, 0u32, 0u32)),
					(Ios, Version(11u32, 0u32, 0u32)),
					(Edge, Version(14u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015TypeofSymbol,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(38u32, 0u32, 0u32)),
					(Safari, Version(9u32, 0u32, 0u32)),
					(OperaMobile, Version(25u32, 0u32, 0u32)),
					(Samsung, Version(3u32, 0u32, 0u32)),
					(Node, Version(0u32, 12u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 13u32)),
					(Firefox, Version(36u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 20u32, 0u32)),
					(Opera, Version(25u32, 0u32, 0u32)),
					(Ios, Version(9u32, 0u32, 0u32)),
					(Edge, Version(12u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015NewTarget,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(46u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(33u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(5u32, 0u32, 0u32)),
					(Firefox, Version(41u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(0u32, 36u32, 0u32)),
					(Opera, Version(33u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(14u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2015Regenerator,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(50u32, 0u32, 0u32)),
					(Safari, Version(10u32, 0u32, 0u32)),
					(OperaMobile, Version(37u32, 0u32, 0u32)),
					(Samsung, Version(5u32, 0u32, 0u32)),
					(Node, Version(6u32, 0u32, 0u32)),
					(Firefox, Version(53u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 1u32, 0u32)),
					(Opera, Version(37u32, 0u32, 0u32)),
					(Ios, Version(10u32, 0u32, 0u32)),
					(Edge, Version(13u32, 0u32, 0u32)),
					(Es, Version(2015u32, 0, 0)),
				])),
			),
			(
				ES2016ExponentiationOperator,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(52u32, 0u32, 0u32)),
					(Safari, Version(10u32, 1u32, 0u32)),
					(OperaMobile, Version(41u32, 0u32, 0u32)),
					(Samsung, Version(6u32, 0u32, 0u32)),
					(Node, Version(7u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 14u32)),
					(Firefox, Version(52u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 3u32, 0u32)),
					(Opera, Version(39u32, 0u32, 0u32)),
					(Ios, Version(10u32, 3u32, 0u32)),
					(Edge, Version(14u32, 0u32, 0u32)),
					(Es, Version(2016u32, 0, 0)),
				])),
			),
			(
				ES2017AsyncToGenerator,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(55u32, 0u32, 0u32)),
					(Safari, Version(11u32, 0u32, 0u32)),
					(OperaMobile, Version(42u32, 0u32, 0u32)),
					(Samsung, Version(6u32, 0u32, 0u32)),
					(Node, Version(7u32, 6u32, 0u32)),
					(Firefox, Version(52u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(1u32, 6u32, 0u32)),
					(Opera, Version(42u32, 0u32, 0u32)),
					(Ios, Version(11u32, 0u32, 0u32)),
					(Edge, Version(15u32, 0u32, 0u32)),
					(Es, Version(2017u32, 0, 0)),
				])),
			),
			(
				ES2018AsyncGeneratorFunctions,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(63u32, 0u32, 0u32)),
					(Safari, Version(12u32, 0u32, 0u32)),
					(OperaMobile, Version(46u32, 0u32, 0u32)),
					(Samsung, Version(8u32, 0u32, 0u32)),
					(Node, Version(10u32, 0u32, 0u32)),
					(Firefox, Version(57u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(3u32, 0u32, 0u32)),
					(Opera, Version(50u32, 0u32, 0u32)),
					(Ios, Version(12u32, 0u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2018u32, 0, 0)),
				])),
			),
			(
				ES2018ObjectRestSpread,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(60u32, 0u32, 0u32)),
					(Safari, Version(11u32, 1u32, 0u32)),
					(OperaMobile, Version(44u32, 0u32, 0u32)),
					(Samsung, Version(8u32, 0u32, 0u32)),
					(Node, Version(8u32, 3u32, 0u32)),
					(Firefox, Version(55u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(2u32, 0u32, 0u32)),
					(Opera, Version(47u32, 0u32, 0u32)),
					(Ios, Version(11u32, 3u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2018u32, 0, 0)),
				])),
			),
			(
				ES2018DotallRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(62u32, 0u32, 0u32)),
					(Safari, Version(11u32, 1u32, 0u32)),
					(OperaMobile, Version(46u32, 0u32, 0u32)),
					(Samsung, Version(8u32, 0u32, 0u32)),
					(Node, Version(8u32, 10u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 15u32)),
					(Firefox, Version(78u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(3u32, 0u32, 0u32)),
					(Opera, Version(49u32, 0u32, 0u32)),
					(Ios, Version(11u32, 3u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2018u32, 0, 0)),
				])),
			),
			(
				ES2018UnicodePropertyRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(64u32, 0u32, 0u32)),
					(Safari, Version(11u32, 1u32, 0u32)),
					(OperaMobile, Version(47u32, 0u32, 0u32)),
					(Samsung, Version(9u32, 0u32, 0u32)),
					(Node, Version(10u32, 0u32, 0u32)),
					(Firefox, Version(78u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(3u32, 0u32, 0u32)),
					(Opera, Version(51u32, 0u32, 0u32)),
					(Ios, Version(11u32, 3u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2018u32, 0, 0)),
				])),
			),
			(
				ES2018NamedCapturingGroupsRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(64u32, 0u32, 0u32)),
					(Safari, Version(11u32, 1u32, 0u32)),
					(OperaMobile, Version(47u32, 0u32, 0u32)),
					(Samsung, Version(9u32, 0u32, 0u32)),
					(Node, Version(10u32, 0u32, 0u32)),
					(Firefox, Version(78u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(3u32, 0u32, 0u32)),
					(Opera, Version(51u32, 0u32, 0u32)),
					(Ios, Version(11u32, 3u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2018u32, 0, 0)),
				])),
			),
			(
				ES2018LookbehindRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(62u32, 0u32, 0u32)),
					(Safari, Version(16u32, 4u32, 0u32)),
					(OperaMobile, Version(46u32, 0u32, 0u32)),
					(Samsung, Version(8u32, 0u32, 0u32)),
					(Node, Version(8u32, 10u32, 0u32)),
					(Firefox, Version(78u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(3u32, 0u32, 0u32)),
					(Opera, Version(49u32, 0u32, 0u32)),
					(Ios, Version(16u32, 4u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2018u32, 0, 0)),
				])),
			),
			(
				ES2019JsonStrings,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(66u32, 0u32, 0u32)),
					(Safari, Version(12u32, 0u32, 0u32)),
					(OperaMobile, Version(47u32, 0u32, 0u32)),
					(Samsung, Version(9u32, 0u32, 0u32)),
					(Node, Version(10u32, 0u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 14u32)),
					(Firefox, Version(62u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(3u32, 0u32, 0u32)),
					(Opera, Version(53u32, 0u32, 0u32)),
					(Ios, Version(12u32, 0u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2019u32, 0, 0)),
				])),
			),
			(
				ES2019OptionalCatchBinding,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(66u32, 0u32, 0u32)),
					(Safari, Version(11u32, 1u32, 0u32)),
					(OperaMobile, Version(47u32, 0u32, 0u32)),
					(Samsung, Version(9u32, 0u32, 0u32)),
					(Node, Version(10u32, 0u32, 0u32)),
					(Firefox, Version(58u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(3u32, 0u32, 0u32)),
					(Opera, Version(53u32, 0u32, 0u32)),
					(Ios, Version(11u32, 3u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2019u32, 0, 0)),
				])),
			),
			(
				ES2020OptionalChaining,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(91u32, 0u32, 0u32)),
					(Safari, Version(13u32, 1u32, 0u32)),
					(OperaMobile, Version(64u32, 0u32, 0u32)),
					(Samsung, Version(16u32, 0u32, 0u32)),
					(Node, Version(16u32, 9u32, 0u32)),
					(Firefox, Version(74u32, 0u32, 0u32)),
					(Deno, Version(1u32, 9u32, 0u32)),
					(Electron, Version(13u32, 0u32, 0u32)),
					(Opera, Version(77u32, 0u32, 0u32)),
					(Ios, Version(13u32, 4u32, 0u32)),
					(Edge, Version(91u32, 0u32, 0u32)),
					(Es, Version(2019u32, 0, 0)),
				])),
			),
			(
				ES2020NullishCoalescingOperator,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(80u32, 0u32, 0u32)),
					(Safari, Version(13u32, 1u32, 0u32)),
					(OperaMobile, Version(57u32, 0u32, 0u32)),
					(Samsung, Version(13u32, 0u32, 0u32)),
					(Node, Version(14u32, 0u32, 0u32)),
					(Firefox, Version(72u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(8u32, 0u32, 0u32)),
					(Opera, Version(67u32, 0u32, 0u32)),
					(Ios, Version(13u32, 4u32, 0u32)),
					(Edge, Version(80u32, 0u32, 0u32)),
					(Es, Version(2020u32, 0, 0)),
				])),
			),
			(
				ES2020OptionalChaining,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(91u32, 0u32, 0u32)),
					(Safari, Version(13u32, 1u32, 0u32)),
					(OperaMobile, Version(64u32, 0u32, 0u32)),
					(Samsung, Version(16u32, 0u32, 0u32)),
					(Node, Version(16u32, 9u32, 0u32)),
					(Firefox, Version(74u32, 0u32, 0u32)),
					(Deno, Version(1u32, 9u32, 0u32)),
					(Electron, Version(13u32, 0u32, 0u32)),
					(Opera, Version(77u32, 0u32, 0u32)),
					(Ios, Version(13u32, 4u32, 0u32)),
					(Edge, Version(91u32, 0u32, 0u32)),
					(Es, Version(2020u32, 0, 0)),
				])),
			),
			(
				ES2020BigInt,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(67u32, 0u32, 0u32)),
					(Safari, Version(14u32, 0u32, 0u32)),
					(OperaMobile, Version(48u32, 0u32, 0u32)),
					(Samsung, Version(9u32, 0u32, 0u32)),
					(Node, Version(10u32, 4u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 14u32)),
					(Firefox, Version(68u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(4u32, 0u32, 0u32)),
					(Opera, Version(54u32, 0u32, 0u32)),
					(Ios, Version(14u32, 0u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2020u32, 0, 0)),
				])),
			),
			(
				ES2021NumericSeparator,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(75u32, 0u32, 0u32)),
					(Safari, Version(13u32, 0u32, 0u32)),
					(OperaMobile, Version(54u32, 0u32, 0u32)),
					(Samsung, Version(11u32, 0u32, 0u32)),
					(Node, Version(12u32, 5u32, 0u32)),
					(Rhino, Version(1u32, 7u32, 14u32)),
					(Firefox, Version(70u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(6u32, 0u32, 0u32)),
					(Opera, Version(62u32, 0u32, 0u32)),
					(Ios, Version(13u32, 0u32, 0u32)),
					(Edge, Version(79u32, 0u32, 0u32)),
					(Es, Version(2021u32, 0, 0)),
				])),
			),
			(
				ES2021LogicalAssignmentOperators,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(85u32, 0u32, 0u32)),
					(Safari, Version(14u32, 0u32, 0u32)),
					(OperaMobile, Version(60u32, 0u32, 0u32)),
					(Samsung, Version(14u32, 0u32, 0u32)),
					(Node, Version(15u32, 0u32, 0u32)),
					(Firefox, Version(79u32, 0u32, 0u32)),
					(Deno, Version(1u32, 2u32, 0u32)),
					(Electron, Version(10u32, 0u32, 0u32)),
					(Opera, Version(71u32, 0u32, 0u32)),
					(Ios, Version(14u32, 0u32, 0u32)),
					(Edge, Version(85u32, 0u32, 0u32)),
					(Es, Version(2021u32, 0, 0)),
				])),
			),
			(
				ES2022ClassStaticBlock,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(94u32, 0u32, 0u32)),
					(Safari, Version(16u32, 4u32, 0u32)),
					(OperaMobile, Version(66u32, 0u32, 0u32)),
					(Samsung, Version(17u32, 0u32, 0u32)),
					(Node, Version(16u32, 11u32, 0u32)),
					(Firefox, Version(93u32, 0u32, 0u32)),
					(Deno, Version(1u32, 14u32, 0u32)),
					(Electron, Version(15u32, 0u32, 0u32)),
					(Opera, Version(80u32, 0u32, 0u32)),
					(Ios, Version(16u32, 4u32, 0u32)),
					(Edge, Version(94u32, 0u32, 0u32)),
					(Es, Version(2022u32, 0, 0)),
				])),
			),
			(
				ES2022PrivatePropertyInObject,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(91u32, 0u32, 0u32)),
					(Safari, Version(15u32, 0u32, 0u32)),
					(OperaMobile, Version(64u32, 0u32, 0u32)),
					(Samsung, Version(16u32, 0u32, 0u32)),
					(Node, Version(16u32, 9u32, 0u32)),
					(Firefox, Version(90u32, 0u32, 0u32)),
					(Deno, Version(1u32, 9u32, 0u32)),
					(Electron, Version(13u32, 0u32, 0u32)),
					(Opera, Version(77u32, 0u32, 0u32)),
					(Ios, Version(15u32, 0u32, 0u32)),
					(Edge, Version(91u32, 0u32, 0u32)),
					(Es, Version(2022u32, 0, 0)),
				])),
			),
			(
				ES2022ClassProperties,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(98u32, 0u32, 0u32)),
					(Safari, Version(16u32, 0u32, 0u32)),
					(OperaMobile, Version(53u32, 0u32, 0u32)),
					(Samsung, Version(11u32, 0u32, 0u32)),
					(Node, Version(12u32, 0u32, 0u32)),
					(Firefox, Version(90u32, 0u32, 0u32)),
					(Deno, Version(1u32, 18u32, 0u32)),
					(Electron, Version(17u32, 0u32, 0u32)),
					(Opera, Version(84u32, 0u32, 0u32)),
					(Ios, Version(16u32, 0u32, 0u32)),
					(Edge, Version(98u32, 0u32, 0u32)),
					(Es, Version(2022u32, 0, 0)),
				])),
			),
			(
				ES2022PrivateMethods,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(84u32, 0u32, 0u32)),
					(Safari, Version(15u32, 0u32, 0u32)),
					(OperaMobile, Version(60u32, 0u32, 0u32)),
					(Samsung, Version(14u32, 0u32, 0u32)),
					(Node, Version(14u32, 6u32, 0u32)),
					(Firefox, Version(90u32, 0u32, 0u32)),
					(Deno, Version(1u32, 0u32, 0u32)),
					(Electron, Version(10u32, 0u32, 0u32)),
					(Opera, Version(70u32, 0u32, 0u32)),
					(Ios, Version(15u32, 0u32, 0u32)),
					(Edge, Version(84u32, 0u32, 0u32)),
					(Es, Version(2022u32, 0, 0)),
				])),
			),
			(
				ES2022MatchIndicesRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(90u32, 0u32, 0u32)),
					(Safari, Version(15u32, 0u32, 0u32)),
					(OperaMobile, Version(64u32, 0u32, 0u32)),
					(Samsung, Version(15u32, 0u32, 0u32)),
					(Node, Version(16u32, 0u32, 0u32)),
					(Firefox, Version(91u32, 0u32, 0u32)),
					(Deno, Version(1u32, 8u32, 0u32)),
					(Electron, Version(13u32, 0u32, 0u32)),
					(Opera, Version(76u32, 0u32, 0u32)),
					(Ios, Version(15u32, 0u32, 0u32)),
					(Edge, Version(90u32, 0u32, 0u32)),
					(Es, Version(2022u32, 0, 0)),
				])),
			),
			(
				ES2024UnicodeSetsRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(112u32, 0u32, 0u32)),
					(Safari, Version(17u32, 0u32, 0u32)),
					(OperaMobile, Version(75u32, 0u32, 0u32)),
					(Node, Version(20u32, 0u32, 0u32)),
					(Firefox, Version(116u32, 0u32, 0u32)),
					(Deno, Version(1u32, 32u32, 0u32)),
					(Electron, Version(24u32, 0u32, 0u32)),
					(Opera, Version(98u32, 0u32, 0u32)),
					(Ios, Version(17u32, 0u32, 0u32)),
					(Edge, Version(112u32, 0u32, 0u32)),
					(Es, Version(2024u32, 0, 0)),
				])),
			),
			(
				ES2025DuplicateNamedCapturingGroupsRegex,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(126u32, 0u32, 0u32)),
					(Electron, Version(31u32, 0u32, 0u32)),
					(Node, Version(23u32, 0u32, 0u32)),
					(Firefox, Version(129u32, 0u32, 0u32)),
					(Safari, Version(17u32, 4u32, 0u32)),
					(Opera, Version(112u32, 0u32, 0u32)),
					(Ios, Version(17u32, 4u32, 0u32)),
					(Edge, Version(126u32, 0u32, 0u32)),
					(Es, Version(2025u32, 0, 0)),
				])),
			),
			(
				ES2025RegexpModifiers,
				EngineTargets::new(FxHashMap::from_iter([
					(Chrome, Version(125u32, 0u32, 0u32)),
					(Edge, Version(125u32, 0u32, 0u32)),
					(Electron, Version(31u32, 0u32, 0u32)),
					(Opera, Version(111u32, 0u32, 0u32)),
					(Node, Version(23u32, 0u32, 0u32)),
					(Firefox, Version(132u32, 0u32, 0u32)),
					(Es, Version(2025u32, 0, 0)),
				])),
			),
		])
	})
}
