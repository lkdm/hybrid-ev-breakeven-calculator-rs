mod parse_dec;

use leptos::*;
use leptos_router::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::SubmitEvent;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="container mx-auto justify-center text-gray-300">
                <Routes>
                    <Route path="" view=FormExample/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn NumberInput(name: String, value: String) -> impl IntoView {
    let (state, set_state) = create_signal(value);
    // let handle_enforce_monetary_rules = move |ev: leptos::ev::KeyboardEvent| {
    //     let key = ev.key().chars().next().unwrap();
    //     let input = event_target_value(&ev);

    //     let void_key: Option<bool> = match key {
    //         // Prevent multiple decimal points, or a decimal directly after a minus.
    //         '.' => Some(input.ends_with('-') || input.contains('.')),
    //         // Prevent minus sign if it's not the first character.
    //         '-' => Some(!input.is_empty()),
    //         _ => Some(
    //             // Prevent any non-numeric characters, except for the minus sign and decimal point.
    //             !(key.is_numeric() || key == '.' || key == '-'),
    //         ),
    //     };

    //     if void_key.unwrap() {
    //         ev.prevent_default();
    //         return;
    //     }
    // };

    let handle_input = move |ev: leptos::ev::Event| {
        let mut input = event_target_value(&ev);
        input = parse_dec::parse_decimal(input);
        set_state(input);
    };

    view! {
        <input
            type="text"
            name={name}
            class="w-full px-3 py-2 rounded-lg bg-gray-300"
            // on:keypress=handle_enforce_monetary_rules
            on:input=handle_input
            // on:blur=handle_input
            onchange="this.form.requestSubmit()"
            prop:value=move || state.get()
        />
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Vehicle {
    upfront_cost: Decimal,
    fuel_litres_consumption_per_km: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputState {
    fuel_cost: Decimal,
    // annual_km_driven: Decimal,
    // fuel_vehicle: Vehicle,
    // hybrid_vehicle: Vehicle,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            fuel_cost: Decimal::ZERO,
            // annual_km_driven: Decimal::ZERO,
            // fuel_vehicle: Vehicle {
            //     upfront_cost: Decimal::ZERO,
            //     fuel_litres_consumption_per_km: Decimal::ZERO,
            // },
            // hybrid_vehicle: Vehicle {
            //     upfront_cost: Decimal::ZERO,
            //     fuel_litres_consumption_per_km: Decimal::ZERO,
            // },
        }
    }
}

// #[derive(Clone)]
// struct OutputState {
//     hybrid_fuel_cost: Decimal,
//     petrol_fuel_cost: Decimal,
//     upfront_cost_difference: Decimal,
//     per_kilometre_fuel_cost_difference: Decimal,
//     breakeven_point_km: Decimal,
//     breakeven_point_years: Decimal,
// }

#[component]
fn FormExample() -> impl IntoView {
    // let (result, set_result) = create_signal::<Option<OutputState>>(None);
    // let state = expect_context::<RwSignal<GlobalState>>();
    // let input = (move || state.with(|state| state.input_state.clone()))();

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let data = match InputState::from_event(&ev) {
            Ok(data) => data,
            Err(err) => {
                logging::error!("Error parsing form data: {:?}", err);
                return;
            }
        };
        logging::log!("Data: {:?}", data);
    };

    view! {
        <form method="GET" on:submit=handle_submit>
            <section class="flex flex-col gap-10">
                <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
                    <legend>Economy Details</legend>
                    <label class="w-full block">
                        Estimated fuel price
                        <NumberInput
                            name="fuel_cost".to_string()
                            value="0.00".to_string()
                        />
                    </label>
                </fieldset>
            </section>
        </form>
            // <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
            //     <legend>Personal Details</legend>
            //     <label>
            //         Kilometres driven per year
            //         <NumberInput handle_input={set_annual_km_driven} value={annual_km_driven}/>
            //     </label>
            // </fieldset>
            // <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
            //     <legend>Hybrid Vehicle Details</legend>
            //     <label>
            //         Estimated drive-away price
            //         <NumberInput handle_input={set_hybrid_upfront_cost} value={hybrid_upfront_cost}/>
            //     </label>
            //     <label>
            //         Estimated fuel economy (L/100km)
            //         <NumberInput handle_input={set_hybrid_efficiency} value={hybrid_efficiency}/>
            //     </label>
            //     <div>
            //         <p>Petrol cost/km: {hybrid_fuel_cost}</p>
            //     </div>
            // </fieldset>
            // <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
            //     <legend>Petrol Vehicle Details</legend>
            //     <label>
            //         Estimated drive-away price
            //         <NumberInput handle_input={set_petrol_upfront_cost} value={petrol_upfront_cost}/>
            //     </label>
            //     <label>
            //         Estimated fuel economy (L/100km)
            //         <NumberInput handle_input={set_petrol_efficiency} value={petrol_efficiency}/>
            //     </label>
            //     <div>
            //         <p>Petrol cost/km: {petrol_fuel_cost}</p>
            //     </div>
            // </fieldset>
            // </section>
            // </ActionForm>
            // <section>
            // <h2>Outcome</h2>
            // <div>
            //     <p>Upfront cost difference: {upfront_cost_difference}</p>
            //     <p>Per kilometre fuel cost difference: {per_kilometre_fuel_cost_difference}</p>
            //     <p>Breakeven point: {breakeven_point}</p>
            //     <p>Breakeven point in years: {breakeven_point_years}</p>
            // </div>
            // </section>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
