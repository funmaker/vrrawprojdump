use openvr::Eye;

fn main() {
	let context = unsafe { openvr::init(openvr::ApplicationType::Scene).unwrap() };
	let system = context.system().unwrap();
	
	println!("{:?}", system.projection_raw(Eye::Left));
	println!("{:?}", system.projection_raw(Eye::Right));
}
