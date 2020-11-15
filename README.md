# bevy-prototype-parallax

A simple parallax component for the bevy engine.

This will allow you to quickly have a scrolling parralax style background.
It should also be useable to have a simple scrolling background too.

It does assume your image repeats nicely infinitely

![Demo](assets/demo.gif)

# Usage

```rust

    App::build()
        ....
        .add_plugin(ParallaxPlugin)

    ....
    let handle = /*load your colormaterial */

    commands
        .spawn(Camera2dComponents::default())
        .with(WindowSize::default())
        .with_children(|cb| {
            // Spawn the layers.
            // We can have as many as we like
            cb.spawn(LayerComponents {
                layer: Layer {
                    speed: speed,
                },
                material: handle,                                                                                
                ..Default::default()
        });
```

Make sure your camera has a window size component. 
This will enable a system that allows, the window size to be known, which allows the plugin to
determine how many times to repeat the image.

Then make sure your layer uses this camera as it's parent element. This allows it so shift
itself depending on how much the camera is offset. Different speeds off different layers will
achieve a repeating effect, a speed of 0 will make the layer static, a speed of 1.0 will 
make it move linearly with the camera.

The sprite components are managed automatically by the layer system, it will only spawn as many
as needed to fill the screen.

Note: doesn't support resizing yet.
Note: only horizontal for now. 

