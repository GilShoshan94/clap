//! Terminal [`Styles`] for help and error output

pub use anstyle::*;

/// Terminal styling definitions
///
/// See also [`Command::styles`][crate::Command::styles].
///
/// # Example
///
/// clap v3 styling
/// ```rust
/// # use clap_builder as clap;
/// # use clap::builder::styling::*;
/// let styles = Styles::styled()
///     .header(AnsiColor::Yellow.on_default())
///     .usage(AnsiColor::Green.on_default())
///     .literal(AnsiColor::Green.on_default())
///     .placeholder(AnsiColor::Green.on_default());
/// ```
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)] // Large enough type that I want an explicit `clone()` for now
pub struct Styles {
    header: Style,
    error: Style,
    usage: Style,
    literal: Style,
    placeholder: Style,
    valid: Style,
    invalid: Style,
    context: Style,
    context_data: Option<Style>,
    context_aliases: Option<Style>,
    context_aliases_data: Option<Style>,
    context_default: Option<Style>,
    context_default_data: Option<Style>,
    context_env: Option<Style>,
    context_env_data: Option<Style>,
    context_possible_values: Option<Style>,
    context_possible_values_data: Option<Style>,
}

impl Styles {
    /// No terminal styling
    pub const fn plain() -> Self {
        Self {
            header: Style::new(),
            error: Style::new(),
            usage: Style::new(),
            literal: Style::new(),
            placeholder: Style::new(),
            valid: Style::new(),
            invalid: Style::new(),
            context: Style::new(),
            context_data: None,
            context_aliases: None,
            context_aliases_data: None,
            context_default: None,
            context_default_data: None,
            context_env: None,
            context_env_data: None,
            context_possible_values: None,
            context_possible_values_data: None,
        }
    }

    /// Default terminal styling
    pub const fn styled() -> Self {
        #[cfg(feature = "color")]
        {
            Self {
                header: Style::new().bold().underline(),
                error: Style::new()
                    .fg_color(Some(Color::Ansi(AnsiColor::Red)))
                    .bold(),
                usage: Style::new().bold().underline(),
                literal: Style::new().bold(),
                placeholder: Style::new(),
                valid: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))),
                invalid: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
                context: Style::new(),
                context_data: None,
                context_aliases: None,
                context_aliases_data: None,
                context_default: None,
                context_default_data: None,
                context_env: None,
                context_env_data: None,
                context_possible_values: None,
                context_possible_values_data: None,
            }
        }
        #[cfg(not(feature = "color"))]
        {
            Self::plain()
        }
    }

    /// General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]
    #[inline]
    pub const fn header(mut self, style: Style) -> Self {
        self.header = style;
        self
    }

    /// Error heading
    #[inline]
    pub const fn error(mut self, style: Style) -> Self {
        self.error = style;
        self
    }

    /// Usage heading
    #[inline]
    pub const fn usage(mut self, style: Style) -> Self {
        self.usage = style;
        self
    }

    /// Literal command-line syntax, e.g. `--help`
    #[inline]
    pub const fn literal(mut self, style: Style) -> Self {
        self.literal = style;
        self
    }

    /// Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]
    #[inline]
    pub const fn placeholder(mut self, style: Style) -> Self {
        self.placeholder = style;
        self
    }

    /// Highlight suggested usage
    #[inline]
    pub const fn valid(mut self, style: Style) -> Self {
        self.valid = style;
        self
    }

    /// Highlight invalid usage
    #[inline]
    pub const fn invalid(mut self, style: Style) -> Self {
        self.invalid = style;
        self
    }

    /// Highlight all specified contexts: `[aliases], [default], [env], and [possible values]`.
    ///
    /// * Specific contexts style can be overwrite individually.
    /// * Also default style for `context_data` if not explicitly set.
    #[inline]
    pub const fn context(mut self, style: Style) -> Self {
        self.context = style;
        self
    }

    /// Highlight data/values within the specified contexts: `[aliases], [default], [env], and [possible values]`
    ///
    /// * Applied only on the contexts that were not individually set.
    /// * If not not explicitly set, fallbacks to `context`'s style.
    #[inline]
    pub const fn context_data(mut self, style: Style) -> Self {
        self.context_data = Some(style);
        self
    }

    /// Highlight `[aliases]` context (overwrite default `context`).
    ///
    /// * If not not explicitly set, fallbacks to `context`'s style.
    /// * Also default style for `context_aliases_data` if not explicitly set.
    #[inline]
    pub const fn context_aliases(mut self, style: Style) -> Self {
        self.context_aliases = Some(style);
        self
    }

    /// Highlight data/values within the `[aliases]` context.
    ///
    /// If not not explicitly set, fallbacks to `context_aliases`'s style.
    #[inline]
    pub const fn context_aliases_data(mut self, style: Style) -> Self {
        self.context_aliases_data = Some(style);
        self
    }

    /// Highlight `[default]` context (overwrite default `context`).
    ///
    /// * If not not explicitly set, fallbacks to `context`'s style.
    /// * Also default style for `context_default_data` if not explicitly set.
    #[inline]
    pub const fn context_default(mut self, style: Style) -> Self {
        self.context_default = Some(style);
        self
    }

    /// Highlight data/values within the `[default]` context.
    ///
    /// If not not explicitly set, fallbacks to `context_default`'s style.
    #[inline]
    pub const fn context_default_data(mut self, style: Style) -> Self {
        self.context_default_data = Some(style);
        self
    }

    /// Highlight `[env]` context (overwrite default `context`).
    ///
    /// * If not not explicitly set, fallbacks to `context`'s style.
    /// * Also default style for `context_env_data` if not explicitly set.
    #[inline]
    pub const fn context_env(mut self, style: Style) -> Self {
        self.context_env = Some(style);
        self
    }

    /// Highlight data/values within the `[env]` context.
    ///
    /// If not not explicitly set, fallbacks to `context_env`'s style.
    #[inline]
    pub const fn context_env_data(mut self, style: Style) -> Self {
        self.context_env_data = Some(style);
        self
    }

    /// Highlight `[possible values]` context (overwrite default `context`).
    ///
    /// * If not not explicitly set, fallbacks to `context`'s style.
    /// * Also default style for `context_possible_values_data` if not explicitly set.
    #[inline]
    pub const fn context_possible_values(mut self, style: Style) -> Self {
        self.context_possible_values = Some(style);
        self
    }

    /// Highlight data/values within the `[possible values]` context.
    ///
    /// If not not explicitly set, fallbacks to `context_possible_values`'s style.
    #[inline]
    pub const fn context_possible_values_data(mut self, style: Style) -> Self {
        self.context_possible_values_data = Some(style);
        self
    }
}

/// Reflection
impl Styles {
    /// General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]
    #[inline(always)]
    pub const fn get_header(&self) -> &Style {
        &self.header
    }

    /// Error heading
    #[inline(always)]
    pub const fn get_error(&self) -> &Style {
        &self.error
    }

    /// Usage heading
    #[inline(always)]
    pub const fn get_usage(&self) -> &Style {
        &self.usage
    }

    /// Literal command-line syntax, e.g. `--help`
    #[inline(always)]
    pub const fn get_literal(&self) -> &Style {
        &self.literal
    }

    /// Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]
    #[inline(always)]
    pub const fn get_placeholder(&self) -> &Style {
        &self.placeholder
    }

    /// Highlight suggested usage
    #[inline(always)]
    pub const fn get_valid(&self) -> &Style {
        &self.valid
    }

    /// Highlight invalid usage
    #[inline(always)]
    pub const fn get_invalid(&self) -> &Style {
        &self.invalid
    }

    /// Highlight specified contexts: `[aliases], [default], [env], and [possible values]`
    /// (Unless overwritten individually per context)
    #[inline(always)]
    pub const fn get_context(&self) -> &Style {
        &self.context
    }

    /// Highlight data/values within the specified contexts: `[aliases], [default], [env], and [possible values]`
    ///
    /// * Applied only on the contexts that were not individually set.
    /// * If `context_data` was not set, defaults to `context`'s style.
    #[inline(always)]
    pub const fn get_context_data(&self) -> &Style {
        match &self.context_data {
            Some(s) => s,
            None => &self.context,
        }
    }

    /// Highlight `[aliases]` context (overwrite default `context`).
    #[inline(always)]
    pub const fn get_context_aliases(&self) -> &Style {
        match &self.context_aliases {
            Some(s) => s,
            None => &self.context,
        }
    }

    /// Highlight data/values within the `[aliases]` context.
    ///
    /// If `context_aliases_data` was not set:
    /// - defaults to `context_aliases`'s style if it was explicitly set.
    /// - otherwise defaults to `context_data`'s style.
    #[inline(always)]
    pub const fn get_context_aliases_data(&self) -> &Style {
        match &self.context_aliases_data {
            Some(s) => s,
            None => match &self.context_aliases {
                Some(b) => b,
                None => self.get_context_data(),
            },
        }
    }

    /// Highlight `[default]` context (overwrite default `context`).
    #[inline(always)]
    pub const fn get_context_default(&self) -> &Style {
        match &self.context_default {
            Some(s) => s,
            None => &self.context,
        }
    }

    /// Highlight data/values within the `[default]` context.
    ///
    /// If `context_default_data` was not set:
    /// - defaults to `context_default`'s style if it was explicitly set.
    /// - otherwise defaults to `context_data`'s style.
    #[inline(always)]
    pub const fn get_context_default_data(&self) -> &Style {
        match &self.context_default_data {
            Some(s) => s,
            None => match &self.context_default {
                Some(b) => b,
                None => self.get_context_data(),
            },
        }
    }

    /// Highlight `[env]` context (overwrite default `context`).
    #[inline(always)]
    pub const fn get_context_env(&self) -> &Style {
        match &self.context_env {
            Some(s) => s,
            None => &self.context,
        }
    }

    /// Highlight data/values within the `[env]` context.
    ///
    /// If `context_env_data` was not set:
    /// - defaults to `context_env`'s style if it was explicitly set.
    /// - otherwise defaults to `context_data`'s style.
    #[inline(always)]
    pub const fn get_context_env_data(&self) -> &Style {
        match &self.context_env_data {
            Some(s) => s,
            None => match &self.context_env {
                Some(b) => b,
                None => self.get_context_data(),
            },
        }
    }

    /// Highlight `[possible values]` context (overwrite default `context`).
    #[inline(always)]
    pub const fn get_context_possible_values(&self) -> &Style {
        match &self.context_possible_values {
            Some(s) => s,
            None => &self.context,
        }
    }

    /// Highlight data/values within the `[possible values]` context.
    ///
    /// If `context_possible_values_data` was not set:
    /// - defaults to `context_possible_values`'s style if it was explicitly set.
    /// - otherwise defaults to `context_data`'s style.
    #[inline(always)]
    pub const fn get_context_possible_values_data(&self) -> &Style {
        match &self.context_possible_values_data {
            Some(s) => s,
            None => match &self.context_possible_values {
                Some(b) => b,
                None => self.get_context_data(),
            },
        }
    }
}

impl super::AppExt for Styles {}

impl Default for Styles {
    fn default() -> Self {
        Self::styled()
    }
}

impl Default for &'_ Styles {
    fn default() -> Self {
        const STYLES: Styles = Styles::styled();
        &STYLES
    }
}
