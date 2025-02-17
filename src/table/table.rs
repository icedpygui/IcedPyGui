//! A table widget for iced
//! Display rows of data into columns


use iced::advanced::graphics::core::Element;
use iced::{Length, Padding};
use iced::widget::{container, row, Space};

use super::style;



// pub struct Table {
//     header: scrollable::Id,
//     body: scrollable::Id,
//     footer: Option<scrollable::Id>,
//     columns: usize,
//     rows: usize,
//     on_sync: fn(scrollable::AbsoluteOffset) -> Message,
//     on_column_drag: Option<fn(usize, f32) -> Message>,
//     on_column_release: Option<Message>,
//     min_width: f32,
//     min_column_width: f32,
//     divider_width: f32,
//     cell_padding: Padding,
//     style: <Theme as style::Catalog>::Style,
//     scrollbar: scrollable::Scrollbar,
// }

// impl Table {
//     fn new(
//         header: scrollable::Id,
//         body: scrollable::Id,
//         footer: Option<scrollable::Id>,
//         columns: usize,
//         rows: usize,
//         on_sync: fn(scrollable::AbsoluteOffset) -> Message,
//         on_column_drag: Option<fn(usize, f32) -> Message>,
//         on_column_release: Option<Message>,
//         min_width: f32,
//         min_column_width: f32,
//         divider_width: f32,
//         cell_padding: Padding,
//         style: <Theme as style::Catalog>::Style,
//         scrollbar: scrollable::Scrollbar,
//         ) -> Self {
//         let Table {
//             header,
//             body,
//             footer,
//             columns,
//             rows,
//             on_sync,
//             on_column_drag,
//             on_column_release,
//             min_width,
//             min_column_width,
//             divider_width,
//             cell_padding,
//             style,
//             scrollbar,
//         };

//         let header = scrollable(style::wrapper::header(
//             row(columns
//                 .iter()
//                 .enumerate()
//                 .map(|(index, column)| {
//                     header_container(
//                         index,
//                         column,
//                         on_column_drag,
//                         on_column_release.clone(),
//                         min_column_width,
//                         divider_width,
//                         cell_padding,
//                         style.clone(),
//                     )
//                 })
//                 .chain(dummy_container(columns, min_width, min_column_width))),
//             style.clone(),
//         ))
//         .id(header)
//         .direction(scrollable::Direction::Both {
//             vertical: scrollable::Scrollbar::new()
//                 .width(0)
//                 .margin(0)
//                 .scroller_width(0),
//             horizontal: scrollable::Scrollbar::new()
//                 .width(0)
//                 .margin(0)
//                 .scroller_width(0),
//         });

//         let body = scrollable(column(rows.iter().enumerate().map(|(row_index, _row)| {
//             style::wrapper::row(
//                 row(columns
//                     .iter()
//                     .enumerate()
//                     .map(|(col_index, column)| {
//                         body_container(
//                             col_index,
//                             row_index,
//                             min_column_width,
//                             divider_width,
//                             cell_padding,
//                         )
//                     })
//                     .chain(dummy_container(columns, min_width, min_column_width))),
//                 style.clone(),
//                 row_index,
//             )
//             .into()
//         })))
//         .id(body)
//         .on_scroll(move |viewport| {
//             let offset = viewport.absolute_offset();

//             (on_sync)(scrollable::AbsoluteOffset { y: 0.0, ..offset })
//         })
//         .direction(scrollable::Direction::Both {
//             horizontal: scrollbar,
//             vertical: scrollbar,
//         })
//         .height(Length::Fill);

//         let footer = footer.map(|footer| {
//             scrollable(style::wrapper::footer(
//                 row(columns
//                     .iter()
//                     .enumerate()
//                     .map(|(index, column)| {
//                         footer_container(
//                             index,
//                             on_column_drag,
//                             on_column_release.clone(),
//                             min_column_width,
//                             divider_width,
//                             cell_padding,
//                             style.clone(),
//                         )
//                     })
//                     .chain(dummy_container(columns, min_width, min_column_width))),
//                 style,
//             ))
//             .id(footer)
//             .direction(scrollable::Direction::Both {
//                 vertical: scrollable::Scrollbar::new()
//                     .width(0)
//                     .margin(0)
//                     .scroller_width(0),
//                 horizontal: scrollable::Scrollbar::new()
//                     .width(0)
//                     .margin(0)
//                     .scroller_width(0),
//             })
//         });

//         let mut column = column![header, body];

//         if let Some(footer) = footer {
//             column = column.push(footer);
//         }

//         column.height(Length::Fill).into()
//     }
// }
/// Some docs
pub fn header_container<'a, Message, Theme, Renderer>(
        index: usize,
        column: Element<'a, Message, Theme, Renderer>,
        column_width: f32,
        resize_offset: Option<f32>,
        on_drag: Option<fn(usize, f32) -> Message>,
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
    let content: Element<Message, Theme, Renderer> = container(column)
        .width(Length::Fill)
        .padding(cell_padding)
        .into();

    with_divider(
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
        .padding(cell_padding).into();

    let spacing = Space::new(divider_width, Length::Shrink);

    row![content, spacing]
        .width(width.max(min_column_width))
        .into()
}

// pub fn footer_container<'a, Message, Theme, Renderer>(
//     index: usize,
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
//     let content = if let Some(footer) = column.footer(index, rows) {
//         container(footer)
//             .width(Length::Fill)
//             .padding(cell_padding)
//             .into()
//     } else {
//         Element::from(Space::with_width(Length::Fill))
//     };

//     with_divider(
//         index,
//         column,
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
    index: usize,
    column_width: f32,
    resize_offset: Option<f32>,
    content: Element<'a, Message, Theme, Renderer>,
    on_drag: Option<fn(usize, f32) -> Message>,
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

                (on_drag)(index, new_width - old_width)
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
    resize_offset: Option<f32>,
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
        .map(|_| {
            (min_width + resize_offset.unwrap_or_default()).max(min_column_width)
        })
        .sum();

    let remaining = min_width - total_width;

    (remaining > 0.0).then(|| container(Space::with_width(remaining)).into())
}
