use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    html! {
        <form method="GET" class="filters" id="search">
            <div class="input-fields">
                <label class="form-group location">
                    <input class="form-control control-left" type="text"
                        placeholder="Enter a city, address, postal code, etc..." name="location" />
                    <div class="absolute-container">
                        <small class="absolute-inner location-feedback" hidden={true}></small>
                    </div>
                </label>
                <label class="form-group distance">
                    <select name="distance" class="form-control control-right">
                        <option value="10">{"10 km"}</option>
                        <option value="25">{"25 km"}</option>
                        <option value="50">{"50 km"}</option>
                        <option value="100" selected={true}>{"100 km"}</option>
                        <option value="200">{"200 km"}</option>
                        <option value="400">{"400 km"}</option>
                        <option value="800">{"800 km"}</option>
                        <option value="all">{"> 200 km"}</option>
                    </select>
                </label>
            </div>
            <button class="submit">{"Search"}</button>
        </form>
    }
}