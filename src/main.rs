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
    let (principle, set_principle) = create_query_signal("p");
    let (rate, set_rate) = create_query_signal("r");
    let (calculated, set_calculated) = create_signal(0);

    create_effect(move |_| {
        let p = principle().unwrap_or(1);
        let r = rate().unwrap_or(1);
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
                    <input
                        type="text"
                        name="p"
                        value={move || principle.get()}
                        inputmode="numeric"
                        on:input=move |ev| {
                            set_principle(event_target_value(&ev).parse().ok())
                        }
                    />
                </div>
                <div>
                    <label for="rate">Rate</label>
                    <input
                        type="text"
                        name="r"
                        value={move || rate.get()}
                        inputmode="numeric"
                        on:input=move |ev| {
                            set_rate(event_target_value(&ev).parse().ok())
                        }
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
