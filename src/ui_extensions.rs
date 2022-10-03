#![allow(unused)]

use std::time::Instant;

// Extensions for egui.ui
use egui::{self, *};

include!("./icon_variants.rs");

impl Icon {

    #[inline(always)]
    fn text(&self) -> String {
        let v = *self as u32;
        if let Some(chr) = char::from_u32(v) {
            chr.into()
        } else {
            String::default()
        }
    }

    #[allow(unused)]
    fn button(&self, ui: &mut Ui) -> Response {
        ui.button(self.text())
    }

    #[allow(unused)]
    fn small_button(&self, ui: &mut Ui) -> Response {
        ui.small_button(self.text())
    }

    #[allow(unused)]
    fn visual(&self, ui: &mut Ui) -> Response {
        ui.label(self.text())
    }

}

impl Into<char> for Icon {
    #[inline(always)]
    fn into(self) -> char {
        char::from_u32(self as u32).unwrap_or('\0')
    }
}

impl Into<String> for Icon {
    #[inline(always)]
    fn into(self) -> String {
        if let Some(value) = char::from_u32(self as u32) {
            String::from(value)
        } else {
            String::default()
        }
    }
}

impl Widget for Icon {
    #[inline(always)]
    fn ui(self, ui: &mut Ui) -> Response {
        let text: String = self.into();
        ui.button(text)
    }
}

pub struct ButtonBar<'a,V>
where V: Copy {
    pub items: &'a [(&'a str,V)],
    pub result: &'a mut Option<V>,
}

impl<'a,V> ButtonBar<'a,V>
where V: Copy {
    #[inline(always)]
    fn new(items: &'a [(&'a str, V)], result: &'a mut Option<V>) -> Self {
        Self {
            items,
            result,
        }
    }
}

impl<'a,V> Widget for ButtonBar<'a,V>
where V: Copy {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut click_response: Option<Response> = None;
        let resp = ui.horizontal(|ui| {
                let width = ui.available_width();
                let item_width = width / (self.items.len() as f32);
                let size = Vec2::new(item_width, ui.available_height());
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.style_mut().visuals.widgets.inactive.rounding = Rounding::none();
                ui.style_mut().visuals.widgets.active.rounding = Rounding::none();
                ui.style_mut().visuals.widgets.hovered.rounding = Rounding::none();
                for item in self.items.iter() {
                    let (_, rect) = ui.allocate_space(size);
                    
                    let btn = Button::new(item.0)
                        .stroke(Stroke::none());

                    let mut btn_resp = ui.put(rect, btn);

                    if btn_resp.clicked() {
                        btn_resp.mark_changed();
                        click_response = Some(btn_resp);
                        *self.result = Some(item.1);
                    }

                }
            });
        if click_response.is_some() {
            resp.response.union(click_response.unwrap())
        } else {
            resp.response
        }
    }
}

pub struct Callback<'a,R> {
    pub callback: Option<Box<dyn FnOnce() -> R + 'a>>,
}

pub trait CallbackOnce<'a,R> {
    fn invoke(self) -> R;
    fn invoke_if(self) -> Option<R>;
}

impl<'a,R> CallbackOnce<'a,R> for Callback<'a,R> {

    /// Panics if the callback has already been consumed.
    #[inline(always)]
    fn invoke(mut self) -> R {
        (
            self.callback
                .take()
                .expect("Callback already invoked.")
        )()
    }

    /// Returns None if the callback has already been consumed.
    #[inline(always)]
    fn invoke_if(mut self) -> Option<R> {
        self.callback
            .take()
            .map(|callback| callback())
    }

}

impl<'a, R> CallbackOnce<'a,R> for Option<Box<dyn FnOnce() -> R + 'a>> {
    #[inline(always)]
    fn invoke(mut self) -> R {
        (
            self.take()
                .expect("Callback already invoked.")
        )()
    }

    #[inline(always)]
    fn invoke_if(mut self) -> Option<R> {
        self.take()
            .map(|callback| callback())
    }
}

impl<'a,F, R> From<F> for Callback<'a, R>
where F: FnOnce() -> R + 'a {
    #[inline(always)]
    fn from(value: F) -> Self {
        Self {
            callback: Some(Box::new(value))
        }
    }
}

// .fill(impl Into<Color32>)
// .frame(bool)
// .sense(Sense)
// .small()
// .stroke(impl Into<Stroke>)
// .wrap(bool)
pub struct FunctionBarButton<'a> {
    text: WidgetText,
    enabled: bool,
    visible: bool,
    callback: Option<Box<dyn FnOnce() + 'a>>,
    id: Option<Id>,
    // Button Options
    fill: Option<Color32>,
    frame: Option<bool>,
    sense: Option<Sense>,
    small: Option<bool>,
    stroke: Option<Stroke>,
    wrap: Option<bool>,
}

impl<'a> FunctionBarButton<'a> {

    #[inline(always)]
    pub fn new<F: FnOnce() + 'a>(text: impl Into<WidgetText>, callback: F) -> Self {
        Self {
            text: text.into(),
            enabled: true,
            visible: true,
            callback: Some(Box::new(callback)),
            id: None,
            fill: None,
            frame: None,
            sense: None,
            small: None,
            stroke: None,
            wrap: None,
        }
    }

    #[inline(always)]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.set_id(id);
        self
    }

    #[inline(always)]
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.set_enabled(enabled);
        self
    }

    #[inline(always)]
    pub fn visible(mut self, visible: impl Into<bool>) -> Self {
        self.set_visible(visible);
        self
    }


    #[inline(always)]
    pub fn set_id(&mut self, id: impl Into<Id>) {
        self.id = Some(id.into());
    }

    #[inline(always)]
    pub fn set_enabled(&mut self, enabled: impl Into<bool>) {
        self.enabled = enabled.into();
    }

    #[inline(always)]
    pub fn set_visible(&mut self, visible: impl Into<bool>) {
        self.visible = visible.into();
    }

    /// Invokes and consumes the callback.
    /// Panics if the callback has been consumed.
    #[inline(always)]
    pub fn invoke(mut self) {
        if let Some(callback) = self.callback.take() {
            (callback)();
        } else {
            panic!("Cannot invoke a consumed callback.");
        }
    }

    /// Invokes and consumes the callback if it has not already been consumed.
    /// Returns true if it was successfully invoked.
    #[inline(always)]
    pub fn invoke_if(mut self) -> bool {
        if let Some(callback) = self.callback.take() {
            (callback)();
            true
        } else {
            false
        }
    }
}

pub struct FunctionBar<'a> {
    actions: Vec<FunctionBarButton<'a>>,
    id: Id,
}

impl<'a> FunctionBar<'a> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            // Create the default id from the current time to guarantee uniqueness.
            id: Id::new(Instant::now()),
        }
    }

    #[inline(always)]
    pub fn with_id<ID>(id: ID) -> Self
    where ID: Into<Id> {
        Self {
            actions: Vec::new(),
            id: id.into(),
        }
    }

    #[inline(always)]
    pub fn action<F, T>(mut self, text: T, callback: F) -> Self
    where
        F: FnOnce() + 'a,
        T: Into<WidgetText> {
        self.actions.push(FunctionBarButton::new(text, callback));
        self
    }

    #[inline(always)]
    pub fn id<ID>(mut self, id: ID) -> Self
    where ID: Into<Id> {
        self.actions
            .last_mut()
            .expect("Cannot assign id to empty collection.")
            .set_id(id);
        self
    }

    #[inline(always)]
    pub fn enabled<T>(mut self, state: T) -> Self
    where T: Into<bool> {
        self.actions
            .last_mut()
            .expect("Cannot assign enabled state to empty collection.")
            .set_enabled(state);
        self
    }

    #[inline(always)]
    pub fn visible<T>(mut self, visible: T) -> Self
    where T: Into<bool> {
        self.actions
            .last_mut()
            .expect("Cannot assign visible state to empty collection.")
            .set_visible(visible);
        self
    }

    pub fn show(mut self, ui: &mut Ui) -> Response {
        // so that we can return the clicked button's response.
        let mut active_btn_response: Option<Response> = None;

        let resp = ui.horizontal(|ui| {
            let width = ui.available_width();
            let item_width = width / (self.actions.len() as f32);
            let size = Vec2::new(item_width, ui.spacing().interact_size.y);
            ui.spacing_mut().item_spacing.x = 0.0;
            for mut item in self.actions.into_iter() {

                ui.allocate_ui(size, |ui| {

                    let btn = Button::new(item.text.clone())
                        .small()
                        .stroke(Stroke::none());

                    ui.set_enabled(item.enabled);
                    ui.set_visible(item.visible);

                    let putter = |ui: &mut Ui| -> Response {
                        ui.put(ui.max_rect(), btn)
                    };

                    let mut btn_resp = item.id.take()
                        .map(|id| { 
                            ui.push_id(id, |ui| { 
                                ui.put(
                                    ui.max_rect(), 
                                    Button::new(item.text.clone())
                                        .small()
                                        .stroke(Stroke::none())
                                )
                            }).inner
                        })
                        .or_else(|| {
                            Some(
                                ui.put(
                                    ui.max_rect(),
                                    Button::new(item.text.clone())
                                        .small()
                                        .stroke(Stroke::none())
                                )
                            )
                        }).unwrap();
    
                    if btn_resp.clicked() {
                        btn_resp.mark_changed();
                        active_btn_response = Some(btn_resp);
                        item.invoke();
                    } else if btn_resp.hovered()
                    || btn_resp.has_focus() {
                        active_btn_response = Some(btn_resp);
                    }

                });

            }
        });

        if active_btn_response.is_some() {
            active_btn_response.unwrap().union(resp.response)
        } else {
            resp.response
        }
    }

}

#[macro_export]
macro_rules! count_vars {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + super::count_vars!($($xs)*));
}

#[macro_export]
macro_rules! function_bar {
    [ $ui:expr; $( $title:expr => $code:block) + ] => {
        let count = super::count_vars!($($title)*);
        let mut click_response: Option<Response> = None;
        let resp = $ui.horizontal(|ui| {
            let width = ui.available_width();
            let item_width = width / (count as f32);
            let size = Vec2::new(item_width, ui.available_height());
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.style_mut().visuals.widgets.inactive.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.active.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.hovered.rounding = Rounding::none();
            ui.style_mut().visuals.widgets.open.rounding = Rounding::none();
            $({
                let (_, rect) = ui.allocate_space(size);
                
                let btn = Button::new($title)
                    .small()
                    .stroke(Stroke::none());

                let mut btn_resp = ui.put(rect, btn);

                if btn_resp.clicked() {
                    btn_resp.mark_changed();
                    click_response = Some(btn_resp);
                    $code
                }
            })+
        });
        if click_response.is_some() {
            resp.response.union(click_response.unwrap())
        } else {
            resp.response
        }
        
    };
}

pub(crate) use function_bar;

pub struct Ballot<'a> {
    state: &'a mut bool,
    mark_color: Option<Color32>,
}

impl<'a> Ballot<'a> {
    fn new(state: &'a mut bool) -> Self {
        Self {
            state,
            mark_color: Some(Color32::from_rgb(255, 195, 0)),
        }
    }

    #[inline(always)]
    fn red(self) -> Self {
        self.color(Color32::RED)
    }

    #[inline(always)]
    fn yellow(self) -> Self {
        self.color(Color32::YELLOW)
    }

    #[inline(always)]
    fn green(self) -> Self {
        self.color(Color32::GREEN)
    }

    #[inline(always)]
    fn blue(self) -> Self {
        self.color(Color32::LIGHT_BLUE)
    }

    #[inline(always)]
    fn color(mut self, mark_color: impl Into<Color32>) -> Self {
        self.mark_color = Some(mark_color.into());
        self
    }
}

impl<'a> Widget for Ballot<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = ui.spacing().interact_size.y;
        let (rect, mut response) = ui.allocate_exact_size(vec2(size,size), Sense::click());

        if ui.is_rect_visible(rect) {
            
            let visuals = ui.style().interact(&response);

            let painter = ui.painter();
            painter.rect(
                rect,
                Rounding::none(),
                visuals.bg_fill,
                visuals.bg_stroke,
            );
            if *self.state {
                let dot_rect = rect.shrink(3.0);
                painter.rect(
                    dot_rect,
                    Rounding::none(),
                    self.mark_color.unwrap_or(visuals.fg_stroke.color),
                    visuals.bg_stroke,
                );  
            }
        }

        // By reacting after rendering, we can prevent the appearance changing in the same frame.
        // This prevents jarring effects in dynamic UIs.
        if response.clicked() {
            *self.state = !*self.state;
            response.mark_changed();
        }
        
        response
    }
}

// pub struct CallOnce<'a,T> {
//     callback: Option<Box<dyn FnOnce(T) + 'a>>
// }

// impl<'a,T> CallOnce<'a,T> {

//     fn new(callback: impl FnOnce(T) + 'a) -> Self {
//         Self {
//             callback: Some(Box::new(callback))
//         }
//     }

//     fn invoke(&mut self, value: T) {
//         if let Some(callback) = self.callback.take() {
//             callback(value);
//         }
//     }
// }

// pub struct TabCallback<'a, T> {

// }

// pub struct Tab<'a> {
//     title: WidgetText,
//     id: Id,

//     // TODO: More settings, such as whether or not this tab is closeable.
//     callback: CallOnce<'a, (&'a mut Ui, usize)>,
// }

// pub type TabList<'a> = Vec<Tab<'a>>;

// pub enum TabEvent {
//     None,
//     SwitchedTab { from: usize, to: usize },
//     RequestClose(usize),
// }

// pub struct TabResponse<R> {
//     inner: InnerResponse<R>,
// }

// impl<R> TabResponse<R> {
//     fn new( response: Response, result: R) -> Self {
//         Self {
//             inner: InnerResponse::new(result, response),
//         }
//     }
// }

// // This won't allow closing of tabs.
// pub struct TabBrowser<'a> {
//     tabs: TabList<'a>,
// }

pub trait UiExtensions {
    // All ui extensions can go here
    /// A button that fills the available width of the ui.
    fn wide_button(&mut self, text: impl Into<WidgetText>) -> Response;
    fn icon(&mut self, icon: Icon) -> Response;
    fn icon_button(&mut self, icon: Icon) -> Response;
    fn icon_small_button(&mut self, icon: Icon) -> Response;
    fn selectable_icon(&mut self, checked: bool, icon: Icon) -> Response;
    fn button_bar<R: Copy>(&mut self, items: &[(&str, R)]) -> InnerResponse<Option<R>>;
    fn ballot(&mut self, checked: &mut bool) -> Response;

}

impl UiExtensions for Ui {
    fn wide_button(&mut self, text: impl Into<WidgetText>) -> Response {
        let mut rect = self.max_rect();
        rect.set_height(rect.height().min(self.spacing().interact_size.y));
        self.put(rect, Button::new(text))
    }

    fn icon(&mut self, icon: Icon) -> Response {
        self.label(icon.text())
    }

    fn icon_button(&mut self, icon: Icon) -> Response {
        self.button(icon.text())
    }

    fn icon_small_button(&mut self, icon: Icon) -> Response {
        self.small_button(icon.text())
    }

    fn selectable_icon(&mut self, checked: bool, icon: Icon) -> Response {
        self.selectable_label(checked, icon.text())
    }

    fn button_bar<R: Copy>(&mut self, items: &[(&str, R)]) -> InnerResponse<Option<R>> {
        let mut output: Option<R> = None;
        let resp = self.add(ButtonBar::new(items, &mut output));
        InnerResponse { inner: output, response: resp }
    }

    fn ballot(&mut self, checked: &mut bool) -> Response {
        self.add(Ballot::new(checked))

    }

}