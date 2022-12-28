#![feature(extern_types)]

use view::App;

mod view;

fn main() {
    yew::Renderer::<App>::new().render();
}