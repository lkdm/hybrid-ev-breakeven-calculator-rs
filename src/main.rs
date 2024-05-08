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
    p: usize,
    r: usize,
}

#[component]
fn FormExample() -> impl IntoView {
    let query = use_query::<CalculatorParams>();
    let (calculated, set_calculated) = create_signal(0);

    let principle = move || query.with(|q| q.as_ref().map(|q| q.p).unwrap_or(1));
    let rate = move || query.with(|q| q.as_ref().map(|q| q.r).unwrap_or(1));

    create_effect(move |_| {
        let p = principle();
        let r = rate();
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
                        value=principle
                        inputmode="numeric"
                        oninput="this.form.requestSubmit()"
                    />
                </div>
                <div>
                    <label for="rate">Rate</label>
                    <input
                        type="text"
                        name="r"
                        value=rate
                        inputmode="numeric"
                        oninput="this.form.requestSubmit()"
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
