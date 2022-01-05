#![allow(dead_code)]
use gloo_console as console;
use gloo_timers::callback::Interval;
use js_sys::Date;
use yew::{html, Component, Context, Html};

struct CurrTime {
    hours: u32,
    minutes: u32,
    seconds: u32,
    year: u32,
    mouth: u32,
    date: u32,
}

impl CurrTime {
    fn new() -> Self {
        let date = Date::new_0();
        Self {
            hours: date.get_hours(),
            minutes: date.get_minutes(),
            seconds: date.get_seconds(),
            year: date.get_full_year(),
            mouth: date.get_month(),
            date: date.get_date(),
        }
    }

    fn renew(&mut self) {
        let date = Date::new_0();
        self.hours = date.get_hours();
        self.minutes = date.get_minutes();
        self.seconds = date.get_seconds();
        self.year = date.get_full_year();
        self.mouth = date.get_month();
        self.date = date.get_date();
    }
}

fn padding_zero(val: u32) -> String {
    if val < 10 {
        format!("0{}", val)
    } else {
        val.to_string()
    }
}

enum Msg {
    Tick,
    ChangeMode,
    HoverSetting,
    LeaveSetting,
    SettingClick,
    SwitchMode,
    ShowDate,
}

enum TimeMode {
    HM,  // 显示小时/分钟
    HMS, // 显示小时/分钟/秒钟
}

struct App {
    curr_time: CurrTime,
    interval: Interval,
    mode: TimeMode,
    fill_color: &'static str,
    rotate_style: &'static str,
    is_setting_show: bool,
    is_locale_date_show: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let callback = _ctx.link().callback(|_| Msg::Tick);
        let interval = Interval::new(1000, move || callback.emit(()));
        Self {
            curr_time: CurrTime::new(),
            interval,
            mode: TimeMode::HM,
            fill_color: "#535c68",
            rotate_style: "",
            is_setting_show: false,
            is_locale_date_show: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.curr_time.renew();
                true
            }
            Msg::ChangeMode => {
                console::log!("");
                true
            }
            Msg::HoverSetting => {
                self.rotate_style = "transform:rotate(180deg)";
                self.fill_color = "#f2f2f2";
                true
            }
            Msg::LeaveSetting => {
                self.rotate_style = "transform:rotate(0deg)";
                self.fill_color = "#535c68";
                true
            }
            Msg::SettingClick => {
                self.is_setting_show = !self.is_setting_show;
                true
            }
            Msg::SwitchMode => {
                match self.mode {
                    TimeMode::HM => {
                        self.mode = TimeMode::HMS;
                    }
                    TimeMode::HMS => {
                        self.mode = TimeMode::HM;
                    }
                }
                self.is_setting_show = false;
                true
            }
            Msg::ShowDate => {
                self.is_locale_date_show = !self.is_locale_date_show;
                self.is_setting_show = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                if self.is_locale_date_show  {
                    <div class="localeDate">{format!("{}/{}/{}", self.curr_time.year, self.curr_time.mouth + 1, self.curr_time.date)}</div>
                }
                <div class="clock">
                    <span class="hours">{padding_zero(self.curr_time.hours)}</span>
                    <i class="dot">{":"}</i>
                    <span class="minutes">{padding_zero(self.curr_time.minutes)}</span>
                    if let TimeMode::HM = self.mode  {
                        <i class="dot">{":"}</i>
                        <span class="seconds">{padding_zero(self.curr_time.seconds)}</span>
                    }
                </div>
                <div class="setting"
                    onmouseover={ctx.link().callback(|_| Msg::HoverSetting)}
                    onmouseleave={ctx.link().callback(|_| Msg::LeaveSetting)}
                    style={self.rotate_style}
                    onclick={ctx.link().callback(|_| Msg::SettingClick)}
                >
                    <svg t="1612362198950" id="setting" class="icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="6475" data-darkreader-inline-fill="" width="48" height="48"
                        >
                        <path id="path"
                        d="M94.421333 599.893333a424.32 424.32 0 0 1 0-175.786666c47.36 5.546667 88.746667-10.112 102.229334-42.709334 13.525333-32.64-4.608-72.96-42.069334-102.528a424.32 424.32 0 0 1 124.288-124.288c29.525333 37.418667 69.888 55.594667 102.528 42.069334 32.64-13.525333 48.298667-54.869333 42.709334-102.229334a424.32 424.32 0 0 1 175.786666 0c-5.546667 47.36 10.112 88.746667 42.709334 102.229334 32.64 13.525333 72.96-4.608 102.528-42.069334a424.32 424.32 0 0 1 124.288 124.288c-37.418667 29.525333-55.594667 69.888-42.069334 102.528 13.525333 32.64 54.869333 48.298667 102.229334 42.709334a424.32 424.32 0 0 1 0 175.786666c-47.36-5.546667-88.746667 10.112-102.229334 42.709334-13.525333 32.64 4.608 72.96 42.069334 102.528a424.32 424.32 0 0 1-124.288 124.288c-29.525333-37.418667-69.888-55.594667-102.528-42.069334-32.64 13.525333-48.298667 54.869333-42.709334 102.229334a424.32 424.32 0 0 1-175.786666 0c5.546667-47.36-10.112-88.746667-42.709334-102.229334-32.64-13.525333-72.96 4.608-102.528 42.069334a424.32 424.32 0 0 1-124.288-124.288c37.418667-29.525333 55.594667-69.888 42.069334-102.528-13.525333-32.64-54.869333-48.298667-102.229334-42.709334zM170.666667 520.96c46.933333 13.013333 85.632 42.752 104.832 89.002667 19.157333 46.293333 12.8 94.72-11.178667 137.045333 4.096 4.352 8.32 8.576 12.672 12.672 42.368-23.978667 90.752-30.293333 137.045333-11.178667 46.250667 19.2 75.989333 57.898667 89.002667 104.832 5.973333 0.170667 11.946667 0.170667 17.92 0 13.013333-46.933333 42.752-85.632 89.002667-104.832 46.293333-19.157333 94.72-12.8 137.045333 11.178667 4.352-4.096 8.576-8.32 12.672-12.672-23.978667-42.368-30.293333-90.752-11.178667-137.045333 19.2-46.250667 57.898667-75.989333 104.832-89.002667 0.170667-5.973333 0.170667-11.946667 0-17.92-46.933333-13.013333-85.632-42.752-104.832-89.002667-19.157333-46.293333-12.8-94.72 11.178667-137.045333a338.56 338.56 0 0 0-12.672-12.672c-42.368 23.978667-90.752 30.293333-137.045333 11.178667C563.712 256.298667 533.973333 217.6 520.96 170.666667a338.56 338.56 0 0 0-17.92 0c-13.013333 46.933333-42.752 85.632-89.002667 104.832-46.293333 19.157333-94.72 12.8-137.045333-11.178667a295.765333 295.765333 0 0 0-12.672 12.672c23.978667 42.368 30.293333 90.752 11.178667 137.045333C256.298667 460.288 217.6 490.026667 170.666667 503.04c-0.170667 5.973333-0.170667 11.946667 0 17.92zM512 640a128 128 0 1 1 0-256 128 128 0 0 1 0 256z m0-85.333333a42.666667 42.666667 0 1 0 0-85.333334 42.666667 42.666667 0 0 0 0 85.333334z"
                        p-id="6476" fill={self.fill_color}></path>
                    </svg>
                </div>
                if self.is_setting_show {
                    <div class="nav">
                        <ul>
                        <li onclick={ctx.link().callback(|_| Msg::ShowDate)}>{"Show Date"}</li>
                        <li onclick={ctx.link().callback(|_| Msg::SwitchMode)}>{"Switch Mode"}</li>
                        </ul>
                    </div>
                }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
