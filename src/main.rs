mod components;

use components::nav::NavBar;
use components::project::{ProjectCard, ProjectGallery};

use yew::function_component;
use yew::prelude::*;

use crate::components::icons::IconId;

#[function_component(HomepageHero)]
fn homepage_hero() -> Html {
    html! {
        <section class="hero is-medium is-bold">
            <div class="hero-body">
                <div class="container has-text-centered-mobile has-text-centered-tablet has-text-left-desktop">
                    <p class="title has-text-white">{"Experiments in code, photography, music, and design; built to simply exist"}</p>
                    <p class="subtitle has-text-white has-text-right">{"Y"}<sub>{"0"}</sub>{"?"}</p>
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Column)]
pub fn column(props: &ColumnProps) -> Html {
    html! {
        <>
            <div class="column is-4-desktop is-4-tablet">
                {props.children.clone()}
            </div>
        </>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <NavBar />
            <HomepageHero />
            <ProjectGallery>
                <Column>
                    <ProjectCard
                        icon_id={IconId::MoonStars}
                        name={"Astro Cal"}
                        short_desc={"Generate calendar events for predictable photo events such as moon/sunrise, golden hour, etc"}
                        site_url={"https://astro-cal.whynotcats.com"}
                        source_url={"https://github.com/whynotcats/astro-cal"}
                        blog_url={"https://blog.whynotcats.com/rust-wasm-yew-app-astro-cal.html"}
                        summary={include_str!("one-pagers\\astro-cal.md")} />
                    <ProjectCard
                        icon_id={IconId::MapLocationDot}
                        name={"Trip Summary"}
                        short_desc={"Generate a summary of your trip, including a map, photos, and a list of places visited"}
                        in_dev={true}
                        summary={include_str!("one-pagers\\trip-summary.md")} />
                    <ProjectCard
                        name={"Social Media Feed"}
                        icon_id={IconId::ColumnsGap}
                        short_desc={"Take back control over your social media accounts and get a custom amalgamated social media feed"}
                        summary={include_str!("one-pagers\\social-media-feed.md")}
                        in_dev={true} />
                </Column>
                <Column>
                    <ProjectCard
                        icon_id={IconId::Twitch}
                        name={"Twitch Games"}
                        short_desc={"Twitch integrated games"}
                        summary={include_str!("one-pagers\\twitch-games.md")}
                        in_dev={true} />
                    <ProjectCard
                        icon_id={IconId::Bento}
                        name={"Bento Generator"}
                        short_desc={"Generate balanced bento boxes with variable ingedients"}
                        summary={include_str!("one-pagers\\bento-generator.md")}
                        in_dev={true} />
                </Column>
                <Column>
                    <ProjectCard
                        icon_id={IconId::JarWheat}
                        name={"Bread Assistant"}
                        short_desc={"Tool to assist managing the schedule of home bread making"}
                        summary={include_str!("one-pagers\\bread-assistant.md")}
                        in_dev={true} />
                    <ProjectCard
                        icon_id={IconId::Linkedin}
                        name={"Cat Linkedin Bot"}
                        short_desc={"ChatGPT bot to generate a linkedin posts (from the perspective of a cat)"}
                        summary={include_str!("one-pagers\\cat-linkedin-bot.md")}
                        in_dev={true} />
                </Column>
            </ProjectGallery>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
