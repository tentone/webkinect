#[macro_use] extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;


use std::io;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const DERP: f32 = 1e1;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn main()
{
	let events_loop = glutin::EventsLoop::new();
	let builder = glutin::WindowBuilder::new().with_title("Window".to_string()).with_dimensions(800, 640).with_vsync();
	let (window, mut device, mut factory, mut main_color, mut main_depth) = gfx_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

	let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
	let mut running = true;

	while running
	{
		events_loop.poll_events(|glutin::Event::WindowEvent{window_id: _, event}|
		{
			use glutin::WindowEvent::*;
			match event
			{
				// ESC to exit
				KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) | Closed => running = false, Resized(_, _) =>
				{
					gfx_glutin::update_views(&window, &mut main_color, &mut main_depth);
				}, _ => (),
			}
		});

		encoder.clear(&main_color, BLACK);
		encoder.flush(&mut device);
		window.swap_buffers().unwrap();
		device.cleanup();
	}
}


fn factorial(n: i64) -> i64
{
	if n == 1
	{
		return 1;
	} else if n == 0 { return 1; }

	return n * factorial(n - 1);
}

fn tests()
{
	println!("Guess the number!");

	let x = 5;
	let y = 10;
	println!("x = {} and y = {}", x, y);

	println!("{}", DERP);

	let mut guess: String = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("{}", guess);

	let spaces = "   ";
	let spaces_len: usize = spaces.len();

	let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

	let mut i: usize = 0;
	while i < months.len()
	{
		println!("{}", months[i]);
		i += 1;
	}

	let x = 5;
	let y = {
		let x = 3;
		x + 1
	};

	println!("The value of y is: {}, x is: {}", y, x);

	print!("{}", factorial(8));

}
