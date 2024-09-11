use std::time::Duration;

use iced::{widget::Canvas, Element, Fill, Theme};
use neora_core::progress_step::ProgressStep;

pub fn main() -> iced::Result {
	iced::application(
		"Neora Installer",
		NeoraInstaller::update,
		NeoraInstaller::view,
	)
	.subscription(NeoraInstaller::subscription)
	.theme(|_| Theme::Light)
	.transparent(true)
	.antialiasing(true)
	.run()
}

#[derive(Debug, Clone)]
pub struct NeoraInstaller {
	pub progress_step: ProgressStep,
}

impl Default for NeoraInstaller {
	fn default() -> Self {
		Self {
			progress_step: ProgressStep {
				steps: vec![
					"Copying Windows files".to_string(),
					"Expanding Windows files".to_string(),
					"Installing features".to_string(),
					"Installing updates".to_string(),
					"Completing installation".to_string(),
				],
				dash_color: [0.0, 0.0, 0.0].into(),
				current_step: 0,
				mode: neora_core::progress_step::Mode::Vertical,
			},
		}
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	Progress,
}

impl NeoraInstaller {
	fn update(&mut self, _message: Message) {
		match _message {
			Message::Progress => {
				self.progress_step.current_step += 1;

				if self.progress_step.current_step
					>= self.progress_step.steps.len()
				{
					self.progress_step.current_step = 0;
				}
			}
		}
	}

	fn subscription(&self) -> iced::Subscription<Message> {
		iced::time::every(Duration::from_secs(1)).map(|_| Message::Progress)
	}

	fn view(&self) -> Element<Message> {
		Canvas::new(&self.progress_step)
			.width(Fill)
			.height(Fill)
			.into()
	}
}
