use crate::{Settings, Viewport};
use fontdue::layout::{GlyphPosition, GlyphRasterConfig};
use fontdue::Metrics;
use iced_graphics::backend;
use iced_graphics::font;
use iced_graphics::Primitive;
use iced_native::mouse;
use iced_native::{Font, HorizontalAlignment, Size, VerticalAlignment};
use log::warn;
use std::{collections::HashMap, fmt, sync::Mutex};

/// A [`raqote`] graphics backend for [`iced`].
///
/// [`raqote`]: https://github.com/jrmuizel/raqote
/// [`iced`]: https://github.com/hecrj/iced
pub struct Backend {
    text_layout: Mutex<fontdue::layout::Layout>,
    glyph_positions: Mutex<Vec<GlyphPosition>>,
    fonts: Mutex<HashMap<&'static str, fontdue::Font>>,
    fallback_font: fontdue::Font,
    glyph_cache: HashMap<GlyphRasterConfig, (Metrics, Vec<u8>)>,
    default_text_size: u16,
}

impl Backend {
    /// Creates a new [`Backend`].
    ///
    /// [`Backend`]: struct.Backend.html
    pub fn new(settings: Settings) -> Self {
        Self {
            text_layout: Mutex::new(fontdue::layout::Layout::new()),
            glyph_positions: Mutex::new(Vec::new()),
            fonts: Mutex::new(HashMap::new()),
            fallback_font: fontdue::Font::from_bytes(
                font::FALLBACK,
                Default::default(),
            )
            .unwrap(),
            glyph_cache: HashMap::new(),
            default_text_size: settings.default_text_size,
        }
    }

    /// Draws the provided primitives in the default framebuffer.
    ///
    /// The text provided as overlay will be rendered on top of the primitives.
    /// This is useful for rendering debug information.
    pub fn draw<T: AsRef<str>>(
        &mut self,
        draw_target: &mut raqote::DrawTarget,
        viewport: &Viewport,
        (primitive, mouse_interaction): &(Primitive, mouse::Interaction),
        overlay_text: &[T],
    ) -> mouse::Interaction {
        let viewport_size = viewport.physical_size();
        let scale_factor = viewport.scale_factor() as f32;

        self.draw_primitive(
            draw_target,
            viewport_size,
            scale_factor,
            primitive,
        );

        *mouse_interaction
    }

    fn draw_primitive(
        &mut self,
        draw_target: &mut raqote::DrawTarget,
        viewport_size: Size<u32>,
        scale_factor: f32,
        primitive: &Primitive,
    ) {
        match primitive {
            Primitive::None => {}
            Primitive::Group { primitives } => {
                for primitive in primitives {
                    self.draw_primitive(
                        draw_target,
                        viewport_size,
                        scale_factor,
                        primitive,
                    );
                }
            }
            Primitive::Text {
                content,
                bounds,
                color,
                size,
                font,
                horizontal_alignment,
                vertical_alignment,
            } => {
                let layout_settings = fontdue::layout::LayoutSettings {
                    x: (bounds.x * scale_factor),
                    y: (bounds.y * scale_factor),
                    max_width: Some(bounds.width * scale_factor),
                    max_height: Some(bounds.height * scale_factor),
                    horizontal_align: match horizontal_alignment {
                        HorizontalAlignment::Left => {
                            fontdue::layout::HorizontalAlign::Left
                        }
                        HorizontalAlignment::Center => {
                            fontdue::layout::HorizontalAlign::Center
                        }
                        HorizontalAlignment::Right => {
                            fontdue::layout::HorizontalAlign::Right
                        }
                    },
                    vertical_align: match vertical_alignment {
                        VerticalAlignment::Top => {
                            fontdue::layout::VerticalAlign::Top
                        }
                        VerticalAlignment::Center => {
                            fontdue::layout::VerticalAlign::Middle
                        }
                        VerticalAlignment::Bottom => {
                            fontdue::layout::VerticalAlign::Bottom
                        }
                    },
                    wrap_style: fontdue::layout::WrapStyle::Word,
                    wrap_hard_breaks: true,
                    include_whitespace: false,
                };
                let mut fonts = self.fonts.lock().unwrap();
                let font = match font {
                    Font::Default => &self.fallback_font,
                    Font::External { name, bytes } => {
                        if fonts.contains_key(name) {
                            fonts.get(name).unwrap()
                        } else {
                            match fontdue::Font::from_bytes(
                                *bytes,
                                Default::default(),
                            ) {
                                Ok(ok) => fonts.entry(name).or_insert(ok),
                                Err(err) => {
                                    warn!(
                                        r#"Using fallback font due error while loading "{}": "{}""#,
                                        name, err
                                    );
                                    &self.fallback_font
                                }
                            }
                        }
                    }
                };
                let mut glyph_positions = self.glyph_positions.lock().unwrap();
                glyph_positions.clear();
                self.text_layout.lock().unwrap().layout_horizontal(
                    &[font],
                    &[&fontdue::layout::TextStyle {
                        text: content.as_ref(),
                        px: *size,
                        font_index: 0,
                    }],
                    &layout_settings,
                    &mut glyph_positions,
                );
                for (c, glyph_pos) in
                    content.chars().zip(glyph_positions.drain(..))
                {
                    let (metrics, coverage) = self
                        .glyph_cache
                        .entry(GlyphRasterConfig {
                            c,
                            px: *size,
                            font_index: 0,
                        })
                        .or_insert_with(|| font.rasterize(c, *size));
                    let mut image_data = Vec::with_capacity(coverage.len());
                    for cov in coverage.iter() {
                        // FIXME: Color space
                        let pixel = (((color.a * *cov as f32).floor() as u32)
                            << 24)
                            | (((color.r * *cov as f32).floor() as u32) << 16)
                            | (((color.g * *cov as f32).floor() as u32) << 8)
                            | ((color.b * *cov as f32).floor() as u32);

                        image_data.push(pixel);
                    }
                    draw_target.draw_image_at(
                        glyph_pos.x,
                        glyph_pos.y,
                        &raqote::Image {
                            width: metrics.width as i32,
                            height: metrics.height as i32,
                            data: &image_data,
                        },
                        &raqote::DrawOptions {
                            blend_mode: raqote::BlendMode::SrcOver,
                            alpha: 1.0,
                            antialias: raqote::AntialiasMode::None,
                        },
                    );
                }
            }
            Primitive::Quad {
                bounds,
                background,
                border_radius,
                border_width,
                border_color,
            } => {
                //
            }
            Primitive::Image { handle, bounds } => {
                //
            }
            Primitive::Svg { handle, bounds } => {
                //
            }
            Primitive::Clip {
                bounds,
                offset,
                content,
            } => {
                //
            }
            Primitive::Translate {
                translation,
                content,
            } => {
                //
            }
            Primitive::Mesh2D { buffers, size } => {
                //
            }
            Primitive::Cached { cache } => {
                self.draw_primitive(
                    draw_target,
                    viewport_size,
                    scale_factor,
                    &*cache,
                );
            }
        }
    }
}

impl fmt::Debug for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Backend")
            .field("draw_target", &"DrawTarget { ... }")
            .field("default_text_size", &self.default_text_size)
            .finish()
    }
}

impl iced_graphics::Backend for Backend {
    fn trim_measurements(&mut self) {
        //
    }
}

impl backend::Text for Backend {
    const ICON_FONT: Font = font::ICONS;
    const CHECKMARK_ICON: char = font::CHECKMARK_ICON;
    const ARROW_DOWN_ICON: char = font::ARROW_DOWN_ICON;

    fn default_size(&self) -> u16 {
        self.default_text_size
    }

    fn measure(
        &self,
        contents: &str,
        size: f32,
        font: Font,
        bounds: Size,
    ) -> (f32, f32) {
        let mut fonts = self.fonts.lock().unwrap();
        let font = match font {
            Font::Default => &self.fallback_font,
            Font::External { name, bytes } => {
                if fonts.contains_key(name) {
                    fonts.get(name).unwrap()
                } else {
                    match fontdue::Font::from_bytes(bytes, Default::default()) {
                        Ok(ok) => fonts.entry(name).or_insert(ok),
                        Err(err) => {
                            warn!(
                                r#"Using fallback font due error while loading "{}": "{}""#,
                                name, err
                            );
                            &self.fallback_font
                        }
                    }
                }
            }
        };

        let layout_settings = fontdue::layout::LayoutSettings {
            x: 0.0,
            y: 0.0,
            max_width: Some(bounds.width),
            max_height: Some(bounds.height),
            horizontal_align: fontdue::layout::HorizontalAlign::Left,
            vertical_align: fontdue::layout::VerticalAlign::Top,
            wrap_style: fontdue::layout::WrapStyle::Word,
            wrap_hard_breaks: true,
            include_whitespace: false,
        };

        let mut glyph_positions = self.glyph_positions.lock().unwrap();
        self.text_layout.lock().unwrap().layout_horizontal(
            &[font],
            &[&fontdue::layout::TextStyle {
                text: contents,
                px: size,
                font_index: 0,
            }],
            &layout_settings,
            &mut glyph_positions,
        );

        let width = glyph_positions
            .iter()
            .fold(0.0f32, |acc, pos| acc.max(pos.x + pos.width as f32));
        let height = glyph_positions
            .iter()
            .fold(0.0f32, |acc, pos| acc.max(pos.y));

        (width, height)
    }
}

#[cfg(feature = "image")]
impl backend::Image for Backend {
    fn dimensions(&self, _handle: &iced_native::image::Handle) -> (u32, u32) {
        (50, 50)
    }
}

#[cfg(feature = "svg")]
impl backend::Svg for Backend {
    fn viewport_dimensions(
        &self,
        _handle: &iced_native::svg::Handle,
    ) -> (u32, u32) {
        (50, 50)
    }
}
