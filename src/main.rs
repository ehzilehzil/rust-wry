use tao::{
    event::*,
    event_loop::*,
    window::*,
};
use wry::*;
use wry::dpi::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();
    let window_inner_size = window.inner_size();
    let main_webview = WebViewBuilder::new()
        .with_bounds(Rect{
            position: Position::Physical(PhysicalPosition { x: 0, y: 120 }),
            size: Size::Physical(PhysicalSize { width: window_inner_size.width, height: window_inner_size.height - 120 }),
        })
        .with_url("https://google.com")
        .build_as_child(&window)
        .unwrap();
    let sub_webview = WebViewBuilder::new()
        .with_bounds(Rect{
            position: Position::Physical(PhysicalPosition { x: 0, y: 0 }),
            size: Size::Physical(PhysicalSize { width: window_inner_size.width, height: 120 }),
        })
        .with_html(include_str!("sub_webview.html"))
        .build_as_child(&window)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent{ event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::RedrawRequested(_) => {
                window.set_background_color(Some((255, 255, 255, 255)));
            },
            Event::WindowEvent{ event: WindowEvent::Resized(window_inner_size), ..} => {
                let _ = main_webview.set_bounds(Rect {
                    position: Position::Physical(PhysicalPosition { x: 0, y: 120 }),
                    size: Size::Physical(PhysicalSize { width: window_inner_size.width, height: window_inner_size.height - 120 }),
                });
                let _ = sub_webview.set_bounds(Rect {
                    position: Position::Physical(PhysicalPosition { x: 0, y: 0 }),
                    size: Size::Physical(PhysicalSize { width: window_inner_size.width, height: 120 }),
                });
            },
            _ => (),
        }
    });
}