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

#[derive(Debug, Clone, Default)]
pub enum Mode {
	#[default]
	Horizontal,
	Vertical,
}

// progress step ui, ProgressStep and a dash for each step
#[derive(Debug, Clone, Default)]
pub struct ProgressStep {
	pub steps: Vec<String>,
	pub current_step: usize,
	pub dash_color: Color,
	pub mode: Mode,
}

impl ProgressStep {
	fn draw_horizontal(
		&self,
		frame: &mut Frame,
		theme: &Theme,
		bounds: Rectangle,
		total_steps: usize,
	) {
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
	}

	fn draw_vertical(
		&self,
		frame: &mut Frame,
		theme: &Theme,
		bounds: Rectangle,
		total_steps: usize,
	) {
		let step_height = bounds.height / total_steps as f32;

		for i in 0..total_steps {
			let y = bounds.y + i as f32 * step_height;

			let color = theme.palette().primary;

			let step = Path::circle(
				[bounds.x + bounds.width / 2.0, y + step_height / 2.0].into(),
				8.0,
			);

			frame.fill(&step, color);

			let message = if i < self.current_step {
				format!("{} ✅", &self.steps[i])
			} else {
				self.steps[i].clone()
			};

			// Draw the text on the right of the circle

			frame.fill_text(Text {
				content: message.clone(),
				position: [
					bounds.x + bounds.width / 2.0 + 8.0 * 16.0,
					y + step_height / 2.0,
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
						bounds.x + bounds.width / 2.0,
						y - step_height / 2.0 + 8.0,
					]
					.into(),
					[
						bounds.x + bounds.width / 2.0,
						y + step_height / 2.0 - 8.0,
					]
					.into(),
				);

				frame.stroke(
					&dash,
					Stroke::default().with_color(self.dash_color),
				);
			}
		}
	}
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

		match self.mode {
			Mode::Horizontal => {
				self.draw_horizontal(&mut frame, theme, bounds, total_steps)
			}
			Mode::Vertical => {
				self.draw_vertical(&mut frame, theme, bounds, total_steps)
			}
		}

		vec![frame.into_geometry()]
	}
}
