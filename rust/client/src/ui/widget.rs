use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use iced::{Element, Length};
use iced::theme::Button;
use iced::widget::{button, row, text, vertical_space};
use zbus::SignalContext;
use common::dbus::DbusEventViewEvent;
use common::model::PluginId;
use crate::dbus::DbusClient;

use crate::model::{NativeUiPropertyValue, NativeUiWidgetId};

#[derive(Clone, Debug)]
pub struct BuiltInWidgetWrapper {
    id: NativeUiWidgetId,
    inner: Arc<RwLock<BuiltInWidget>>,
}

impl BuiltInWidgetWrapper {
    pub fn widget(id: NativeUiWidgetId, widget_type: &str, properties: HashMap<String, NativeUiPropertyValue>) -> Self {
        let widget = match widget_type.as_ref() {
            "placeholdername:textcontent" => BuiltInWidget::TextContent {
                content: vec![]
            },
            "placeholdername:link" => BuiltInWidget::Link {
                href: properties.get("href").map(|href| href.as_string()).unwrap().unwrap().to_owned(),
                content: vec![],
            },
            "placeholdername:tag" => BuiltInWidget::Tag {
                content: vec![]
            },
            "placeholdername:metadata_item" => BuiltInWidget::MetadataItem {
                content: vec![]
            },
            "placeholdername:separator" => BuiltInWidget::Separator,
            "placeholdername:metadata" => BuiltInWidget::Metadata {
                content: vec![]
            },
            "placeholdername:image" => BuiltInWidget::Image,
            "placeholdername:h1" => BuiltInWidget::H1 {
                content: vec![]
            },
            "placeholdername:h2" => BuiltInWidget::H2 {
                content: vec![]
            },
            "placeholdername:h3" => BuiltInWidget::H3 {
                content: vec![]
            },
            "placeholdername:h4" => BuiltInWidget::H4 {
                content: vec![]
            },
            "placeholdername:h5" => BuiltInWidget::H5 {
                content: vec![]
            },
            "placeholdername:h6" => BuiltInWidget::H6 {
                content: vec![]
            },
            "placeholdername:horizontal_break" => BuiltInWidget::HorizontalBreak,
            "placeholdername:code_block" => BuiltInWidget::CodeBlock {
                content: vec![]
            },
            "placeholdername:code" => BuiltInWidget::Code {
                content: vec![]
            },
            "placeholdername:content" => BuiltInWidget::Content {
                content: vec![]
            },
            "placeholdername:detail" => BuiltInWidget::Detail {
                content: vec![]
            },
            _ => panic!("widget_type {} not supported", widget_type)
        };

        BuiltInWidgetWrapper::new(id, widget)
    }

    pub fn container(id: NativeUiWidgetId) -> Self {
        BuiltInWidgetWrapper::new(id, BuiltInWidget::Container { content: vec![] })
    }

    pub fn text(id: NativeUiWidgetId, text: &str) -> Self {
        BuiltInWidgetWrapper::new(id, BuiltInWidget::Text(text.to_owned()))
    }

    fn new(id: NativeUiWidgetId, widget: BuiltInWidget) -> Self {
        Self {
            id,
            inner: Arc::new(RwLock::new(widget)),
        }
    }

    fn get(&self) -> RwLockReadGuard<'_, BuiltInWidget> {
        self.inner.read().expect("lock is poisoned")
    }

    fn get_mut(&self) -> RwLockWriteGuard<'_, BuiltInWidget> {
        self.inner.write().expect("lock is poisoned")
    }

    pub fn render_widget<'a>(&self) -> Element<'a, BuiltInWidgetEvent> {
        let widget = self.get();
        match &*widget {
            BuiltInWidget::Text(text_content) => {
                text(text_content).into()
            }
            BuiltInWidget::TextContent { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::Link { content, href } => {
                let content: Element<_> = row(render_children(content))
                    .into();

                button(content)
                    .style(Button::Text)
                    .on_press(BuiltInWidgetEvent::LinkClick { href: href.to_owned() })
                    .into()
            }
            BuiltInWidget::Tag { content } => {
                let content: Element<_> = row(render_children(content))
                    .into();

                button(content)
                    .on_press(BuiltInWidgetEvent::TagClick { widget_id: self.id })
                    .into()
            }
            BuiltInWidget::MetadataItem { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::Separator => {
                text("Separator").into()
            }
            BuiltInWidget::Metadata { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::Image => {
                text("Image").into()
            }
            BuiltInWidget::H1 { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::H2 { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::H3 { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::H4 { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::H5 { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::H6 { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::HorizontalBreak => {
                text("HorizontalBreak").into()
            }
            BuiltInWidget::CodeBlock { content } => {
                text("CodeBlock").into()
            }
            BuiltInWidget::Code { content } => {
                text("Code").into()
            }
            BuiltInWidget::Content { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::Detail { content } => {
                row(render_children(content))
                    .into()
            }
            BuiltInWidget::Container { content } => {
                row(render_children(content))
                    .into()
            }
        }
    }

    pub fn append_child(&self, child: BuiltInWidgetWrapper) {
        let mut parent = self.get_mut();
        match *parent {
            BuiltInWidget::Link { ref mut content, .. } => {
                content.push(child)
            }
            BuiltInWidget::Tag { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::MetadataItem { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::Metadata { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::H1 { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::H2 { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::H3 { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::H4 { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::H5 { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::H6 { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::CodeBlock { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::Code { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::Content { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::Detail { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::Text(_) => {
                panic!("text cannot be a parent")
            }
            BuiltInWidget::TextContent { ref mut content } => {
                content.push(child)
            }
            BuiltInWidget::Separator => {
                panic!("separator cannot be a parent")
            }
            BuiltInWidget::Image => {
                panic!("image cannot be a parent")
            }
            BuiltInWidget::HorizontalBreak => {
                panic!("horizontal-break cannot be a parent")
            }
            BuiltInWidget::Container { ref mut content } => {
                content.push(child)
            }
        };
    }

    pub fn set_children(&self, new_children: Vec<BuiltInWidgetWrapper>) {
        let mut container = self.get_mut();
        match *container {
            BuiltInWidget::TextContent { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::Link { ref mut content, .. } => {
                *content = new_children
            }
            BuiltInWidget::Tag { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::MetadataItem { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::Metadata { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::H1 { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::H2 { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::H3 { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::H4 { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::H5 { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::H6 { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::CodeBlock { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::Code { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::Content { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::Detail { ref mut content } => {
                *content = new_children
            }
            BuiltInWidget::Text(_) => {
                panic!("text cannot be a parent")
            }
            BuiltInWidget::Separator => {
                panic!("separator cannot be a parent")
            }
            BuiltInWidget::Image => {
                panic!("image cannot be a parent")
            }
            BuiltInWidget::HorizontalBreak => {
                panic!("horizontal-break cannot be a parent")
            }
            BuiltInWidget::Container { ref mut content } => {
                *content = new_children
            }
        };
    }
}

pub fn render_children<'a>(
    content: &[BuiltInWidgetWrapper]
) -> Vec<Element<'a, BuiltInWidgetEvent>> {
    return content
        .into_iter()
        .map(|child| child.render_widget())
        .collect();
}


#[derive(Debug)]
enum BuiltInWidget {
    Text(String),
    TextContent {
        content: Vec<BuiltInWidgetWrapper>
    },
    Link {
        href: String,
        content: Vec<BuiltInWidgetWrapper>,
    },
    Tag {
        // color: String,
        // icon: String,
        content: Vec<BuiltInWidgetWrapper>,
        // onClick
    },
    MetadataItem {
        content: Vec<BuiltInWidgetWrapper>
    },
    Separator,
    Metadata {
        content: Vec<BuiltInWidgetWrapper>
    },
    Image,
    H1 {
        content: Vec<BuiltInWidgetWrapper>
    },
    H2 {
        content: Vec<BuiltInWidgetWrapper>
    },
    H3 {
        content: Vec<BuiltInWidgetWrapper>
    },
    H4 {
        content: Vec<BuiltInWidgetWrapper>
    },
    H5 {
        content: Vec<BuiltInWidgetWrapper>
    },
    H6 {
        content: Vec<BuiltInWidgetWrapper>
    },
    HorizontalBreak,
    CodeBlock {
        content: Vec<BuiltInWidgetWrapper>
    },
    Code {
        content: Vec<BuiltInWidgetWrapper>
    },
    Content {
        content: Vec<BuiltInWidgetWrapper>
    },
    Detail {
        content: Vec<BuiltInWidgetWrapper>
    },
    Container {
        content: Vec<BuiltInWidgetWrapper>
    },
}

#[derive(Clone, Debug)]
pub enum BuiltInWidgetEvent {
    LinkClick {
        href: String
    },
    TagClick {
        widget_id: NativeUiWidgetId
    },
}

impl BuiltInWidgetEvent {
    pub async fn handle(&self, signal_context: &SignalContext<'_>, plugin_id: PluginId) {
        match self {
            BuiltInWidgetEvent::LinkClick { href } => {
                todo!("href {:?}", href)
            }
            BuiltInWidgetEvent::TagClick { widget_id } => {
                let event_view_event = DbusEventViewEvent {
                    event_name: "onClick".to_owned(),
                    widget_id: widget_id.clone(),
                };

                DbusClient::view_event_signal(signal_context, &plugin_id.to_string(), event_view_event)
                    .await
                    .unwrap();
            }
        }
    }
}