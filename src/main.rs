#[macro_use]
extern crate cssparser;

/// Define an enum type with unit variants that each correspond to a CSS keyword.
macro_rules! define_css_keyword_enum {
    (pub enum $name:ident { $($variant:ident = $css:expr,)+ }) => {
        #[allow(missing_docs)]
        #[cfg_attr(feature = "servo", derive(Deserialize, Serialize))]
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            /// Parse this property from an already-tokenized identifier.
            pub fn from_ident(ident: &str) -> Result<$name, ()> {
                match_ignore_ascii_case! { ident,
                    $($css => Ok($name::$variant),)+
                    _ => Err(())
                }
            }
        }
    };
}

define_css_keyword_enum! {
    pub enum Orientation {
        Auto = "auto",
        Portrait = "portrait",
        Landscape = "landscape",
    }
}

fn main() {}
