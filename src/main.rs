// use wasm_bindgen::prelude::*;
use yew::function_component;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component]
fn App() -> Html {
    html! {
        <>
        <nav class="navbar is-sticky-top navigation" role="navigation" aria-label="main navigation">
        <div class="container is-mobile">
            <div class="navbar-brand">
                <a class="navbar-item" href="/">
                    <span class="icon-text has-background-primary has-text-white p-2">
                        <span class="icon">
                            <Icon icon_id={IconId::FontAwesomeSolidCat} />
                        </span>
                        <span>{"Why Not Cats?"}</span>
                    </span>
                </a>
                <a class="navbar-item" href="https://blog.whynotcats.com">
                    <span class="icon-text">
                        <span class="icon">
                            <Icon icon_id={IconId::BootstrapJournalBookmarkFill} />
                        </span>
                        <span>{"Blog"}</span>
                    </span>
                </a>
                <a href="https://github.com/whynotcats" class="navbar-item is-hidden-touch">
                    <span class="icon-text">
                        <span class="icon">
                            <Icon icon_id={IconId::BootstrapGithub} />
                        </span>
                        <span>{"Github"}</span>
                    </span>
                </a>
                <div class="navbar-item is-hidden-desktop is-justify-content-end is-flex-grow-1">
                <a href="https://github.com/whynotcats" class="navbar-item">
                    <span class="icon-text is-hidden-touch">
                        <span class="icon">
                            <Icon icon_id={IconId::BootstrapGithub} />
                        </span>
                        <span>{"Github"}</span>
                    </span>
                    <span class="icon is-hidden-dekstop">
                        <Icon icon_id={IconId::BootstrapGithub} />
                    </span>
                </a>
                <a target="_blank" href="https://twitter.com/whynotcats" class="navbar-item">
                    <span class="icon">
                        <Icon icon_id={IconId::BootstrapTwitter} />
                    </span>
                </a>
                <a target="_blank" href="https://glass.photo/whynotcats" class="navbar-item">
                    <span class="icon">
                        <Icon icon_id={IconId::BootstrapCameraFill} />
                    </span>
                </a>
                </div>
            </div>
            <div class="navbar-end">
                <a target="_blank" href="https://twitter.com/whynotcats" class="navbar-item is-hidden-touch">
                    <span class="icon">
                        <Icon icon_id={IconId::BootstrapTwitter} />
                    </span>
                </a>
                <a target="_blank" href="https://glass.photo/whynotcats" class="navbar-item is-hidden-touch">
                    <span class="icon">
                        <Icon icon_id={IconId::BootstrapCameraFill} />
                    </span>
                </a>
            </div>
        </div>
        </nav>

        <section class="hero is-medium is-bold">
            <div class="hero-body">
                <div class="container has-text-centered-mobile has-text-centered-tablet has-text-left-desktop">
                    <p class="title has-text-white">{"Experiments in code, photography, music, and design; built to simply exist"}</p>
                    <p class="subtitle has-text-white has-text-right">{"Y"}<sub>{"0"}</sub>{"?"}</p>
                </div>
            </div>
        </section>

        <section class="section pb-0">
        <div class="container">
            <h2 class="title has-text-centered-mobile has-text-centered-tablet has-text-left-desktop">{"Projects"}</h2>
            <div class="columns is-multiline">
                <div class="column is-4-widescreen is-6-desktop is-6-tablet">
                    <div class="card card-lg match-height">
                        <div class="card-content">
                            <span class="media icon-text is-align-items-center">
                                <span class="icon mb-1 is-rounded is-large has-text-white has-background-primary media-left">
                                    <Icon icon_id={IconId::BootstrapMoonStars} />
                                </span>
                                <span class="media-content title m-1 is-3">{"Ideal Photo Times"}</span>
                            </span>
                            <div class="content">
                                <p class="text">{"Generate calendar events for predictable photo events such as moon/sunrise, golden hour, etc"}</p>
                            </div>
                        </div>
                        <footer class="card-footer has-text-centered">
                            <a href="#" class="has-background-primary has-text-white card-footer-item">{"Site"}</a>
                            <a href="#" class="card-footer-item">{"View Source"}</a>
                            <a href="#" class="card-footer-item">{"Blog Post"}</a>
                        </footer>
                    </div>
                </div>
            </div>
        </div>
        </section>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
