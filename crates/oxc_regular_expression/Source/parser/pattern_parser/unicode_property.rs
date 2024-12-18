use phf::{Set, phf_set};

// https://tc39.es/ecma262/2024/multipage/text-processing.html#table-nonbinary-unicode-properties
pub fn is_valid_unicode_property(name:&str, value:&str) -> bool {
	if matches!(name, "General_Category" | "gc") {
		return GC_PROPERTY_VALUES.contains(value);
	}

	if matches!(name, "Script" | "sc") {
		return SC_PROPERTY_VALUES.contains(value);
	}

	if matches!(name, "Script_Extensions" | "scx") {
		return SC_PROPERTY_VALUES.contains(value) || SCX_PROPERTY_VALUES.contains(value);
	}

	false
}

pub fn is_valid_lone_unicode_property(name_or_value:&str) -> bool {
	BINARY_UNICODE_PROPERTIES.contains(name_or_value)
}
/// This should be used with `UnicodeSetsMode`
pub fn is_valid_lone_unicode_property_of_strings(name_or_value:&str) -> bool {
	BINARY_UNICODE_PROPERTIES_OF_STRINGS.contains(name_or_value)
}

// spellchecker:off
// # General_Category (gc)
// https://unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt
static GC_PROPERTY_VALUES:Set<&'static str> = phf_set! {
	"C", "Other",
	"Cc", "Control", "cntrl",
	"Cf", "Format",
	"Cn", "Unassigned",
	"Co", "Private_Use",
	"Cs", "Surrogate",
	"L", "Letter",
	"LC", "Cased_Letter",
	"Ll", "Lowercase_Letter",
	"Lm", "Modifier_Letter",
	"Lo", "Other_Letter",
	"Lt", "Titlecase_Letter",
	"Lu", "Uppercase_Letter",
	"M", "Mark", "Combining_Mark",
	"Mc", "Spacing_Mark",
	"Me", "Enclosing_Mark",
	"Mn", "Nonspacing_Mark",
	"N", "Number",
	"Nd", "Decimal_Number", "digit",
	"Nl", "Letter_Number",
	"No", "Other_Number",
	"P", "Punctuation", "punct",
	"Pc", "Connector_Punctuation",
	"Pd", "Dash_Punctuation",
	"Pe", "Close_Punctuation",
	"Pf", "Final_Punctuation",
	"Pi", "Initial_Punctuation",
	"Po", "Other_Punctuation",
	"Ps", "Open_Punctuation",
	"S", "Symbol",
	"Sc", "Currency_Symbol",
	"Sk", "Modifier_Symbol",
	"Sm", "Math_Symbol",
	"So", "Other_Symbol",
	"Z", "Separator",
	"Zl", "Line_Separator",
	"Zp", "Paragraph_Separator",
	"Zs", "Space_Separator"
};

static SC_PROPERTY_VALUES:Set<&'static str> = phf_set! {
	"Adlm", "Adlam",
	"Aghb", "Caucasian_Albanian",
	"Ahom",
	"Arab", "Arabic",
	"Armi", "Imperial_Aramaic",
	"Armn", "Armenian",
	"Avst", "Avestan",
	"Bali", "Balinese",
	"Bamu", "Bamum",
	"Bass", "Bassa_Vah",
	"Batk", "Batak",
	"Beng", "Bengali",
	"Bhks", "Bhaiksuki",
	"Bopo", "Bopomofo",
	"Brah", "Brahmi",
	"Brai", "Braille",
	"Bugi", "Buginese",
	"Buhd", "Buhid",
	"Cakm", "Chakma",
	"Cans", "Canadian_Aboriginal",
	"Cari", "Carian",
	"Cham",
	"Cher", "Cherokee",
	"Chrs", "Chorasmian",
	"Copt", "Coptic", "Qaac",
	"Cpmn", "Cypro_Minoan",
	"Cprt", "Cypriot",
	"Cyrl", "Cyrillic",
	"Deva", "Devanagari",
	"Diak", "Dives_Akuru",
	"Dogr", "Dogra",
	"Dsrt", "Deseret",
	"Dupl", "Duployan",
	"Egyp", "Egyptian_Hieroglyphs",
	"Elba", "Elbasan",
	"Elym", "Elymaic",
	"Ethi", "Ethiopic",
	"Gara", "Garay",
	"Geor", "Georgian",
	"Glag", "Glagolitic",
	"Gong", "Gunjala_Gondi",
	"Gonm", "Masaram_Gondi",
	"Goth", "Gothic",
	"Gran", "Grantha",
	"Grek", "Greek",
	"Gujr", "Gujarati",
	"Gukh", "Gurung_Khema",
	"Guru", "Gurmukhi",
	"Hang", "Hangul",
	"Hani", "Han",
	"Hano", "Hanunoo",
	"Hatr", "Hatran",
	"Hebr", "Hebrew",
	"Hira", "Hiragana",
	"Hluw", "Anatolian_Hieroglyphs",
	"Hmng", "Pahawh_Hmong",
	"Hmnp", "Nyiakeng_Puachue_Hmong",
	"Hrkt", "Katakana_Or_Hiragana",
	"Hung", "Old_Hungarian",
	"Ital", "Old_Italic",
	"Java", "Javanese",
	"Kali", "Kayah_Li",
	"Kana", "Katakana",
	"Kawi",
	"Khar", "Kharoshthi",
	"Khmr", "Khmer",
	"Khoj", "Khojki",
	"Kits", "Khitan_Small_Script",
	"Knda", "Kannada",
	"Krai", "Kirat_Rai",
	"Kthi", "Kaithi",
	"Lana", "Tai_Tham",
	"Laoo", "Lao",
	"Latn", "Latin",
	"Lepc", "Lepcha",
	"Limb", "Limbu",
	"Lina", "Linear_A",
	"Linb", "Linear_B",
	"Lisu",
	"Lyci", "Lycian",
	"Lydi", "Lydian",
	"Mahj", "Mahajani",
	"Maka", "Makasar",
	"Mand", "Mandaic",
	"Mani", "Manichaean",
	"Marc", "Marchen",
	"Medf", "Medefaidrin",
	"Mend", "Mende_Kikakui",
	"Merc", "Meroitic_Cursive",
	"Mero", "Meroitic_Hieroglyphs",
	"Mlym", "Malayalam",
	"Modi",
	"Mong", "Mongolian",
	"Mroo", "Mro",
	"Mtei", "Meetei_Mayek",
	"Mult", "Multani",
	"Mymr", "Myanmar",
	"Nagm", "Nag_Mundari",
	"Nand", "Nandinagari",
	"Narb", "Old_North_Arabian",
	"Nbat", "Nabataean",
	"Newa",
	"Nkoo", "Nko",
	"Nshu", "Nushu",
	"Ogam", "Ogham",
	"Olck", "Ol_Chiki",
	"Onao", "Ol_Onal",
	"Orkh", "Old_Turkic",
	"Orya", "Oriya",
	"Osge", "Osage",
	"Osma", "Osmanya",
	"Ougr", "Old_Uyghur",
	"Palm", "Palmyrene",
	"Pauc", "Pau_Cin_Hau",
	"Perm", "Old_Permic",
	"Phag", "Phags_Pa",
	"Phli", "Inscriptional_Pahlavi",
	"Phlp", "Psalter_Pahlavi",
	"Phnx", "Phoenician",
	"Plrd", "Miao",
	"Prti", "Inscriptional_Parthian",
	"Rjng", "Rejang",
	"Rohg", "Hanifi_Rohingya",
	"Runr", "Runic",
	"Samr", "Samaritan",
	"Sarb", "Old_South_Arabian",
	"Saur", "Saurashtra",
	"Sgnw", "SignWriting",
	"Shaw", "Shavian",
	"Shrd", "Sharada",
	"Sidd", "Siddham",
	"Sind", "Khudawadi",
	"Sinh", "Sinhala",
	"Sogd", "Sogdian",
	"Sogo", "Old_Sogdian",
	"Sora", "Sora_Sompeng",
	"Soyo", "Soyombo",
	"Sund", "Sundanese",
	"Sunu", "Sunuwar",
	"Sylo", "Syloti_Nagri",
	"Syrc", "Syriac",
	"Tagb", "Tagbanwa",
	"Takr", "Takri",
	"Tale", "Tai_Le",
	"Talu", "New_Tai_Lue",
	"Taml", "Tamil",
	"Tang", "Tangut",
	"Tavt", "Tai_Viet",
	"Telu", "Telugu",
	"Tfng", "Tifinagh",
	"Tglg", "Tagalog",
	"Thaa", "Thaana",
	"Thai",
	"Tibt", "Tibetan",
	"Tirh", "Tirhuta",
	"Tnsa", "Tangsa",
	"Todr", "Todhri",
	"Toto",
	"Tutg", "Tulu_Tigalari",
	"Ugar", "Ugaritic",
	"Vaii", "Vai",
	"Vith", "Vithkuqi",
	"Wara", "Warang_Citi",
	"Wcho", "Wancho",
	"Xpeo", "Old_Persian",
	"Xsux", "Cuneiform",
	"Yezi", "Yezidi",
	"Yiii", "Yi",
	"Zanb", "Zanabazar_Square",
	"Zinh", "Inherited", "Qaai",
	"Zyyy", "Common",
	"Zzzz", "Unknown",
};

static SCX_PROPERTY_VALUES:Set<&'static str> = phf_set! {
	// Empty
};

// Table 66: Binary Unicode property aliases
// https://tc39.es/ecma262/2024/multipage/text-processing.html#table-binary-unicode-properties
static BINARY_UNICODE_PROPERTIES:Set<&'static str> = phf_set! {
	"ASCII",
	"ASCII_Hex_Digit",
	"AHex",
	"Alphabetic",
	"Alpha",
	"Any",
	"Assigned",
	"Bidi_Control",
	"Bidi_C",
	"Bidi_Mirrored",
	"Bidi_M",
	"Case_Ignorable",
	"CI",
	"Cased",
	"Changes_When_Casefolded",
	"CWCF",
	"Changes_When_Casemapped",
	"CWCM",
	"Changes_When_Lowercased",
	"CWL",
	"Changes_When_NFKC_Casefolded",
	"CWKCF",
	"Changes_When_Titlecased",
	"CWT",
	"Changes_When_Uppercased",
	"CWU",
	"Dash",
	"Default_Ignorable_Code_Point",
	"DI",
	"Deprecated",
	"Dep",
	"Diacritic",
	"Dia",
	"Emoji",
	"Emoji_Component",
	"EComp",
	"Emoji_Modifier",
	"EMod",
	"Emoji_Modifier_Base",
	"EBase",
	"Emoji_Presentation",
	"EPres",
	"Extended_Pictographic",
	"ExtPict",
	"Extender",
	"Ext",
	"Grapheme_Base",
	"Gr_Base",
	"Grapheme_Extend",
	"Gr_Ext",
	"Hex_Digit",
	"Hex",
	"IDS_Binary_Operator",
	"IDSB",
	"IDS_Trinary_Operator",
	"IDST",
	"ID_Continue",
	"IDC",
	"ID_Start",
	"IDS",
	"Ideographic",
	"Ideo",
	"Join_Control",
	"Join_C",
	"Logical_Order_Exception",
	"LOE",
	"Lowercase",
	"Lower",
	"Math",
	"Noncharacter_Code_Point",
	"NChar",
	"Pattern_Syntax",
	"Pat_Syn",
	"Pattern_White_Space",
	"Pat_WS",
	"Quotation_Mark",
	"QMark",
	"Radical",
	"Regional_Indicator",
	"RI",
	"Sentence_Terminal",
	"STerm",
	"Soft_Dotted",
	"SD",
	"Terminal_Punctuation",
	"Term",
	"Unified_Ideograph",
	"UIdeo",
	"Uppercase",
	"Upper",
	"Variation_Selector",
	"VS",
	"White_Space",
	"space",
	"XID_Continue",
	"XIDC",
	"XID_Start",
	"XIDS",
};

// Table 67: Binary Unicode properties of strings
// https://tc39.es/ecma262/2024/multipage/text-processing.html#table-binary-unicode-properties-of-strings
static BINARY_UNICODE_PROPERTIES_OF_STRINGS:Set<&'static str> = phf_set! {
	"Basic_Emoji",
	"Emoji_Keycap_Sequence",
	"RGI_Emoji_Modifier_Sequence",
	"RGI_Emoji_Flag_Sequence",
	"RGI_Emoji_Tag_Sequence",
	"RGI_Emoji_ZWJ_Sequence",
	"RGI_Emoji",
};
// spellchecker:on
