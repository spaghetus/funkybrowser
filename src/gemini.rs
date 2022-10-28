use serde::{Deserialize, Serialize};

use crate::{FromData, Protocol, Renderer};

#[derive(Default)]
pub struct Gemini {}

impl Protocol for Gemini {
	fn fetch(&self, url: &url::Url) -> Option<crate::ProtoData> {
		todo!()
	}

	fn matches(&self, url: &url::Url) -> bool {
		todo!()
	}
}

#[derive(Default)]
pub struct Gemtext {}

#[derive(Clone, Serialize, Deserialize)]
pub struct GeminiRenderData(pub String);
impl FromData for GeminiRenderData {
	fn from_data(data: &crate::ProtoData) -> Option<Self>
	where
		Self: std::marker::Sized,
	{
		if data.mime == "text/gemini" {
			Some(GeminiRenderData(
				String::from_utf8_lossy(&data.data).to_string(),
			))
		} else {
			None
		}
	}
}

impl Renderer for Gemini {
	type Data = GeminiRenderData;

	fn render(&mut self, ui: &mut eframe::egui::Ui, data: &Self::Data) {
		ui.label(&data.0);
	}

	fn matches(&self, mime: &str) -> bool {
		mime == "text/gemini"
	}
}
