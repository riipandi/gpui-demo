//! Themed `Button` component for the demo's design system.
//!
//! Following the shadcn model (presentation owned by the app, behavior by the
//! foundation), this module wraps the unstyled `gpui_base::Button` in a themed
//! component. Pick a [`ButtonVariant`] and [`ButtonSize`]; all visual tokens
//! come from [`crate::theme`].
//!
//! The raw, unstyled base is re-exported as [`BaseButton`] for the rare case
//! you need to opt out of the preset and style a button by hand.
//!
//! ```ignore
//! use ui::{Button, ButtonVariant, ButtonSize};
//!
//! Button::new("save").variant(ButtonVariant::Primary).child("Save");
//! Button::new("cancel").variant(ButtonVariant::Ghost).size(ButtonSize::Sm).child("Cancel");
//! ```

use gpui::prelude::*;
use gpui::{AnyElement, App, ElementId, Interactivity, RenderOnce, StyleRefinement, Window};

// Re-export the unstyled base as `BaseButton` — the escape hatch.
pub use gpui_base::Button as BaseButton;
use gpui_base::Selectable;

use crate::theme;

/// Visual style of a [`Button`], mapped to the demo's carbon palette.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonVariant {
    /// Solid accent fill — the primary call-to-action.
    Primary,
    /// Translucent surface with a strong hairline border.
    Secondary,
    /// Bare control chrome, no border.
    Ghost,
    /// Transparent with a hairline border.
    Outline,
    /// Accent fill reserved for destructive actions.
    Destructive,
}

/// Control height and horizontal padding of a [`Button`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

/// A themed button: the demo's `Button` component.
///
/// Wraps [`BaseButton`], delegating layout, interaction, and accessibility
/// behavior while owning the visual preset (variant + size).
#[derive(IntoElement)]
pub struct Button {
    inner: BaseButton,
    variant: ButtonVariant,
    size: ButtonSize,
}

impl Button {
    /// Creates a button with the default [`ButtonVariant::Primary`] and
    /// [`ButtonSize::Md`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        let mut this = Self {
            inner: BaseButton::new(id),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
        };
        this.apply_style();
        this
    }

    /// Sets the visual variant. Safe to call before `.child(...)` / `.on_click(...)`.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self.apply_style();
        self
    }

    /// Sets the control size. Safe to call before `.child(...)` / `.on_click(...)`.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self.apply_style();
        self
    }

    /// Re-applies the full variant + size preset so switching variants never
    /// leaves stale style fields behind (every branch fully specifies border,
    /// background, text color, height, padding, and font size).
    fn apply_style(&mut self) {
        let mut inner = std::mem::replace(&mut self.inner, BaseButton::new("__style"));

        inner = match self.variant {
            ButtonVariant::Primary => inner
                .bg(theme::accent())
                .text_color(theme::text_inverted())
                .border_0(),
            ButtonVariant::Secondary => inner
                .bg(theme::surface())
                .text_color(theme::text())
                .border_1()
                .border_color(theme::border_strong()),
            ButtonVariant::Ghost => inner
                .bg(theme::control())
                .text_color(theme::text())
                .border_0(),
            ButtonVariant::Outline => inner
                .bg(gpui::transparent_black())
                .text_color(theme::text())
                .border_1()
                .border_color(theme::border()),
            ButtonVariant::Destructive => inner
                .bg(theme::accent())
                .text_color(theme::text_inverted())
                .border_0(),
        };

        inner = match self.size {
            ButtonSize::Sm => inner.h_8().px_3().text_sm(),
            ButtonSize::Md => inner.h_9().px_4().text_base(),
            ButtonSize::Lg => inner.h_10().px_5().text_lg(),
        };

        self.inner = inner;
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.inner.render(window, cx)
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.inner.interactivity()
    }
}

impl StatefulInteractiveElement for Button {}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl Selectable for Button {
    fn selected(self, selected: bool) -> Self {
        Self {
            inner: self.inner.selected(selected),
            ..self
        }
    }

    fn is_selected(&self) -> bool {
        self.inner.is_selected()
    }
}
