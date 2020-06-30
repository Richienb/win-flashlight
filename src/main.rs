use winrt::*;

import!(
    dependencies
        os
    types
		windows::devices::lights::*
);

async fn asyncify(fn: Result) {

}

fn main() {
	use windows::devices::lights::*;

	let a = Lamp::get_default_async();


	match a {
    Ok(v) => {
		let lamp = a.get_results().unwrap();
	}
    Err(e) => println!("e: {:?}", e),
}

	println!("a")
	// println!("{}", lampb.brightness_level().unwrap());
}
