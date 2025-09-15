//! Provides types for representing output styles

use std::{borrow::Cow, collections::HashMap, fmt::Display, process::Output};

use crate::{
    style::{AdaptiveStyle, Style, StyledString},
    widgets::Span,
};

/// Returns if the terminal background is light. If this cannot be determined, return false.
pub fn is_light() -> bool {
    terminal_light::luma().map_or(false, |luma| luma > 0.6)
}

/// A type for storing output styles and templates
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Theme<'a> {
    /// Stores output styles as a hashmap with string keys.
    pub styles: HashMap<String, AdaptiveStyle>,

    /// Stores output templates for e.g. warnings
    pub templates: HashMap<String, OutputTemplate<'a>>,
}

impl<'a> Theme<'a> {
    /// Returns an empty theme object
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            templates: HashMap::new(),
        }
    }

    /// Add a style to the theme
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn style<K, S>(mut self, key: K, style: S) -> Self
    where
        K: AsRef<str>,
        S: Into<AdaptiveStyle>,
    {
        self.styles.insert(key.as_ref().to_string(), style.into());
        self
    }

    /// Add a template to the theme
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn template<K: AsRef<str>>(mut self, key: K, template: OutputTemplate<'a>) -> Self {
        self.templates.insert(key.as_ref().to_string(), template);
        self
    }

    /// Add a style
    pub fn add_style<K, S>(&mut self, key: K, style: S)
    where
        K: AsRef<str>,
        S: Into<AdaptiveStyle>,
    {
        self.styles.insert(key.as_ref().to_string(), style.into());
    }

    /// Get a style by key
    pub fn get_style<K: AsRef<str>>(&mut self, key: K) -> Option<&AdaptiveStyle> {
        self.styles.get(key.as_ref())
    }

    /// Apply style by key
    pub fn apply_style<K, D>(&self, key: K, content: D) -> StyledString<D>
    where
        K: AsRef<str>,
        D: Display,
    {
        let style = self.styles.get(key.as_ref()).copied().unwrap_or_default();

        style.apply(content)
    }
}

/// The type for an output template which is implemented with the [Span] widget.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutputTemplate<'a> {
    /// The key of the associated [AdaptiveStyle] in the [Theme]
    associated_style: Option<String>,
    before: Option<Span<'a>>,
    after: Option<Span<'a>>,
}

impl<'a> OutputTemplate<'a> {
    /// Create a new output template with the given style, before [Span] and after [Span]
    pub fn new<T, S>(style: Option<S>, before: Option<T>, after: Option<T>) -> Self
    where
        T: Into<Span<'a>>,
        S: AsRef<str>,
    {
        Self {
            associated_style: style.map(|s| s.as_ref().to_string()),
            before: before.map(Into::into),
            after: after.map(Into::into),
        }
    }
}
