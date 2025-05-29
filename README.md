# Bevy Workshop

## Installing Bevy

Copied From [Bevy's README](https://github.com/bevyengine/bevy/edit/main/README.md)

### Getting Started

We recommend checking out the [Quick Start Guide](https://bevyengine.org/learn/quick-start/introduction) for a brief introduction.

Follow the [Setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup) to ensure your development environment is set up correctly.
Once set up, you can quickly try out the [examples](https://github.com/bevyengine/bevy/tree/latest/examples) by cloning this repo and running the following commands:

```sh
# Switch to the correct version (latest release, default is main development branch)
git checkout latest
# Runs the "breakout" example
cargo run --example breakout
```

To draw a window with standard functionality enabled, use:

```rust
use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .run();
}
```

### Fast Compiles


Bevy can be built just fine using default configuration on stable Rust. However for really fast iterative compiles, you should enable the "fast compiles" setup by [following the instructions here](https://bevyengine.org/learn/quick-start/getting-started/setup).
