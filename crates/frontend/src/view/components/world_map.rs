use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;

use crate::view::components::world_map::leaflet::{log, MapPoint};

mod leaflet {
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::{JsObject, JsValue};
    use web_sys::{Element, HtmlElement, Node};

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MapPoint {
        pub lng: f64,
        pub lat: f64,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MapOptions {
        pub zoom_control: bool,
        pub zoom: f64,
        pub min_zoom: f64,
        pub center: MapPoint,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LayerOptions {
        pub max_zoom: f64,
        pub no_wrap: bool,
        pub attribution: &'static str,
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(value: &JsValue);

        pub type Map;

        #[wasm_bindgen(js_namespace = L)]
        pub fn map(element: &HtmlElement, opts: &JsValue) -> Map;

        #[wasm_bindgen(method)]
        pub fn invalidateSize(this: &Map, animate: bool);

        #[wasm_bindgen(method)]
        pub fn setView(this: &Map, center: &JsValue, zoom: f64);

        pub type TileLayer;

        #[wasm_bindgen(constructor, js_namespace = L)]
        pub fn new(url_template: &str, options: &JsValue) -> TileLayer;

        #[wasm_bindgen(method)]
        pub fn addTo(this: &TileLayer, map: &Map);
    }
}

pub struct WorldMap {
    container: HtmlElement,
    map: leaflet::Map,
}

impl WorldMap {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}

impl Component for WorldMap {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");

        let options = leaflet::MapOptions {
            zoom: 2.0,
            min_zoom: 2.0,
            zoom_control: true,
            center: MapPoint { lat: 0.0, lng: 0.0 },
        };

        let map = leaflet::map(&container, &serde_wasm_bindgen::to_value(&options).unwrap());

        let layer_options = leaflet::LayerOptions {
            max_zoom: 16.0,
            no_wrap: true,
            attribution: "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors",
        };

        let layer = leaflet::TileLayer::new(
            "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
            &serde_wasm_bindgen::to_value(&layer_options).unwrap(),
        );

        layer.addTo(&map);

        Self { container, map }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            {self.render_map()}
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.map.invalidateSize(false);
        }
    }
}
