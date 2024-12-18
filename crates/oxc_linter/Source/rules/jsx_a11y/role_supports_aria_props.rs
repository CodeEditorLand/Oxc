use cow_utils::CowUtils;
use oxc_ast::{
	AstKind,
	ast::{JSXAttributeItem, JSXOpeningElement},
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use phf::phf_set;

use crate::{
	AstNode,
	context::LintContext,
	globals::{VALID_ARIA_PROPS, VALID_ARIA_ROLES},
	rule::Rule,
	utils::{
		get_element_type,
		get_jsx_attribute_name,
		get_string_literal_prop_value,
		has_jsx_prop_ignore_case,
	},
};

declare_oxc_lint!(
	/// ### What it does
	///
	/// Enforce that elements with explicit or implicit roles defined contain only `aria-*` properties supported by that `role`. Many ARIA attributes (states and properties) can only be used on elements with particular roles. Some elements have implicit roles, such as `<a href="#" />`, which will resolve to `role="link"`.
	///
	/// ### Example
	///
	/// Examples of **incorrect** code for this rule:
	/// ```jsx
	/// <ul role="radiogroup" "aria-labelledby"="foo">
	///     <li aria-required tabIndex="-1" role="radio" aria-checked="false">Rainbow Trout</li>
	///     <li aria-required tabIndex="-1" role="radio" aria-checked="false">Brook Trout</li>
	///     <li aria-required tabIndex="0" role="radio" aria-checked="true">Lake Trout</li>
	/// </ul>
	/// ```
	///
	/// Examples of **correct** code for this rule:
	/// ```jsx
	/// <ul role="radiogroup" aria-required "aria-labelledby"="foo">
	///     <li tabIndex="-1" role="radio" aria-checked="false">Rainbow Trout</li>
	///     <li tabIndex="-1" role="radio" aria-checked="false">Brook Trout</li>
	///     <li tabIndex="0" role="radio" aria-checked="true">Lake Trout</li>
	/// </ul>
	/// ```
	///
	RoleSupportsAriaProps,
	correctness
);

#[derive(Debug, Default, Clone)]
pub struct RoleSupportsAriaProps;

fn default(span:Span, attr_name:&str, role:&str) -> OxcDiagnostic {
	OxcDiagnostic::warn(format!("The attribute {attr_name} is not supported by the role {role}."))
		.with_help(format!("Try to remove invalid attribute {attr_name}."))
		.with_label(span)
}

fn is_implicit_diagnostic(span:Span, attr_name:&str, role:&str, el_name:&str) -> OxcDiagnostic {
	OxcDiagnostic::warn(format!(
		"The attribute {attr_name} is not supported by the role {role}. This role is implicit on \
		 the element {el_name}."
	))
	.with_help(format!("Try to remove invalid attribute {attr_name}."))
	.with_label(span)
}

impl Rule for RoleSupportsAriaProps {
	fn run<'a>(&self, node:&AstNode<'a>, ctx:&LintContext<'a>) {
		let AstKind::JSXOpeningElement(jsx_el) = node.kind() else {
			return;
		};

		let el_type = get_element_type(ctx, jsx_el);

		let role = has_jsx_prop_ignore_case(jsx_el, "role");

		let role_value = role.map_or_else(
			|| get_implicit_role(jsx_el, &el_type),
			|i| get_string_literal_prop_value(i),
		);

		let is_implicit = role_value.is_some() && role.is_none();

		if let Some(role_value) = role_value {
			if !VALID_ARIA_ROLES.contains(role_value) {
				return;
			}

			let invalid_props = get_invalid_aria_props_for_role(role_value);

			for attr in &jsx_el.attributes {
				if let JSXAttributeItem::Attribute(attr) = attr {
					let name = get_jsx_attribute_name(&attr.name);

					let name = name.cow_to_lowercase();

					if invalid_props.contains(&&name.as_ref()) {
						ctx.diagnostic(if is_implicit {
							is_implicit_diagnostic(attr.span, &name, role_value, &el_type)
						} else {
							default(attr.span, &name, role_value)
						});
					}
				}
			}
		}
	}
}

/// ref: <https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.9.0/src/util/getImplicitRole.js>
fn get_implicit_role<'a>(
	node:&'a JSXOpeningElement<'a>,
	element_type:&str,
) -> Option<&'static str> {
	let implicit_role = match element_type {
		"a" | "area" | "link" => {
			match has_jsx_prop_ignore_case(node, "href") {
				Some(_) => "link",
				None => "",
			}
		},
		"article" => "article",
		"aside" => "complementary",
		"body" => "document",
		"button" => "button",
		"datalist" | "select" => "listbox",
		"details" => "group",
		"dialog" => "dialog",
		"form" => "form",
		"h1" | "h2" | "h3" | "h4" | "h5" | "h6" => "heading",
		"hr" => "separator",
		"img" => {
			has_jsx_prop_ignore_case(node, "alt").map_or("img", |i| {
				get_string_literal_prop_value(i)
					.map_or("img", |v| if v.is_empty() { "" } else { "img" })
			})
		},
		"input" => {
			has_jsx_prop_ignore_case(node, "type").map_or("textbox", |input_type| {
				match get_string_literal_prop_value(input_type) {
					Some("button" | "image" | "reset" | "submit") => "button",
					Some("checkbox") => "checkbox",
					Some("radio") => "radio",
					Some("range") => "slider",
					_ => "textbox",
				}
			})
		},
		"li" => "listitem",
		"menu" => {
			has_jsx_prop_ignore_case(node, "type").map_or("", |v| {
				get_string_literal_prop_value(v)
					.map_or("", |v| if v == "toolbar" { "toolbar" } else { "" })
			})
		},
		"menuitem" => {
			has_jsx_prop_ignore_case(node, "type").map_or("", |v| {
				match get_string_literal_prop_value(v) {
					Some("checkbox") => "menuitemcheckbox",
					Some("command") => "menuitem",
					Some("radio") => "menuitemradio",
					_ => "",
				}
			})
		},
		"meter" | "progress" => "progressbar",
		"nav" => "navigation",
		"ol" | "ul" => "list",
		"option" => "option",
		"output" => "status",
		"section" => "region",
		"tbody" | "tfoot" | "thead" => "rowgroup",
		"textarea" => "textbox",
		_ => "",
	};

	if VALID_ARIA_ROLES.contains(implicit_role) {
		Some(implicit_role)
	} else {
		None
	}
}

fn get_invalid_aria_props_for_role(role_value:&str) -> Vec<&&str> {
	// ref: https://github.com/A11yance/aria-query/blob/fff6f07c714e8048f4fe084cec74f24248e5673d/scripts/roles.json
	let valid_props_for_value:phf::Set<&'static str> = match role_value {
		"alert" | "banner" | "blockquote" | "command" | "complementary" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
			}
		},
		"alertdialog" | "dialog" | "window" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-modal",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"application" | "graphics-object" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"article" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-setsize"
			}
		},
		"button" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-pressed",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"caption" | "code" | "deletion" | "emphasis" | "generic" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"cell" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-colindex",
				"aria-colspan",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
				"aria-rowindex",
				"aria-rowspan"
			}
		},
		"checkbox" | "switch" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-checked",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription"
			}
		},
		"columnheader" | "rowheader" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-colindex",
				"aria-colspan",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
				"aria-rowindex",
				"aria-rowspan",
				"aria-selected",
				"aria-sort"
			}
		},
		"combobox" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-autocomplete",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
			}
		},
		"composite" | "group" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"contentinfo" | "definition" | "directory" | "document" | "feed" | "figure" | "form"
		| "img" | "landmark" | "list" | "log" | "main" | "marquee" | "math" | "navigation"
		| "note" | "region" | "roletype" | "rowgroup" | "search" | "section" | "sectionhead"
		| "status" | "structure" | "tabpanel" | "term" | "time" | "timer" | "tooltip"
		| "widget" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"doc-abstract"
		| "doc-acknowledgments"
		| "doc-afterword"
		| "doc-appendix"
		| "doc-backlink"
		| "doc-bibliography" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"doc-biblioentry" | "doc-endnote" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-level",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-setsize"
			}
		},
		"doc-biblioref" | "doc-chapter" | "doc-colophon" | "doc-conclusion" | "doc-cover"
		| "doc-credit" | "doc-credits" | "doc-dedication" | "doc-endnotes" | "doc-epigraph"
		| "doc-epilogue" | "doc-errata" | "doc-example" | "doc-footnote" | "doc-foreword"
		| "doc-glossary" | "doc-glossref" | "doc-index" | "doc-introduction" | "doc-noteref"
		| "doc-notice" | "doc-pagelist" | "doc-part" | "doc-preface" | "doc-prologue"
		| "doc-qna" | "doc-subtitle" | "doc-tip" | "doc-toc" | "graphics-document"
		| "graphics-symbol" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"doc-pagebreak" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-orientation",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"doc-pullquote" | "none" => phf_set! {},
		"grid" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-colcount",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-multiselectable",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-roledescription",
				"aria-rowcount"
			}
		},
		"gridcell" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-colindex",
				"aria-colspan",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
				"aria-rowindex",
				"aria-rowspan",
				"aria-selected"
			}
		},
		"heading" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-level",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
			}
		},
		"input" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"insertion" | "paragraph" | "presentation" | "strong" | "subscript" | "superscript" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"link" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"listbox" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-multiselectable",
				"aria-orientation",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
			}
		},
		"listitem" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-level",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-setsize"
			}
		},
		"mark" => {
			phf_set! {
				"aria-atomic",
				"aria-braillelabel",
				"aria-brailleroledescription",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-description",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription"
			}
		},
		"menu" | "menubar" | "select" | "toolbar" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-orientation",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
			}
		},
		"menuitem" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-setsize"
			}
		},
		"menuitemcheckbox" | "menuitemradio" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-checked",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
				"aria-setsize"
			}
		},
		"meter" | "progressbar" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
				"aria-valuemax",
				"aria-valuemin",
				"aria-valuenow",
				"aria-valuetext",
			}
		},
		"option" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-checked",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-setsize",
				"aria-selected",
			}
		},
		"radio" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-checked",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-setsize"
			}
		},
		"radiogroup" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-orientation",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription"
			}
		},
		"range" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
				"aria-valuemax",
				"aria-valuemin",
				"aria-valuenow"
			}
		},
		"row" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-colindex",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-level",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-rowindex",
				"aria-selected",
				"aria-setsize"
			}
		},
		"scrollbar" | "separator" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-orientation",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
				"aria-valuemax",
				"aria-valuemin",
				"aria-valuenow",
				"aria-valuetext",
			}
		},
		"searchbox" | "textbox" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-autocomplete",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-multiline",
				"aria-owns",
				"aria-placeholder",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription"
			}
		},
		"slider" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-orientation",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-roledescription",
				"aria-valuemax",
				"aria-valuemin",
				"aria-valuenow",
				"aria-valuetext",
			}
		},
		"spinbutton" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
				"aria-valuetext",
				"aria-valuemax",
				"aria-valuemin",
				"aria-valuenow",
			}
		},
		"tab" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-setsize",
				"aria-selected",
			}
		},
		"table" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-colcount",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
				"aria-rowcount"
			}
		},
		"tablist" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-level",
				"aria-live",
				"aria-multiselectable",
				"aria-orientation",
				"aria-owns",
				"aria-relevant",
				"aria-roledescription",
			}
		},
		"tree" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-multiselectable",
				"aria-orientation",
				"aria-owns",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
			}
		},
		"treegrid" => {
			phf_set! {
				"aria-activedescendant",
				"aria-atomic",
				"aria-busy",
				"aria-colcount",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-errormessage",
				"aria-flowto",
				"aria-grabbed",
				"aria-hidden",
				"aria-invalid",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-live",
				"aria-multiselectable",
				"aria-orientation",
				"aria-owns",
				"aria-readonly",
				"aria-relevant",
				"aria-required",
				"aria-roledescription",
				"aria-rowcount"
			}
		},
		"treeitem" => {
			phf_set! {
				"aria-atomic",
				"aria-busy",
				"aria-checked",
				"aria-controls",
				"aria-current",
				"aria-describedby",
				"aria-details",
				"aria-disabled",
				"aria-dropeffect",
				"aria-expanded",
				"aria-flowto",
				"aria-grabbed",
				"aria-haspopup",
				"aria-hidden",
				"aria-keyshortcuts",
				"aria-label",
				"aria-labelledby",
				"aria-level",
				"aria-live",
				"aria-owns",
				"aria-posinset",
				"aria-relevant",
				"aria-roledescription",
				"aria-selected",
				"aria-setsize"
			}
		},
		_ => unreachable!("role value is not valid"),
	};

	VALID_ARIA_PROPS
		.iter()
		.filter(|i| !valid_props_for_value.contains(i))
		.collect::<Vec<_>>()
}

#[test]
fn test() {
	use crate::tester::Tester;

	fn settings() -> serde_json::Value {
		serde_json::json!({
			"settings": { "jsx-a11y": {
				"components": {
					"Link": "a"
				}
			} }
		})
	}

	let pass = vec![
		(r"<Foo bar />", None, None, None),
		(r"<div />", None, None, None),
		(r#"<div id="main" />"#, None, None, None),
		(r"<div role />", None, None, None),
		(r#"<div role="presentation" {...props} />"#, None, None, None),
		(r"<Foo.Bar baz={true} />", None, None, None),
		(r#"<Link href="/" aria-checked />"#, None, None, None),
		(r#"<a href="/" aria-expanded />"#, None, None, None),
		(r#"<a href="/" aria-atomic />"#, None, None, None),
		(r#"<a href="/" aria-busy />"#, None, None, None),
		(r#"<a href="/" aria-controls />"#, None, None, None),
		(r#"<a href="/" aria-current />"#, None, None, None),
		(r#"<a href="/" aria-describedby />"#, None, None, None),
		(r#"<a href="/" aria-disabled />"#, None, None, None),
		(r#"<a href="/" aria-dropeffect />"#, None, None, None),
		(r#"<a href="/" aria-flowto />"#, None, None, None),
		(r#"<a href="/" aria-haspopup />"#, None, None, None),
		(r#"<a href="/" aria-grabbed />"#, None, None, None),
		(r#"<a href="/" aria-hidden />"#, None, None, None),
		(r#"<a href="/" aria-label />"#, None, None, None),
		(r#"<a href="/" aria-labelledby />"#, None, None, None),
		(r#"<a href="/" aria-live />"#, None, None, None),
		(r#"<a href="/" aria-owns />"#, None, None, None),
		(r#"<a href="/" aria-relevant />"#, None, None, None),
		(r"<a aria-checked />", None, None, None),
		(r#"<area href="/" aria-expanded />"#, None, None, None),
		(r#"<area href="/" aria-atomic />"#, None, None, None),
		(r#"<area href="/" aria-busy />"#, None, None, None),
		(r#"<area href="/" aria-controls />"#, None, None, None),
		(r#"<area href="/" aria-describedby />"#, None, None, None),
		(r#"<area href="/" aria-disabled />"#, None, None, None),
		(r#"<area href="/" aria-dropeffect />"#, None, None, None),
		(r#"<area href="/" aria-flowto />"#, None, None, None),
		(r#"<area href="/" aria-grabbed />"#, None, None, None),
		(r#"<area href="/" aria-haspopup />"#, None, None, None),
		(r#"<area href="/" aria-hidden />"#, None, None, None),
		(r#"<area href="/" aria-label />"#, None, None, None),
		(r#"<area href="/" aria-labelledby />"#, None, None, None),
		(r#"<area href="/" aria-live />"#, None, None, None),
		(r#"<area href="/" aria-owns />"#, None, None, None),
		(r#"<area href="/" aria-relevant />"#, None, None, None),
		(r"<area aria-checked />", None, None, None),
		(r#"<link href="/" aria-expanded />"#, None, None, None),
		(r#"<link href="/" aria-atomic />"#, None, None, None),
		(r#"<link href="/" aria-busy />"#, None, None, None),
		(r#"<link href="/" aria-controls />"#, None, None, None),
		(r#"<link href="/" aria-describedby />"#, None, None, None),
		(r#"<link href="/" aria-disabled />"#, None, None, None),
		(r#"<link href="/" aria-dropeffect />"#, None, None, None),
		(r#"<link href="/" aria-flowto />"#, None, None, None),
		(r#"<link href="/" aria-grabbed />"#, None, None, None),
		(r#"<link href="/" aria-haspopup />"#, None, None, None),
		(r#"<link href="/" aria-hidden />"#, None, None, None),
		(r#"<link href="/" aria-label />"#, None, None, None),
		(r#"<link href="/" aria-labelledby />"#, None, None, None),
		(r#"<link href="/" aria-live />"#, None, None, None),
		(r#"<link href="/" aria-owns />"#, None, None, None),
		(r#"<link href="/" aria-relevant />"#, None, None, None),
		(r"<link aria-checked />", None, None, None),
		(r#"<img alt="" aria-checked />"#, None, None, None),
		(r#"<img alt="foobar" aria-busy />"#, None, None, None),
		(r#"<menu type="toolbar" aria-activedescendant />"#, None, None, None),
		(r#"<menu type="toolbar" aria-atomic />"#, None, None, None),
		(r#"<menu type="toolbar" aria-busy />"#, None, None, None),
		(r#"<menu type="toolbar" aria-controls />"#, None, None, None),
		(r#"<menu type="toolbar" aria-describedby />"#, None, None, None),
		(r#"<menu type="toolbar" aria-disabled />"#, None, None, None),
		(r#"<menu type="toolbar" aria-dropeffect />"#, None, None, None),
		(r#"<menu type="toolbar" aria-flowto />"#, None, None, None),
		(r#"<menu type="toolbar" aria-grabbed />"#, None, None, None),
		(r#"<menu type="toolbar" aria-hidden />"#, None, None, None),
		(r#"<menu type="toolbar" aria-label />"#, None, None, None),
		(r#"<menu type="toolbar" aria-labelledby />"#, None, None, None),
		(r#"<menu type="toolbar" aria-live />"#, None, None, None),
		(r#"<menu type="toolbar" aria-owns />"#, None, None, None),
		(r#"<menu type="toolbar" aria-relevant />"#, None, None, None),
		(r"<menu aria-checked />", None, None, None),
		(r#"<menuitem type="command" aria-atomic />"#, None, None, None),
		(r#"<menuitem type="command" aria-busy />"#, None, None, None),
		(r#"<menuitem type="command" aria-controls />"#, None, None, None),
		(r#"<menuitem type="command" aria-describedby />"#, None, None, None),
		(r#"<menuitem type="command" aria-disabled />"#, None, None, None),
		(r#"<menuitem type="command" aria-dropeffect />"#, None, None, None),
		(r#"<menuitem type="command" aria-flowto />"#, None, None, None),
		(r#"<menuitem type="command" aria-grabbed />"#, None, None, None),
		(r#"<menuitem type="command" aria-hidden />"#, None, None, None),
		(r#"<menuitem type="command" aria-label />"#, None, None, None),
		(r#"<menuitem type="command" aria-labelledby />"#, None, None, None),
		(r#"<menuitem type="command" aria-live />"#, None, None, None),
		(r#"<menuitem type="command" aria-owns />"#, None, None, None),
		(r#"<menuitem type="command" aria-relevant />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-checked />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-atomic />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-busy />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-controls />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-describedby />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-disabled />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-dropeffect />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-flowto />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-grabbed />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-hidden />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-invalid />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-label />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-labelledby />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-live />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-owns />"#, None, None, None),
		(r#"<menuitem type="checkbox" aria-relevant />"#, None, None, None),
		(r#"<menuitem type="radio" aria-checked />"#, None, None, None),
		(r#"<menuitem type="radio" aria-atomic />"#, None, None, None),
		(r#"<menuitem type="radio" aria-busy />"#, None, None, None),
		(r#"<menuitem type="radio" aria-controls />"#, None, None, None),
		(r#"<menuitem type="radio" aria-describedby />"#, None, None, None),
		(r#"<menuitem type="radio" aria-disabled />"#, None, None, None),
		(r#"<menuitem type="radio" aria-dropeffect />"#, None, None, None),
		(r#"<menuitem type="radio" aria-flowto />"#, None, None, None),
		(r#"<menuitem type="radio" aria-grabbed />"#, None, None, None),
		(r#"<menuitem type="radio" aria-hidden />"#, None, None, None),
		(r#"<menuitem type="radio" aria-invalid />"#, None, None, None),
		(r#"<menuitem type="radio" aria-label />"#, None, None, None),
		(r#"<menuitem type="radio" aria-labelledby />"#, None, None, None),
		(r#"<menuitem type="radio" aria-live />"#, None, None, None),
		(r#"<menuitem type="radio" aria-owns />"#, None, None, None),
		(r#"<menuitem type="radio" aria-relevant />"#, None, None, None),
		(r#"<menuitem type="radio" aria-posinset />"#, None, None, None),
		(r#"<menuitem type="radio" aria-setsize />"#, None, None, None),
		(r"<menuitem aria-checked />", None, None, None),
		(r#"<menuitem type="foobar" aria-checked />"#, None, None, None),
		(r#"<input type="button" aria-expanded />"#, None, None, None),
		(r#"<input type="button" aria-pressed />"#, None, None, None),
		(r#"<input type="button" aria-atomic />"#, None, None, None),
		(r#"<input type="button" aria-busy />"#, None, None, None),
		(r#"<input type="button" aria-controls />"#, None, None, None),
		(r#"<input type="button" aria-describedby />"#, None, None, None),
		(r#"<input type="button" aria-disabled />"#, None, None, None),
		(r#"<input type="button" aria-dropeffect />"#, None, None, None),
		(r#"<input type="button" aria-flowto />"#, None, None, None),
		(r#"<input type="button" aria-grabbed />"#, None, None, None),
		(r#"<input type="button" aria-hidden />"#, None, None, None),
		(r#"<input type="button" aria-label />"#, None, None, None),
		(r#"<input type="button" aria-labelledby />"#, None, None, None),
		(r#"<input type="button" aria-live />"#, None, None, None),
		(r#"<input type="button" aria-owns />"#, None, None, None),
		(r#"<input type="button" aria-relevant />"#, None, None, None),
		(r#"<input type="image" aria-expanded />"#, None, None, None),
		(r#"<input type="image" aria-pressed />"#, None, None, None),
		(r#"<input type="image" aria-atomic />"#, None, None, None),
		(r#"<input type="image" aria-busy />"#, None, None, None),
		(r#"<input type="image" aria-controls />"#, None, None, None),
		(r#"<input type="image" aria-describedby />"#, None, None, None),
		(r#"<input type="image" aria-disabled />"#, None, None, None),
		(r#"<input type="image" aria-dropeffect />"#, None, None, None),
		(r#"<input type="image" aria-flowto />"#, None, None, None),
		(r#"<input type="image" aria-grabbed />"#, None, None, None),
		(r#"<input type="image" aria-haspopup />"#, None, None, None),
		(r#"<input type="image" aria-hidden />"#, None, None, None),
		(r#"<input type="image" aria-label />"#, None, None, None),
		(r#"<input type="image" aria-labelledby />"#, None, None, None),
		(r#"<input type="image" aria-live />"#, None, None, None),
		(r#"<input type="image" aria-owns />"#, None, None, None),
		(r#"<input type="image" aria-relevant />"#, None, None, None),
		(r#"<input type="reset" aria-expanded />"#, None, None, None),
		(r#"<input type="reset" aria-pressed />"#, None, None, None),
		(r#"<input type="reset" aria-atomic />"#, None, None, None),
		(r#"<input type="reset" aria-busy />"#, None, None, None),
		(r#"<input type="reset" aria-controls />"#, None, None, None),
		(r#"<input type="reset" aria-describedby />"#, None, None, None),
		(r#"<input type="reset" aria-disabled />"#, None, None, None),
		(r#"<input type="reset" aria-dropeffect />"#, None, None, None),
		(r#"<input type="reset" aria-flowto />"#, None, None, None),
		(r#"<input type="reset" aria-grabbed />"#, None, None, None),
		(r#"<input type="reset" aria-haspopup />"#, None, None, None),
		(r#"<input type="reset" aria-hidden />"#, None, None, None),
		(r#"<input type="reset" aria-label />"#, None, None, None),
		(r#"<input type="reset" aria-labelledby />"#, None, None, None),
		(r#"<input type="reset" aria-live />"#, None, None, None),
		(r#"<input type="reset" aria-owns />"#, None, None, None),
		(r#"<input type="reset" aria-relevant />"#, None, None, None),
		(r#"<input type="submit" aria-expanded />"#, None, None, None),
		(r#"<input type="submit" aria-pressed />"#, None, None, None),
		(r#"<input type="submit" aria-atomic />"#, None, None, None),
		(r#"<input type="submit" aria-busy />"#, None, None, None),
		(r#"<input type="submit" aria-controls />"#, None, None, None),
		(r#"<input type="submit" aria-describedby />"#, None, None, None),
		(r#"<input type="submit" aria-disabled />"#, None, None, None),
		(r#"<input type="submit" aria-dropeffect />"#, None, None, None),
		(r#"<input type="submit" aria-flowto />"#, None, None, None),
		(r#"<input type="submit" aria-grabbed />"#, None, None, None),
		(r#"<input type="submit" aria-haspopup />"#, None, None, None),
		(r#"<input type="submit" aria-hidden />"#, None, None, None),
		(r#"<input type="submit" aria-label />"#, None, None, None),
		(r#"<input type="submit" aria-labelledby />"#, None, None, None),
		(r#"<input type="submit" aria-live />"#, None, None, None),
		(r#"<input type="submit" aria-owns />"#, None, None, None),
		(r#"<input type="submit" aria-relevant />"#, None, None, None),
		(r#"<input type="checkbox" aria-atomic />"#, None, None, None),
		(r#"<input type="checkbox" aria-busy />"#, None, None, None),
		(r#"<input type="checkbox" aria-checked />"#, None, None, None),
		(r#"<input type="checkbox" aria-controls />"#, None, None, None),
		(r#"<input type="checkbox" aria-describedby />"#, None, None, None),
		(r#"<input type="checkbox" aria-disabled />"#, None, None, None),
		(r#"<input type="checkbox" aria-dropeffect />"#, None, None, None),
		(r#"<input type="checkbox" aria-flowto />"#, None, None, None),
		(r#"<input type="checkbox" aria-grabbed />"#, None, None, None),
		(r#"<input type="checkbox" aria-hidden />"#, None, None, None),
		(r#"<input type="checkbox" aria-invalid />"#, None, None, None),
		(r#"<input type="checkbox" aria-label />"#, None, None, None),
		(r#"<input type="checkbox" aria-labelledby />"#, None, None, None),
		(r#"<input type="checkbox" aria-live />"#, None, None, None),
		(r#"<input type="checkbox" aria-owns />"#, None, None, None),
		(r#"<input type="checkbox" aria-relevant />"#, None, None, None),
		(r#"<input type="radio" aria-atomic />"#, None, None, None),
		(r#"<input type="radio" aria-busy />"#, None, None, None),
		(r#"<input type="radio" aria-checked />"#, None, None, None),
		(r#"<input type="radio" aria-controls />"#, None, None, None),
		(r#"<input type="radio" aria-describedby />"#, None, None, None),
		(r#"<input type="radio" aria-disabled />"#, None, None, None),
		(r#"<input type="radio" aria-dropeffect />"#, None, None, None),
		(r#"<input type="radio" aria-flowto />"#, None, None, None),
		(r#"<input type="radio" aria-grabbed />"#, None, None, None),
		(r#"<input type="radio" aria-hidden />"#, None, None, None),
		(r#"<input type="radio" aria-label />"#, None, None, None),
		(r#"<input type="radio" aria-labelledby />"#, None, None, None),
		(r#"<input type="radio" aria-live />"#, None, None, None),
		(r#"<input type="radio" aria-owns />"#, None, None, None),
		(r#"<input type="radio" aria-relevant />"#, None, None, None),
		(r#"<input type="radio" aria-posinset />"#, None, None, None),
		(r#"<input type="radio" aria-setsize />"#, None, None, None),
		(r#"<input type="range" aria-valuemax />"#, None, None, None),
		(r#"<input type="range" aria-valuemin />"#, None, None, None),
		(r#"<input type="range" aria-valuenow />"#, None, None, None),
		(r#"<input type="range" aria-orientation />"#, None, None, None),
		(r#"<input type="range" aria-atomic />"#, None, None, None),
		(r#"<input type="range" aria-busy />"#, None, None, None),
		(r#"<input type="range" aria-controls />"#, None, None, None),
		(r#"<input type="range" aria-describedby />"#, None, None, None),
		(r#"<input type="range" aria-disabled />"#, None, None, None),
		(r#"<input type="range" aria-dropeffect />"#, None, None, None),
		(r#"<input type="range" aria-flowto />"#, None, None, None),
		(r#"<input type="range" aria-grabbed />"#, None, None, None),
		(r#"<input type="range" aria-haspopup />"#, None, None, None),
		(r#"<input type="range" aria-hidden />"#, None, None, None),
		(r#"<input type="range" aria-invalid />"#, None, None, None),
		(r#"<input type="range" aria-label />"#, None, None, None),
		(r#"<input type="range" aria-labelledby />"#, None, None, None),
		(r#"<input type="range" aria-live />"#, None, None, None),
		(r#"<input type="range" aria-owns />"#, None, None, None),
		(r#"<input type="range" aria-relevant />"#, None, None, None),
		(r#"<input type="email" aria-disabled />"#, None, None, None),
		(r#"<input type="password" aria-disabled />"#, None, None, None),
		(r#"<input type="search" aria-disabled />"#, None, None, None),
		(r#"<input type="tel" aria-disabled />"#, None, None, None),
		(r#"<input type="url" aria-disabled />"#, None, None, None),
		(r"<input aria-disabled />", None, None, None),
		// TODO: This should pass, but it doesn't. Because the current code does not determine if
		// the attribute value is null, undefined, etc.
		//(r#"<h2 role="presentation" aria-level={null} />"#, None, None, None),
		//(r#"<h2 role="presentation" aria-level={undefined} />"#, None, None, None),
		(r"<button aria-pressed />", None, None, None),
		(r"<form aria-hidden />", None, None, None),
		(r"<h1 aria-hidden />", None, None, None),
		(r"<h2 aria-hidden />", None, None, None),
		(r"<h3 aria-hidden />", None, None, None),
		(r"<h4 aria-hidden />", None, None, None),
		(r"<h5 aria-hidden />", None, None, None),
		(r"<h6 aria-hidden />", None, None, None),
		(r"<hr aria-hidden />", None, None, None),
		(r"<li aria-current />", None, None, None),
		(r"<meter aria-atomic />", None, None, None),
		(r"<option aria-atomic />", None, None, None),
		(r"<progress aria-atomic />", None, None, None),
		(r"<textarea aria-hidden />", None, None, None),
		(r"<select aria-expanded />", None, None, None),
		(r"<datalist aria-expanded />", None, None, None),
		(r#"<div role="heading" aria-level />"#, None, None, None),
		(r#"<div role="heading" aria-level="1" />"#, None, None, None),
	];

	let fail = vec![
		(r#"<a href="/" aria-checked />"#, None, None, None),
		(r#"<area href="/" aria-checked />"#, None, None, None),
		(r#"<link href="/" aria-checked />"#, None, None, None),
		(r#"<img alt="foobar" aria-checked />"#, None, None, None),
		(r#"<menu type="toolbar" aria-checked />"#, None, None, None),
		(r"<aside aria-checked />", None, None, None),
		(r"<ul aria-expanded />", None, None, None),
		(r"<details aria-expanded />", None, None, None),
		(r"<dialog aria-expanded />", None, None, None),
		(r"<aside aria-expanded />", None, None, None),
		(r"<article aria-expanded />", None, None, None),
		(r"<body aria-expanded />", None, None, None),
		(r"<li aria-expanded />", None, None, None),
		(r"<nav aria-expanded />", None, None, None),
		(r"<ol aria-expanded />", None, None, None),
		(r"<output aria-expanded />", None, None, None),
		(r"<section aria-expanded />", None, None, None),
		(r"<tbody aria-expanded />", None, None, None),
		(r"<tfoot aria-expanded />", None, None, None),
		(r"<thead aria-expanded />", None, None, None),
		(r#"<input type="radio" aria-invalid />"#, None, None, None),
		(r#"<input type="radio" aria-selected />"#, None, None, None),
		(r#"<input type="radio" aria-haspopup />"#, None, None, None),
		(r#"<input type="checkbox" aria-haspopup />"#, None, None, None),
		(r#"<input type="reset" aria-invalid />"#, None, None, None),
		(r#"<input type="image" aria-invalid />"#, None, None, None),
		(r#"<input type="button" aria-invalid />"#, None, None, None),
		(r#"<menuitem type="command" aria-invalid />"#, None, None, None),
		(r#"<menuitem type="radio" aria-selected />"#, None, None, None),
		(r#"<menu type="toolbar" aria-haspopup />"#, None, None, None),
		(r#"<menu type="toolbar" aria-invalid />"#, None, None, None),
		(r#"<menu type="toolbar" aria-expanded />"#, None, None, None),
		(r#"<link href="/" aria-invalid />"#, None, None, None),
		(r#"<area href="/" aria-invalid />"#, None, None, None),
		(r#"<a href="/" aria-invalid />"#, None, None, None),
		(r#"<Link href="/" aria-checked />"#, None, Some(settings()), None),
	];

	Tester::new(RoleSupportsAriaProps::NAME, RoleSupportsAriaProps::CATEGORY, pass, fail)
		.test_and_snapshot();
}
