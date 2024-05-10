use leptos::*;
use leptos_router::*;
use serde::Serialize;

#[track_caller]
pub fn create_query_struct_signal<T>() -> (Memo<Result<T, ParamsError>>, SignalSetter<Option<T>>)
where
    T: Params + Serialize + PartialEq + Clone,
{
    create_query_struct_signal_with_options::<T>(NavigateOptions::default())
}

#[track_caller]
pub fn create_query_struct_signal_with_options<T>(
    nav_options: NavigateOptions,
) -> (Memo<Result<T, ParamsError>>, SignalSetter<Option<T>>)
where
    T: Params + Serialize + PartialEq + Clone,
{
    let query = use_query::<T>();
    let navigate = use_navigate();
    let location = use_location();

    let set = SignalSetter::map(move |data: Option<T>| {
        let qs = serde_urlencoded::to_string(Some(data)).unwrap();
        let path = location.pathname.get_untracked();
        let hash = location.hash.get_untracked();
        let new_url = format!("{}?{}{}", path, qs, hash);
        navigate(&new_url, nav_options.clone());
    });
    (query, set)
}
