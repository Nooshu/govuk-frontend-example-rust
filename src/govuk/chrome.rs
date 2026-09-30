//! Chrome components (header, footer, navigation, …).

use crate::govuk::attributes::{
    attribute_if, attributes, classes_if, content, content_indent, flag_if,
};
use crate::govuk::button::render_button;
use crate::govuk::escape::{
    def, def_truthy, escape, get, indent, items, length, loose_eq, out, str_val, trim, truthy,
};
use crate::govuk::params::{v_bool, v_safe, v_str, Params, Value};

const LOGO_CROWN: &str = r#"    <g>
      <circle cx="20" cy="17.6" r="3.7"/>
      <circle cx="10.2" cy="23.5" r="3.7"/>
      <circle cx="3.7" cy="33.2" r="3.7"/>
      <circle cx="31.7" cy="30.6" r="3.7"/>
      <circle cx="43.3" cy="17.6" r="3.7"/>
      <circle cx="53.2" cy="23.5" r="3.7"/>
      <circle cx="59.7" cy="33.2" r="3.7"/>
      <circle cx="31.7" cy="30.6" r="3.7"/>
      <path d="M33.1,9.8c.2-.1.3-.3.5-.5l4.6,2.4v-6.8l-4.6,1.5c-.1-.2-.3-.3-.5-.5l1.9-5.9h-6.7l1.9,5.9c-.2.1-.3.3-.5.5l-4.6-1.5v6.8l4.6-2.4c.1.2.3.3.5.5l-2.6,8c-.9,2.8,1.2,5.7,4.1,5.7h0c3,0,5.1-2.9,4.1-5.7l-2.6-8ZM37,37.9s-3.4,3.8-4.1,6.1c2.2,0,4.2-.5,6.4-2.8l-.7,8.5c-2-2.8-4.4-4.1-5.7-3.8.1,3.1.5,6.7,5.8,7.2,3.7.3,6.7-1.5,7-3.8.4-2.6-2-4.3-3.7-1.6-1.4-4.5,2.4-6.1,4.9-3.2-1.9-4.5-1.8-7.7,2.4-10.9,3,4,2.6,7.3-1.2,11.1,2.4-1.3,6.2,0,4,4.6-1.2-2.8-3.7-2.2-4.2.2-.3,1.7.7,3.7,3,4.2,1.9.3,4.7-.9,7-5.9-1.3,0-2.4.7-3.9,1.7l2.4-8c.6,2.3,1.4,3.7,2.2,4.5.6-1.6.5-2.8,0-5.3l5,1.8c-2.6,3.6-5.2,8.7-7.3,17.5-7.4-1.1-15.7-1.7-24.5-1.7h0c-8.8,0-17.1.6-24.5,1.7-2.1-8.9-4.7-13.9-7.3-17.5l5-1.8c-.5,2.5-.6,3.7,0,5.3.8-.8,1.6-2.3,2.2-4.5l2.4,8c-1.5-1-2.6-1.7-3.9-1.7,2.3,5,5.2,6.2,7,5.9,2.3-.4,3.3-2.4,3-4.2-.5-2.4-3-3.1-4.2-.2-2.2-4.6,1.6-6,4-4.6-3.7-3.7-4.2-7.1-1.2-11.1,4.2,3.2,4.3,6.4,2.4,10.9,2.5-2.8,6.3-1.3,4.9,3.2-1.8-2.7-4.1-1-3.7,1.6.3,2.3,3.3,4.1,7,3.8,5.4-.5,5.7-4.2,5.8-7.2-1.3-.2-3.7,1-5.7,3.8l-.7-8.5c2.2,2.3,4.2,2.7,6.4,2.8-.7-2.3-4.1-6.1-4.1-6.1h10.6,0Z"/>
    </g>"#;

const LOGO_LOGOTYPE: &str = r#"    <circle class="govuk-logo-dot" cx="226" cy="36" r="7.3"/>
    <path d="M93.94 41.25c.4 1.81 1.2 3.21 2.21 4.62 1 1.4 2.21 2.41 3.61 3.21s3.21 1.2 5.22 1.2 3.61-.4 4.82-1c1.4-.6 2.41-1.4 3.21-2.41.8-1 1.4-2.01 1.61-3.01s.4-2.01.4-3.01v.14h-10.86v-7.02h20.07v24.08h-8.03v-5.56c-.6.8-1.38 1.61-2.19 2.41-.8.8-1.81 1.2-2.81 1.81-1 .4-2.21.8-3.41 1.2s-2.41.4-3.81.4a18.56 18.56 0 0 1-14.65-6.63c-1.6-2.01-3.01-4.41-3.81-7.02s-1.4-5.62-1.4-8.83.4-6.02 1.4-8.83a20.45 20.45 0 0 1 19.46-13.65c3.21 0 4.01.2 5.82.8 1.81.4 3.61 1.2 5.02 2.01 1.61.8 2.81 2.01 4.01 3.21s2.21 2.61 2.81 4.21l-7.63 4.41c-.4-1-1-1.81-1.61-2.61-.6-.8-1.4-1.4-2.21-2.01-.8-.6-1.81-1-2.81-1.4-1-.4-2.21-.4-3.61-.4-2.01 0-3.81.4-5.22 1.2-1.4.8-2.61 1.81-3.61 3.21s-1.61 2.81-2.21 4.62c-.4 1.81-.6 3.71-.6 5.42s.8 5.22.8 5.22Zm57.8-27.9c3.21 0 6.22.6 8.63 1.81 2.41 1.2 4.82 2.81 6.62 4.82S170.2 24.39 171 27s1.4 5.62 1.4 8.83-.4 6.02-1.4 8.83-2.41 5.02-4.01 7.02-4.01 3.61-6.62 4.82-5.42 1.81-8.63 1.81-6.22-.6-8.63-1.81-4.82-2.81-6.42-4.82-3.21-4.41-4.01-7.02-1.4-5.62-1.4-8.83.4-6.02 1.4-8.83 2.41-5.02 4.01-7.02 4.01-3.61 6.42-4.82 5.42-1.81 8.63-1.81Zm0 36.73c1.81 0 3.61-.4 5.02-1s2.61-1.81 3.61-3.01 1.81-2.81 2.21-4.41c.4-1.81.8-3.61.8-5.62 0-2.21-.2-4.21-.8-6.02s-1.2-3.21-2.21-4.62c-1-1.2-2.21-2.21-3.61-3.01s-3.21-1-5.02-1-3.61.4-5.02 1c-1.4.8-2.61 1.81-3.61 3.01s-1.81 2.81-2.21 4.62c-.4 1.81-.8 3.61-.8 5.62 0 2.41.2 4.21.8 6.02.4 1.81 1.2 3.21 2.21 4.41s2.21 2.21 3.61 3.01c1.4.8 3.21 1 5.02 1Zm36.32 7.96-12.24-44.15h9.83l8.43 32.77h.4l8.23-32.77h9.83L200.3 58.04h-12.24Zm74.14-7.96c2.18 0 3.51-.6 3.51-.6 1.2-.6 2.01-1 2.81-1.81s1.4-1.81 1.81-2.81a13 13 0 0 0 .8-4.01V13.9h8.63v28.15c0 2.41-.4 4.62-1.4 6.62-.8 2.01-2.21 3.61-3.61 5.02s-3.41 2.41-5.62 3.21-4.62 1.2-7.02 1.2-5.02-.4-7.02-1.2c-2.21-.8-4.01-1.81-5.62-3.21s-2.81-3.01-3.61-5.02-1.4-4.21-1.4-6.62V13.9h8.63v26.95c0 1.61.2 3.01.8 4.01.4 1.2 1.2 2.21 2.01 2.81.8.8 1.81 1.4 2.81 1.81 0 0 1.34.6 3.51.6Zm34.22-36.18v18.92l15.65-18.92h10.82l-15.03 17.32 16.03 26.83h-10.21l-11.44-20.21-5.62 6.22v13.99h-8.83V13.9"/>"#;

const FOOTER_LICENCE_LOGO: &str = r#"<svg
            aria-hidden="true"
            focusable="false"
            class="govuk-footer__licence-logo"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 483.2 195.7"
            height="17"
            width="41"
          >
            <path
              fill="currentColor"
              d="M421.5 142.8V.1l-50.7 32.3v161.1h112.4v-50.7zm-122.3-9.6A47.12 47.12 0 0 1 221 97.8c0-26 21.1-47.1 47.1-47.1 16.7 0 31.4 8.7 39.7 21.8l42.7-27.2A97.63 97.63 0 0 0 268.1 0c-36.5 0-68.3 20.1-85.1 49.7A98 98 0 0 0 97.8 0C43.9 0 0 43.9 0 97.8s43.9 97.8 97.8 97.8c36.5 0 68.3-20.1 85.1-49.7a97.76 97.76 0 0 0 149.6 25.4l19.4 22.2h3v-87.8h-80l24.3 27.5zM97.8 145c-26 0-47.1-21.1-47.1-47.1s21.1-47.1 47.1-47.1 47.2 21 47.2 47S123.8 145 97.8 145"
            />
          </svg>"#;

const FOOTER_COPYRIGHT_HREF: &str = "https://www.nationalarchives.gov.uk/information-management/re-using-public-sector-information/uk-government-licensing-framework/crown-copyright/";

const PAGINATION_ARROW_PREVIOUS: &str = r#"  <svg class="govuk-pagination__icon govuk-pagination__icon--prev" xmlns="http://www.w3.org/2000/svg" height="13" width="15" aria-hidden="true" focusable="false" viewBox="0 0 15 13">
    <path d="m6.5938-0.0078125-6.7266 6.7266 6.7441 6.4062 1.377-1.449-4.1856-3.9768h12.896v-2h-12.984l4.2931-4.293-1.414-1.414z"></path>
  </svg>"#;

const PAGINATION_ARROW_NEXT: &str = r#"  <svg class="govuk-pagination__icon govuk-pagination__icon--next" xmlns="http://www.w3.org/2000/svg" height="13" width="15" aria-hidden="true" focusable="false" viewBox="0 0 15 13">
    <path d="m8.107-0.0078125-1.4136 1.414 4.2926 4.293h-12.986v2h12.896l-4.1855 3.9766 1.377 1.4492 6.7441-6.4062-6.7246-6.7266z"></path>
  </svg>"#;

fn render_logo(p: &Params) -> String {
    let true_v = v_bool(true);
    let use_logotype = truthy(def(p.get("useLogotype"), &true_v));
    let (width, view_box) = if use_logotype {
        ("162", "324")
    } else {
        ("32", "64")
    };
    let aria_label = p.get("ariaLabelText");
    let role = if truthy(aria_label) {
        "img"
    } else {
        "presentation"
    };
    let mut out_s = String::new();
    out_s.push_str("\n  <svg\n    focusable=\"false\"\n    role=\"");
    out_s.push_str(role);
    out_s.push_str("\"\n    xmlns=\"http://www.w3.org/2000/svg\"\n");
    out_s.push_str(&format!(
        "    viewBox=\"0 0 {view_box} 60\"\n    height=\"30\"\n    width=\"{width}\"\n"
    ));
    out_s.push_str("    fill=\"currentcolor\"");
    out_s.push_str(&attribute_if("class", p.get("classes")));
    out_s.push_str(&attribute_if("aria-label", aria_label));
    out_s.push_str(&attributes(p.get("attributes")));
    out_s.push_str("\n  >");
    if truthy(aria_label) {
        out_s.push_str(&format!("<title>{}</title>", out(aria_label)));
    }
    out_s.push_str("    ");
    out_s.push_str(&indent(&trim(LOGO_CROWN), 2, false));
    out_s.push('\n');
    if use_logotype {
        out_s.push_str("      ");
        out_s.push_str(&indent(&trim(LOGO_LOGOTYPE), 2, false));
        out_s.push('\n');
    }
    out_s.push_str("  </svg>\n");
    out_s
}

pub fn render_generic_header(p: &Params) -> String {
    let ns_fb = v_str("govuk-generic");
    let namespace = out(def(p.get("_namespace"), &ns_fb));
    let container_fb = v_str("govuk-width-container");
    let url_fb = v_str("/");
    format!(
        r#"<div class="{namespace}-header{}"{}>
  <div class="{namespace}-header__container {}">
    <div class="{namespace}-header__logo">
      <a href="{}" class="{namespace}-header__homepage-link">
        {}
      </a>
    </div>
  </div>
</div>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        out(def_truthy(p.get("containerClasses"), &container_fb)),
        out(def_truthy(p.get("url"), &url_fb)),
        crate::govuk::attributes::content_params(p, "logoHtml", "logoText")
    )
}

pub fn render_header(p: &Params) -> String {
    let logo = render_logo(&Params::from_pairs(&[
        ("classes", v_str("govuk-header__logotype")),
        ("ariaLabelText", v_str("GOV.UK")),
    ]));
    let mut logo_content = format!("  {}\n", trim(&logo));
    let product_name = p.get("productName");
    if truthy(product_name) {
        logo_content.push_str(&format!(
            r#"<span class="govuk-header__product-name">{}</span>"#,
            out(product_name)
        ));
    }
    let homepage = v_str("//gov.uk");
    render_generic_header(&Params::from_pairs(&[
        ("_namespace", v_str("govuk")),
        ("logoHtml", v_safe(indent(&logo_content, 8, false))),
        ("url", def_truthy(p.get("homepageUrl"), &homepage).clone()),
        ("containerClasses", p.get("containerClasses").clone()),
        ("classes", p.get("classes").clone()),
        ("attributes", p.get("attributes").clone()),
    ]))
}

pub fn render_footer(p: &Params) -> String {
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-footer{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');
    out_s.push_str(&format!(
        r#"  <div class="govuk-width-container{}">"#,
        classes_if(p.get("containerClasses"))
    ));
    out_s.push_str(&render_logo(&Params::from_pairs(&[
        ("classes", v_str("govuk-footer__crown")),
        ("useLogotype", v_bool(false)),
    ])));
    out_s.push('\n');

    let navigation = items(p.get("navigation"));
    if !navigation.is_empty() {
        out_s.push_str("      <div class=\"govuk-footer__navigation\">\n");
        for nav in navigation {
            let full = v_str("full");
            out_s.push_str(&format!(
                r#"          <div class="govuk-footer__section govuk-grid-column-{}">"#,
                out(def_truthy(get(nav, &["width"]), &full))
            ));
            out_s.push('\n');
            out_s.push_str(&format!(
                r#"            <h2 class="govuk-footer__heading govuk-heading-m">{}</h2>"#,
                out(get(nav, &["title"]))
            ));
            out_s.push('\n');
            let links = items(get(nav, &["items"]));
            if !links.is_empty() {
                let mut list_classes = String::new();
                let columns = get(nav, &["columns"]);
                if truthy(columns) {
                    list_classes =
                        format!(" govuk-footer__list--columns-{}", escape(&str_val(columns)));
                }
                out_s.push_str(&format!(
                    r#"              <ul class="govuk-footer__list{list_classes}">"#
                ));
                out_s.push('\n');
                for link in links {
                    if !truthy(get(link, &["href"])) || !truthy(get(link, &["text"])) {
                        continue;
                    }
                    out_s.push_str("                    <li class=\"govuk-footer__list-item\">\n");
                    out_s.push_str(&format!(
                        r#"                      <a class="govuk-footer__link" href="{}"{}>"#,
                        out(get(link, &["href"])),
                        attributes(get(link, &["attributes"]))
                    ));
                    out_s.push('\n');
                    out_s.push_str(&format!(
                        "                        {}\n",
                        out(get(link, &["text"]))
                    ));
                    out_s.push_str("                      </a>\n                    </li>\n");
                }
                out_s.push_str("              </ul>\n");
            }
            out_s.push_str("          </div>\n");
        }
        out_s.push_str("      </div>\n");
        out_s.push_str("      <hr class=\"govuk-footer__section-break\">\n");
    }

    out_s.push_str("    <div class=\"govuk-footer__meta\">\n");
    out_s.push_str("      <div class=\"govuk-footer__meta-item govuk-footer__meta-item--grow\">\n");

    let meta = p.get("meta");
    if truthy(meta) {
        let support = v_str("Support links");
        out_s.push_str(&format!(
            r#"        <h2 class="govuk-visually-hidden">{}</h2>"#,
            out(def_truthy(get(meta, &["visuallyHiddenTitle"]), &support))
        ));
        out_s.push('\n');
        let links = items(get(meta, &["items"]));
        if !links.is_empty() {
            out_s.push_str("        <ul class=\"govuk-footer__inline-list\">\n");
            for link in links {
                out_s.push_str("          <li class=\"govuk-footer__inline-list-item\">\n");
                out_s.push_str(&format!(
                    r#"            <a class="govuk-footer__link" href="{}"{}>"#,
                    out(get(link, &["href"])),
                    attributes(get(link, &["attributes"]))
                ));
                out_s.push('\n');
                out_s.push_str(&format!("              {}\n", out(get(link, &["text"]))));
                out_s.push_str("            </a>\n          </li>\n");
            }
            out_s.push_str("        </ul>\n");
        }
        if truthy(get(meta, &["text"])) || truthy(get(meta, &["html"])) {
            out_s.push_str("        <div class=\"govuk-footer__meta-custom\">\n");
            out_s.push_str("          ");
            out_s.push_str(&content_indent(meta, "html", "text", 10));
            out_s.push_str("\n        </div>\n");
        }
    }

    // Explicit null skips licence; missing/undefined still shows it.
    let licence = p.get("contentLicence");
    if !matches!(licence, Value::Null) {
        out_s.push_str("          ");
        out_s.push_str(FOOTER_LICENCE_LOGO);
        out_s.push('\n');
        out_s.push_str("          <span class=\"govuk-footer__licence-description\">\n");
        if truthy(get(licence, &["html"])) || truthy(get(licence, &["text"])) {
            out_s.push_str("            ");
            out_s.push_str(&content_indent(licence, "html", "text", 12));
            out_s.push('\n');
        } else {
            out_s.push_str(
                "            All content is available under the\n            <a\n              class=\"govuk-footer__link\"\n              href=\"https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/\"\n              rel=\"license\"\n            >Open Government Licence v3.0</a>, except where otherwise stated\n",
            );
        }
        out_s.push_str("          </span>\n");
    }

    out_s.push_str("      </div>\n");
    out_s.push_str("      <div class=\"govuk-footer__meta-item\">\n");
    out_s.push_str(&format!(
        "        <a\n          class=\"govuk-footer__link govuk-footer__copyright-logo\"\n          href=\"{FOOTER_COPYRIGHT_HREF}\"\n        >\n"
    ));
    let copyright = p.get("copyright");
    if truthy(get(copyright, &["html"])) || truthy(get(copyright, &["text"])) {
        out_s.push_str("          ");
        out_s.push_str(&content_indent(copyright, "html", "text", 10));
        out_s.push('\n');
    } else {
        out_s.push_str("          \u{00a9} Crown copyright\n");
    }
    out_s.push_str("        </a>\n      </div>\n");
    out_s.push_str("    </div>\n  </div>\n</div>");
    out_s
}

pub fn render_breadcrumbs(p: &Params) -> String {
    let mut class_names = String::from("govuk-breadcrumbs");
    let classes = p.get("classes");
    if truthy(classes) {
        class_names.push(' ');
        class_names.push_str(&str_val(classes));
    }
    if truthy(p.get("collapseOnMobile")) {
        class_names.push_str(" govuk-breadcrumbs--collapse-on-mobile");
    }
    let crumb = v_str("Breadcrumb");
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<nav class="{}"{} aria-label="{}">"#,
        escape(&class_names),
        attributes(p.get("attributes")),
        out(def(p.get("labelText"), &crumb))
    ));
    out_s.push('\n');
    out_s.push_str("  <ol class=\"govuk-breadcrumbs__list\">\n");
    for item in items(p.get("items")) {
        let href = get(item, &["href"]);
        if truthy(href) {
            out_s.push_str("    <li class=\"govuk-breadcrumbs__list-item\">\n");
            out_s.push_str(&format!(
                r#"      <a class="govuk-breadcrumbs__link" href="{}"{}>{}</a>"#,
                out(href),
                attributes(get(item, &["attributes"])),
                content(item, "html", "text")
            ));
            out_s.push_str("\n    </li>\n");
        } else {
            out_s.push_str(&format!(
                r#"    <li class="govuk-breadcrumbs__list-item" aria-current="page">{}</li>"#,
                content(item, "html", "text")
            ));
            out_s.push('\n');
        }
    }
    out_s.push_str("  </ol>\n</nav>");
    out_s
}

pub fn render_language_navigation(p: &Params) -> String {
    let lang = v_str("Language");
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<nav class="govuk-language-navigation{}"{} aria-label="{}">"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        out(def(p.get("ariaLabel"), &lang))
    ));
    out_s.push('\n');
    out_s.push_str("  <ul class=\"govuk-language-navigation__list\">\n");
    for item in items(p.get("items")) {
        let href = get(item, &["href"]);
        out_s.push_str("    <li class=\"govuk-language-navigation__list-item\">\n");
        if truthy(get(item, &["current"])) || !truthy(href) {
            out_s.push_str(&format!(
                r#"      <span class="govuk-language-navigation__text{}""#,
                classes_if(get(item, &["classes"]))
            ));
            out_s.push_str("\n        aria-current=\"true\"");
            out_s.push_str(&attribute_if("lang", get(item, &["lang"])));
            out_s.push_str(&attribute_if("dir", get(item, &["dir"])));
            out_s.push_str(&attributes(get(item, &["attributes"])));
            out_s.push('>');
            out_s.push_str(&content(item, "html", "text"));
            out_s.push_str("</span>\n");
        } else {
            let mut href_lang = get(item, &["hrefLang"]).clone();
            if !truthy(&href_lang) {
                href_lang = get(item, &["lang"]).clone();
            }
            out_s.push_str(&format!(
                r#"      <a class="govuk-language-navigation__link{}" href="{}" rel="alternate"{}{}{}{}>"#,
                classes_if(get(item, &["classes"])),
                out(href),
                attribute_if("lang", get(item, &["lang"])),
                attribute_if("hreflang", &href_lang),
                attribute_if("dir", get(item, &["dir"])),
                attributes(get(item, &["attributes"]))
            ));
            out_s.push_str(&content(item, "html", "text"));
            let description = get(item, &["languageDescriptionText"]);
            if truthy(description) {
                out_s.push_str(&format!(
                    r#"<span class="govuk-visually-hidden"> {}</span>"#,
                    out(description)
                ));
            }
            out_s.push_str("      </a>\n");
        }
        out_s.push_str("    </li>\n");
    }
    out_s.push_str("  </ul>\n</nav>");
    out_s
}

pub fn render_service_navigation(p: &Params) -> String {
    let slots = p.get("slots");
    let menu_default = Value::String("Menu".into());
    let menu_button_text = def_truthy(p.get("menuButtonText"), &menu_default).clone();
    let nav_id_default = Value::String("navigation".into());
    let navigation_id = out(def_truthy(p.get("navigationId"), &nav_id_default));

    let end_slot = get(slots, &["end"]);
    let end_slot_html = match end_slot {
        Value::String(_) | Value::Safe(_) => end_slot.clone(),
        Value::Object(_) => get(end_slot, &["html"]).clone(),
        _ => Value::Undefined,
    };
    let end_slot_inline = match end_slot {
        Value::Object(obj) => loose_eq(obj.get("align"), &Value::String("inline".into())),
        _ => false,
    };

    let common_attributes = format!(
        "class=\"govuk-service-navigation{}\"\ndata-module=\"govuk-service-navigation\"{}\n",
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
    );

    let mut inner = String::new();
    inner.push_str(&format!(
        r#"  <div class="govuk-width-container{}">"#,
        flag_if(
            " govuk-service-navigation__inlining-container",
            &Value::Bool(end_slot_inline)
        ),
    ));
    inner.push_str("\n\n    ");
    let start = get(slots, &["start"]);
    if truthy(start) {
        inner.push_str(&str_val(start));
    }
    inner.push_str("<div class=\"govuk-service-navigation__container\">\n      \n");

    let service_name = p.get("serviceName");
    if truthy(service_name) {
        inner.push_str("        <span class=\"govuk-service-navigation__service-name\">\n");
        let service_url = p.get("serviceUrl");
        if truthy(service_url) {
            inner.push_str(&format!(
                r#"            <a href="{}" class="govuk-service-navigation__link">"#,
                out(service_url)
            ));
            inner.push('\n');
            inner.push_str("              ");
            inner.push_str(&out(service_name));
            inner.push_str("\n            </a>\n");
        } else {
            inner.push_str(&format!(
                r#"            <span class="govuk-service-navigation__text">{}</span>"#,
                out(service_name)
            ));
            inner.push('\n');
        }
        inner.push_str("        </span>\n");
    }
    inner.push_str("\n      \n");

    let mut navigation_items = Vec::new();
    for item in items(p.get("navigation")) {
        if truthy(item) {
            navigation_items.push(item);
        }
    }
    let collapse_default = Value::Bool(navigation_items.len() > 1);
    let collapse = truthy(def(p.get("collapseNavigationOnMobile"), &collapse_default));

    let navigation_start = get(slots, &["navigationStart"]);
    let navigation_end = get(slots, &["navigationEnd"]);
    if !navigation_items.is_empty() || truthy(navigation_start) || truthy(navigation_end) {
        inner.push_str(&format!(
            r#"        <nav aria-label="{}" class="govuk-service-navigation__wrapper{}">"#,
            out(def_truthy(p.get("navigationLabel"), &menu_button_text)),
            classes_if(p.get("navigationClasses")),
        ));
        inner.push('\n');
        if collapse {
            let menu_button_label = p.get("menuButtonLabel");
            let mut aria_label = String::new();
            if truthy(menu_button_label) && !loose_eq(menu_button_label, &menu_button_text) {
                aria_label = format!(r#" aria-label="{}""#, out(menu_button_label));
            }
            inner.push_str(&format!(
                r#"          <button type="button" class="govuk-service-navigation__toggle govuk-js-service-navigation-toggle" aria-controls="{}"{} hidden aria-hidden="true">"#,
                navigation_id, aria_label
            ));
            inner.push('\n');
            inner.push_str("            ");
            inner.push_str(&out(&menu_button_text));
            inner.push_str("\n          </button>\n");
        }
        inner.push_str(&format!(
            "\n          <ul class=\"govuk-service-navigation__list\" id=\"{navigation_id}\" >\n\n            "
        ));
        if truthy(navigation_start) {
            inner.push_str(&str_val(navigation_start));
        }
        inner.push('\n');

        for item in &navigation_items {
            let active = truthy(get(item, &["active"])) || truthy(get(item, &["current"]));
            let link_inner = if active {
                format!(
                    "\n                                    \n                  <strong class=\"govuk-service-navigation__active-fallback\">{}</strong>\n",
                    content(item, "html", "text")
                )
            } else {
                format!(
                    "\n                                    \n{}",
                    content(item, "html", "text")
                )
            };

            let mut aria_current = String::new();
            if active {
                let value = if truthy(get(item, &["current"])) {
                    "page"
                } else {
                    "true"
                };
                aria_current = format!(r#" aria-current="{value}""#);
            }

            inner.push_str("              \n");
            inner.push_str(&format!(
                r#"              <li class="govuk-service-navigation__item{}">"#,
                flag_if(
                    " govuk-service-navigation__item--active",
                    &Value::Bool(active)
                ),
            ));
            inner.push('\n');
            let href = get(item, &["href"]);
            if truthy(href) {
                inner.push_str(&format!(
                    r#"                  <a class="govuk-service-navigation__link" href="{}"{}{}>{}
                  </a>
"#,
                    out(href),
                    aria_current,
                    attributes(get(item, &["attributes"])),
                    link_inner,
                ));
            } else if truthy(get(item, &["html"])) || truthy(get(item, &["text"])) {
                inner.push_str(&format!(
                    r#"                  <span class="govuk-service-navigation__text"{}>{}
                  </span>
"#,
                    aria_current, link_inner,
                ));
            }
            inner.push_str("              </li>\n\n");
        }

        inner.push_str("            ");
        if truthy(navigation_end) {
            inner.push_str(&str_val(navigation_end));
        }
        inner.push_str("</ul>\n        </nav>\n");
    }

    inner.push_str("    </div>\n\n    ");
    if truthy(&end_slot_html) {
        inner.push_str(&str_val(&end_slot_html));
    }
    inner.push_str("</div>\n");

    let service_info = Value::String("Service information".into());
    if truthy(p.get("serviceName")) || truthy(get(slots, &["start"])) || truthy(&end_slot_html) {
        format!(
            "  <section aria-label=\"{}\" {}>\n    {}\n  </section>\n",
            out(def(p.get("ariaLabel"), &service_info)),
            common_attributes,
            inner,
        )
    } else {
        format!("  <div {common_attributes}>\n    {inner}\n  </div>\n")
    }
}

pub fn render_pagination(p: &Params) -> String {
    let previous = p.get("previous");
    let next = p.get("next");
    let block_level = !truthy(p.get("items")) && (truthy(next) || truthy(previous));

    let landmark = Value::String("Pagination".into());
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<nav class="govuk-pagination{}{}" aria-label="{}"{}>"#,
        flag_if(" govuk-pagination--block", &Value::Bool(block_level)),
        classes_if(p.get("classes")),
        out(def_truthy(p.get("landmarkLabel"), &landmark)),
        attributes(p.get("attributes")),
    ));
    out_s.push('\n');

    if truthy(previous) && truthy(get(previous, &["href"])) {
        out_s.push_str(&pagination_arrow_link(
            previous,
            "prev",
            block_level,
            &pagination_link_label(previous, "Previous"),
        ));
    }

    let entries = p.get("items");
    if truthy(entries) {
        out_s.push_str("  <ul class=\"govuk-pagination__list\">\n");
        for item in items(entries) {
            if matches!(item, Value::Null | Value::Undefined) || length(item) == 0 {
                continue;
            }
            out_s.push_str("      ");
            out_s.push_str(&indent(&pagination_page_item(item), 2, false));
            out_s.push('\n');
        }
        out_s.push_str("  </ul>\n");
    }

    if truthy(next) && truthy(get(next, &["href"])) {
        out_s.push_str(&pagination_arrow_link(
            next,
            "next",
            block_level,
            &pagination_link_label(next, "Next"),
        ));
    }

    out_s.push_str("</nav>");
    out_s
}

fn pagination_link_label(link: &Value, fallback: &str) -> String {
    let html = get(link, &["html"]);
    let text = get(link, &["text"]);
    if truthy(html) {
        trim(&indent(&trim(&str_val(html)), 8, false))
    } else if truthy(text) {
        out(text)
    } else {
        format!("{fallback}<span class=\"govuk-visually-hidden\"> page</span>")
    }
}

fn pagination_arrow_link(link: &Value, kind: &str, block_level: bool, label: &str) -> String {
    let arrow = if kind == "prev" {
        PAGINATION_ARROW_PREVIOUS
    } else {
        PAGINATION_ARROW_NEXT
    };

    let mut out_s = String::new();
    out_s.push_str(&format!(r#"  <div class="govuk-pagination__{kind}">"#));
    out_s.push('\n');
    out_s.push_str(&format!(
        r#"    <a class="govuk-link govuk-pagination__link" href="{}" rel="{}"{}>"#,
        out(get(link, &["href"])),
        kind,
        attributes(get(link, &["attributes"])),
    ));
    out_s.push('\n');
    if block_level || kind == "prev" {
        out_s.push_str(&indent(arrow, 4, true));
        out_s.push('\n');
    }
    let label_text = get(link, &["labelText"]);
    out_s.push_str(&format!(
        r#"      <span class="govuk-pagination__link-title{}">"#,
        flag_if(
            " govuk-pagination__link-title--decorated",
            &Value::Bool(block_level && !truthy(label_text)),
        ),
    ));
    out_s.push('\n');
    out_s.push_str("        ");
    out_s.push_str(label);
    out_s.push_str("\n      </span>\n");
    if truthy(label_text) && block_level {
        out_s.push_str("      <span class=\"govuk-visually-hidden\">:</span>\n");
        out_s.push_str(&format!(
            r#"      <span class="govuk-pagination__link-label">{}</span>"#,
            out(label_text)
        ));
        out_s.push('\n');
    }
    if !block_level && kind == "next" {
        out_s.push_str(&indent(arrow, 4, true));
        out_s.push('\n');
    }
    out_s.push_str("    </a>\n  </div>\n");
    out_s
}

fn pagination_page_item(item: &Value) -> String {
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<li class="govuk-pagination__item{}{}">"#,
        flag_if(" govuk-pagination__item--current", get(item, &["current"])),
        flag_if(
            " govuk-pagination__item--ellipsis",
            get(item, &["ellipsis"])
        ),
    ));
    out_s.push('\n');
    if truthy(get(item, &["ellipsis"])) {
        out_s.push_str("    &ctdot;\n");
    } else {
        let page_fallback = Value::String(format!("Page {}", str_val(get(item, &["number"]))));
        out_s.push_str(&format!(
            r#"    <a class="govuk-link govuk-pagination__link" href="{}" aria-label="{}"{}{}>"#,
            out(get(item, &["href"])),
            out(def(get(item, &["visuallyHiddenText"]), &page_fallback)),
            flag_if(r#" aria-current="page""#, get(item, &["current"])),
            attributes(get(item, &["attributes"])),
        ));
        out_s.push('\n');
        out_s.push_str("      ");
        out_s.push_str(&out(get(item, &["number"])));
        out_s.push_str("\n    </a>\n");
    }
    out_s.push_str("  </li>");
    out_s
}

pub fn render_cookie_banner(p: &Params) -> String {
    let aria_default = Value::String("Cookie banner".into());
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-cookie-banner{}" data-nosnippet role="region" aria-label="{}"{}{}>"#,
        classes_if(p.get("classes")),
        out(def_truthy(p.get("ariaLabel"), &aria_default)),
        flag_if(" hidden", p.get("hidden")),
        attributes(p.get("attributes")),
    ));
    out_s.push('\n');

    for message in items(p.get("messages")) {
        out_s.push_str(&format!(
            r#"  <div class="govuk-cookie-banner__message{} govuk-width-container"{}{}{}>"#,
            classes_if(get(message, &["classes"])),
            attribute_if("role", get(message, &["role"])),
            attributes(get(message, &["attributes"])),
            flag_if(" hidden", get(message, &["hidden"])),
        ));
        out_s.push_str("\n\n");
        out_s.push_str("    <div class=\"govuk-grid-row\">\n");
        out_s.push_str("      <div class=\"govuk-grid-column-two-thirds\">\n");
        if truthy(get(message, &["headingHtml"])) || truthy(get(message, &["headingText"])) {
            out_s.push_str("        <h2 class=\"govuk-cookie-banner__heading govuk-heading-m\">\n");
            out_s.push_str("          ");
            out_s.push_str(&content_indent(message, "headingHtml", "headingText", 10));
            out_s.push('\n');
            out_s.push_str("        </h2>\n");
        }
        out_s.push_str("        <div class=\"govuk-cookie-banner__content\">\n");
        let html = get(message, &["html"]);
        let text = get(message, &["text"]);
        if truthy(html) {
            out_s.push_str("          ");
            out_s.push_str(&indent(&trim(&str_val(html)), 10, false));
            out_s.push('\n');
        } else if truthy(text) {
            out_s.push_str(&format!(
                r#"          <p class="govuk-body">{}</p>"#,
                out(text)
            ));
            out_s.push('\n');
        }
        out_s.push_str("        </div>\n      </div>\n    </div>\n\n");

        // Go: `if actions := items(...); actions != nil` — empty slice is non-nil, so always enter.
        let actions = items(get(message, &["actions"]));
        // Match Go: items() always returns a slice (possibly empty), never nil for missing → empty.
        // But `actions != nil` is always true for items() return. Empty list still opens button group!
        // Looking at Go items() - for non-array returns nil?
        // Actually in Go: items on missing returns nil slice which IS nil, so `actions != nil` is false.
        // For empty array [], len is 0 but slice is non-nil.
        // Our items() returns &[] for non-array — empty slice, and we should only enter if the value was an array.
        let actions_val = get(message, &["actions"]);
        if matches!(actions_val, Value::Array(_)) {
            out_s.push_str("    <div class=\"govuk-button-group\">\n");
            for action in actions {
                out_s.push_str("      ");
                out_s.push_str(&indent(&trim(&cookie_banner_action(action)), 6, false));
                out_s.push('\n');
            }
            out_s.push_str("    </div>\n");
        }

        out_s.push_str("\n  </div>\n");
    }

    out_s.push_str("</div>");
    out_s
}

fn cookie_banner_action(action: &Value) -> String {
    let href = get(action, &["href"]);
    if !truthy(href) || str_val(get(action, &["type"])) == "button" {
        let button_type = Value::String("button".into());
        return render_button(&Params::from_pairs(&[
            ("text", get(action, &["text"]).clone()),
            (
                "type",
                def_truthy(get(action, &["type"]), &button_type).clone(),
            ),
            ("name", get(action, &["name"]).clone()),
            ("value", get(action, &["value"]).clone()),
            ("classes", get(action, &["classes"]).clone()),
            ("href", href.clone()),
            ("attributes", get(action, &["attributes"]).clone()),
        ]));
    }
    format!(
        r#"<a class="govuk-link{}" href="{}"{}>{}</a>"#,
        classes_if(get(action, &["classes"])),
        out(href),
        attributes(get(action, &["attributes"])),
        out(get(action, &["text"])),
    )
}
