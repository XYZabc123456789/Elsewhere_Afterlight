use super::*;

static REGISTRY: OnceLock<Mutex<JobValueRegistry>> = OnceLock::new();

pub fn get_registry() -> MutexGuard<'static, JobValueRegistry> {
    REGISTRY
        .get_or_init(|| Mutex::new(JobValueRegistry::new()))
        .lock()
        .unwrap()
}

pub fn release_registry(registry: MutexGuard<'static, JobValueRegistry>) {
    drop(registry)
}

#[derive(Debug)]
#[wasm_bindgen(js_name = JobValueRegistry)]
pub struct JobValueRegistry {
    values: SlotMap<JobValueKey, JobValue>,
}

impl JobValueRegistry {
    pub fn new() -> Self {
        Self {
            values: SlotMap::with_key(),
        }
    }

    pub fn insert(&mut self, value: JobValue) -> JobValueKey {
        self.values.insert(value)
    }

    pub fn get(&mut self, key: JobValueKey) -> Res<JobValue> {
        self.values
            .remove(key)
            .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid Key").into())
    }
}

#[wasm_bindgen]
pub struct RegistryLock {
    guard: std::sync::MutexGuard<'static, JobValueRegistry>,
}

#[wasm_bindgen]
impl RegistryLock {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            guard: get_registry(),
        }
    }

    pub fn release(self) {
        release_registry(self.guard)
    }

    pub fn get(&mut self, handle: JobValueHandle) -> Result<Option<JsValue>, JsError> {
        let r = self.guard.get(handle.get_key());
        let r = match r {
            Ok(r) => r,
            Err(e) => return Err(JsError::new(&e.to_string())),
        };
        let v = r.get_data();
        let data = v
            .downcast::<JsData>()
            .map_err(|_| JsError::new("Cannot be read by js"))?;
        let jsv = to_value(&data)?;
        Ok(Some(jsv))
    }

    pub fn insert(&mut self, value: JsValue) -> Result<JobValueHandle, JsError> {
        let value: JsData = from_value(value).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JobValueHandle::from(self.guard.insert(value.into())))
    }
}