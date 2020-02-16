mod css_rule;

pub use stdweb;

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

			fn mounted(&mut self) -> ShouldRender {
				use $crate::{stdweb::{js, web::Element}, STYLE_IDS};
				use std::{any::TypeId, sync::atomic::Ordering};

				let id = *STYLE_IDS.lock().unwrap().entry(TypeId::of::<Self>()).or_insert_with(|| {
					let id = $crate::NEXT_ID.fetch_add(1, Ordering::Relaxed);
					let rules = concat!($( $crate::css_rule!($rule), ":", $val, ";" )*);
					js! {
						const style = document.createElement("style");
						document.head.appendChild(style);
						style.sheet.insertRule(".sc" + @{id} + "{" + @{rules} + "}");
					}
					id
				});

				let el = self.node_ref.cast::<Element>().unwrap();
				js! { @{el}.className += " sc" + @{id}; };

				false
			}

			fn update(&mut self, _: Self::Message) -> ShouldRender {
				false
			}

			fn view(&self) -> Html {
				use yew::virtual_dom::VChild;

				VChild::<$child>::new(self.props.clone(), self.node_ref.clone()).into()
			}
		}
	};
}
