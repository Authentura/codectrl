use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct FontSizes {
    pub body: f32,
    pub button: f32,
    pub heading: f32,
    pub monospace: f32,
    pub extra_large: f32,
}

impl FontSizes {
    pub fn scale(&mut self, scale: f32) {
        if (*self == Self::smallest() && scale < 0.0)
            || (*self == Self::largest() && scale > 0.0)
        {
            return;
        }

        self.body += scale;
        self.button += scale;
        self.heading += scale;
        self.monospace += scale;
        self.extra_large += scale;
    }

    pub fn smallest() -> Self {
        Self {
            body: 4.0,
            button: 4.0,
            heading: 12.0,
            monospace: 4.0,
            extra_large: 24.0,
        }
    }

    pub fn largest() -> Self {
        Self {
            body: 28.0,
            button: 28.0,
            heading: 36.0,
            monospace: 28.0,
            extra_large: 48.0,
        }
    }
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            body: 16.0,
            button: 16.0,
            heading: 24.0,
            monospace: 16.0,
            extra_large: 36.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ApplicationSettings {
    pub font_sizes: FontSizes,
}
