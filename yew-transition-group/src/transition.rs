use crate::timeout::Timeout;
use yew::prelude::*;

/// The Props for Transition.
#[derive(Debug, PartialEq, Properties)]
pub struct TransitionProps {
    /// The wrapped Children.
    pub children: Children,
    /// in From react-transition-group. Signal for the wrapped element to appear.
    pub enter: Option<bool>,

    /// The timeout for appear, enter, exit.
    pub timeout: Timeout,

    /// The Callback to be notified of state changes.
    pub notification: Callback<TransitionState>,
}

impl TransitionProps {
    fn enter(&self) -> bool {
        self.enter.unwrap_or(false)
    }
}

/// The four states that the children components see.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransitionState {
    /// The first state.
    Entering,
    /// The second state.
    Entered,
    /// The third state.
    Exiting,
    /// The fourth state.
    Exited,
}

#[derive(Debug)]
pub enum TransitionStateComplete {
    /// The child component does not exist in the DOM.
    BeforeEnter,
    /// The element exists in the DOM.
    Mounted,
    /// The four states that will be passed to the wrapped components.
    TransitionState(TransitionState),
}

#[derive(Debug)]
pub struct Tick;

/// A component that wraps other components and handles transitions, making available `TransitionState` to the wrapped children.
#[derive(Debug)]
pub struct Transition {
    state: TransitionStateComplete,
    saved_enter: Option<bool>,
}

impl Component for Transition {
    type Message = Tick;

    type Properties = TransitionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Transition {
            state: TransitionStateComplete::BeforeEnter,
            saved_enter: Some(false),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.state = match self.state {
            TransitionStateComplete::BeforeEnter => TransitionStateComplete::Mounted,
            TransitionStateComplete::Mounted => {
                let enter = ctx.props().timeout.enter();
                ctx.link().send_future(async move {
                    gloo_timers::future::TimeoutFuture::new(enter).await;
                    Tick
                });
                ctx.props().notification.emit(TransitionState::Entering);

                TransitionStateComplete::TransitionState(TransitionState::Entering)
            }
            TransitionStateComplete::TransitionState(state) => {
                let new_state = match state {
                    TransitionState::Entering => TransitionState::Entered,
                    TransitionState::Entered => TransitionState::Exiting,
                    TransitionState::Exiting => TransitionState::Exited,
                    TransitionState::Exited => TransitionState::Exited,
                };
                ctx.props().notification.emit(new_state);

                TransitionStateComplete::TransitionState(new_state)
            }
        };
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        match (ctx.props().enter(), self.saved_enter) {
            (true, Some(true)) => true,
            (true, Some(false)) => {
                let duration = if ctx.props().timeout.appear() == 0 {
                    self.state =
                        TransitionStateComplete::TransitionState(TransitionState::Entering);
                    ctx.props().notification.emit(TransitionState::Entering);
                    ctx.props().timeout.enter()
                } else {
                    self.state = TransitionStateComplete::Mounted;
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
                    self.state = TransitionStateComplete::TransitionState(TransitionState::Exiting);

                    let duration = ctx.props().timeout.exit();

                    ctx.link().send_future(async move {
                        gloo_timers::future::TimeoutFuture::new(duration).await;
                        Tick
                    });
                } else {
                    self.state = TransitionStateComplete::TransitionState(TransitionState::Exited);
                }
                self.saved_enter = Some(false);
                true
            }
            (false, Some(false)) => true,
            (true, None) => true,
            (false, None) => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.state {
            TransitionStateComplete::BeforeEnter => html! {<> </>},
            TransitionStateComplete::Mounted => html! { <>{ ctx.props().children.clone() } </>  },
            TransitionStateComplete::TransitionState(_) => {
                html! {
                    <>{ ctx.props().children.clone() }</>

                }
            }
        }
    }
}
