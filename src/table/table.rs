//! Display rows of data into columns

// Many thanks to "tarkah <admin@tarkah.dev>"
// at https://github.com/tarkah/iced_table
// for the code used use in the table.
// Modifications were made to fit in with IPG


use iced::advanced::graphics::core::Element;
use iced::{Length, Padding};
use iced::widget::{container, row, Space};

use super::style;

/// Some docs
pub fn single_row_container<'a, Message, Theme, Renderer>(
        id: usize,
        index: usize,
        column: Element<'a, Message, Theme, Renderer>,
        column_width: f32,
        resize_offset: Option<f32>,
        on_drag: Option<fn((usize, usize), f32) -> Message>,
        on_release: Option<Message>,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
        style: <Theme as style::Catalog>::Style,
    )  -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + 'a,
        Message: 'a + Clone,
{
    let content: Element<Message, Theme, Renderer> = 
        container(column)
            .width(Length::Fill)
            .padding(cell_padding)
            .into();

    with_divider(
        id,
        index,
        column_width,
        resize_offset,
        content,
        on_drag,
        on_release,
        min_column_width,
        divider_width,
        style,
    )
}

pub fn body_container<'a, Message, Theme, Renderer>(
    column: Element<'a, Message, Theme, Renderer>,
    column_width: f32,
    resize_offset: Option<f32>,
    min_column_width: f32,
    divider_width: f32,
    cell_padding: Padding,
    row_max_height: Option<f32>,
) -> Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + 'a,
    Theme: style::Catalog + container::Catalog + 'a,
    Message: 'a + Clone,
{
    let width = column_width + resize_offset.unwrap_or_default();

    let content: Element<'a, Message, Theme, Renderer> = 
        container(column)
        .width(Length::Fill)
        .padding(cell_padding)
        .into();

    let spacing = Space::new(divider_width, Length::Shrink);

    let height = if let Some(max) = row_max_height {
        Length::Fixed(max)
    } else {
        Length::Shrink
    };

    row![content, spacing]
        .width(width.max(min_column_width))
        .clip(true)
        .height(height)
        .into()
}

// pub fn footer_container<'a, Message, Theme, Renderer>(
//     index: usize,
//     footer: Element<'a, Message, Theme, Renderer>,
//     column_width: f32,
//     resize_offset: Option<f32>,
//     on_drag: Option<fn(usize, f32) -> Message>,
//     on_release: Option<Message>,
//     min_column_width: f32,
//     divider_width: f32,
//     cell_padding: Padding,
//     style: <Theme as style::Catalog>::Style,
// ) -> Element<'a, Message, Theme, Renderer>
// where
//     Renderer: iced::advanced::Renderer + 'a,
//     Theme: style::Catalog + container::Catalog + 'a,
//     Message: 'a + Clone,
// {
//     let content = 
//         container(footer)
//             .width(Length::Fill)
//             .padding(cell_padding)
//             .into();

//     with_divider(
//         index,
//         column_width,
//         resize_offset,
//         content,
//         on_drag,
//         on_release,
//         min_column_width,
//         divider_width,
//         style,
//     )
// }

use super::divider::Divider;
/// Some docs
pub fn with_divider<'a, Message, Theme, Renderer>(
    id: usize,
    index: usize,
    column_width: f32,
    resize_offset: Option<f32>,
    content: Element<'a, Message, Theme, Renderer>,
    on_drag: Option<fn((usize, usize), f32) -> Message>,
    on_release: Option<Message>,
    min_column_width: f32,
    divider_width: f32,
    style: <Theme as style::Catalog>::Style,
) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced::advanced::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + 'a,
        Message: 'a + Clone,
{
    let width =
        (column_width + resize_offset.unwrap_or_default()).max(min_column_width);
    if let Some((on_drag, on_release)) = on_drag.zip(on_release) {
        let old_width = column_width;
        container(Divider::new(
            content,
            divider_width,
            move |offset| {
                let new_width = (old_width + offset).max(min_column_width);

                (on_drag)((id, index), new_width - old_width)
            },
            on_release,
            style,
        ))
        .width(width)
        .into()
    } else {
        row![content, Space::new(divider_width, Length::Shrink)]
            .width(width)
            .into()
    }
}

// Used to enforce "min_width"
pub fn dummy_container<'a, Message, Theme, Renderer>(
    column_widths: Vec<f32>,
    resize_offset: Vec<Option<f32>>,
    min_width: f32,
    min_column_width: f32,
) -> Option<Element<'a, Message, Theme, Renderer>>
where
    Renderer: iced::advanced::Renderer + 'a,
    Theme: style::Catalog + container::Catalog + 'a,
    Message: 'a + Clone,
{
    let total_width: f32 = column_widths
        .iter()
        .enumerate()
        .map(|(idx, width)| {
            (*width + resize_offset[idx].unwrap_or_default()).max(min_column_width)
        })
        .sum();
    
    let remaining = min_width - total_width;
  
    (remaining > 0.0).then(|| container(Space::with_width(remaining)).into())
}
