use yew::{function_component, html, AttrValue, Html, Properties};
use yew_icons::{Icon as YewIcon, IconId as YewIconId};

// This is a way for us to efectively extend any number of SVG Icon libraries

// First we have an IconId enum which is the primary identifier we will interact with in our application
#[derive(Clone, Copy, PartialEq)]
pub enum IconId {
    BlogIcon,
    Github,
    Mastodon,
    Camera,
    Twitter,
    Twitch,
    MoonStars,
    SolidCat,
    MapLocationDot,
    JarWheat,
    Linkedin,
    ColumnsGap,
    InfoFilled,
    Bento,
    Default,
}

impl Default for IconId {
    fn default() -> IconId {
        IconId::Default
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum CustomIconId {
    Bento,
    Default
}

// Essentially an intermediate enum which serves as glue between our IconId interface enum and all the different icon libraries we may use
#[derive(Clone, Copy, PartialEq)]
pub enum IconType {
    Custom(CustomIconId),
    Yew(YewIconId),
}

impl IconId {
    // We need to be able to get a wrapped value to use in our components so we don't have to type out the full wrapper each time
    pub fn value(&self) -> IconType {
        match self {
            IconId::MoonStars => IconType::Yew(YewIconId::BootstrapMoonStars),
            IconId::SolidCat => IconType::Yew(YewIconId::FontAwesomeSolidCat),
            IconId::Github => IconType::Yew(YewIconId::BootstrapGithub),
            IconId::BlogIcon => IconType::Yew(YewIconId::BootstrapJournalBookmarkFill),
            IconId::Camera => IconType::Yew(YewIconId::BootstrapCameraFill),
            IconId::Twitter => IconType::Yew(YewIconId::BootstrapTwitter),
            IconId::Mastodon => IconType::Yew(YewIconId::BootstrapMastodon),
            IconId::MapLocationDot => IconType::Yew(YewIconId::FontAwesomeSolidMapLocationDot),
            IconId::JarWheat => IconType::Yew(YewIconId::FontAwesomeSolidJarWheat),
            IconId::Linkedin => IconType::Yew(YewIconId::BootstrapLinkedin),
            IconId::Twitch => IconType::Yew(YewIconId::BootstrapTwitch),
            IconId::ColumnsGap  => IconType::Yew(YewIconId::BootstrapColumnsGap),
            IconId::InfoFilled  => IconType::Yew(YewIconId::BootstrapInfoCircleFill),
            IconId::Bento => IconType::Custom(CustomIconId::Bento),
            _ => IconType::Custom(CustomIconId::Default)
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SVGProps {
    #[prop_or(AttrValue::from("24"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::from("24"))]
    pub height: AttrValue,
}

#[function_component(Bento)]
pub fn bento(props: &SVGProps) -> Html {
    html! {
        <svg fill="currentColor" height={props.height.clone()} width={props.width.clone()} viewbox="0 0 16 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink"
            viewBox="0 0 400 400" space="preserve">
            <g>
                <g>
                    <path d="M328.347,0H50.087C22.469,0,0,22.469,0,50.087v278.26c0,27.618,22.469,50.087,50.087,50.087h278.26
                        c27.618,0,50.087-22.469,50.087-50.087V50.087C378.434,22.469,355.965,0,328.347,0z M144.695,144.695h50.087
                        c9.206,0,16.696,7.49,16.696,16.696v50.087h-66.782V144.695z M33.391,50.087c0-9.206,7.49-16.696,16.696-16.696h61.217v178.087
                        H33.391V50.087z M178.087,345.043h-128c-9.206,0-16.696-7.49-16.696-16.696v-83.478h144.695V345.043z M345.043,328.347
                        c0,9.206-7.49,16.696-16.696,16.696H211.478V244.869h133.565V328.347z M345.043,211.478H244.869v-50.087
                        c0-27.618-22.469-50.087-50.087-50.087h-50.087V33.391h183.652c9.206,0,16.696,7.49,16.696,16.696V211.478z"/>
                </g>
            </g>
        </svg>
    }
}
#[derive(Properties, PartialEq)]
pub struct CustomIconProps {
    pub id: CustomIconId,
}

#[function_component(CustomIcon)]
pub fn custom_icon(props: &CustomIconProps) -> Html {
    match props.id {
        CustomIconId::Bento => html! { <Bento /> },
        CustomIconId::Default => html! { <div /> },
    }
}

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub id: IconId,
}

// This is the component we will call in our application
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    match props.id.value() {
        IconType::Custom(id) => html! { <CustomIcon id={id} /> },
        IconType::Yew(id) => html! {<YewIcon icon_id={id} />},
    }
}
