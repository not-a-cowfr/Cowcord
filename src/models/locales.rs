#![allow(non_camel_case_types)]

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Locales {
	ar,
	bg,
	cs,
	da,
	de,
	el,
	#[serde(rename = "en-GB")]
	en_GB,
	#[serde(rename = "en-US")]
	en_US,
	#[serde(rename = "es-ES")]
	es_ES,
	#[serde(rename = "es-41")]
	es_41,
	fi,
	fr,
	hi,
	hr,
	hu,
	id,
	it,
	ja,
	ko,
	lt,
	nl,
	no,
	pl,
	#[serde(rename = "pt-BR")]
	pt_BR,
	ro,
	ru,
	#[serde(rename = "sv-SE")]
	sv_SE,
	th,
	tr,
	uk,
	vi,
	#[serde(rename = "zh-CN")]
	zh_CN,
	#[serde(rename = "zh-TW")]
	zh_TW,
}

impl fmt::Display for Locales {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		let s = match self {
			| Locales::ar => "Arabic",
			| Locales::bg => "Bulgarian",
			| Locales::cs => "Czech",
			| Locales::da => "Danish",
			| Locales::de => "German",
			| Locales::el => "Greek",
			| Locales::en_GB => "English, UK",
			| Locales::en_US => "English, US",
			| Locales::es_ES => "Spanish",
			| Locales::es_41 => "Spanish, LATAM",
			| Locales::fi => "Finnish",
			| Locales::fr => "French",
			| Locales::hi => "Hindi",
			| Locales::hr => "Croatian",
			| Locales::hu => "Hungarian",
			| Locales::id => "Indonesian",
			| Locales::it => "Italian",
			| Locales::ja => "Japanese",
			| Locales::ko => "Korean",
			| Locales::lt => "Lithuanian",
			| Locales::nl => "Dutch",
			| Locales::no => "Norwegian",
			| Locales::pl => "Polish",
			| Locales::pt_BR => "Portuguese, Brazilian",
			| Locales::ro => "Romanian",
			| Locales::ru => "Russian",
			| Locales::sv_SE => "Swedish",
			| Locales::th => "Thai",
			| Locales::tr => "Turkish",
			| Locales::uk => "Ukrainian",
			| Locales::vi => "Vietnamese",
			| Locales::zh_CN => "Chinese, China",
			| Locales::zh_TW => "Chinese, Taiwan",
		};
		write!(f, "{}", s)
	}
}
