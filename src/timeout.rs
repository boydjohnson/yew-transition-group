///! The timeout is a way to specify One timeout or different timeouts for appear, enter, exit.

#[derive(Debug, Default, PartialEq)]
pub struct Timeout {
    timeout: Option<u32>,
    appear: Option<u32>,
    enter: Option<u32>,
    exit: Option<u32>,
}

impl Timeout {
    pub fn new(t: u32) -> Self {
        Timeout {
            timeout: Some(t),
            ..Default::default()
        }
    }

    pub fn with_appear(mut self, t: u32) -> Self {
        self.appear = Some(t);
        self
    }

    pub fn with_enter(mut self, t: u32) -> Self {
        self.enter = Some(t);
        self
    }

    pub fn with_exit(mut self, t: u32) -> Self {
        self.exit = Some(t);
        self
    }

    pub fn exit(&self) -> u32 {
        self.timeout.or(self.exit).unwrap_or(0)
    }

    pub fn enter(&self) -> u32 {
        self.timeout.or(self.enter).unwrap_or(0)
    }

    pub fn appear(&self) -> u32 {
        self.timeout.or(self.appear).or(self.enter).unwrap_or(0)
    }
}
