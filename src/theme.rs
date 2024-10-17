use csscolorparser::Color;

pub struct Theme {
    background: Color,
    inputBackground: Color,
    color: Color,
    borderColor: Color,
    borderRadius: String,
    boxShadow: String,
    width: String,
}

impl Theme {
    pub fn default() -> Self {
        Self::light()
    }
    pub fn light() -> Self {
        Self {
            background: "#fff".parse::<Color>().unwrap(),
            inputBackground: "#f4f4f4".parse::<Color>().unwrap(),
            color: "#262626".parse::<Color>().unwrap(),
            borderColor: "#d4d4d4".parse::<Color>().unwrap(),
            borderRadius: "4px".to_string(),
            boxShadow: "0px 8px 16px rgba(0, 0, 0, 0.1)".to_string(),
            width: "280px".to_string(),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: "rgba(40, 40, 40, 0.95)".parse::<Color>().unwrap(),
            inputBackground: "#454545".parse::<Color>().unwrap(),
            color: "#e3e3e3".parse::<Color>().unwrap(),
            borderColor: "#575657".parse::<Color>().unwrap(),
            borderRadius: "4px".to_string(),
            boxShadow: "0px 8px 16px rgba(0, 0, 0, 0.1)".to_string(),
            width: "280px".to_string(),
        }
    }

    pub fn custom(
        background: Color,
        input_background: Color,
        color: Color,
        border_color: Color,
        border_radius: String,
        box_shadow: String,
        width: String,
    ) -> Self {
        Self {
            background,
            inputBackground: input_background,
            color,
            borderColor: border_color,
            borderRadius: border_radius,
            boxShadow: box_shadow,
            width,
        }
    }

    pub fn background(&mut self, background: Color) -> &mut Self {
        self.background = background;
        self
    }

    pub fn input_background(&mut self, input_background: Color) -> &mut Self {
        self.inputBackground = input_background;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn border_color(&mut self, border_color: Color) -> &mut Self {
        self.borderColor = border_color;
        self
    }

    pub fn border_radius(&mut self, border_radius: String) -> &mut Self {
        self.borderRadius = border_radius;
        self
    }

    pub fn box_shadow(&mut self, box_shadow: String) -> &mut Self {
        self.boxShadow = box_shadow;
        self
    }

    pub fn width(&mut self, width: String) -> &mut Self {
        self.width = width;
        self
    }

    pub fn to_style(&self) -> String {
        format!(
            "--lpc-background: {}; \
                 --lpc-input-background: {}; \
                 --lpc-color: {}; \
                 --lpc-border-color: {}; \
                 --lpc-border-radius: {}; \
                 --lpc-box-shadow: {}; \
                 --lpc-width: {};",
            self.background.to_hex_string(),
            self.inputBackground.to_hex_string(),
            self.color.to_hex_string(),
            self.borderColor.to_hex_string(),
            self.borderRadius,
            self.boxShadow,
            self.width
        )
    }
}
