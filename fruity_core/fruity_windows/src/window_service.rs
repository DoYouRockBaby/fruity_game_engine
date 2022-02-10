use fruity_core::resource::resource::Resource;
use fruity_core::signal::Signal;

pub trait WindowService: Resource {
    fn close(&self);
    fn set_resizable(&self, resizable: bool);
    fn get_size(&self) -> (u32, u32);
    fn get_scale_factor(&self) -> f64;
    fn get_cursor_position(&self) -> (u32, u32);
    fn set_size(&self, width: u32, height: u32);
    fn set_title(&self, title: &str);
    fn on_enter_loop(&self) -> &Signal<()>;
    fn on_start_update(&self) -> &Signal<()>;
    fn on_end_update(&self) -> &Signal<()>;
    fn on_resize(&self) -> &Signal<(u32, u32)>;
    fn on_cursor_moved(&self) -> &Signal<(u32, u32)>;
}
