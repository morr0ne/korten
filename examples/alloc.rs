use korten::Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

fn main() {}
