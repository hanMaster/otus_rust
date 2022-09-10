use iced::keyboard::KeyCode::Escape;
use iced::{
    executor, keyboard, window, Application, Column, Command, Container, Element, Length, Row,
    Settings, Subscription, Text, Toggler,
};
use iced_native::Event;
use std::fmt::Debug;
use sock_client::SocketClient;

// cargo run --package client
pub fn main() -> iced::Result {
    Model::run(Settings {
        window: window::Settings {
            size: (450, 270),
            resizable: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct Model {
    is_on: bool,
    power: f64,
    status: String,
    should_exit: bool,
}

#[derive(Debug)]
enum Message {
    EventOccurred(Event),
    Synced(std::io::Result<(u8, f64)>),
    SwitchedOn(std::io::Result<()>),
    Toggle,
}

impl Application for Model {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Model, Command<Self::Message>) {
        (
            Model {
                is_on: false,
                power: 500.0,
                status: "Not connected".to_string(),
                should_exit: false,
            },
            Command::perform(sync(), Message::Synced),
        )
    }

    fn title(&self) -> String {
        String::from("TCP smart socket controller")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Synced(state) => {
                if let Ok((state, pwr)) = state {
                    self.is_on = state == 1u8;
                    self.power = pwr;
                    self.status = "Synced".to_string()
                } else {
                    self.status = "Not connected".to_string()
                }
            }
            Message::Toggle => {
                if self.is_on {
                    self.is_on = false;
                    return Command::perform(switch_off(), Message::SwitchedOn);
                }
                self.is_on = true;
                return Command::perform(switch_on(), Message::SwitchedOn);
            }

            Message::SwitchedOn(result) => {
                if result.is_ok() {
                    return Command::perform(sync(), Message::Synced);
                }
            }

            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
                    if key_code == Escape {
                        self.should_exit = true;
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> Element<Self::Message> {
        let title = Row::new()
            .push(Text::new("TCP smart socket".to_string()).size(60))
            .height(Length::Units(80));

        let toggler = Toggler::new(self.is_on, "On/Off".to_string(), |_| Message::Toggle)
            .size(30)
            .width(Length::Shrink);

        let power = Text::new(format!("Power consumption: {:.2} Watt", self.power)).size(30);

        let status = Row::new().push(
            Text::new(format!("Status: {}", self.status))
                .size(30)
                .width(Length::Fill),
        );

        let body = Column::new()
            .spacing(30)
            .push(toggler)
            .push(power)
            .height(Length::Fill);

        let content = Column::new()
            .padding(20)
            .push(title)
            .push(body)
            .push(status);
        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .into()
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

async fn sync() -> std::io::Result<(u8, f64)> {
    let (state, pwr) = SocketClient::get_status()?;
    Ok((state, pwr))
}

async fn switch_on() -> std::io::Result<()> {
    SocketClient::switch_on()?;
    Ok(())
}

async fn switch_off() -> std::io::Result<()> {
    SocketClient::switch_off()?;
    Ok(())
}
