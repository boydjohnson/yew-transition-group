/// The timeout is a way to specify One timeout or different timeouts for appear, enter, exit.
///
/// Specify one timeout for all three transitions.
///   
/// ```rust
///     use yew_transition_group::Timeout;
///
///     let timeout = Timeout::new(300);
/// ```
///
/// A different timeout for enter (and appear) than for exit.
///
/// ```rust
///     use yew_transition_group::Timeout;
///
///     let timeout = Timeout::default().with_enter(100).with_exit(200);
/// ```

#[derive(Debug, Default, PartialEq)]
pub struct Timeout {
    timeout: Option<u32>,
    appear: Option<u32>,
    enter: Option<u32>,
    exit: Option<u32>,
}

impl Timeout {
    /// Constructor for just a single timeout value for all 3 transitions.
    pub fn new(t: u32) -> Self {
        Timeout {
            timeout: Some(t),
            ..Default::default()
        }
    }

    /// Builder function for adding a duration for the appear -> entering transition.
    pub fn with_appear(mut self, t: u32) -> Self {
        self.appear = Some(t);
        self
    }

    /// Builder function for adding a duration for the entering -> entered transition.
    pub fn with_enter(mut self, t: u32) -> Self {
        self.enter = Some(t);
        self
    }

    /// Builder function for adding a duration for the exiting -> exited transition.
    pub fn with_exit(mut self, t: u32) -> Self {
        self.exit = Some(t);
        self
    }

    /// Getter for exiting -> exited transition.
    pub fn exit(&self) -> u32 {
        self.timeout.or(self.exit).unwrap_or(0)
    }

    /// Getter for entering -> entered transition.
    pub fn enter(&self) -> u32 {
        self.timeout.or(self.enter).unwrap_or(0)
    }

    /// Getter for appear -> entering transition.
    pub fn appear(&self) -> u32 {
        self.timeout.or(self.appear).or(self.enter).unwrap_or(0)
    }
}
