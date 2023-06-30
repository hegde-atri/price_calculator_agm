use iced::executor;
use iced::theme;
use iced::widget::tooltip::Position;
use iced::widget::{checkbox, column, container, tooltip, Text, Tooltip};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    landing_page: bool,
    fb_campaigns: bool,
    yt_campaigns: bool,
    video_editing: bool,
    email_marketing: bool,
    sms_marketing: bool,
    brochures: bool,
    crm: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    LandingPage(bool),
    FbCampaigns(bool),
    YtCampaigns(bool),
    VideoEditing(bool),
    EmailMarketing(bool),
    SmsMarketing(bool),
    Brochures(bool),
    Crm(bool),
}

impl Application for Example {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                landing_page: false,
                fb_campaigns: false,
                yt_campaigns: false,
                video_editing: false,
                email_marketing: false,
                sms_marketing: false,
                brochures: false,
                crm: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("AGM Price Calculator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::LandingPage(landing_page) => {
                self.landing_page = landing_page;
            }
            Message::FbCampaigns(fb_campaigns) => {
                self.fb_campaigns = fb_campaigns;
            }
            Message::YtCampaigns(yt_campaigns) => {
                self.yt_campaigns = yt_campaigns;
            }
            Message::VideoEditing(video_editing) => {
                self.video_editing = video_editing;
            }
            Message::EmailMarketing(email_marketing) => {
                self.email_marketing = email_marketing;
            }
            Message::SmsMarketing(sms_marketing) => {
                self.sms_marketing = sms_marketing;
            }
            Message::Brochures(brochures) => {
                self.brochures = brochures;
            }
            Message::Crm(crm) => {
                self.crm = crm;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        // price function
        fn calculate_price(cur: &Example) -> u32 {
            let mut total = 0;

            if cur.landing_page {
                total += 100;
            }

            if cur.fb_campaigns && cur.yt_campaigns {
                total += 2000;
            } else if cur.fb_campaigns {
                total += 1200;
            } else if cur.yt_campaigns {
                total += 1200;
            }

            if cur.video_editing {
                total += 1200;
            }
            if cur.email_marketing {
                total += 400;
            }
            if cur.sms_marketing {
                total += 400;
            }
            if cur.brochures {
                total += 800;
            }
            if cur.crm {
                total += 200;
            }

            total
        }
        let price_string = format!("Price: £{}", calculate_price(&self));
        let price: Text = Text::new(price_string).size(30);

        let landing_page = tooltip(
            checkbox("Landing Page", self.landing_page, Message::LandingPage),
            "£100",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let fb_campaigns = tooltip(
            checkbox(
                "Facebook Campaigns",
                self.fb_campaigns,
                Message::FbCampaigns,
            ),
            "£1200",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let yt_campaigns = tooltip(
            checkbox("Youtube Campaigns", self.yt_campaigns, Message::YtCampaigns),
            "£1200",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let video_editing = tooltip(
            checkbox("Video Editing", self.video_editing, Message::VideoEditing),
            "£1200",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let email_marketing = tooltip(
            checkbox(
                "Email Marketing",
                self.email_marketing,
                Message::EmailMarketing,
            ),
            "£400",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let sms_marketing = tooltip(
            checkbox("SMS Marketing", self.sms_marketing, Message::SmsMarketing),
            "£400",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let brochures = tooltip(
            checkbox("Brochures", self.brochures, Message::Brochures),
            "£800",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let crm = tooltip(
            checkbox("CRM", self.crm, Message::Crm),
            "£200",
            Position::FollowCursor,
        )
        .gap(20)
        .style(theme::Container::Box);

        let heading: Text = Text::new("AGM Price Calculator").size(50);

        let content = column![
            heading,
            landing_page,
            fb_campaigns,
            yt_campaigns,
            video_editing,
            email_marketing,
            sms_marketing,
            brochures,
            crm,
            price,
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
