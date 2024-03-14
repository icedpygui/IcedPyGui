#![allow(unused_imports)]
#![allow(unused)]

use crate::ipg_widgets::ipg_enums::IpgWidgets;
use crate::app;
use crate::access_state;

use iced::highlighter::{self, Highlighter};
// use iced::keyboard;
use iced::theme;
use iced::widget::{ Column,
    button, Button, column, container, horizontal_space, pick_list, row, text,
    text_editor, tooltip,
};
use iced::{
    Alignment, Element, Font, Length, Renderer
};

use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;




#[derive(Debug)]
pub struct IpgTextEditor {
    pub id: usize,
    pub file_name: String,
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    is_loading: bool,
    is_dirty: bool,
}

impl IpgTextEditor {
    pub fn new(
            id: usize,
            file_name: String,
            ) -> Self 
    {
        Self {
            id,
            file_name,
            file: None,
            content: text_editor::Content::new(),
            theme: highlighter::Theme::SolarizedDark,
            is_loading: false,
            is_dirty: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TEMessage {
    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
}

// pub fn construct_text_editor_controls(te: &IpgTextEditor) -> Element<'static, app::Message> {
    
//     let new_icon: Element<'static, TEMessage> = new_icon();
//     let open_icon: Element<'static, TEMessage> = open_icon();
//     let save_icon: Element<'static, TEMessage> = save_icon();

//     let controls: Element<'static, TEMessage> = row![
//         action(new_icon, "New file", Some(TEMessage::NewFile)),
//         action(
//             open_icon,
//             "Open file",
//             (!te.is_loading).then_some(TEMessage::OpenFile)
//         ),
//         action(
//             save_icon,
//             "Save file",
//             te.is_dirty.then_some(TEMessage::SaveFile)
//         ),
//         horizontal_space(Length::Fill),
//         pick_list(
//             highlighter::Theme::ALL,
//             Some(te.theme),
//             TEMessage::ThemeSelected
//         )
//         .text_size(14)
//         .padding([5, 10])
//     ]
//     .spacing(10)
//     .align_items(Alignment::Center)
//     .into();

//     let col: Element<TEMessage> = Column::with_children(vec![
//         controls.into(),
//         text_editor(&te.content)
//             .on_action(TEMessage::ActionPerformed)
//             .highlight::<Highlighter>(
//                 highlighter::Settings {
//                     theme: te.theme,
//                     extension: te
//                         .file
//                         .as_deref()
//                         .and_then(Path::extension)
//                         .and_then(ffi::OsStr::to_str)
//                         .map(str::to_string)
//                         .unwrap_or(String::from("rs")),
//                 },
//                 |highlight, _theme| highlight.to_format()
//             ).into(),
//     ])
//     .spacing(10)
//     .padding(10)
//     .into();

//     col.map(app::Message::TextEditor)

// }

// pub fn text_editor_update(message: TEMessage) {
//     let mut state = access_state();

//     for widget in state.widgets.iter_mut() {
//         match widget {      
            
//             IpgWidgets::IpgButton(_) => (),
//             // IpgWidgets::IpgCard(_) => (),
//             IpgWidgets::IpgCheckBox(_) => (),
//             IpgWidgets::IpgDatePicker(_) => (),
//             IpgWidgets::IpgMenuBar(_) => (),
//             IpgWidgets::IpgMenuItem(_) => (),
//             IpgWidgets::IpgPickList(_) => (),
//             IpgWidgets::IpgProgressBar(_) => (),
//             IpgWidgets::IpgRadio(_) => (),
//             IpgWidgets::IpgSelectableText(_) => (),
//             IpgWidgets::IpgSlider(_) => (),
//             IpgWidgets::IpgSpace(_) => (),
//             IpgWidgets::IpgTable(_) => (),
//             IpgWidgets::IpgText(_) => (),
//             IpgWidgets::IpgTextEditor(te) => {
//                 match_message(message.clone(), te)
//             }
//             IpgWidgets::IpgTextInput(_) => (),
//         }
//     }

//     drop(state);

//     fn match_message(message: TEMessage, te: &mut IpgTextEditor) {

//         match message {
//             TEMessage::ActionPerformed(action) => {
//                 te.is_dirty = te.is_dirty || action.is_edit();

//                 te.content.perform(action);

//             }
//             TEMessage::ThemeSelected(theme) => {
//                 te.theme = theme;
//             }
//             TEMessage::NewFile => {
//                 if !te.is_loading {
//                     te.file_name = "".to_string();
//                     te.content = text_editor::Content::new();
//                 }
//             }
//             TEMessage::OpenFile => {
//                 if te.is_loading {
//                 } else {
//                     te.is_loading = true;
//                 }
//             }
//             TEMessage::FileOpened(result) => {
//                 te.is_loading = false;
//                 te.is_dirty = false;

//                 if let Ok((path, contents)) = result {
//                     te.file = Some(path);
//                     te.content = text_editor::Content::with_text(&contents);
//                 }
//             }
//             TEMessage::SaveFile => {
//                 if te.is_loading {
//                 } else {
//                     te.is_loading = true;
//                     save_file(te.file.clone(), te.content.text());
//                 }
//             }
//             TEMessage::FileSaved(result) => {
//                 te.is_loading = false;

//                 if let Ok(path) = result {
//                     te.file = Some(path);
//                     te.is_dirty = false;
//                 }

//             }
//     }
//     }
// }


#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

// fn default_file() -> PathBuf {
//     PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
// }

// async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
//     let picked_file = rfd::AsyncFileDialog::new()
//         .set_title("Open a text file...")
//         .pick_file()
//         .await
//         .ok_or(Error::DialogClosed)?;

//     load_file(picked_file.path().to_owned()).await
// }

// async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
//     let contents = tokio::fs::read_to_string(&path)
//         .await
//         .map(Arc::new)
//         .map_err(|error| Error::IoError(error.kind()))?;

//     Ok((path, contents))
// }

// async fn save_file(
//     path: Option<PathBuf>,
//     contents: String,
// ) -> Result<PathBuf, Error> {
//     let path = if let Some(path) = path {
//         path
//     } else {
//         rfd::AsyncFileDialog::new()
//             .save_file()
//             .await
//             .as_ref()
//             .map(rfd::FileHandle::path)
//             .map(Path::to_owned)
//             .ok_or(Error::DialogClosed)?
//     };

//     tokio::fs::write(&path, contents)
//         .await
//         .map_err(|error| Error::IoError(error.kind()))?;

//     Ok(path)
// }

// fn action<'a, TEMessage: Clone + 'a>(
//     content: impl Into<Element<'a, TEMessage, Renderer>>,
//     label: &'a str,
//     on_press: Option<TEMessage>,
// ) -> Element<'a, TEMessage, Renderer> {
//     let action: Button<'a, TEMessage, Renderer> = button(container(content).width(30).center_x()).into();

//     if let Some(on_press) = on_press {
//         tooltip(
//             action.on_press(on_press),
//             label,
//             tooltip::Position::FollowCursor,
//         )
//         .style(theme::Container::Box)
//         .into()
//     } else {
//         action.style(theme::Button::Secondary).into()
//     }
// }

// fn new_icon<'a, TEMessage>() -> Element<'a, TEMessage> {
//     icon('\u{0e800}')
// }

// fn save_icon<'a, TEMessage>() -> Element<'a, TEMessage> {
//     icon('\u{0e801}')
// }

// fn open_icon<'a, TEMessage>() -> Element<'a, TEMessage> {
//     icon('\u{0f115}')
// }

// fn icon<'a, TEMessage>(codepoint: char) -> Element<'a, TEMessage> {
//     const ICON_FONT: Font = Font::with_name("editor-icons");

//     text(codepoint).font(ICON_FONT).into()
// }
