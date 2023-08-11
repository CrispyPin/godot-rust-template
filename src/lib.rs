use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct ExampleThing {}

#[methods]
impl ExampleThing {
	fn new(_owner: &Node) -> Self {
		Self {}
	}

	#[method]
	fn _ready(&self) {
		godot_print!("hello from rust");
	}
}

fn init(handle: InitHandle) {
	handle.add_class::<ExampleThing>();
}

godot_init!(init);
