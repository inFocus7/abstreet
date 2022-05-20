use widgetry::Color;

lazy_static::lazy_static! {
    /// Rotate through these colors for neighborhoods. Use 4-color (ehem, 6-color?) theorem to make
    /// adjacent things different
    pub static ref NEIGHBORHOODS: [Color; 6] = [
        Color::BLUE.alpha(0.3),
        Color::YELLOW.alpha(0.3),
        Color::GREEN.alpha(0.3),
        Color::PURPLE.alpha(0.3),
        Color::PINK.alpha(0.3),
        Color::ORANGE.alpha(0.3),
    ];

    pub static ref CELLS: [Color; 12] = [
        Color::hex("#8dd3c7"),
        Color::hex("#ffffb3"),
        Color::hex("#bebada"),
        Color::hex("#fb8072"),
        Color::hex("#80b1d3"),
        Color::hex("#fdb462"),
        Color::hex("#b3de69"),
        Color::hex("#fccde5"),
        Color::hex("#d9d9d9"),
        Color::hex("#bc80bd"),
        Color::hex("#ccebc5"),
        Color::hex("#ffed6f"),
    ];

    pub static ref FILTER_OUTER: Color = Color::RED;
    pub static ref FILTER_INNER: Color = Color::WHITE;

    pub static ref PLAN_ROUTE_BEFORE: Color = Color::RED;
    pub static ref PLAN_ROUTE_AFTER: Color = Color::CYAN;
    pub static ref PLAN_ROUTE_BIKE: Color = Color::GREEN;
    pub static ref PLAN_ROUTE_WALK: Color = Color::BLUE;
}

pub const DISCONNECTED_CELL: Color = Color::RED.alpha(0.5);

pub const OUTLINE: Color = Color::BLACK;

pub const HIGHLIGHT_BOUNDARY_UNZOOMED: Color = Color::RED.alpha(0.6);
pub const HIGHLIGHT_BOUNDARY_ZOOMED: Color = Color::RED.alpha(0.5);

pub const SHORTCUT_PATH: Color = Color::RED;

pub const BLOCK_IN_BOUNDARY: Color = Color::BLUE.alpha(0.5);
pub const BLOCK_IN_FRONTIER: Color = Color::CYAN.alpha(0.2);
