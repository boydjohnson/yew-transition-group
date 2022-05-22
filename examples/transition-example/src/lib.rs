use std::collections::BTreeMap;
use yew::prelude::*;
use yew_transition_group::{Timeout, Transition, TransitionState};

pub struct App {
    open: bool,
    styles: BTreeMap<TransitionState, String>,
    state: Option<TransitionState>,
}

pub enum AppMsg {
    Boop,
    StateTransition(TransitionState),
}

impl Component for App {
    type Message = AppMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut styles = BTreeMap::new();
        styles.insert(TransitionState::Entered, "opacity: 1.0".into());
        styles.insert(TransitionState::Entering, "opacity: 0.0".into());
        styles.insert(TransitionState::Exited, "opacity: 0.0".into());
        styles.insert(TransitionState::Exiting, "opacity: 1.0".into());

        App {
            open: false,
            styles,
            state: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Boop => {
                self.open = !self.open;
            }
            AppMsg::StateTransition(state) => {
                self.state = Some(state);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = if let Some(context) = self.state {
            format!(
                "{};transition: opacity 300ms ease-in-out;",
                self.styles[&context].clone()
            )
        } else {
            "opacity: 0;transition: opacity 300ms ease-in-out;".into()
        };

        let button = if self.open {
            let on_click = ctx.link().callback(|_| AppMsg::Boop);

            html! { <button onclick={on_click}>{ "Beep"}</button>}
        } else {
            let on_click = ctx.link().callback(|_| AppMsg::Boop);
            html! { <button onclick={on_click}>{ "Boop" }</button>}
        };

        let notification = ctx.link().callback(AppMsg::StateTransition);

        let timeout = Timeout::new(300);
        html! {
            <>
        <div>{ button }</div>

        <div><Transition enter={ self.open } notification={ notification } timeout={timeout}><p style={ style }>{"Hello, World!" }</p></Transition></div>
            </>
        }
    }
}
