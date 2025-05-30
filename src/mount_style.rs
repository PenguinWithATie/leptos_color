use cfg_if::cfg_if;
use html::ElementChild as _;
use leptos::*;
use prelude::CustomAttribute;
pub fn mount_style(id: &str, content: &'static str) {
    let id = format!("leptos-color-id-{id}");
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::view;
            use leptos_meta::Style;

            if let Some(context) = SSRMountStyleContext::use_context() {
                context.push_style(id, content.to_string());
                return;
            }

            let _ = view! {
                <Style id=id>
                    {content}
                </Style>
            };
        } else {
            use leptos::prelude::document;
            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style#{id}"))
                .expect("query style element error");

            if style.is_some() {
                return;
            }

            let style = document()
                .create_element("style")
                .expect("create style element error");
            _ = style.set_attribute("id", &id);
            style.set_text_content(Some(content));
            _ = head.prepend_with_node_1(&style);
        }
    }
}

use leptos::{
    attr::Attribute,
    context::{Provider, ProviderProps},
    prelude::*,
    tachys::{
        hydration::Cursor,
        renderer::types,
        view::{any_view::AnyViewState, Position, PositionState},
    },
};
use std::collections::HashMap;

#[component]
pub fn SSRMountStyleProvider<Chil>(children: TypedChildren<Chil>) -> impl IntoView
where
    Chil: IntoView + 'static,
{
    let context = SSRMountStyleContext::default();

    let children = Provider(
        ProviderProps::builder()
            .value(context.clone())
            .children(children)
            .build(),
    )
    .into_any();
    SSRMountStyle { context, children }
}

#[derive(Debug, Clone)]
pub struct SSRMountStyleContext {
    styles: ArcStoredValue<HashMap<String, String>>,
}

impl SSRMountStyleContext {
    pub fn use_context() -> Option<Self> {
        use_context()
    }

    pub fn push_style(&self, k: String, v: String) {
        self.styles.write_value().insert(k, v);
    }

    fn default() -> Self {
        Self {
            styles: Default::default(),
        }
    }

    fn html_len(&self) -> usize {
        const TEMPLATE_LEN: usize = r#"<style id=""></style>"#.len();
        let mut html_len = 0;
        let styles = self.styles.write_value();

        styles.iter().for_each(|(k, v)| {
            html_len += k.len() + v.len() + TEMPLATE_LEN;
        });

        html_len
    }

    fn to_html(self) -> String {
        let mut styles = self.styles.write_value();
        styles
            .drain()
            .into_iter()
            .map(|(k, v)| format!(r#"<style id="{k}">{v}</style>"#))
            .collect::<String>()
    }
}

pub struct SSRMountStyle {
    context: SSRMountStyleContext,
    children: AnyView,
}

pub struct SSRMountStyleState {
    state: AnyViewState,
}

impl Render for SSRMountStyle {
    type State = SSRMountStyleState;

    fn build(self) -> Self::State {
        let state = self.children.build();
        SSRMountStyleState { state }
    }

    fn rebuild(self, state: &mut Self::State) {
        self.children.rebuild(&mut state.state);
    }
}

impl AddAnyAttr for SSRMountStyle {
    type Output<SomeNewAttr: Attribute> = Self;

    fn add_any_attr<NewAttr: Attribute>(self, _attr: NewAttr) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        self
    }
}

impl RenderHtml for SSRMountStyle {
    type AsyncOutput = Self;

    const MIN_LENGTH: usize = 0;

    fn html_len(&self) -> usize {
        self.children.html_len() + self.context.html_len()
    }

    fn dry_resolve(&mut self) {
        self.children.dry_resolve();
    }

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) {
        self.children
            .to_html_with_buf(buf, position, escape, mark_branches);

        let head_loc = buf
            .find("<head>")
            .expect("you are using SSRMountStyleProvider without a <head> tag");
        let marker_loc = buf
            .find(r#"<meta name="thaw-ui-style""#)
            .unwrap_or(head_loc + 6);

        buf.insert_str(marker_loc, &self.context.to_html());
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut leptos::tachys::ssr::StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) where
        Self: Sized,
    {
        self.children
            .to_html_async_with_buf::<OUT_OF_ORDER>(buf, position, escape, mark_branches);

        buf.with_buf(|buf| {
            let head_loc = buf
                .find("<head>")
                .expect("you are using SSRMountStyleProvider without a <head> tag");
            let marker_loc = buf
                .find(r#"<meta name="thaw-ui-style""#)
                .unwrap_or(head_loc + 6);
            buf.insert_str(marker_loc, &self.context.to_html());
        });
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        let state = self.children.hydrate::<FROM_SERVER>(cursor, position);
        SSRMountStyleState { state }
    }
}

impl Mountable for SSRMountStyleState {
    fn unmount(&mut self) {
        self.state.unmount();
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        self.state.mount(parent, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.state.insert_before_this(child)
    }
}
