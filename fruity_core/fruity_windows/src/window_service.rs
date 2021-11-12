use fruity_core::resource::resource::Resource;
use fruity_core::signal::Signal;

pub trait WindowService: Resource {
    fn close(&self);
    fn set_resizable(&self, resizable: bool);
    fn get_size(&self) -> (usize, usize);
    fn get_scale_factor(&self) -> f64;
    fn get_cursor_position(&self) -> (usize, usize);
    fn set_size(&self, width: usize, height: usize);
    fn set_title(&self, title: &str);
    fn on_enter_loop(&self) -> &Signal<()>;
    fn on_start_update(&self) -> &Signal<()>;
    fn on_end_update(&self) -> &Signal<()>;
    fn on_resize(&self) -> &Signal<(usize, usize)>;
    fn on_cursor_moved(&self) -> &Signal<(usize, usize)>;
}
