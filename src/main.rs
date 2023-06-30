use iced::executor;
use iced::widget::{checkbox, column, container};
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
        let landing_page = checkbox("Landing Page", self.landing_page, Message::LandingPage);
        let fb_campaigns = checkbox(
            "Facebook Campaigns",
            self.fb_campaigns,
            Message::FbCampaigns,
        );
        let yt_campaigns = checkbox("Youtube Campaigns", self.yt_campaigns, Message::YtCampaigns);
        let video_editing = checkbox("Video Editing", self.video_editing, Message::VideoEditing);
        let email_marketing = checkbox(
            "Email Marketing",
            self.email_marketing,
            Message::EmailMarketing,
        );
        let sms_marketing = checkbox("SMS Marketing", self.sms_marketing, Message::SmsMarketing);
        let brochures = checkbox("Brochures", self.brochures, Message::Brochures);
        let crm = checkbox("CRM", self.crm, Message::Crm);

        let content = column![
            landing_page,
            fb_campaigns,
            yt_campaigns,
            video_editing,
            email_marketing,
            sms_marketing,
            brochures,
            crm
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
