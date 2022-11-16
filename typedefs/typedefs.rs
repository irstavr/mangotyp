type Integer32 = i32;

#[serde(tag = "test", content = "result")]
enum HealthStatus {
    Protein(i32),
    Triglycerid(i32),
    Fats(i32),
}

struct Person {
    name: String,
    age: u32,
    has_gut_issues: bool,
}

type User = Option<Person>;

struct ZoeAnalysis {
    user: User,
    list_of_tests: Vec<String>,
    all_exams: HashMap<String, HealthStatus>,
}

