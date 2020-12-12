mod css_rule;

pub use wasm_bindgen;
pub use web_sys;

use lazy_static::lazy_static;
use std::{
	any::TypeId,
	collections::HashMap,
	sync::{atomic::AtomicI32, Mutex},
};

pub static NEXT_ID: AtomicI32 = AtomicI32::new(0);
lazy_static! {
	pub static ref STYLE_IDS: Mutex<HashMap<TypeId, i32>> = Mutex::default();
}

#[macro_export]
macro_rules! styled {
	($vis:vis $name:ident : $child:ty { $($rule:ident : $val:expr);*; }) => {
		$vis struct $name {
			// TODO: store instance of child component and proxy each method to child, instead of sending props to child
			// in the view method
			props: <$child as Component>::Properties,
			node_ref: ::yew::NodeRef,
		}
		impl Component for $name {
			type Message = ();
			type Properties = <$child as Component>::Properties;

			fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
				use yew::NodeRef;

				Self { props, node_ref: NodeRef::default() }
			}

			fn update(&mut self, _: Self::Message) -> ShouldRender {
				false
			}

			fn change(&mut self, _props: Self::Properties) -> ShouldRender {
				false
			}

			fn view(&self) -> Html {
				use yew::virtual_dom::VChild;

				VChild::<$child>::new(self.props.clone(), self.node_ref.clone(), None).into()
			}

			fn rendered(&mut self, first_render: bool) {
				use $crate::{wasm_bindgen::JsCast, web_sys::{self, CssStyleSheet, Element, HtmlStyleElement}, STYLE_IDS};
				use std::{any::TypeId, sync::atomic::Ordering};

				if first_render {
					let id = *STYLE_IDS.lock().unwrap().entry(TypeId::of::<Self>()).or_insert_with(|| {
						let id = $crate::NEXT_ID.fetch_add(1, Ordering::Relaxed);
						let rules = concat!($( $crate::css_rule!($rule), ":", $val, ";" )*);
						let window = web_sys::window().expect("no window");
						let document = window.document().expect("no document");
						let style = document.create_element("style").unwrap().unchecked_into::<HtmlStyleElement>();
						document.head().expect("no head").append_child(&style).unwrap();
						let sheet = style.sheet().expect("no sheet").unchecked_into::<CssStyleSheet>();
						sheet.insert_rule(&format!(".sc{} {{ {} }}", id, rules)).unwrap();
						id
					});

					let el = self.node_ref.cast::<Element>().unwrap();
					el.class_list().add_1(&format!("sc{}", id)).unwrap();
				}
			}
		}
	};
}
