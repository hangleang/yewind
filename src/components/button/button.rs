use super::style::Style;
use super::shape::Shape;
use super::button_type::Type;
use crate::utils::{Color, Size};
use crate::helpers::get_class_concat_with;

use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Button {
    props: Props,
	link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub shape: Shape,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub block: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub kind: Type,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub key: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    pub children: Children,
}

pub enum Msg {
	OnClicked(MouseEvent),
}

impl Component for Button {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			props, link,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender { 
		match msg {
			Msg::OnClicked(e) => self.props.on_click.emit(e),
		}

		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender { 
		self.props.neq_assign(props)
	}	

	fn view(&self) -> Html {
		let Props { style, shape, color, size, block, disabled, kind, id, class, key, node_ref, children, .. } = self.props.to_owned();

		let mut classes = Classes::from(vec![
			String::from("focus:outline-none"),
			get_class_concat_with(style, color),
			shape.to_string(),
			get_class_concat_with("text", size),
		]);
		if style.has_bg_color() {
			if !color.is_light() {
				classes.push("text-white")
			}
		}
		classes.push(match size {
			Size::XSmall => "py-1.5 px-3",
			Size::Small => "py-2 px-4",
			Size::Default => "py-2 px-5",
			Size::Large => "py-2 px-6",
			Size::XLarge => "py-2.5 px-7",
		});
		if block {
			classes.push("w-full");
		}
		classes = classes.extend(class);

		html! {
			<button 
				id=id
				class=classes
				key=key
				ref=node_ref
				type=kind
				disabled=disabled
			>
				{ children }
			</button>
		}
	}
}