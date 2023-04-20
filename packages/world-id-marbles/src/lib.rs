#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::Result;
use indoc::formatdoc;
use resvg::{
    tiny_skia::{Color, Pixmap, Transform},
    usvg::{self, TreeParsing},
};
use std::{fmt::Debug, fs, path::Path};

pub use ruint::aliases::U256;

const COLORS: [&str; 36] = [
    "#FF0000", "#FF2B00", "#FF5500", "#FF8000", "#FFAA00", "#FFD500", "#FFFF00", "#D4FF00",
    "#AAFF00", "#80FF00", "#55FF00", "#2BFF00", "#00FF00", "#00FF2A", "#00FF2A", "#00FF80",
    "#00FFAA", "#00FFD4", "#00FFFF", "#00D4FF", "#00AAFF", "#0080FF", "#0055FF", "#002AFF",
    "#0000FF", "#2A00FF", "#0000FF", "#5500FF", "#8000FF", "#AA00FF", "#D500FF", "#FF00FF",
    "#FF00D5", "#FF00AA", "#FF0080", "#FF0055",
];

pub trait Seedable {
    fn to_seed(self) -> U256;
}

impl Seedable for u32 {
    fn to_seed(self) -> U256 {
        U256::try_from(self).unwrap()
    }
}
impl Seedable for U256 {
    fn to_seed(self) -> U256 {
        self
    }
}

impl Seedable for usize {
    fn to_seed(self) -> U256 {
        U256::try_from(self).unwrap()
    }
}

impl Seedable for String {
    fn to_seed(self) -> U256 {
        U256::from_str_radix(&self, 10).unwrap()
    }
}
impl Seedable for &str {
    fn to_seed(self) -> U256 {
        U256::from_str_radix(self, 10).unwrap()
    }
}

pub struct Marble {
    seed: U256,
}

impl Marble {
    /// Create a new marble with the given seed.
    ///
    /// # Panics
    ///
    /// Panics if the seed cannot be converted to a `U256`.
    pub fn new(seed: impl Seedable) -> Self {
        Self {
            seed: seed.to_seed(),
        }
    }

    fn random_number<T, E>(&mut self, max: T) -> T
    where
        E: Debug,
        T: Seedable + TryFrom<U256, Error = E> + Copy,
    {
        let max = max.to_seed();

        let result = self.seed % max;
        self.seed /= max;

        result.try_into().unwrap()
    }

    fn random_sort<T>(&mut self, mut arr: Vec<T>) -> Vec<T> {
        let mut shuffled = vec![];

        while !arr.is_empty() {
            let k = self.random_number(arr.len());

            shuffled.push(arr.remove(k));
        }

        shuffled
    }

    fn random_color(&mut self) -> &str {
        COLORS[self.random_number(COLORS.len())]
    }

    /// Build the SVG for the marble.
    #[must_use]
    pub fn build_svg(&mut self) -> String {
        let shapes = vec![
            formatdoc!(
                r#"
                <g filter="url(#blur)" opacity=".9">
                    <ellipse cx="33.545" cy="32.494" fill="{color}" rx="33.545" ry="32.494" transform="matrix(-.48289 -.87568 .7985 -.602 9.46 74.034)"/>
                </g>
            "#,
                color = self.random_color()
            ),
            formatdoc!(
                r#"
                <g filter="url(#blur)" opacity=".8">
                    <path fill="{color}" d="M78.824-16.686c17.78 14.541 4.24 87.76-2.637 82.948-4.194-2.935-9.153-27.765-22.32-38.405-8.418-6.802-23.488-1.839-33.086-1.137-24.614 1.8 40.115-58.069 58.043-43.406Z"/>
                </g>
            "#,
                color = self.random_color()
            ),
            formatdoc!(
                r#"
                <g filter="url(#blur)" opacity=".8">
                    <ellipse cx="39.533" cy="39.042" fill="{color}" rx="39.533" ry="39.042" transform="matrix(-.2882 -.95757 .93652 -.35062 13.847 67.74)" />
                </g>
            "#,
                color = self.random_color()
            ),
        ];

        formatdoc!(
            r##"
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 80 80" transform="rotate({rotation} 40 40)">
                    <g clip-path="url(#a)">
                        <circle cx="40" cy="40" r="40" fill="#F8F8F8" />
                        {shapes}
                    </g>
                    <defs>
                        <filter id="blur" width="300" height="300" x="0" y="0" color-interpolation-filters="sRGB" filterUnits="userSpaceOnUse">
                            <feGaussianBlur result="effect1_foregroundBlur_557_59789" stdDeviation="9.6" />
                        </filter>
                        <clipPath id="a">
                            <rect width="80" height="80" fill="#fff" rx="40" />
                        </clipPath>
                    </defs>
                </svg>
        "##,
            shapes = self.random_sort(shapes).join(""),
            rotation = self.random_number(359_u32),
        )
    }

    /// Render the marble as a PNG.
    /// The PNG is returned as a vector of bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if the marble cannot be rendered.
    /// This can happen if the SVG fails to be parsed or the PNG cannot be encoded.
    pub fn render_png(&mut self, size: u32) -> Result<Vec<u8>> {
        let svg = self.build_svg();
        let tree = usvg::Tree::from_data(svg.as_bytes(), &usvg::Options::default())?;

        let mut pixmap = Pixmap::new(size, size).ok_or_else(|| {
            anyhow::anyhow!("Failed to create pixmap with size {}x{}", size, size)
        })?;
        pixmap.fill(Color::TRANSPARENT);

        resvg::render(
            &tree,
            resvg::FitTo::Width(size),
            Transform::default(),
            pixmap.as_mut(),
        )
        .ok_or_else(|| anyhow::anyhow!("Failed to render SVG"))?;

        Ok(pixmap.encode_png()?)
    }

    /// Save the marble as a PNG.
    ///
    /// # Errors
    ///
    /// This function will return an error if the marble cannot be rendered or saved.
    pub fn save_png<P: AsRef<Path>>(&mut self, size: u32, path: P) -> Result<()> {
        fs::write(path, self.render_png(size)?)?;

        Ok(())
    }
}
