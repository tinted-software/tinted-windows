use iced::{
	alignment::{Horizontal, Vertical},
	font::Weight,
	mouse,
	widget::{
		canvas::{Frame, Geometry, Path, Program, Stroke, Text},
		text::{LineHeight, Shaping},
	},
	Color, Font, Rectangle, Renderer, Theme,
};

// progress step ui, ProgressStep and a dash for each step
#[derive(Debug, Clone, Default)]
pub struct ProgressStep {
	pub steps: Vec<String>,
	pub current_step: usize,
	pub dash_color: Color,
}

impl<Message> Program<Message> for ProgressStep {
	type State = ();

	fn draw(
		&self,
		_state: &(),
		renderer: &Renderer,
		theme: &Theme,
		bounds: Rectangle,
		_cursor: mouse::Cursor,
	) -> Vec<Geometry> {
		let mut frame = Frame::new(renderer, bounds.size());

		let total_steps = self.steps.len();
		let step_width = bounds.width / total_steps as f32;

		for i in 0..total_steps {
			let x = bounds.x + i as f32 * step_width;

			let color = theme.palette().primary;

			let step = Path::circle(
				[x + step_width / 2.0, bounds.y + bounds.height / 2.0].into(),
				8.0,
			);

			frame.fill(&step, color);

			let message = if i < self.current_step {
				format!("{} ✅", &self.steps[i])
			} else {
				self.steps[i].clone()
			};

			frame.fill_text(Text {
				content: message,
				position: [
					x + step_width / 2.0,
					bounds.y + bounds.height / 2.0 - 24.0,
				]
				.into(),
				size: 16.0.into(),
				color: theme.palette().text,
				horizontal_alignment: Horizontal::Center,
				vertical_alignment: Vertical::Center,
				line_height: LineHeight::default(),
				font: Font {
					weight: if i == self.current_step {
						Weight::Bold
					} else {
						Weight::Normal
					},
					..Default::default()
				},
				shaping: Shaping::Advanced,
			});

			// Draw dashes between steps (starting from the right of the second step to the left of the last step) without going through the circles
			if i > 0 && i < total_steps {
				let dash = Path::line(
					[
						x - step_width / 2.0 + 8.0,
						bounds.y + bounds.height / 2.0,
					]
					.into(),
					[
						x + step_width / 2.0 - 8.0,
						bounds.y + bounds.height / 2.0,
					]
					.into(),
				);

				frame.stroke(
					&dash,
					Stroke::default().with_color(self.dash_color),
				);
			}
		}

		vec![frame.into_geometry()]
	}
}
