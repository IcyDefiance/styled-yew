use styled_yew::styled;
use wasm_bindgen::prelude::*;
use yew::{html, Children, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Properties)]
struct DivProps {
	children: Children,
}
struct Div {
	props: DivProps,
}
impl Component for Div {
	type Message = ();
	type Properties = DivProps;

	fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
		Self { props }
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! { <div>{ self.props.children.clone() }</div> }
	}
}

styled!(Red : Div {
	color: "red";
});

styled!(Green : Div {
	color: "green";
});

styled!(Blue : Div {
	color: "blue";
});

struct App {}
impl Component for App {
	type Message = ();
	type Properties = ();

	fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<>
				<Red>{ "test" }</Red>
				<Green>{ "test" }</Green>
				<Blue>{ "test" }</Blue>
			</>
		}
	}
}

#[wasm_bindgen(start)]
pub fn main() {
	yew::start_app::<App>();
}
