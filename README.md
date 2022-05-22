# yew-transition-group

Based off of `react-transition-group`.

`react-transition-group` has `Transition`, `CssTransition`, and `TransitionGroup`.

`yew-transition-group` has only `Transition` right now. When I get time, `TransitionGroup` will be implemented. I don't think `CssTransition` is feasible right now.

## Usage

Cargo.toml
```
[dependencies]
yew-transition-group = "0.0.1"
```

In view
```rust
html !{
<Transition enter={ enter } timeout={ Timeout::new(200) } notification={ notification_callback }>
    <p>{ "Hello World" }</p>
</Transition>

}
```

Where enter is a boolean, controlling when you want the transition to appear, and notification_callback is a `Callback<TransitionState>` where you get notified about changes to the `TransitionState` from `Entering` -> `Entered` -> `Exiting` -> `Exited`.

Check out the [example](/examples/transition-example/) for more information.
