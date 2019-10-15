#[macro_use] extern crate gfx;

extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

// Colors
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];

// Square geometry
const SQUARE: &[Vertex] = &[
	Vertex { pos: [0.5, -0.5], uv: [1.0, 0.0], color: RED },
	Vertex { pos: [-0.5, -0.5], uv: [0.0, 0.0], color: WHITE },
	Vertex { pos: [-0.5, 0.5], uv: [0.0, 1.0], color: GREEN },
	Vertex { pos: [0.5, 0.5], uv: [1.0, 1.0], color: BLUE },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_texture",
        target: gfx::RenderTarget<ColorFormat> = "target",
    }
}

fn load_texture<F, R>(factory: &mut F, path: &str) -> gfx::handle::ShaderResourceView<R, [f32; 4]>  where F: gfx::Factory<R>, R: gfx::Resources
{
	let img = image::open(path).unwrap().to_rgba();
	let (width, height) = img.dimensions();
	let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
	let (_, view) = factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&img]).unwrap();
	view
}

pub fn main()
{

	let events_loop = glutin::EventsLoop::new();
	let builder = glutin::WindowBuilder::new().with_title("Window".to_string()).with_dimensions(800, 640).with_vsync();
	let (window, mut device, mut factory, target, mut depth_target) = gfx_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

	let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

	let pso = factory.create_pipeline_simple(
		include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "./src/shaders/quad.vert")),
		include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "./src/shaders/quad.frag")),
		pipe::new()
	).unwrap();

	let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(SQUARE, INDICES);

	let texture = load_texture(&mut factory, "./src/textures/texture.jpg");
	let sampler = factory.create_sampler_linear();

	let mut data = pipe::Data {
		vbuf: vertex_buffer,
		texture: (texture, sampler),
		target: target
	};

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
					gfx_glutin::update_views(&window, &mut data.target, &mut depth_target);
				}, _ => (),
			}
		});

		encoder.clear(&mut data.target, BLACK);
		encoder.draw(&slice, &pso, &data);
		encoder.flush(&mut device);

		window.swap_buffers().unwrap();
		device.cleanup();
	}
}

/*
use std::io;

const DERP: f32 = 1e1;

fn factorial(n: i64) -> i64
{
	if n == 1
	{
		return 1;
	} else if n == 0 { return 1; }

	return n * factorial(n - 1);
}

fn main()
{
	let s = String::from("xixi");
	{
		let s = String::from("derp");
	}

	println!("{}", s);
	return;

	println!("Guess the number!");

	let x = 5;
	let y = 10;
	println!("x = {} and y = {}", x, y);

	println!("{}", DERP);

	let mut guess: String = String::new();
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	println!("{}", guess);

	let spaces = "   ";
	let _spaces_len: usize = spaces.len();

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
*/
