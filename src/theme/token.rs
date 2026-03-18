pub mod space {
    /// 4px
    pub const XS: f32 = 4.0;
    /// 8px
    pub const SM: f32 = 8.0;
    /// 12px
    pub const MD: f32 = 12.0;
    /// 16px
    pub const LG: f32 = 16.0;
    /// 24px
    pub const XL: f32 = 24.0;
    /// 32px
    pub const XXL: f32 = 32.0;
}

pub mod radius {
    /// 6px
    pub const SM: f32 = 6.0;
    /// 10px
    pub const MD: f32 = 10.0;
    /// 14px
    pub const LG: f32 = 14.0;
    /// 999px — full pill
    pub const PILL: f32 = 999.0;
}

pub mod color {
    use iced::Color;

    /// #000000 — black background
    pub const BG: Color = Color::from_rgb(0.00, 0.00, 0.00);
    /// #FFFFFF — white text
    pub const TEXT: Color = Color::from_rgb(1.00, 1.00, 1.00);
    /// #FFFFFF — white
    pub const PRIMARY: Color = Color::from_rgb(1.00, 1.00, 1.00);
    /// #1FA659 — green
    pub const SUCCESS: Color = Color::from_rgb(0.12, 0.65, 0.35);
    /// #E69E1F — amber
    pub const WARNING: Color = Color::from_rgb(0.90, 0.62, 0.12);
    /// #CC3D3D — red
    pub const DANGER: Color = Color::from_rgb(0.80, 0.24, 0.24);
}
