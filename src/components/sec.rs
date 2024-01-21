#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct SectionProps<'a> {
    sec_id: &'a str,
    header_text: Option<&'a str>,
    p_text: Option<&'a str>,
    outer_style: Option<&'a str>,
    inner_style: Option<&'a str>,
}

#[component]
pub fn Sec<'a>(cx: Scope<'a , SectionProps>) -> Element<'a> {
    render! {
        section {
                id: cx.props.sec_id,
                class: cx.props.outer_style ,
                div {
                    class: cx.props.inner_style,
                    h2 {
                        class: "text-2xl font-semibold",
                        {cx.props.header_text}
                    }
                    p {
                        {cx.props.p_text}
                    }
                }
            }
    }
}
