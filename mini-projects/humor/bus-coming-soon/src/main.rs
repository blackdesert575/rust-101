fn main() {
    let bus: Option<Bus> = None;

    println!("電子站牌顯示：{}", bus.coming_soon_message());
}

struct Bus;

impl Bus {
    fn coming_soon_message(&self) -> &'static str {
        "🚍 公車即將抵達（Coming Soon）"
    }
}

trait BusInfo {
    fn coming_soon_message(&self) -> &'static str;
}

impl BusInfo for Option<Bus> {
    fn coming_soon_message(&self) -> &'static str {
        match self {
            Some(bus) => bus.coming_soon_message(),
            None => "🚫 公車沒有要來可以不要寫 coming soon 嗎。",
        }
    }
}