use serde::Deserialize;

#[derive(Deserialize)]
pub struct CaseFile<TIn, TOut> {
    pub cases: Vec<Case<TIn, TOut>>,
}

#[derive(Deserialize)]
pub struct Case<TIn, TOut> {
    pub id: String,
    pub input: TIn,
    pub output: TOut,
}

pub fn load_cases<TIn, TOut>(path: &str) -> CaseFile<TIn, TOut>
where
    TIn: for<'de> Deserialize<'de>,
    TOut: for<'de> Deserialize<'de>,
{
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", path, e));
    serde_json::from_str(&data)
        .unwrap_or_else(|e| panic!("failed to parse {}: {}", path, e))
}
