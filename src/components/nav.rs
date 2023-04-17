use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};

use crate::components::icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct NavIconProps {
    pub id: IconId,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Properties, PartialEq)]
pub struct NavIconPropsWithText {
    pub id: IconId,
    pub text: AttrValue,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(NavIcon)]
pub fn icon(props: &NavIconProps) -> Html {
    html! {
        <span class={classes!("icon", props.class.clone())}>
            <Icon id={props.id} />
        </span>
    }
}

#[function_component(NavIconWithText)]
pub fn icon_with_text(props: &NavIconPropsWithText) -> Html {
    html! {
        <span class={classes!("icon-text", props.class.clone())}>
            <NavIcon id={props.id} />
            <span>{&props.text}</span>
        </span>
    }
}

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
    <nav class="navbar is-sticky-top navigation" role="navigation" aria-label="main navigation">
    <div class="container is-mobile">
        <div class="navbar-brand">
            <a class="navbar-item" href="/">
                <NavIconWithText class="has-background-primary has-text-white p-2" id={IconId::SolidCat} text={"Why Not Cats?"} />
            </a>
            <a class="navbar-item" href="https://blog.whynotcats.com">
                <NavIconWithText id={IconId::BlogIcon} text={"Blog"} />
            </a>
            <a href="https://github.com/whynotcats" class="navbar-item is-hidden-touch">
                <NavIconWithText id={IconId::Github} text={"Github"} />
            </a>

            <div class="navbar-item is-hidden-desktop is-justify-content-end is-flex-grow-1">
                <a target="_blank" rel="me" href="https://hachyderm.io/@whynotcats" class="navbar-item">
                    <NavIcon id={IconId::Mastodon} />
                </a>
                <a href="https://github.com/whynotcats" class="navbar-item">
                    <NavIconWithText class="is-hidden-touch" id={IconId::Github} text={"Github"} />
                    <NavIcon class="is-hidden-dekstop" id={IconId::Github} />
                </a>
                <a target="_blank" href="https://twitter.com/whynotcats" class="navbar-item">
                    <NavIcon id={IconId::Twitter} />
                </a>
                <a target="_blank" href="https://glass.photo/whynotcats" class="navbar-item">
                    <NavIcon id={IconId::Camera} />
                </a>
            </div>
        </div>

        <div class="navbar-end">
            <a target="_blank" href="https://twitter.com/whynotcats" class="navbar-item is-hidden-touch">
                <NavIcon id={IconId::Twitter} />
            </a>
            <a target="_blank" href="https://glass.photo/whynotcats" class="navbar-item is-hidden-touch">
                <NavIcon id={IconId::Camera} />
            </a>
            <a target="_blank" rel="me" href="https://hachyderm.io/@whynotcats" class="navbar-item is-hidden-touch">
                <NavIcon id={IconId::Mastodon} />
            </a>
        </div>
    </div>
    </nav>
    }
}
