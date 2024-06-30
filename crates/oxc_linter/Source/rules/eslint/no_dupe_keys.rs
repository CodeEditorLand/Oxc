use oxc_ast::{
    ast::{ObjectPropertyKind, PropertyKind},
    AstKind,
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::{GetSpan, Span};
use rustc_hash::FxHashMap;

use crate::{context::LintContext, rule::Rule, AstNode};

fn no_dupe_keys_diagnostic(span0: Span, span1: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("eslint(no-dupe-keys): Disallow duplicate keys in object literals")
        .with_help("Consider removing the duplicated key")
        .with_labels([span0, span1])
}

#[derive(Debug, Default, Clone)]
pub struct NoDupeKeys;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow duplicate keys in object literals
    ///
    /// ### Why is this bad?
    ///
    /// Multiple properties with the same key in object literals can cause unexpected behavior in your application.
    ///
    /// ### Example
    /// ```javascript
    /// var foo = {
    ///     bar: "baz",
    ///     bar: "qux"
    /// }
    /// ```
    NoDupeKeys,
    correctness
);

impl Rule for NoDupeKeys {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::ObjectExpression(obj_expr) = node.kind() else {
            return;
        };
        let mut map = FxHashMap::default();
        for prop in &obj_expr.properties {
            let ObjectPropertyKind::ObjectProperty(prop) = prop else {
                continue;
            };
            let Some(name) = prop.key.static_name() else {
                return;
            };
            if let Some((prev_kind, prev_span)) = map.insert(name, (prop.kind, prop.key.span())) {
                if prev_kind == PropertyKind::Init
                    || prop.kind == PropertyKind::Init
                    || prev_kind == prop.kind
                {
                    ctx.diagnostic(no_dupe_keys_diagnostic(prev_span, prop.key.span()));
                }
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        ("var foo = { __proto__: 1, two: 2};", None),
        ("var x = { foo: 1, bar: 2 };", None),
        ("var x = { '': 1, bar: 2 };", None),
        ("var x = { '': 1, ' ': 2 };", None),
        ("var x = { '': 1, [null]: 2 };", None),
        ("var x = { '': 1, [a]: 2 };", None),
        ("var x = { [a]: 1, [a]: 2 };", None),
        ("+{ get a() { }, set a(b) { } };", None),
        ("var x = { a: b, [a]: b };", None),
        ("var x = { a: b, ...c }", None),
        ("var x = { get a() {}, set a (value) {} };", None),
        ("var x = { a: 1, b: { a: 2 } };", None),
        ("var x = ({ null: 1, [/(?<zero>0)/]: 2 })", None),
        ("var {a, a} = obj", None),
        // Syntax:error: the '0' prefixed octal literals is not allowed.
        // ("var x = { 012: 1, 12: 2 };", None),
        ("var x = { 1_0: 1, 1: 2 };", None),
        // NOTE: This should fail when we get read the big int value
        ("var x = { 1n: 1, 1: 2 };", None),
    ];

    let fail = vec![
        ("var x = { a: b, ['a']: b };", None),
        ("var x = { y: 1, y: 2 };", None),
        ("var x = { '': 1, '': 2 };", None),
        ("var x = { '': 1, [``]: 2 };", None),
        ("var foo = { 0x1: 1, 1: 2};", None),
        ("var x = { 012: 1, 10: 2 };", None),
        ("var x = { 0b1: 1, 1: 2 };", None),
        ("var x = { 0o1: 1, 1: 2 };", None),
        ("var x = { 1_0: 1, 10: 2 };", None),
        ("var x = { \"z\": 1, z: 2 };", None),
        ("var foo = {\n  bar: 1,\n  bar: 1,\n}", None),
        ("var x = { a: 1, get a() {} };", None),
        ("var x = { a: 1, set a(value) {} };", None),
        ("var x = { a: 1, b: { a: 2 }, get b() {} };", None),
        ("var x = ({ '/(?<zero>0)/': 1, [/(?<zero>0)/]: 2 })", None),
    ];

    Tester::new(NoDupeKeys::NAME, pass, fail).test_and_snapshot();
}
