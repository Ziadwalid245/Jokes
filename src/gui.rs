use crate::get_joke::{Error, Joke};
use iced::widget::{Button, Column, Container, Text};
use iced::{executor, Application, Command, Length, Settings, Theme};

pub fn my_app() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug)]
pub enum MyApp {
    Loading,
    Loaded { joke: Joke },
    Error,
}
#[derive(Debug, Clone)]
pub enum Message {
    JokeFetched(Result<Joke, Error>),
    GetJoke,
}
impl Application for MyApp {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;
    fn title(&self) -> String {
        String::from("get a joke")
    }
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            MyApp::Loading,
            Command::perform(Joke::get_joke(), |value| Message::JokeFetched(value)),
        )
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::GetJoke => {
                *self = MyApp::Loading;
                Command::perform(Joke::get_joke(), |value| Message::JokeFetched(value))
            }

            Message::JokeFetched(Ok(joke)) => {
                *self = MyApp::Loaded { joke };
                Command::none()
            }
            Message::JokeFetched(Err(_error)) => {
                *self = MyApp::Error;
                Command::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let content = match self {
            MyApp::Loading => Column::new().push(Text::new("Fetching Joke")),
            MyApp::Loaded { joke } => {
                let setup_text = Text::new(joke.setup.clone());
                let delivery = Text::new(joke.delivery.clone());
                let get_new_joke = Button::new("get a joke").on_press(Message::GetJoke);
                Column::new()
                    .push(setup_text)
                    .push(delivery)
                    .push(get_new_joke)
            }
            MyApp::Error => {
                let error = Text::new("something went wrong ﾍ(･_|");
                let get_new_joke = Button::new("get a joke").on_press(Message::GetJoke);
                Column::new().push(error).push(get_new_joke)
            }
        };
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
