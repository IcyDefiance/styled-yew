mod css_rule;

pub use stdweb;

use lazy_static::lazy_static;
use std::{
	any::TypeId,
	collections::HashMap,
	sync::{atomic::AtomicUsize, RwLock},
};

pub static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
lazy_static! {
	pub static ref STYLE_STATES: RwLock<HashMap<TypeId, usize>> = RwLock::default();
}

#[macro_export]
macro_rules! styled {
	($name:ident : $child:ty { $($rule:ident : $val:expr);*; }) => {
		struct $name {
			// TODO: store instance of child component and proxy each method to child, instead of sending props to child
			// in the view method
			props: <$child as Component>::Properties,
			// TODO: node_ref is unavailable inside mounted() and in the first update() call, so we have to call update
			// until it is available. this feels dirty and inefficient.
			link: ComponentLink<Self>,
			node_ref: ::yew::NodeRef,
			added_class: bool,
		}
		impl Component for $name {
			type Message = ();
			type Properties = <$child as Component>::Properties;

			fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
				use std::sync::Mutex;
				use yew::NodeRef;

				Self { props, link, node_ref: NodeRef::default(), added_class: false }
			}

			fn mounted(&mut self) -> ShouldRender {
				use $crate::{stdweb::js, STYLE_STATES};
				use std::{any::TypeId, sync::atomic::Ordering};

				let typeid = TypeId::of::<Self>();
				if !STYLE_STATES.read().unwrap().contains_key(&typeid) {
					let id = $crate::NEXT_ID.fetch_add(1, Ordering::Relaxed);
					let rules = concat!($( $crate::css_rule!($rule), ":", $val, ";" )*);
					js! {
						const style = document.createElement("style");
						document.head.appendChild(style);
						style.sheet.insertRule(".sc" + @{id.to_string()} + "{" + @{rules} + "}");
					}
					STYLE_STATES.write().unwrap().insert(typeid, id);
				}

				self.link.send_message(());
				false
			}

			fn update(&mut self, _msg: Self::Message) -> ShouldRender {
				use $crate::{STYLE_STATES, stdweb::{js, web::Element}};
				use std::any::TypeId;

				if !self.added_class {
					if let Some(el) = self.node_ref.try_into::<Element>() {
						let id = *STYLE_STATES.read().unwrap().get(&TypeId::of::<Self>()).unwrap();
						js! { @{el}.className += " sc" + @{id.to_string()}; };
						self.added_class = true;
					} else {
						self.link.send_message(());
					}
				}
				false
			}

			fn view(&self) -> Html {
				use yew::virtual_dom::VChild;

				VChild::<$child>::new(self.props.clone(), self.node_ref.clone()).into()
			}
		}
	};
}
