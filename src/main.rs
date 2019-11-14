#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
use gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

use winit;
use log;
use arrayvec;
use gfx_hal;

pub fn main()
{
    simple_logger::init().unwrap();

    // Create a new window
    let mut window_state = WinitState::default();
    let mut hal_state = HalState::new(&winit_state.window);
    let mut local_state = LocalState::default();

    // Running loop
    loop  {
        let inputs = UserInput::poll_events_loop(&mut winit_state.event_loop);
        if inputs.end_requested {
            break;
        }

        local_state.update_from_input(inputs);

        if let Err(e) = do_the_render(&mut hal_state, &local_state) {
            error!("Rendering Error: {:?}", e);
            break;
        }
    }

    pub fn do_the_render(hal: &mut HalState, locals: &LocalState) -> Result<(), &str> {
        hal.draw_clear_frame(locals.color())
    }
}


pub struct Submission {
    pub command_buffers: IntoIterator<Item = &'a gfx_hal::command::Submittable<B, C, gfx_hal::command::Primary>>,
    pub wait_semaphores: IntoIterator<Item = (&'a std::borrow::Borrow<B::Semaphore>, gfx_hal::command::PipelineStage)>,
    pub signal_semaphores: IntoIterator<Item = &'a std::borrow::Borrow<B::Semaphore>>,
}

impl HalState {
    pub fn draw_clear_frame(&mut self, color: [f32; 4]) -> Result<(), &'static str> {
        // Submission
        let command_buffers: arrayvec::ArrayVec<[_; 1]> = [the_command_buffer].into();
        let wait_semaphores: arrayvec::ArrayVec<[_; 1]> = [(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
        let signal_semaphores: arrayvec::ArrayVec<[_; 1]> = [render_finished].into();
        let present_wait_semaphores: arrayvec::ArrayVec<[_; 1]> = [render_finished].into();
        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores,
        };
        unsafe {
            the_command_queue.submit(submission, Some(flight_fence));
            the_swapchain.present(&mut the_command_queue, i_u32, present_wait_semaphores).map_err(|_|"Failed to present into the swapchain!")
        }
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
