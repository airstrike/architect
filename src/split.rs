// Modified from iced_aw to work with iced master branch (eventually 0.13)
// https://github.com/iced-rs/iced_aw/blob/main/src/widgets/split.rs
// https://github.com/iced-rs/iced_aw/blob/main/src/style/split.rs

// MIT License

// Copyright (c) 2020 Kaiden42

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Use a split to split the available space in two parts to display two different elements.
//!
//! *This API requires the following crate features to be activated: split*

use iced::{
    advanced::{
        layout::{Limits, Node},
        overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    border, event,
    mouse::{self, Cursor},
    theme::palette,
    touch,
    widget::Row,
    Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle, Shadow, Size,
    Theme, Vector,
};

/// A split can divide the available space by half to display two different elements.
/// It can split horizontally or vertically.
///
/// # Example
/// ```ignore
/// # use iced_aw::split::{State, Axis, Split};
/// # use iced::widget::Text;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     Resized(u16),
/// }
///
/// let first = Text::new("First");
/// let second = Text::new("Second");
///
/// let split = Split::new(first, second, Some(300), Axis::Vertical, Message::Resized);
/// ```
#[allow(missing_debug_implementations)]
pub struct Split<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// The first element of the [`Split`].
    first: Element<'a, Message, Theme, Renderer>,
    /// The second element of the [`Split`].
    second: Element<'a, Message, Theme, Renderer>,
    /// The position of the divider.
    divider_position: Option<u16>,
    /// The axis to split at.
    axis: Axis,
    /// The padding around the elements of the [`Split`].
    padding: f32,
    /// The spacing between the elements of the [`Split`].
    /// This is also the width of the divider.
    spacing: f32,
    /// The width of the [`Split`].
    width: Length,
    /// The height of the [`Split`].
    height: Length,
    /// The minimum size of the first element of the [`Split`].
    min_size_first: u16,
    /// The minimum size of the second element of the [`Split`].
    min_size_second: u16,
    /// The message that is send when the divider of the [`Split`] is moved.
    on_resize: Box<dyn Fn(u16) -> Message>,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Split<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog,
{
    /// Creates a new [`Split`].
    ///
    /// It expects:
    ///     - The first [`Element`] to display
    ///     - The second [`Element`] to display
    ///     - The position of the divider. If none, the space will be split in half.
    ///     - The [`Axis`] to split at.
    ///     - The message that is send on moving the divider
    pub fn new<A, B, F>(
        first: A,
        second: B,
        divider_position: Option<u16>,
        axis: Axis,
        on_resize: F,
    ) -> Self
    where
        A: Into<Element<'a, Message, Theme, Renderer>>,
        B: Into<Element<'a, Message, Theme, Renderer>>,
        F: 'static + Fn(u16) -> Message,
    {
        Self {
            first: first.into(),
            // first: Container::new(first.into())
            //     .width(Length::Fill)
            //     .height(Length::Fill)
            //     .into(),
            second: second.into(),
            // second: Container::new(second.into())
            //     .width(Length::Fill)
            //     .height(Length::Fill)
            //     .into(),
            divider_position,
            axis,
            padding: 0.0,
            spacing: 5.0,
            width: Length::Fill,
            height: Length::Fill,
            min_size_first: 5,
            min_size_second: 5,
            on_resize: Box::new(on_resize),
            class: Theme::default(),
        }
    }

    /// Sets the padding of the [`Split`] around the inner elements.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing of the [`Split`] between the elements.
    /// This will also be the width of the divider.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the width of the [`Split`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Split`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the minimum size of the first element of the [`Split`].
    #[must_use]
    pub fn min_size_first(mut self, size: u16) -> Self {
        self.min_size_first = size;
        self
    }

    /// Sets the minimum size of the second element of the [`Split`].
    #[must_use]
    pub fn min_size_second(mut self, size: u16) -> Self {
        self.min_size_second = size;
        self
    }

    /// Sets the style of the [`Split`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Split`].
    // #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Split<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.first), Tree::new(&self.second)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.first, &self.second]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let space = Row::<Message, Theme, Renderer>::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .layout(tree, renderer, limits);

        match self.axis {
            Axis::Horizontal => horizontal_split(tree, self, renderer, limits, &space),
            Axis::Vertical => vertical_split(tree, self, renderer, limits, &space),
        }
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let split_state: &mut State = state.state.downcast_mut();
        let mut children = layout.children();

        let first_layout = children
            .next()
            .expect("Native: Layout should have a first layout");
        let first_status = self.first.as_widget_mut().on_event(
            &mut state.children[0],
            event.clone(),
            first_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let divider_layout = children
            .next()
            .expect("Native: Layout should have a divider layout");
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if divider_layout
                    .bounds()
                    .expand(5.0)
                    .contains(cursor.position().unwrap_or_default())
                {
                    split_state.dragging = true;
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if split_state.dragging {
                    split_state.dragging = false;
                }
            }

            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if split_state.dragging {
                    let position = match self.axis {
                        Axis::Horizontal => position.y,
                        Axis::Vertical => position.x,
                    };

                    shell.publish((self.on_resize)(position as u16));
                }
            }

            _ => {}
        }

        let second_layout = children
            .next()
            .expect("Native: Layout should have a second layout");
        let second_status = self.second.as_widget_mut().on_event(
            &mut state.children[1],
            event,
            second_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        first_status.merge(second_status)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let first_layout = children
            .next()
            .expect("Graphics: Layout should have a first layout");
        let first_mouse_interaction = self.first.as_widget().mouse_interaction(
            &state.children[0],
            first_layout,
            cursor,
            viewport,
            renderer,
        );
        let divider_layout = children
            .next()
            .expect("Graphics: Layout should have a divider layout");
        let divider_mouse_interaction = if divider_layout
            .bounds()
            .expand(5.0)
            .contains(cursor.position().unwrap_or_default())
        {
            match self.axis {
                Axis::Horizontal => mouse::Interaction::ResizingVertically,
                Axis::Vertical => mouse::Interaction::ResizingHorizontally,
            }
        } else {
            mouse::Interaction::default()
        };
        let second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");
        let second_mouse_interaction = self.second.as_widget().mouse_interaction(
            &state.children[1],
            second_layout,
            cursor,
            viewport,
            renderer,
        );
        first_mouse_interaction
            .max(second_mouse_interaction)
            .max(divider_mouse_interaction)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        // TODO: clipping!
        let mut children = layout.children();

        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();
        let is_mouse_over = cursor.is_over(bounds);

        let status = if is_mouse_over {
            let state = tree.state.downcast_ref::<State>();

            if state.dragging {
                Status::Dragging
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        let style = theme.style(&self.class, status);

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds: content_layout.bounds(),
                border: style.border,
                shadow: Shadow::default(),
            },
            style
                .background
                .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        let first_layout = children
            .next()
            .expect("Graphics: Layout should have a first layout");

        let bounds_first = first_layout.bounds();
        let is_mouse_over_first = cursor.is_over(bounds_first);

        let status_first = if is_mouse_over_first {
            let state = tree.state.downcast_ref::<State>();

            if state.dragging {
                Status::Dragging
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        let style_first = theme.style(&self.class, status_first);

        // First
        renderer.fill_quad(
            renderer::Quad {
                bounds: bounds_first,
                border: style_first.first_border,
                shadow: Shadow::default(),
            },
            style_first
                .first_background
                .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        self.first.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style::default(),
            first_layout,
            cursor,
            viewport,
        );

        let divider_layout = children
            .next()
            .expect("Graphics: Layout should have a divider layout");

        // Second
        let second_layout = children
            .next()
            .expect("Graphics: Layout should have a second layout");

        let bounds_second = second_layout.bounds();
        let is_mouse_over_second = cursor.is_over(bounds_second);

        let status_second = if is_mouse_over_second {
            let state = tree.state.downcast_ref::<State>();

            if state.dragging {
                Status::Dragging
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        let style_second = theme.style(&self.class, status_second);

        renderer.fill_quad(
            renderer::Quad {
                bounds: bounds_second,
                border: style_second.second_border,
                shadow: Shadow::default(),
            },
            style_second
                .second_background
                .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        self.second.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            &renderer::Style::default(),
            second_layout,
            cursor,
            viewport,
        );

        let bounds_divider = divider_layout.bounds();
        let is_mouse_over_divider = cursor.is_over(bounds_divider.expand(5.0));

        let status_divider = if is_mouse_over_divider {
            let state = tree.state.downcast_ref::<State>();

            if state.dragging {
                Status::Dragging
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        let style_divider = theme.style(&self.class, status_divider);

        renderer.fill_quad(
            renderer::Quad {
                bounds: divider_layout.bounds(),
                border: style_divider.divider_border,
                shadow: Shadow::default(),
            },
            style_divider.divider_background,
        );
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let mut children = layout.children();
        let first_layout = children.next().expect("Missing Split First window");
        let _divider_layout = children.next().expect("Missing Split Divider");
        let second_layout = children.next().expect("Missing Split Second window");

        let (first_state, second_state) = state.children.split_at_mut(1);

        self.first
            .as_widget()
            .operate(&mut first_state[0], first_layout, renderer, operation);
        self.second
            .as_widget()
            .operate(&mut second_state[0], second_layout, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let mut children = layout.children();
        let first_layout = children.next()?;
        let _divider_layout = children.next()?;
        let second_layout = children.next()?;

        let first = &mut self.first;
        let second = &mut self.second;

        // Not pretty but works to get two mutable references
        // https://stackoverflow.com/a/30075629
        let (first_state, second_state) = state.children.split_at_mut(1);

        first
            .as_widget_mut()
            .overlay(&mut first_state[0], first_layout, renderer, translation)
            .or_else(|| {
                second.as_widget_mut().overlay(
                    &mut second_state[0],
                    second_layout,
                    renderer,
                    translation,
                )
            })
    }
}

/// Do a horizontal split.
fn horizontal_split<'a, Message, Theme, Renderer>(
    tree: &mut Tree,
    split: &Split<'a, Message, Theme, Renderer>,
    renderer: &Renderer,
    limits: &Limits,
    space: &Node,
) -> Node
where
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog,
{
    if space.bounds().height
        < split.spacing + f32::from(split.min_size_first + split.min_size_second)
    {
        return Node::with_children(
            space.bounds().size(),
            vec![
                split.first.as_widget().layout(
                    &mut tree.children[0],
                    renderer,
                    &limits.clone().shrink(Size::new(0.0, space.bounds().height)),
                ),
                Node::new(Size::new(space.bounds().height, split.spacing)),
                split.second.as_widget().layout(
                    &mut tree.children[1],
                    renderer,
                    &limits.clone().shrink(Size::new(0.0, space.bounds().width)),
                ),
            ],
        );
    }

    let divider_position = split
        .divider_position
        .unwrap_or_else(|| (space.bounds().height / 2.0) as u16)
        .max((split.spacing / 2.0) as u16);
    let divider_position = (divider_position - (split.spacing / 2.0) as u16).clamp(
        split.min_size_first,
        space.bounds().height as u16 - split.min_size_second - split.spacing as u16,
    );

    let padding = Padding::from(split.padding as u16);
    let first_limits = limits
        .clone()
        .shrink(Size::new(
            0.0,
            space.bounds().height - f32::from(divider_position),
        ))
        .shrink(padding);
    let mut first = split
        .first
        .as_widget()
        .layout(&mut tree.children[0], renderer, &first_limits);
    first.move_to_mut(Point::new(
        space.bounds().x + split.padding,
        space.bounds().y + split.padding,
    ));

    let mut divider = Node::new(Size::new(space.bounds().width, split.spacing));
    divider.move_to_mut(Point::new(space.bounds().x, f32::from(divider_position)));

    let second_limits = limits
        .clone()
        .shrink(Size::new(0.0, f32::from(divider_position) + split.spacing))
        .shrink(padding);
    let mut second =
        split
            .second
            .as_widget()
            .layout(&mut tree.children[1], renderer, &second_limits);
    second.move_to_mut(Point::new(
        space.bounds().x + split.padding,
        space.bounds().y + f32::from(divider_position) + split.spacing + split.padding,
    ));

    Node::with_children(space.bounds().size(), vec![first, divider, second])
}

/// Do a vertical split.
fn vertical_split<'a, Message, Theme, Renderer>(
    tree: &mut Tree,
    split: &Split<'a, Message, Theme, Renderer>,
    renderer: &Renderer,
    limits: &Limits,
    space: &Node,
) -> Node
where
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog,
{
    if space.bounds().width
        < split.spacing + f32::from(split.min_size_first + split.min_size_second)
    {
        return Node::with_children(
            space.bounds().size(),
            vec![
                split.first.as_widget().layout(
                    &mut tree.children[0],
                    renderer,
                    &limits.clone().shrink(Size::new(space.bounds().width, 0.0)),
                ),
                Node::new(Size::new(split.spacing, space.bounds().height)),
                split.second.as_widget().layout(
                    &mut tree.children[1],
                    renderer,
                    &limits.clone().shrink(Size::new(space.bounds().width, 0.0)),
                ),
            ],
        );
    }

    let divider_position = split
        .divider_position
        .unwrap_or_else(|| (space.bounds().width / 2.0) as u16)
        .max((split.spacing / 2.0) as u16);
    let divider_position = (divider_position - (split.spacing / 2.0) as u16).clamp(
        split.min_size_first,
        space.bounds().width as u16 - split.min_size_second - split.spacing as u16,
    );

    let padding = Padding::from(split.padding as u16);
    let first_limits = limits
        .clone()
        .shrink(Size::new(
            space.bounds().width - f32::from(divider_position),
            0.0,
        ))
        .shrink(padding);
    let mut first = split
        .first
        .as_widget()
        .layout(&mut tree.children[0], renderer, &first_limits);
    first.move_to_mut(Point::new(
        space.bounds().x + split.padding,
        space.bounds().y + split.padding,
    ));

    let mut divider = Node::new(Size::new(split.spacing, space.bounds().height));
    divider.move_to_mut(Point::new(f32::from(divider_position), space.bounds().y));

    let second_limits = limits
        .clone()
        .shrink(Size::new(f32::from(divider_position) + split.spacing, 0.0))
        .shrink(padding);
    let mut second =
        split
            .second
            .as_widget()
            .layout(&mut tree.children[1], renderer, &second_limits);
    second.move_to_mut(Point::new(
        space.bounds().x + f32::from(divider_position) + split.spacing + split.padding,
        space.bounds().y + split.padding,
    ));

    Node::with_children(space.bounds().size(), vec![first, divider, second])
}

impl<'a, Message, Theme, Renderer> From<Split<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Theme: Catalog + 'a,
{
    fn from(split_pane: Split<'a, Message, Theme, Renderer>) -> Self {
        Element::new(split_pane)
    }
}

/// The state of a [`Split`].
#[derive(Clone, Debug, Default)]
pub struct State {
    /// If the divider is dragged by the user.
    dragging: bool,
}

impl State {
    /// Creates a new [`State`] for a [`Split`].
    ///
    /// It expects:
    ///     - The optional position of the divider. If none, the available space will be split in half.
    ///     - The [`Axis`] to split at.
    #[must_use]
    pub const fn new() -> Self {
        Self { dragging: false }
    }
}

/// The axis to split at.
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    /// Split horizontally.
    Horizontal,
    /// Split vertically.
    Vertical,
}

impl Default for Axis {
    fn default() -> Self {
        Self::Vertical
    }
}

/// The possible statuses of a [`Split`].
pub enum Status {
    /// The [`Split`] can be dragged.
    Active,
    /// The [`Split`] can be dragged and it is being hovered.
    Hovered,
    /// The [`Split`] is being dragged.
    Dragging,
    /// The [`Split`] cannot be dragged.
    Disabled,
}

/// The style of a [`Split`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The optional background of the [`Split`].
    pub background: Option<Background>,
    /// The optional background of the first element of the [`Split`].
    pub first_background: Option<Background>,
    /// The optional background of the second element of the [`Split`].
    pub second_background: Option<Background>,
    /// The [`Border`] of the [`Split`].
    pub border: Border,
    /// The [`Border`] of the [`Split`].
    pub first_border: Border,
    /// The [`Border`] of the [`Split`].
    pub second_border: Border,
    /// The background of the divider of the [`Split`].
    pub divider_background: Background,
    /// The [`Border`] of the divider of the [`Split`].
    pub divider_border: Border,
}

impl Style {
    /// Updates the [`Style`] with the given [`Background`].
    pub fn with_background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            first_background: None,
            second_background: None,
            border: Border::default(),
            first_border: Border::default(),
            second_border: Border::default(),
            divider_background: Background::Color(Color::TRANSPARENT),
            divider_border: Border::default(),
        }
    }
}

/// The theme catalog of a [`Split`].
pub trait Catalog {
    /// The item class of the [`Split`].
    type Class<'a>;

    /// The default class produced by the [`Split`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Split`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(base)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn base(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong);

    match status {
        Status::Active => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            divider_background: Background::Color(palette.primary.strong.color),
            ..base
        },
        Status::Dragging => Style {
            background: Some(Background::Color(palette.primary.weak.color)),
            divider_background: Background::Color(palette.primary.base.color),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        border: border::rounded(2),
        ..Style::default()
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        ..style
    }
}