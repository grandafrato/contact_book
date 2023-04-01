use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

fn main() {
    ContactsApp::run(Settings::default()).unwrap();
}

struct ContactsApp;

impl Sandbox for ContactsApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        "Contact List".to_owned()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        container(
            column![text("Hello").size(50)]
                .padding(20)
                .align_items(Alignment::Center)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }
}
