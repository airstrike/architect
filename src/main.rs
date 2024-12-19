use iced::widget::{button, center, column, row};
use iced::Length::Fill;
use iced::{Element, Task, Theme};

fn main() -> iced::Result {
    iced::application("Architect (iced example)", App::update, App::view)
        .theme(App::theme)
        .window(iced::window::Settings {
            size: [500.0, 500.0].into(),
            ..Default::default()
        })
        .run_with(App::new)
}

#[derive(Default)]
struct App {
    screen: Screen,
    users: Users,
}

#[derive(Default)]
pub enum Users {
    #[default]
    Loading,
    Loaded(Vec<User>),
}

pub type User = String;

#[derive(Debug, Clone)]
enum Message {
    Navigate(Screen),
    Top(top::Message),
    Users(users::Message),
    LoadUsers(Result<Vec<String>, String>),
}

#[derive(Debug, Clone)]
enum Screen {
    Top(top::Screen),
    Users,
}

// Hacky, sorry
impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Screen::Top(_) => write!(f, "Top"),
            Screen::Users => write!(f, "Users"),
        }
    }
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        // let task = Task::perform(/* any other initialization tasks here */);
        (Self::default(), Task::none())
    }

    pub fn theme(&self) -> Theme {
        iced::Theme::Ferra
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(Screen::Users) => {
                self.screen = Screen::Users;
                if let Users::Loading = self.users {
                    return Task::perform(load_users(), Message::LoadUsers);
                }
            }
            Message::Navigate(other_screen) => {
                self.screen = other_screen;
            }
            Message::Top(message) => {
                if let Screen::Top(screen) = &mut self.screen {
                    screen.update(message);
                }
            }
            Message::Users(message) => {
                if let Screen::Users = &mut self.screen {
                    return users::update(message).map(Message::Users);
                }
            }
            Message::LoadUsers(result) => match result {
                Ok(users) => {
                    self.users = Users::Loaded(users);
                }
                Err(error) => {
                    eprintln!("User loading failed with error: {}", error);
                }
            },
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let screen = match &self.screen {
            Screen::Top(screen) => screen.view().map(Message::Top),
            Screen::Users => users::view(&self.users).map(Message::Users),
        };

        let top_button = button("Top")
            .on_press(Message::Navigate(Screen::Top(top::Screen::default())))
            .style(|theme, _status| tab_style(theme, &self.screen.to_string() == "Top"));

        let users_button = button("Users")
            .on_press(Message::Navigate(Screen::Users))
            .style(|theme, _status| tab_style(theme, &self.screen.to_string() == "Users"));

        let buttons = row![top_button, users_button]
            .width(Fill)
            .padding(10)
            .spacing(2);

        column![buttons, center(screen)].into()
    }
}

fn tab_style(theme: &Theme, is_active: bool) -> button::Style {
    if is_active {
        button::Style {
            background: Some(theme.extended_palette().primary.base.color.into()),
            text_color: theme.extended_palette().primary.base.text.into(),
            ..Default::default()
        }
    } else {
        button::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            ..Default::default()
        }
    }
}

// simulate some slow loading of users and return a Result<Vec<String>, String>
async fn load_users() -> Result<Vec<String>, String> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok(vec!["Alice".to_string(), "Bob".to_string()])
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Top(top::Screen::default())
    }
}

mod top {
    use iced::widget::{center, text};
    use iced::Element;

    #[derive(Debug, Default, Clone)]
    pub struct Screen;

    #[derive(Debug, Clone)]
    pub enum Message {}

    impl Screen {
        pub fn update(&mut self, _message: Message) {}
        pub fn view(&self) -> Element<Message> {
            center(text("Top Screen")).into()
        }
    }
}

mod users {
    use iced::widget::{container, text, Column};
    use iced::{Center, Element, Length, Task};

    use crate::Users;

    #[derive(Debug, Clone)]
    pub enum Message {}

    #[must_use]
    pub fn update(_message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view<'a>(users: &'a Users) -> Element<'a, Message> {
        match users {
            Users::Loading => container(text("Loading users...")).into(),
            Users::Loaded(users) => Column::from_iter(users.iter().map(|user| {
                container(text(user))
                    .padding(2)
                    .style(container::rounded_box)
                    .width(Length::Fill)
                    .into()
            }))
            .width(Length::Fixed(200.0))
            .align_x(Center)
            .spacing(10)
            .into(),
        }
    }
}
