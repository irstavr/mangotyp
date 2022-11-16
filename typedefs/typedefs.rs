type Integer32 = i32;

#[serde(tag = "test", content = "result")]
enum HealthStatus {
    Protein(i32),
    Triglycerid(i32),
    Fats(i32),
}