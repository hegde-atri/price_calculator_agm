#![windows_subsystem = "windows"]

use iced::{Application, Command, Element, Length, Settings, Theme};
use iced::alignment;
use iced::executor;
use iced::theme;
use iced::widget::{checkbox, column, container, Text};

fn main() -> iced::Result {
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
        let landing_page = checkbox("Landing Page", self.landing_page, Message::LandingPage);
        let fb_campaigns = checkbox(
            "Facebook Campaigns",
            self.fb_campaigns,
            Message::FbCampaigns,
        );
        let yt_campaigns = checkbox("Youtube Campaigns", self.yt_campaigns, Message::YtCampaigns);
        let video_editing = checkbox(
            "Video Editing (720 seconds per month)",
            self.video_editing,
            Message::VideoEditing,
        );
        let email_marketing = checkbox(
            "Email Marketing",
            self.email_marketing,
            Message::EmailMarketing,
        );
        let sms_marketing = checkbox("SMS Marketing", self.sms_marketing, Message::SmsMarketing);
        let brochures = checkbox("Brochures", self.brochures, Message::Brochures);
        let crm = checkbox("CRM", self.crm, Message::Crm);

        let heading: Text = Text::new("AGM Price Calculator").size(50);
        let subtitle: Text = Text::new("Monthly prices")
            .size(20)
            .style(theme::Text::Color([0.4, 0.4, 0.4].into()));

        let mut contents = column![];

        contents = contents.push(heading);
        contents = contents.push(subtitle);
        contents = contents.push(landing_page);
        contents = contents.push(fb_campaigns);
        contents = contents.push(yt_campaigns);
        contents = contents.push(video_editing);
        contents = contents.push(email_marketing);
        contents = contents.push(sms_marketing);
        contents = contents.push(brochures);
        contents = contents.push(crm);
        contents = contents.push(price);

        container(contents.spacing(20).padding(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_y(alignment::Vertical::Center)
            .align_x(alignment::Horizontal::Center)
            .into()
    }
}
