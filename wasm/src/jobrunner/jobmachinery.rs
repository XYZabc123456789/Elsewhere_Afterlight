use super::*;

#[wasm_bindgen]
pub struct Job {
    pub kind: JobKind,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum JsData {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsData>),
    Object(std::collections::HashMap<String, JsData>),
}

new_key_type! {
    pub struct JobValueKey;
}

#[derive(Debug)]
pub struct JobValue(Box<dyn Any + Send + Sync>);

impl JobValue {
    #[inline(always)]
    pub fn get_data(self) -> Box<dyn Any + Send + Sync> {
        self.0
    }
}

impl From<Box<dyn Any + Send + Sync>> for JobValue {
    fn from(value: Box<dyn Any + Send + Sync>) -> Self {
        JobValue(value)
    }
}

impl From<JsData> for JobValue {
    fn from(value: JsData) -> Self {
        JobValue(Box::new(value))
    }
}

#[derive(Debug)]
#[wasm_bindgen]
/// Only meant to be used by js
pub struct JobValueHandle(JobValueKey);

impl JobValueHandle {
    #[inline(always)]
    pub fn get_key(self) -> JobValueKey {
        self.0
    }
}

impl From<JobValueKey> for JobValueHandle {
    #[inline(always)]
    fn from(value: JobValueKey) -> Self {
        Self(value)
    }
}

#[wasm_bindgen]
impl Job {
    #[wasm_bindgen(js_name = executeJob)]
    pub async fn js_execute_job(
        self,
        input: Option<JobValueHandle>,
    ) -> Result<Option<JobValueHandle>, JsError> {
        self.execute_job(input.map(|h| h.get_key()))
            .await
            .map(|r| r.map(|r| JobValueHandle(r)))
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(constructor)]
    pub fn new(kind: JobKind) -> Self {
        Self {
            kind
        }
    }
}

impl Job {
    pub async fn execute_job(self, input: Option<JobValueKey>) -> Res<Option<JobValueKey>> {
        let (tx, rx) = oneshot::channel();
        let mut reg = get_registry();
        let i = match input {
            Some(i) => Some(reg.get(i)?),
            None => None,
        };
        release_registry(reg);
        rayon::spawn(move || {
            let result = self.execute(i);
            let _ = tx.send(result);
        });
        let output = rx.await??;
        match output {
            Some(v) => {
                let mut reg = get_registry();
                let k = reg.insert(v);
                release_registry(reg);
                Ok(Some(k))
            }
            None => Ok(None),
        }
    }
}
