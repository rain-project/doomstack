use crate::doom::{
    messages::{errors::*, helps::*},
    Attribute, Description, KeepOriginal, Setting, Wrap,
};
use proc_macro2::Span;
use proc_macro_error::{Diagnostic, Level};

pub(crate) struct Settings {
    pub description: Description,
    pub keep_original: Option<KeepOriginal>,
    pub wrap: Option<Wrap>,
}

#[derive(Default)]
struct Collector {
    description: Option<Description>,
    keep_original: Option<KeepOriginal>,
    wrap: Option<Wrap>,
}

impl Settings {
    pub fn from_attributes<A>(attributes: A, item_span: Span) -> Self
    where
        A: IntoIterator<Item = Attribute>,
    {
        let mut collector = Collector::default();

        // Collect `attributes`' `Setting`s in `Collector`,
        // deduplicating `Description`s, `KeepOriginal`s and `Wrap`s

        for attribute in attributes {
            match attribute.setting {
                Setting::Description(description) => {
                    if collector.description.is_some() {
                        // Multiple `Description`s in `attributes`

                        Diagnostic::spanned(
                            attribute.spans.kind,
                            Level::Error,
                            MULTIPLE_DESCRIPTIONS.to_string(),
                        )
                        .help(SINGLE_DESCRIPTION.to_string())
                        .abort();
                    }

                    collector.description = Some(description);
                }

                Setting::KeepOriginal(keep_original) => {
                    if collector.keep_original.is_some() {
                        // Multiple `KeepOriginal`s in `attributes`

                        Diagnostic::spanned(
                            attribute.spans.kind,
                            Level::Error,
                            MULTIPLE_KEEP_ORIGINALS.to_string(),
                        )
                        .help(OPTIONAL_KEEP_ORIGINAL.to_string())
                        .abort();
                    }

                    collector.keep_original = Some(keep_original);
                }

                Setting::Wrap(wrap) => {
                    if collector.wrap.is_some() {
                        // Multiple `Wrap`s in `attributes`

                        Diagnostic::spanned(
                            attribute.spans.kind,
                            Level::Error,
                            MULTIPLE_WRAPS.to_string(),
                        )
                        .help(OPTIONAL_WRAP.to_string())
                        .abort();
                    }

                    collector.wrap = Some(wrap);
                }
            }
        }

        // Ensure all mandatory `Setting`s (`Description`) appeared in `attributes`

        if collector.description.is_none() {
            // `Description` missing from `attributes`

            Diagnostic::spanned(item_span, Level::Error, MISSING_DESCRIPTION.to_string())
                .help(SINGLE_DESCRIPTION.to_string())
                .abort();
        }

        Settings {
            description: collector.description.unwrap(),
            keep_original: collector.keep_original,
            wrap: collector.wrap,
        }
    }
}
