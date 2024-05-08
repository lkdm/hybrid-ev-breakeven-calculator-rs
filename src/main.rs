use leptos::*;
use leptos_router::*;

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
pub fn NumberInput(
    handle_input: SignalSetter<Option<String>>,
    value: Memo<Option<String>>,
) -> impl IntoView {
    let handle_enforce_monetary_rules = move |ev: leptos::ev::KeyboardEvent| {
        let key = ev.key().chars().next().unwrap();
        let mut input = event_target_value(&ev);

        // Only allow one decimal point
        if key == '.' && input.contains('.') {
            if input.ends_with("..") {
                input.pop();
            }
            ev.prevent_default();
        }

        // Only allow two digits after the decimal point
        if input.contains('.') {
            let parts: Vec<&str> = input.split('.').collect();
            if parts.len() == 2 && parts[1].len() >= 2 {
                ev.prevent_default();
                return;
            }
        }

        // Only allow numeric characters and the decimal point
        if !(key.is_numeric() || key == '.') {
            ev.prevent_default();
        }
    };
    view! {
        <input
            type="number"
            class="w-full px-3 py-2 rounded-lg bg-gray-300"
            on:keypress=handle_enforce_monetary_rules
            on:input=move |ev| {
                handle_input.set(Some(event_target_value(&ev)))
            }
            prop:value=value
        />
    }
}

#[component]
fn FormExample() -> impl IntoView {
    // Inputs
    let (fuel_cost, set_fuel_cost) = create_query_signal::<String>("fuel_cost");
    let (hybrid_upfront_cost, set_hybrid_upfront_cost) =
        create_query_signal::<String>("hybrid_upfront_cost");
    let (hybrid_efficiency, set_hybrid_efficiency) =
        create_query_signal::<String>("hybrid_efficiency");
    let (petrol_upfront_cost, set_petrol_upfront_cost) =
        create_query_signal::<String>("petrol_upfront_cost");
    let (petrol_efficiency, set_petrol_efficiency) =
        create_query_signal::<String>("petrol_efficiency");
    let (annual_km_driven, set_annual_km_driven) =
        create_query_signal::<String>("annual_km_driven");

    let (hybrid_fuel_cost, set_hybrid_fuel_cost) = create_signal(String::new());
    let (petrol_fuel_cost, set_petrol_fuel_cost) = create_signal(String::new());
    let (upfront_cost_difference, set_upfront_cost_difference) = create_signal(String::new());
    let (breakeven_point, set_breakeven_point) = create_signal(String::new());
    let (per_kilometre_fuel_cost_difference, set_per_kilometre_fuel_cost_difference) =
        create_signal(String::new());
    let (breakeven_point_years, set_breakeven_point_years) = create_signal(String::new());

    create_effect(move |_| {
        let _fuel_cost: f64 = fuel_cost.get().unwrap().parse().ok().unwrap_or(0.0);
        logging::log!("Fuel cost: {}", _fuel_cost);
        let _hybrid_upfront_cost: f64 = hybrid_upfront_cost
            .get()
            .unwrap()
            .parse()
            .ok()
            .unwrap_or(0.0);
        logging::log!("Hybrid upfront cost: {}", _hybrid_upfront_cost);

        let _hybrid_litres_per_distance_unit: f64 =
            hybrid_efficiency.get().unwrap().parse().ok().unwrap_or(0.0);
        logging::log!("Hybrid efficiency: {}", _hybrid_litres_per_distance_unit);

        let _petrol_upfront_cost: f64 = petrol_upfront_cost
            .get()
            .unwrap()
            .parse()
            .ok()
            .unwrap_or(0.0);
        logging::log!("Petrol upfront cost: {}", _petrol_upfront_cost);

        let _petrol_litres_per_distance_unit: f64 =
            petrol_efficiency.get().unwrap().parse().ok().unwrap_or(0.0);
        logging::log!("Petrol efficiency: {}", _petrol_litres_per_distance_unit);

        // Calculate fuel costs
        let _hybrid_fuel_cost = _fuel_cost * _hybrid_litres_per_distance_unit / 100.0;
        logging::log!("Hybrid fuel cost: {}", _hybrid_fuel_cost);
        let _petrol_fuel_cost = _fuel_cost * _petrol_litres_per_distance_unit / 100.0;
        logging::log!("Petrol fuel cost: {}", _petrol_fuel_cost);
        let _per_kilometre_fuel_cost_difference = _petrol_fuel_cost - _hybrid_fuel_cost;
        logging::log!(
            "Per kilometre fuel cost difference: {}",
            _per_kilometre_fuel_cost_difference
        );

        let _annual_km_driven: f64 = annual_km_driven.get().unwrap().parse().ok().unwrap_or(0.0);

        // Calculate upfront cost difference
        let _upfront_cost_difference = _hybrid_upfront_cost - _petrol_upfront_cost;

        // Calculate break-even point
        let _breakeven_point_kilometres =
            _upfront_cost_difference / _per_kilometre_fuel_cost_difference;
        logging::log!("Breakeven point: {}", _breakeven_point_kilometres);
        let _breakeven_point_years = _breakeven_point_kilometres / _annual_km_driven;

        // Setters
        set_hybrid_fuel_cost(format!("{:.2}", _hybrid_fuel_cost));
        set_petrol_fuel_cost(format!("{:.2}", _petrol_fuel_cost));
        set_upfront_cost_difference(format!("{:.2}", _upfront_cost_difference));
        set_breakeven_point(format!("{:.2} kilometres", _breakeven_point_kilometres));
        set_per_kilometre_fuel_cost_difference(format!(
            "{:.2}",
            _per_kilometre_fuel_cost_difference
        ));
        set_breakeven_point_years(format!("{:.2} years", _breakeven_point_years));
    });

    view! {
        <Form method="GET" action="">
            <section class="flex flex-col gap-10">
            <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
                <legend>Economy Details</legend>
                <label class="w-full block">
                    Estimated fuel price
                    <NumberInput handle_input={set_fuel_cost} value={fuel_cost}/>
                </label>
            </fieldset>
            <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
                <legend>Personal Details</legend>
                <label>
                    Kilometres driven per year
                    <NumberInput handle_input={set_annual_km_driven} value={annual_km_driven}/>
                </label>
            </fieldset>
            <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
                <legend>Hybrid Vehicle Details</legend>
                <label>
                    Estimated drive-away price
                    <NumberInput handle_input={set_hybrid_upfront_cost} value={hybrid_upfront_cost}/>
                </label>
                <label>
                    Estimated fuel economy (L/100km)
                    <NumberInput handle_input={set_hybrid_efficiency} value={hybrid_efficiency}/>
                </label>
                <div>
                    <p>Petrol cost/km: {hybrid_fuel_cost}</p>
                </div>
            </fieldset>
            <fieldset class="w-full grid sm:grid-cols-1 md:grid-cols-3 gap-4">
                <legend>Petrol Vehicle Details</legend>
                <label>
                    Estimated drive-away price
                    <NumberInput handle_input={set_petrol_upfront_cost} value={petrol_upfront_cost}/>
                </label>
                <label>
                    Estimated fuel economy (L/100km)
                    <NumberInput handle_input={set_petrol_efficiency} value={petrol_efficiency}/>
                </label>
                <div>
                    <p>Petrol cost/km: {petrol_fuel_cost}</p>
                </div>
            </fieldset>
            </section>
            </Form>
            <section>
            <h2>Outcome</h2>
            <div>
                <p>Upfront cost difference: {upfront_cost_difference}</p>
                <p>Per kilometre fuel cost difference: {per_kilometre_fuel_cost_difference}</p>
                <p>Breakeven point: {breakeven_point}</p>
                <p>Breakeven point in years: {breakeven_point_years}</p>
            </div>
            </section>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
