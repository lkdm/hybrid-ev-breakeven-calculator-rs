mod hooks;
mod parse_dec;

use leptos::*;
use leptos_router::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::SubmitEvent;
extern crate console_error_panic_hook;

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    view! {
        <Router>
            <header class="bg-gray-800 text-white text-center p-4">
                <h1 class="text-2xl">Hybrid EV Breakeven Calculator</h1>
                <nav>
                    <ul class="flex justify-center space-x-4">
                        <li>
                            <a href="/">Home</a>
                        </li>
                        <li>
                            <a href="https://github.com/lkdm/hybrid-ev-breakeven-calculator-rs">Github Repository</a>
                        </li>
                    </ul>
                </nav>
            </header>
            <main class="container mx-auto justify-center text-gray-300">
                <Routes>
                    <Route path="" view=FormExample/>
                </Routes>
            </main>
            <footer class="bg-gray-800 text-white text-center p-4">
                <aside>
                    Built with Rust and Tailwind.
                </aside>
            </footer>
        </Router>
    }
}

#[component]
pub fn NumberInput(name: String, value: String) -> impl IntoView {
    // A NumberInput enforces that the input can parse into a Decimal object.
    // Even though the Form takes care of saving the data to state, this input
    // has its own signal, because it needs to parse the input.
    let (state, set_state) = create_signal(value);
    let handle_input = move |ev: leptos::ev::Event| {
        let mut input = event_target_value(&ev);
        input = parse_dec::parse_decimal(input);
        set_state(input);
    };

    view! {
        <input
            type="text"
            name={name}
            class="w-full px-3 py-2 rounded-lg bg-transparent flex h-10 w-full rounded-md border border-white text-sm"
            on:input=handle_input
            onchange="this.form.requestSubmit()"
            prop:value=move || state.get()
        />
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Params)]
struct InputState {
    fuel_cost: Decimal,
    annual_km_driven: Decimal,
    ice_upfront_cost: Decimal,
    ice_fuel_litres_per_km: Decimal,
    hybrid_upfront_cost: Decimal,
    hybrid_fuel_litres_per_km: Decimal,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            fuel_cost: Decimal::ZERO,
            annual_km_driven: Decimal::ZERO,
            ice_upfront_cost: Decimal::ZERO,
            ice_fuel_litres_per_km: Decimal::ZERO,
            hybrid_upfront_cost: Decimal::ZERO,
            hybrid_fuel_litres_per_km: Decimal::ZERO,
        }
    }
}

#[derive(Clone)]
struct OutputState {
    hybrid_fuel_cost: Decimal,
    petrol_fuel_cost: Decimal,
    upfront_cost_difference: Decimal,
    per_kilometre_fuel_cost_difference: Decimal,
    breakeven_point_km: Decimal,
    breakeven_point_years: Decimal,
}
impl Default for OutputState {
    fn default() -> Self {
        OutputState {
            hybrid_fuel_cost: Decimal::ZERO,
            petrol_fuel_cost: Decimal::ZERO,
            upfront_cost_difference: Decimal::ZERO,
            per_kilometre_fuel_cost_difference: Decimal::ZERO,
            breakeven_point_km: Decimal::ZERO,
            breakeven_point_years: Decimal::ZERO,
        }
    }
}

#[component]
fn FieldSet(legend: String, children: Children) -> impl IntoView {
    let children = children().nodes.into_iter().collect_view();
    view! {
        <fieldset class="
            w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4 border border-gray-700 rounded-lg
            px-3 py-2
        ">
        //  class="w-full px-3 py-2 rounded-lg bg-transparent flex h-10 w-full rounded-md border border-white text-sm"

            <legend class="text-xl">{legend}</legend>
            {children}
        </fieldset>
    }
}

#[component]
fn Field(label: String, children: Children) -> impl IntoView {
    let children = children().nodes.into_iter().collect_view();
    view! {
        <label class="w-full block grid w-full max-w-sm items-center gap-1.5">
            {label}
            {children}
        </label>
    }
}

struct ArithmeticError;

fn break_even_point_km(
    upfront_cost_difference: Decimal,
    per_kilometre_fuel_cost_difference: Decimal,
) -> Result<Decimal, ArithmeticError> {
    if per_kilometre_fuel_cost_difference == Decimal::ZERO {
        return Err(ArithmeticError);
    }
    let breakeven_point_km = upfront_cost_difference / per_kilometre_fuel_cost_difference;
    Ok(breakeven_point_km)
}

fn break_even_point_years(
    breakeven_point_km: Decimal,
    annual_km_driven: Decimal,
) -> Result<Decimal, ArithmeticError> {
    if annual_km_driven == Decimal::ZERO {
        return Err(ArithmeticError);
    }
    let breakeven_point_years = breakeven_point_km / annual_km_driven;
    Ok(breakeven_point_years)
}

#[component]
fn FormExample() -> impl IntoView {
    let (result, set_result) = create_signal::<OutputState>(OutputState::default());
    let (query, set_query) = hooks::create_query_struct_signal::<InputState>();
    let default_values = query.get().unwrap_or_default();

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let data = match InputState::from_event(&ev) {
            Ok(data) => data,
            Err(err) => {
                logging::error!("Error parsing form data: {:?}", err);
                return;
            }
        };
        set_query(Some(data.clone()));
        let hybrid_fuel_cost = data.fuel_cost * data.hybrid_fuel_litres_per_km;
        let petrol_fuel_cost = data.fuel_cost * data.ice_fuel_litres_per_km;
        let upfront_cost_difference = data.hybrid_upfront_cost - data.ice_upfront_cost;
        let per_kilometre_fuel_cost_difference = petrol_fuel_cost - hybrid_fuel_cost;
        let breakeven_point_km =
            break_even_point_km(upfront_cost_difference, per_kilometre_fuel_cost_difference)
                .unwrap_or(Decimal::ZERO);
        let breakeven_point_years =
            break_even_point_years(breakeven_point_km, data.annual_km_driven)
                .unwrap_or(Decimal::ZERO);
        let output = OutputState {
            hybrid_fuel_cost,
            petrol_fuel_cost,
            upfront_cost_difference,
            per_kilometre_fuel_cost_difference,
            breakeven_point_km,
            breakeven_point_years,
        };
        set_result(output);
        logging::log!("Data: {:?}", data);
    };

    view! {
        <form method="GET" on:submit=handle_submit>
            <section class="flex flex-col gap-10">
                <FieldSet legend="General Details".to_string()>
                    <Field label="Estimated fuel price".to_string()>
                        <NumberInput
                            name="fuel_cost".to_string()
                            value={default_values.fuel_cost.to_string()}
                        />
                    </Field>
                    <Field label="Kilometres driven per year".to_string()>
                        <NumberInput
                            name="annual_km_driven".to_string()
                            value={default_values.annual_km_driven.to_string()}
                        />
                    </Field>
                </FieldSet>
                <FieldSet legend="Hybrid Car Details".to_string()>
                    <Field label="Estimated drive-away price".to_string()>
                        <NumberInput
                            name="hybrid_upfront_cost".to_string()
                            value={default_values.hybrid_upfront_cost.to_string()}
                        />
                    </Field>
                    <Field label="Estimated fuel economy (L/100km)".to_string()>
                        <NumberInput
                            name="hybrid_fuel_litres_per_km".to_string()
                            value={default_values.hybrid_fuel_litres_per_km.to_string()}
                        />
                    </Field>
                    <div>
                        <p>Petrol cost/km: {move || result.get().hybrid_fuel_cost.round_dp(2).to_string()}</p>
                    </div>
                </FieldSet>
                <FieldSet legend="Fuel Vehicle Details".to_string()>
                    <Field label="Estimated drive-away price".to_string()>
                        <NumberInput
                            name="ice_upfront_cost".to_string()
                            value={default_values.ice_upfront_cost.to_string()}
                        />
                    </Field>
                    <Field label="Estimated fuel economy (L/100km)".to_string()>
                        <NumberInput
                            name="ice_fuel_litres_per_km".to_string()
                            value={default_values.ice_fuel_litres_per_km.to_string()}
                        />
                    </Field>
                    <div>
                        <p>Petrol cost/km: {move || result.get().petrol_fuel_cost.round_dp(2).to_string()}</p>
                    </div>
                </FieldSet>
            </section>
        </form>
        <section>
            <h2>Outcome</h2>
            <dl>
                <dt>Upfront cost difference</dt>
                <dd>{move || result.get().upfront_cost_difference.round_dp(2).to_string()}</dd>

                <dt>Per kilometre fuel cost difference</dt>
                <dd>{move || result.get().per_kilometre_fuel_cost_difference.round_dp(2).to_string()}</dd>
            </dl>

            <dl>
                <dt>Breakeven point (km)</dt>
                <dd>{move || result.get().breakeven_point_km.round_dp(2).to_string()} km</dd>
                <dt>Breakeven point (years)</dt>
                <dd>{move || result.get().breakeven_point_years.round_dp(2).to_string()}</dd>
            </dl>
        </section>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
