mod components;

use components::nav::NavBar;
use components::project::{ProjectCard, ProjectGallery};

use yew::function_component;
use yew::prelude::*;
use yew_icons::IconId;

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

#[function_component]
fn App() -> Html {
    html! {
        <>
            <NavBar />

            <HomepageHero />

            <ProjectGallery>
                <ProjectCard
                    icon_id={IconId::BootstrapMoonStars}
                    name={"Astro Cal"}
                    short_desc={"Generate calendar events for predictable photo events such as moon/sunrise, golden hour, etc"}
                    site_url={"https://astro-cal.whynotcats.com"}
                    source_url={"https://github.com/whynotcats/astro-cal"}
                    blog_url={"https://blog.whynotcats.com/rust-wasm-yew-app-astro-cal.html"}
                />
            </ProjectGallery>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
