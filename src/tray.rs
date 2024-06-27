use clipboard::{ClipboardContext, ClipboardProvider};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{menu::{accelerator::{Accelerator, Modifiers}, Menu, MenuEvent, MenuItem, PredefinedMenuItem}, Icon, TrayIconBuilder, TrayIconEvent};

const ICON: &[u8] = include_bytes!("./icons/tray.png");

fn copy_address(address: String) {
	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
	ctx.set_contents(address).unwrap();
}

pub fn main(addresses: Vec<String>) {
	let copy_address_string = addresses.clone();

	let event_loop = EventLoopBuilder::new().build();

	let tray_menu = Menu::new();

	let prompt_item = MenuItem::new(
		"Server Address",
		false,
		None
	);

	let copy_address_items = {
		let mut items = Vec::new();
		for address in addresses {
			items.push(
				MenuItem::new(
					address,
					true,
					None
				)
			);
		}
		items
	};

	let quit_item = MenuItem::new(
		"Quit",
		true,
		Some(Accelerator::new(
			Some(Modifiers::META),
			tray_icon::menu::accelerator::Code::KeyQ
		))
	);

	tray_menu.append(&prompt_item).unwrap();
	for item in &copy_address_items {
		tray_menu.append(item).unwrap();
	}
	tray_menu.append(&PredefinedMenuItem::separator()).unwrap();
	tray_menu.append(&quit_item).unwrap();

	let mut tray_icon = None;

	let menu_channel = MenuEvent::receiver();
	let tray_channel = TrayIconEvent::receiver();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::WaitUntil(
			std::time::Instant::now() + std::time::Duration::from_millis(16)
		);

		if let tao::event::Event::NewEvents(tao::event::StartCause::Init) = event {
			let icon_image = image::load_from_memory(
				ICON,
			).unwrap().into_rgba8();

			let width = icon_image.width();
			let height = icon_image.height();

			let icon = Icon::from_rgba(
				icon_image.into_raw(),
				width,
				height
			).unwrap();

			tray_icon = Some(
				TrayIconBuilder::new()
					.with_menu(Box::new(tray_menu.clone()))
					.with_tooltip("Mobile Mouse Server")
					.with_icon(icon)
					.with_icon_as_template(true)
					.build()
					.unwrap()
			);

			#[cfg(target_os = "macos")]
            unsafe {
                use core_foundation::runloop::{CFRunLoopGetMain, CFRunLoopWakeUp};

                let rl = CFRunLoopGetMain();
                CFRunLoopWakeUp(rl);
            }
		}

		if let Ok(event) = menu_channel.try_recv() {
			if event.id == quit_item.id() {
				tray_icon.take();
				*control_flow = ControlFlow::Exit;
			}
			for (copy_index, copy_item) in copy_address_items.iter().enumerate() {
				if event.id == copy_item.id() {
					copy_address(copy_address_string[copy_index].clone());
				}
			}
		}

		if let Ok(_event) = tray_channel.try_recv() {}
	})
}
