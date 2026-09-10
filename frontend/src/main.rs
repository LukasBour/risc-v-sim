use iced::Element;
use iced::widget::{button, column, container, text};

pub fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
        Message::Decrement => *counter -= 1,
        Message::Reset => *counter = 0,
    }
}

fn view(counter: &u64) -> Element<'_, Message> {
    container(
        column![
            button("Increment").on_press(Message::Increment),
            button("Decrement").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset),
            text(counter)
        ]
        .spacing(10),
    )
    .into()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}
