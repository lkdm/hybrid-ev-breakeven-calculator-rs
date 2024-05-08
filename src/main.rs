use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
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
            name="principle"
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
    let (principle, set_principle) = create_query_signal::<String>("p");
    let (rate, set_rate) = create_query_signal::<String>("r");
    let (calculated, set_calculated) = create_signal(0.0);

    create_effect(move |_| {
        let p: f64 = principle().unwrap().parse().ok().unwrap();
        let r: f64 = rate().unwrap().parse().ok().unwrap();
        set_calculated(p * r)
    });

    view! {
        <div>
            <h2>Form</h2>
            <Form method="GET" action="">
            <fieldset>
                <legend>Your Strategy</legend>
                <div>
                    <label for="principle">Principle</label>
                    <NumberInput handle_input={set_principle} value={principle}/>
                </div>
                <div>
                    <label for="rate">Rate</label>
                    <NumberInput handle_input={set_rate} value={rate}/>
                </div>
            </fieldset>
            </Form>
            <h2>Calculated</h2>
            <p>{move || calculated.get()}</p>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
