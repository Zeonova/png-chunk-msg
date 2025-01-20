use iced::widget::{
    button, center, column, horizontal_space, row, scrollable, text, text_input, vertical_space,
};
use iced::Element;
use rfd::FileDialog;

mod png_tools;

pub fn main() -> iced::Result {
    iced::application("PNG-MSG", Application::update, Application::view).run()
}
#[derive(Default)]
struct Application {
    file_path: String,
    chunk_type: String,
    chunk_data: String,
    output_msg: String,
}

#[derive(Debug, Clone)]
enum Message {
    FileButtonPressed,
    EncodeButtonPressed,
    DecodeButtonPressed,
    RemoveButtonPressed,
    PrintButtonPressed,
    ChunkTypeInputChanged(String),
    ChunkDataInputChanged(String),
}

impl Application {
    fn update(&mut self, message: Message) {
        match message {
            Message::FileButtonPressed => {
                if let Some(path) = FileDialog::new().add_filter("PNG", &["png"]).pick_file() {
                    self.file_path = path.display().to_string();
                }
            }
            Message::ChunkTypeInputChanged(text) => {
                if text.len() > 4 || text.chars().any(|c| !c.is_ascii_alphabetic()) {
                    return;
                }
                self.chunk_type = text;
            }
            Message::ChunkDataInputChanged(text) => {
                self.chunk_data = text;
            }
            Message::EncodeButtonPressed => {
                match png_tools::encode(&self.file_path, &self.chunk_type, &self.chunk_data) {
                    Ok(path) => {
                        self.output_msg = format!("Chunk encoded to {}", path);
                    }
                    Err(e) => {
                        self.output_msg = format!("Error: {}", e);
                    }
                }
            }
            Message::DecodeButtonPressed => {
                match png_tools::decode(&self.file_path, &self.chunk_type) {
                    Ok(data) => {
                        self.output_msg = format!("Chunk data: {}", data);
                    }
                    Err(e) => {
                        self.output_msg = format!("Error: {}", e);
                    }
                }
            }
            Message::RemoveButtonPressed => {
                match png_tools::remove(&self.file_path, &self.chunk_type) {
                    Ok(path) => {
                        self.output_msg = format!("Chunk removed from {}", path);
                    }
                    Err(e) => {
                        self.output_msg = format!("Error: {}", e);
                    }
                }
            }
            Message::PrintButtonPressed => match png_tools::print(&self.file_path) {
                Ok(data) => {
                    self.output_msg = format!("PNG data: {}", data);
                }
                Err(e) => {
                    self.output_msg = format!("Error: {}", e);
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let able = !(self.file_path.is_empty() || self.chunk_type.is_empty());

        let encode_able = !self.file_path.is_empty()
            && !self.chunk_type.is_empty()
            && !self.chunk_data.is_empty();

        let file = row![
            text_input("PNG FILE PATH", &self.file_path),
            button(text("SELECT PNG FILE")).on_press(Message::FileButtonPressed)
        ]
        .spacing(10);

        let chunk = row![
            text_input("CHUNK TYPE 4 BYTE", &self.chunk_type)
                .on_input(Message::ChunkTypeInputChanged),
            text_input("CHUNK DATA", &self.chunk_data).on_input(Message::ChunkDataInputChanged),
            horizontal_space(),
            button(text("ENCODE"))
                .on_press_maybe(encode_able.then(|| Message::EncodeButtonPressed)),
            button(text("DECODE")).on_press_maybe(able.then(|| (Message::DecodeButtonPressed))),
            button(text("REMOVE")).on_press_maybe(able.then(|| (Message::RemoveButtonPressed))),
            button(text("PRINT")).on_press_maybe(
                (!self.file_path.is_empty()).then(|| (Message::PrintButtonPressed))
            ),
        ]
        .spacing(10);
        let output = scrollable(column![text(&self.output_msg), vertical_space().height(30)]);
        let content = column![file, chunk, output].spacing(10).padding(10);
        center(content).into()
    }
}
