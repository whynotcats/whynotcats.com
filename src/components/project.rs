use crate::components::nav::NavIcon;
use web_sys::MouseEvent;
use yew::{
    classes, create_portal, function_component, html, use_state, AttrValue, Callback, Children,
    Classes, Html, Properties,
};

use super::icons::IconId;
use markdown;

#[derive(Properties, PartialEq)]
pub struct ProjectModalProps {
    #[prop_or(false)]
    pub is_active: bool,
    pub callback: Callback<MouseEvent>,
    pub title: AttrValue,
    pub summary: AttrValue,
    #[prop_or_default]
    pub site_url: AttrValue,
    #[prop_or_default]
    pub source_url: AttrValue,
    #[prop_or_default]
    pub blog_url: AttrValue,
    #[prop_or(true)]
    pub in_dev: bool,
}

#[function_component(ProjectModal)]
pub fn project_modal(props: &ProjectModalProps) -> Html {
    let active = match props.is_active {
        true => "is-active",
        false => "",
    };

    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();

    let md = markdown::to_html(&props.summary.clone());
    println!("{}", &md);
    create_portal(
        html! {
            <div class={classes!("modal", active)}>
                <div class="modal-background" onclick={props.callback.clone()}></div>
                    <div class="modal-card">
                        <header class="modal-card-head">
                        <p class="modal-card-title">{props.title.clone()}</p>
                        <button class="delete" onclick={props.callback.clone()} aria-label="close"></button>
                        </header>
                        <section class="modal-card-body">
                        <div class="content">{Html::from_html_unchecked(AttrValue::from(md))}</div>
                        </section>
                        <ProjectCardFooter
                            site_url={props.site_url.clone()}
                            source_url={props.source_url.clone()}
                            blog_url={props.blog_url.clone()}
                            in_dev={props.in_dev}
                            class={classes!("modal-card-foot")}
                    />
                    </div>
                <button class="modal-close is-large" onclick={props.callback.clone()} aria-label="close"></button>
            </div>
        },
        body.into(),
    )
}

#[derive(Properties, PartialEq)]
pub struct ProjectCardFooterProps {
    #[prop_or_default]
    pub site_url: AttrValue,
    #[prop_or_default]
    pub source_url: AttrValue,
    #[prop_or_default]
    pub blog_url: AttrValue,
    #[prop_or_default]
    pub in_dev: bool,
    pub class: Classes,
}

#[function_component(ProjectCardFooter)]
fn project_card_footer(props: &ProjectCardFooterProps) -> Html {
    // TODO: This is a hack to get the modal footer to look right
    let link_class = if props.class.contains("modal") {
        classes!("button")
    } else {
        classes!("card-footer-item")
    };

    html! {
        <footer class={props.class.clone()}>
            if !props.site_url.is_empty() {
                <a href={props.site_url.clone()} class={classes!("has-background-primary", "has-text-white", link_class.clone())}>{"Site"}</a>
            }
            if !props.source_url.is_empty() {
                <a href={props.source_url.clone()} class={link_class.clone()}>{"View Source"}</a>
            }
            if !props.blog_url.is_empty() {
                <a href={props.blog_url.clone()} class={link_class.clone()}>{"Blog Post"}</a>
            }
            if props.in_dev {
                <span class="card-footer-item">{"In Development"}</span>
            }
        </footer>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    #[prop_or_default]
    pub icon_id: IconId,
    pub name: AttrValue,
    #[prop_or_default]
    pub short_desc: AttrValue,
    #[prop_or_default]
    pub summary: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub site_url: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub source_url: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub blog_url: AttrValue,
    #[prop_or(false)]
    pub in_dev: bool,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let modal_state = use_state(|| false);
    let close_modal = {
        let state = modal_state.clone();

        Callback::from(move |_e: MouseEvent| {
            state.set(false);
        })
    };

    let open_modal = {
        let state = modal_state.clone();

        Callback::from(move |_e: MouseEvent| {
            state.set(true);
        })
    };

    html! {
        <div class="card block card-lg match-height">
            <div class="card-content" onclick={open_modal}>
                <NavIcon id={IconId::InfoFilled} class="mb-1 is-rounded is-small has-text-primary is-pulled-right" />
                <span class="media icon-text is-align-items-center">
                    <NavIcon class="mb-1 is-rounded is-large has-text-white has-background-primary media-left" id={props.icon_id} />
                    <span class="media-content title m-1 is-3">{props.name.clone()}</span>
                </span>
                <div class="content">
                    <p class="text">{props.short_desc.clone()}</p>
                </div>
            </div>
            <ProjectCardFooter class="card-footer has-text-centered"
                    site_url={props.site_url.clone()}
                    source_url={props.source_url.clone()}
                    blog_url={props.blog_url.clone()}
                    in_dev={props.in_dev}
            />
            <ProjectModal is_active={*modal_state}
                callback={close_modal}
                title={props.name.clone()}
                summary={props.summary.clone()}
                site_url={props.site_url.clone()}
                source_url={props.source_url.clone()}
                blog_url={props.blog_url.clone()}
                in_dev={props.in_dev}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectGalleryProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ProjectGallery)]
pub fn project_gallery(props: &ProjectGalleryProps) -> Html {
    html! {
        <section class="section pb-0">
            <div class="container">
                <h2 class="title has-text-centered-mobile has-text-centered-tablet has-text-left-desktop">{"Projects"}</h2>
                <div class="columns is-multiline">
                        {for props.children.iter()}
                </div>
            </div>
        </section>
    }
}
