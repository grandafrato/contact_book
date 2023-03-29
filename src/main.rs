use iced::{Element, Sandbox, Settings};

fn main() {
    Hello::run(Settings::default()).unwrap();
}

struct Hello;

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        "Contact List".to_owned()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        "Hello, World".into()
    }
}
