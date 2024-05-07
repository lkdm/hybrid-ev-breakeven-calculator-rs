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
fn FormExample() -> impl IntoView {
    let query = use_query_map();
    let count = move || query().get("count").cloned().unwrap_or_default();

    view! {
        <div>
            <h2>Form</h2>
            <Form method="GET" action="">
                <input
                    type="text"
                    name="count"
                    value=count
                    oninput="this.form.requestSubmit()"
                />
                <input type="submit"/>
            </Form>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
