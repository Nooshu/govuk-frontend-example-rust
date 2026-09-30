//! Page shell HTML built from GOV.UK components (Askama layout wraps content).

use crate::govuk::{must_render, params, v_obj, v_safe, v_str, Value};
use crate::httpx::Assets;
use crate::session::SessionData;
use askama::Template;

pub struct Page<'a> {
    pub title: &'a str,
    pub content: String,
    pub back_href: Option<&'a str>,
    pub breadcrumbs: bool,
    pub sensitive: bool,
    pub welsh: bool,
    pub show_feedback: bool,
    pub assets: &'a Assets,
    pub session: &'a SessionData,
    pub return_path: &'a str,
}

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate<'a> {
    html_lang: &'a str,
    page_title: String,
    stylesheet_href: &'a str,
    app_module_href: &'a str,
    js_enabled_snippet: &'a str,
    skip_link: String,
    cookie_banner: String,
    header: String,
    service_nav: String,
    phase_banner: String,
    before_content: String,
    demo_banner: String,
    content: String,
    feedback: String,
    footer: String,
}

pub fn render_page(page: Page<'_>) -> String {
    let service_name = if page.welsh {
        crate::config::SERVICE_NAME_CY
    } else {
        crate::config::SERVICE_NAME
    };
    let homepage = if page.welsh { "/cy" } else { "/" };

    let skip = must_render(
        "skip-link",
        &params(&[
            ("href", v_str("#main-content")),
            (
                "text",
                v_str(if page.welsh {
                    "Neidio i'r prif gynnwys"
                } else {
                    "Skip to main content"
                }),
            ),
        ]),
    );

    let header = must_render("header", &params(&[("homepageUrl", v_str(homepage))]));

    let service_nav = must_render(
        "service-navigation",
        &params(&[
            ("serviceName", v_str(service_name)),
            ("serviceUrl", v_str(homepage)),
        ]),
    );

    let phase = must_render(
        "phase-banner",
        &params(&[
            (
                "tag",
                v_obj(params(&[(
                    "text",
                    v_str(if page.welsh { "Enghraifft" } else { "Example" }),
                )])),
            ),
            (
                "html",
                v_safe(if page.welsh {
                    r#"Mae hon yn arddangosiad – nid gwasanaeth llywodraeth byw mohono. Bydd eich <a class="govuk-link" href="/about">adborth</a> yn helpu i wella’r enghraifft."#
                } else {
                    r#"This is a demonstration – it is not a live government service. Your <a class="govuk-link" href="/about">feedback</a> will help us improve the example."#
                }),
            ),
        ]),
    );

    let demo_banner = must_render(
        "notification-banner",
        &params(&[
            (
                "titleText",
                v_str(if page.welsh { "Pwysig" } else { "Important" }),
            ),
            (
                "text",
                v_str(if page.welsh {
                    "Mae hwn yn arddangosiad byw. Nid gwasanaeth llywodraeth go iawn mohono."
                } else {
                    "This is a live demo. It is not a real government service."
                }),
            ),
            ("classes", v_str("app-demo-banner")),
            ("titleId", v_str("app-demo-banner-title")),
        ]),
    );

    let before = if let Some(href) = page.back_href {
        must_render(
            "back-link",
            &params(&[("href", v_str(href)), ("text", v_str("Back"))]),
        )
    } else if page.breadcrumbs {
        must_render(
            "breadcrumbs",
            &params(&[(
                "items",
                Value::Array(vec![
                    v_obj(params(&[("text", v_str("Home")), ("href", v_str("/"))])),
                    v_obj(params(&[("text", v_str(page.title))])),
                ]),
            )]),
        )
    } else {
        String::new()
    };

    let cookie_banner = if page.session.cookie_choice.is_none() {
        let banner = must_render(
            "cookie-banner",
            &params(&[(
                "messages",
                Value::Array(vec![v_obj(params(&[
                    (
                        "headingText",
                        v_str(format!("Cookies on {service_name}")),
                    ),
                    (
                        "text",
                        v_str(
                            "We use analytics cookies to understand how you use this example service. This example does not set analytics cookies.",
                        ),
                    ),
                    (
                        "actions",
                        Value::Array(vec![
                            v_obj(params(&[
                                ("text", v_str("Accept analytics cookies")),
                                ("type", v_str("submit")),
                                ("name", v_str("cookies")),
                                ("value", v_str("accept")),
                            ])),
                            v_obj(params(&[
                                ("text", v_str("Reject analytics cookies")),
                                ("type", v_str("submit")),
                                ("name", v_str("cookies")),
                                ("value", v_str("reject")),
                            ])),
                            v_obj(params(&[
                                ("text", v_str("View cookies")),
                                ("href", v_str("/cookies")),
                            ])),
                        ]),
                    ),
                ]))]),
            )]),
        );
        format!(
            r#"<form method="post" action="/cookie-choices" novalidate>
      <input type="hidden" name="csrf" value="{}">
      <input type="hidden" name="returnPath" value="{}">
      {banner}
    </form>"#,
            html_escape(&page.session.csrf),
            html_escape(page.return_path)
        )
    } else {
        String::new()
    };

    let mut footer_items = vec![
        footer_link("Help", "/help"),
        footer_link("Licence fees", "/fees"),
        footer_link("Service updates", "/updates"),
        footer_link("Guidance", "/guidance"),
        footer_link("Cookies", "/cookies"),
        footer_link("Accessibility", "/accessibility"),
        footer_link("About this example", "/about"),
    ];
    // Catalogue / examples links are always listed; demos routes may 404 when demos are off.
    footer_items.push(footer_link("Component catalogue", "/components"));
    footer_items.push(footer_link("Example pages", "/examples"));

    let footer = must_render(
        "footer",
        &params(&[(
            "meta",
            v_obj(params(&[("items", Value::Array(footer_items))])),
        )]),
    );

    let feedback = if page.show_feedback {
        must_render(
            "feedback",
            &params(&[
                ("titleText", v_str("Help us improve this service")),
                (
                    "html",
                    v_safe(
                        r#"<p class="govuk-body">This example does not send feedback. <a class="govuk-link" href="/help">Get help with this example</a>.</p>"#,
                    ),
                ),
            ]),
        )
    } else {
        String::new()
    };

    let lang = if page.welsh { "cy" } else { "en" };
    let page_title = if page.title == service_name {
        format!("{service_name} – GOV.UK")
    } else {
        format!("{} – {service_name} – GOV.UK", page.title)
    };

    LayoutTemplate {
        html_lang: lang,
        page_title,
        stylesheet_href: &page.assets.stylesheet_href,
        app_module_href: &page.assets.app_module_href,
        js_enabled_snippet: &page.assets.policy.js_enabled_snippet,
        skip_link: skip,
        cookie_banner,
        header,
        service_nav,
        phase_banner: phase,
        before_content: before,
        demo_banner,
        content: page.content,
        feedback,
        footer,
    }
    .render()
    .unwrap_or_else(|e| format!("<!-- template error: {e} -->"))
}

fn footer_link(text: &str, href: &str) -> Value {
    v_obj(params(&[("text", v_str(text)), ("href", v_str(href))]))
}

pub fn html_escape(s: &str) -> String {
    crate::govuk::escape(s)
}
