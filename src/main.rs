use winit;

pub fn main()
{
    // Create a new window
    let mut window = WinitState::default();

    // Running loop
    let mut running = true;
    while running {
        window.events.poll_events(|event| match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested, ..
            } => running = false, _ => (),
        });
    }
}

/// Structure to store the state of the window
pub struct WinitState {
    pub events: winit::EventsLoop,
    pub window: winit::Window
}

impl WinitState {
    /// Constructs a new `EventsLoop` and `Window` pair.
    ///
    /// The specified title and size are used, other elements are default.
    pub fn new(title: &str, size: winit::dpi::LogicalSize) -> Result<Self, winit::CreationError> {
        let events_loop = winit::EventsLoop::new();

        let output = winit::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(size)
            .build(&events_loop);

        return output.map(|window| Self { events: events_loop, window})
    }
}
pub const WINDOW_NAME: &str = "Window";

impl Default for WinitState {
    /// Makes an 800x600 window with the `WINDOW_NAME` value as the title.
    /// ## Panics
    /// If a `CreationError` occurs.
    fn default() -> Self {
        Self::new(
            WINDOW_NAME,
            winit::dpi::LogicalSize {
                width: 800.0,
                height: 600.0,
            },
        )
            .expect("Could not create a window!")
    }
}
