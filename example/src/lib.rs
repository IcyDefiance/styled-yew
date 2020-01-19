use styled_yew::styled;
use wasm_bindgen::prelude::*;
use yew::{html, Children, Component, ComponentLink, Html, Properties, Renderable, ShouldRender};

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

	fn view(&self) -> Html {
		html! { <div>{ self.props.children.render() }</div> }
	}
}

styled!((red) Red : Div {
	color: "red";
});

styled!((green) Green : Div {
	color: "green";
});

styled!((blue) Blue : Div {
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

#[wasm_bindgen]
pub fn main() {
	yew::start_app::<App>();
}
