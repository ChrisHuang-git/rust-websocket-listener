use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ExampleMessage {
    pub int1: i64,
    pub int2: i64,
    pub int3: i64,
    pub int4: i64,
    pub int5: i64,
    pub int6: i64,
    pub float1: f64,
    pub float2: f64,
    pub float3: f64
}
impl ExampleMessage {
    pub fn is_after_time(self, time: i64) -> bool {
        self.int1 > time
    }
}