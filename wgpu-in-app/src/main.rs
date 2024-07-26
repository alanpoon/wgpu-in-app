#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {}

#[cfg(all(
    not(target_os = "android"),
    not(target_os = "ios"),
    not(target_arch = "wasm32")
))]
fn main() {
    use app_surface::AppSurface;
    use wgpu_in_app::WgpuCanvas;
    use winit::{
        event::{ElementState, Event, KeyEvent, WindowEvent},
        event_loop::EventLoop,
        keyboard::{Key, KeyCode, NamedKey, PhysicalKey},
    };
    use std::{sync::{Arc,Mutex}, thread::sleep, time::Duration};
    use rand::Rng;
    let events_loop = EventLoop::new().unwrap();
    let size = winit::dpi::Size::Logical(winit::dpi::LogicalSize {
        width: 1200.0,
        height: 800.0,
    });
    let builder = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_max_inner_size(size)
        .with_transparent(true)
        .with_title("wgpu on Desktop");
    let window = builder.build(&events_loop).unwrap();

    let app_view = pollster::block_on(AppSurface::new(window));
    let canvas = Arc::new(Mutex::new(WgpuCanvas::new(app_view, 0)));
    let  canvas_c = canvas.clone();
    std::thread::spawn(move||{
        loop{

            sleep(Duration::from_secs(5));
            let num = rand::thread_rng().gen_range(0,5);
            canvas_c.lock().unwrap().change_example(num);
        }
        
    });
    let _ = events_loop.run(move |event, elwt| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        println!("Window minimized!");
                    } else {
                        canvas.lock().unwrap().resize();
                    }
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(key),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } =>{
                    let mut canvas = canvas.lock().unwrap();
                    match key {
                    
                        KeyCode::Digit1 => canvas.change_example(1),
                        KeyCode::Digit2 => canvas.change_example(2),
                        KeyCode::Digit3 => canvas.change_example(3),
                        KeyCode::Digit4 => canvas.change_example(4),
                        KeyCode::Digit5 => canvas.change_example(5),
                        _ => canvas.change_example(0),
                    }
                 } 
                ,
                WindowEvent::RedrawRequested => {
                    let mut canvas = canvas.lock().unwrap();
                    canvas.enter_frame();
                    canvas.app_surface.view.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
        }
    });
}
