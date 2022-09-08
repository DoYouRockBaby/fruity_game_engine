use crate::ui::context::UIContext;
use crate::ui::elements::display::Text;
use crate::ui::elements::input::Button;
use crate::ui::elements::layout::Empty;
use crate::ui::elements::layout::Row;
use crate::ui::elements::layout::RowItem;
use crate::ui::elements::UIElement;
use crate::ui::elements::UISize;
use crate::ui::elements::UIWidget;
use fruity_core::convert::FruityTryFrom;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::AnyResourceReference;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use std::sync::Arc;

pub fn draw_editor_resource_reference<T: Resource + ?Sized>(
    name: &str,
    value: Box<dyn SerializableObject>,
    on_update: Box<dyn Fn(&UIContext, Box<dyn SerializableObject>) + Send + Sync + 'static>,
) -> UIElement {
    let value = if let Ok(value) =
        ResourceReference::<T>::fruity_try_from(Serialized::NativeObject(value))
    {
        value
    } else {
        return Empty {}.elem();
    };

    Row {
        children: vec![
            RowItem {
                size: UISize::Units(40.0),
                child: Text {
                    text: name.to_string(),
                    ..Default::default()
                }
                .elem(),
            },
            RowItem {
                size: UISize::Fill,
                child: Button {
                    label: value.get_name(),
                    on_click: Arc::new(|_| {}),
                    accept_drag: Some(Arc::new(|_, item| {
                        if let Some(resource) = item.downcast_ref::<AnyResourceReference>() {
                            resource.downcast::<T>().is_some()
                        } else {
                            item.downcast_ref::<ResourceReference<T>>().is_some()
                        }
                    })),
                    on_drag: Some(Arc::new(move |ctx, resource| {
                        let resource = if let Some(resource) =
                            resource.downcast_ref::<AnyResourceReference>()
                        {
                            resource.downcast::<T>()
                        } else {
                            resource
                                .downcast_ref::<ResourceReference<T>>()
                                .map(|resource| resource.clone())
                        };

                        if let Some(resource) = resource {
                            on_update(ctx, Box::new(resource))
                        }
                    })),
                    ..Default::default()
                }
                .elem(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
    .elem()
}
