use yew::prelude::*;

use crate::timeout::Timeout;

#[derive(Debug, PartialEq, Properties)]
pub struct TransitionProps {
    children: Children,
    /// in From react-transition-group. Signal for the wrapped element to appear.
    enter: Option<bool>,

    timeout: Timeout,
}

impl TransitionProps {
    fn enter(&self) -> bool {
        self.enter.unwrap_or(false)
    }
}

#[derive(Debug)]
pub enum TransitionStateInner {
    /// The child component does not exist in the DOM.
    BeforeEnter,
    /// The element exists in the DOM.
    Mounted,
    /// Exit transition is done.
    Exited,
    /// The start of the beginning transition.
    Entering,
    /// The end of the beginning transition.
    Entered,
    /// The start of the end transition.
    Exiting,
}

pub struct Tick;

/// A component that wraps other components and handles transitions, making available `TransitionState` to the wrapped children.
#[derive(Debug)]
pub struct Transition {
    state: TransitionStateInner,
    saved_enter: Option<bool>,
}

impl Component for Transition {
    type Message = Tick;

    type Properties = TransitionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Transition {
            state: TransitionStateInner::BeforeEnter,
            saved_enter: Some(false),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match self.state {
            TransitionStateInner::BeforeEnter => self.state = TransitionStateInner::Mounted,
            TransitionStateInner::Mounted => self.state = TransitionStateInner::Entering,
            TransitionStateInner::Exited => self.state = TransitionStateInner::Exited,
            TransitionStateInner::Entering => self.state = TransitionStateInner::Entered,
            TransitionStateInner::Entered => self.state = TransitionStateInner::Exiting,
            TransitionStateInner::Exiting => self.state = TransitionStateInner::Exited,
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        match (ctx.props().enter(), self.saved_enter) {
            (true, Some(true)) => false,
            (true, Some(false)) => {
                let duration = if ctx.props().timeout.appear() == 0 {
                    self.state = TransitionStateInner::Entering;
                    ctx.props().timeout.enter()
                } else {
                    self.state = TransitionStateInner::Mounted;
                    ctx.props().timeout.appear()
                };

                ctx.link().send_future(async move {
                    gloo_timers::future::TimeoutFuture::new(duration).await;
                    Tick
                });

                self.saved_enter = Some(true);
                true
            }
            (false, Some(true)) => {
                if ctx.props().timeout.exit() != 0 {
                    self.state = TransitionStateInner::Exiting;

                    let duration = ctx.props().timeout.exit();

                    ctx.link().send_future(async move {
                        gloo_timers::future::TimeoutFuture::new(duration).await;
                        Tick
                    });
                } else {
                    self.state = TransitionStateInner::Exited
                }

                true
            }
            (false, Some(false)) => false,
            (true, None) => false,
            (false, None) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if matches!(self.state, TransitionStateInner::BeforeEnter) {
            html! {}
        } else {
            html! { <> { ctx.props().children.clone() } </> }
        }
    }
}
