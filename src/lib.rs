use std::sync::{Arc, Mutex};

use eframe::egui::Ui;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

#[cfg(feature = "gemini")]
pub mod gemini;
#[cfg(feature = "gemini")]
use gemini::*;

pub enum TabContent<P: Protocol, R: Renderer> {
	Dummy,
	FileView {
		thread: std::thread::JoinHandle<()>,
		view: Arc<Mutex<FileView<P, R>>>,
	},
}

/// Macro that generates a Protocol struct which contains many Protocol structs
#[macro_export]
macro_rules! proto_multi {
	($struct:ident, $($cfg:meta, $i:ident,)+) => {
		#[derive(Default)]
		#[allow(non_snake_case)]
		struct $struct {
			$(
				#[cfg($cfg)]
				$i: $i,
			)*
		}

		impl Protocol for $struct {
			fn matches(&self, _: &Url) -> bool {
				true
			}

			fn fetch(&self, url: &Url) -> Option<ProtoData> {
				$(
						if Protocol::matches(&self.$i, url) {
							if let Some(d) = self.$i.fetch(url) {
								return Some(d);
							}
						}
				)*
				return None
			}
		}
	};
}

/// Does the same thing as proto_multi for Renderer structs.
#[macro_export]
macro_rules! rend_multi {
	($struct:ident, $dstruct:ident, $($cfg:meta, $i:ident,)+) => {
		#[derive(Default)]
		#[allow(non_snake_case)]
		struct $struct {
			$(
				#[cfg($cfg)]
				$i: $i,
			)*
		}

		#[derive(Default, Clone, Serialize, Deserialize)]
		#[allow(non_snake_case)]
		struct $dstruct {
			mime: String,
			$(
				#[cfg($cfg)]
				$i: Option<<$i as Renderer>::Data>,
			)*
		}

		impl FromData for $dstruct {
			fn from_data(data: &ProtoData) -> Option<Self> {
				Some($dstruct {
					mime: data.mime.clone(),
					$(
						#[cfg($cfg)]
						$i: <$i as Renderer>::Data::from_data(data),
					)*
				})
			}
		}

		impl Renderer for $struct {

			type Data = $dstruct;

			fn matches(&self, _: &str) -> bool {
				true
			}

			fn render(&mut self, ui: &mut Ui, data: &Self::Data) {
				$(
						if Renderer::matches(&self.$i, &data.mime) {
							if let Some(d) = &data.$i {
								self.$i.render(ui, d);
								return;
							}
						}
				)*
				ui.label(format!("Unsupported mimetype {}", data.mime));
			}
		}
	};
}

proto_multi!(Protocols, feature = "gemini", Gemini,);
rend_multi!(Renderers, RenderersData, feature = "gemini", Gemini,);

#[derive(Serialize, Deserialize)]
pub struct FileView<P: Protocol, R: Renderer> {
	pub load_state: LoadState,
	pub url: Url,
	pub proto: Option<P>,
	pub rend: Option<R>,
}

#[derive(Serialize, Deserialize)]
pub enum LoadState {
	FindProtocol,
	FindRenderer,
	Deser,
	Render,
}

#[derive(Serialize, Deserialize)]
pub struct ProtoData {
	pub mime: String,
	pub data: Box<[u8]>,
}

pub trait Protocol {
	fn fetch(&self, url: &Url) -> Option<ProtoData>;
	fn matches(&self, url: &Url) -> bool;
}

pub trait Renderer {
	type Data: Clone + Sized + FromData + Serialize + DeserializeOwned;

	fn render(&mut self, ui: &mut Ui, data: &Self::Data);

	fn matches(&self, mime: &str) -> bool;
}

pub trait FromData {
	fn from_data(data: &ProtoData) -> Option<Self>
	where
		Self: std::marker::Sized;
}
