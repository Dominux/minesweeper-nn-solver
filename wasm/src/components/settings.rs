use std::str::FromStr;

use yew::prelude::*;

#[function_component(Settings)]
pub fn settings_component() -> Html {
    let height = use_state(|| 10_u16);
    let width = use_state(|| 10_u16);
    let bombs = use_state(|| 10_u16);
    let is_valid = use_state(|| true);

    let oninput_height = {
        let height = height.clone();
        Callback::from(move |e: Event| {
            if let Some(n) = parse_input_event(e) {
                height.set(n)
            }
        })
    };

    let oninput_width = {
        let width = width.clone();
        Callback::from(move |e: Event| {
            if let Some(n) = parse_input_event(e) {
                width.set(n)
            }
        })
    };

    let oninput_bombs = {
        let bombs = bombs.clone();
        Callback::from(move |e: Event| {
            if let Some(n) = parse_input_event(e) {
                bombs.set(n)
            }
        })
    };

    // Validating bombs amount
    {
        let is_valid = is_valid.clone();
        use_effect_with_deps(
            move |(height, width, bombs)| {
                let new_is_valid = ((*height.clone()) * (*width.clone())) > *bombs.clone();
                is_valid.set(new_is_valid);
                || ()
            },
            (height.clone(), width.clone(), bombs.clone()),
        );
    }

    // TODO: add styles
    // TODO: bind emitting values into parent component by submitting form
    html!(
        <form action="">
            <div>
                <pre>{"Height"}</pre>
                <input
                    value={height.to_string()}
                    id="height"
                    type="number"
                    min="1"
                    max={u8::MAX.to_string()}
                    onchange={oninput_height}
                />
            </div>
            <div>
                <pre>{"Width"}</pre>
                <input
                    value={width.to_string()}
                    id="width"
                    type="number"
                    min="1"
                    max={u8::MAX.to_string()}
                    onchange={oninput_width}
                />
            </div>
            <div>
                <pre>{"Bombs"}</pre>
                <input
                    value={bombs.to_string()}
                    id="bombs"
                    type="number"
                    min="1"
                    max={u16::MAX.to_string()}
                    onchange={oninput_bombs}
                />
            </div>

            <button disabled={!*is_valid.clone()}>{"Start"}</button>
        </form>
    )
}

fn parse_input_event<T>(e: Event) -> Option<T>
where
    T: FromStr,
{
    e.target_unchecked_into::<web_sys::HtmlInputElement>()
        .value()
        .parse()
        .ok()
}
