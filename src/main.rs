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

#[derive(Params, PartialEq)]
struct CalculatorParams {
    principle: usize,
    rate: usize,
}

#[component]
fn FormExample() -> impl IntoView {
    let (principle, set_principle) = create_query_signal::<usize>("p");
    let (rate, set_rate) = create_query_signal::<usize>("r");
    let (calculated, set_calculated) = create_signal(0);

    create_effect(move |_| {
        let p = principle().unwrap_or(1);
        let r = rate().unwrap_or(1);
        set_calculated(p * r)
    });

    let handle_enforce_numeric = move |ev: leptos::ev::KeyboardEvent| {
        if !ev.key().chars().next().unwrap().is_numeric() {
            ev.prevent_default();
        }
    };

    view! {
        <div>
            <h2>Form</h2>
            <Form method="GET" action="">
            <fieldset>
                <legend>Your Strategy</legend>
                <div>
                    <label for="principle">Principle</label>
                    <input
                        type="number"
                        name="principle"
                        inputmode="numeric"
                        on:keypress=handle_enforce_numeric
                        on:input=move |ev| {
                            set_principle(event_target_value(&ev).parse().ok())
                        }
                        prop:value=principle
                    />
                </div>
                <div>
                    <label for="rate">Rate</label>
                    <input
                        type="number"
                        name="r"
                        value={move || rate.get()}
                        inputmode="numeric"
                        on:keypress=handle_enforce_numeric
                        on:input=move |ev| {
                            set_rate(event_target_value(&ev).parse().ok())
                        }
                        prop:value=rate
                    />
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
