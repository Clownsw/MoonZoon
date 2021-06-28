use crate::*;
use std::collections::BTreeMap;
use std::borrow::Cow;

mod background;
mod font;
mod color;
mod padding;

pub use background::Background;
pub use font::Font;
pub use color::{Color, NamedColor};
pub use padding::Padding;

type StaticCSSProps<'a> = BTreeMap<&'a str, Cow<'a, str>>;
type DynamicCSSProps = BTreeMap<&'static str, BoxedCssSignal>;

type BoxedCssSignal = Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>;

fn box_css_signal(signal: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static) -> BoxedCssSignal {
    Box::new(signal.map(|value| {
        Box::new(value) as Box<dyn IntoOptionCowStr<'static>>
    }))
}

pub trait Style<'a>: Default {
    fn new() -> Self {
        Self::default()
    }
    
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps);

    fn update_raw_el_style<T: RawEl>(self, mut raw_el: T) -> T {
        let (static_css_props, dynamic_css_props) = self.into_css_props();
        for (name, value) in static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        for (name, value) in dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        raw_el
    }

}




