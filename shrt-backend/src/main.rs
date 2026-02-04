use cot::Project;
use shrt_backend::ShrtProject;

#[cot::main]
fn main() -> impl Project {
    ShrtProject
}
