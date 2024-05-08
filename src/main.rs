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
    let query = use_query::<CalculatorParams>();
    let principle = move || {
        query.with(|q| {
            q.as_ref()
                .map(|params| params.principle)
                .unwrap_or_default()
        })
    };

    let rate = move || query.with(|q| q.as_ref().map(|params| params.rate).unwrap_or_default());

    let calculated = move || {
        let principle = principle();
        let rate = rate();
        principle * rate
    };

    // create_effect(move |_| {
    //     set_calculated(principle * rate, principle())
    //   // immediately prints "Value: 0" and subscribes to `a`
    //   // log::debug!("Value: {}", a());
    // });

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
                        name="principle"
                        value=principle
                        inputmode="numeric"
                        oninput="this.form.requestSubmit()"
                    />
                </div>
                <div>
                    <label for="rate">Rate</label>
                    <input
                        type="text"
                        name="rate"
                        value=rate
                        inputmode="numeric"
                        oninput="this.form.requestSubmit()"
                    />
                </div>
            </fieldset>
            </Form>
            <h2>Calculated</h2>
            <p>{calculated()}</p>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
