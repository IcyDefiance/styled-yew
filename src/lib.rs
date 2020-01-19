pub use lazy_static;
pub use stdweb;

use std::sync::atomic::AtomicUsize;

pub static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

// TODO: add all the css rules
// BETTER TODO, MAYBE: use a proc macro to validate and minify a string
#[rustfmt::skip]
#[macro_export]
macro_rules! css_rule {
	(color) => {"color"};
	(maxWidth) => {"max-width"};
}

#[macro_export]
macro_rules! styled {
	(($scope:ident) $name:ident : $child:ty { $($rule:ident : $val:expr);*; }) => {
		// TODO: eliminate these variables, probably by using a proc macro, else figure out how to keep them private
		// without forcing the user to provide an ident
		mod $scope {
			use $crate::lazy_static::lazy_static;
			use std::sync::atomic::{AtomicBool, Ordering};

			pub static ADDED_CSS: AtomicBool = AtomicBool::new(false);
			lazy_static! {
				pub static ref ID: String = $crate::NEXT_ID.fetch_add(1, Ordering::Relaxed).to_string();
			}
		}

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

				link.send_message(());
				Self { props, link, node_ref: NodeRef::default(), added_class: false }
			}

			fn mounted(&mut self) -> ShouldRender {
				use $scope::{ADDED_CSS, ID};
				use std::sync::atomic::Ordering;
				use $crate::stdweb::js;

				if !ADDED_CSS.swap(true, Ordering::AcqRel) {
					let rules = concat!($( $crate::css_rule!($rule), ":", $val, ";" )*);
					js! {
						const style = document.createElement("style");
						document.head.appendChild(style);
						style.sheet.insertRule(".sc" + @{&*ID} + "{" + @{rules} + "}");
					}
				}
				false
			}

			fn update(&mut self, _msg: Self::Message) -> ShouldRender {
				use $scope::ID;
				use $crate::stdweb::{js, web::Element};

				if !self.added_class {
					if let Some(el) = self.node_ref.try_into::<Element>() {
						js! { @{el}.className += " sc" + @{&*ID}; };
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
