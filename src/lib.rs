use ambient_app::AppWrapper;
use mobile_entry_point::mobile_entry_point;
use winit::{event_loop::EventLoopBuilder, window::{Window, WindowBuilder}};
use ambient::{self, client::{self, init}};
use ambient_app::ffi::IOSViewObj;
// #[mobile_entry_point]
// pub extern "C" fn main() {
//     // let event_loop = EventLoopBuilder::new().build();
//     // ambient::new(event_loop);
// }
use std::ffi::CString;
use tracing::{event, Level};
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_subscriber::Layer;
use tracing_subscriber::filter::LevelFilter;

pub struct CustomLayer;
impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        for field in event.fields() {
            println!("  field={}", field.name());
        }
    }
}
// Function to wrap NSLog
// fn nslog(message: &str) {
//     let cstr = CString::new(message).unwrap();
//     unsafe {
//         objc::msg_send![class!(NSLog), NSLog(cstr.as_ptr())];
//     }
// }
#[no_mangle]
#[cfg(target_os="ios")]
pub fn enter_frame(obj: *mut libc::c_void) {
    // 获取到指针指代的 Rust 对象的可变借用
    let obj = unsafe 
    { &mut *(obj as *mut AppWrapper) };
    //obj.run_with_view(client::init);
}
#[no_mangle]
#[cfg(target_os="ios")]
pub fn create_wgpu_canvas(ios_obj: IOSViewObj) -> *mut libc::c_void {

    let subscriber = Registry::default().with(CustomLayer).with(LevelFilter::INFO);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");
    println!(
        "create_wgpu_canvas, maximum frames: {}",
        ios_obj.maximum_frames
    );
    let obj = ambient::new_ios(ios_obj);
    // 使用 Box 对 Rust 对象进行装箱操作。
    // 我们无法将 Rust 对象直接传递给外部语言，通过装箱来传递此对象的胖指针
    let box_obj = Box::new(obj);
    // into_raw 返回指针的同时，将此对象的内存管理权转交给调用方
    Box::into_raw(box_obj) as *mut libc::c_void
}