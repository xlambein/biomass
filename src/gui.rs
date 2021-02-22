use bevy::prelude::{
	AppBuilder, Assets, IntoSystem, Plugin, Res, ResMut, Resources, TextureAtlas, World,
};
use bevy_egui::{
	egui::{
		self, widgets::Image, Align, Color32, FontDefinitions, FontFamily, Label, Layout, Pos2,
		Rect, Stroke, TextStyle, TextureId, Ui, Vec2,
	},
	EguiContext, EguiPlugin,
};

use crate::{
	constants::{ASTEROID_SPRITES, DNA_SPRITE, EGUI_TEXTURE_ATLAS_ID, MENU_WIDTH, RECIPES},
	current_recipe, Biomass, CurrentIngredients, Recipe,
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_plugin(EguiPlugin)
			.add_startup_system(setup_egui.system())
			.add_system(side_panel.system());
	}
}

pub fn setup_egui(_world: &mut World, resources: &mut Resources) {
	let mut egui_ctx = resources.get_mut::<EguiContext>().unwrap();

	let texture_atlases = resources.get::<Assets<TextureAtlas>>().unwrap();
	let atlas = texture_atlases.iter().next().unwrap().1;

	egui_ctx.set_egui_texture(EGUI_TEXTURE_ATLAS_ID, atlas.texture.clone());

	let mut fonts = FontDefinitions::default();
	fonts
		.family_and_size
		.insert(TextStyle::Small, (FontFamily::Proportional, 20.0));
	fonts
		.family_and_size
		.insert(TextStyle::Body, (FontFamily::Proportional, 24.0));
	fonts
		.family_and_size
		.insert(TextStyle::Heading, (FontFamily::Proportional, 32.0));
	egui_ctx.ctx.set_fonts(fonts);

	let mut visuals: egui::Visuals = egui_ctx.ctx.style().visuals.clone();
	visuals.override_text_color = Some(Color32::BLACK);
	visuals.widgets.noninteractive.bg_fill = Color32::from_gray(204);
	visuals.widgets.noninteractive.bg_stroke = Stroke::new(0., Color32::WHITE);
	visuals.text_cursor_width = 50.;
	egui_ctx.ctx.set_visuals(visuals);
}

fn texture_atlas_uv(texture_atlas: &TextureAtlas, index: u32) -> Rect {
	let size = texture_atlas.size;
	let rect = texture_atlas.textures[index as usize];
	Rect::from_min_max(
		Pos2::new(rect.min.x / size.x, rect.min.y / size.y),
		Pos2::new(rect.max.x / size.x, rect.max.y / size.y),
	)
}

fn image(texture_atlas: &TextureAtlas, index: u32, size: impl Into<Vec2>) -> Image {
	Image::new(TextureId::User(EGUI_TEXTURE_ATLAS_ID), size)
		.uv(texture_atlas_uv(texture_atlas, index))
}

fn draw_dna(ui: &mut Ui, texture_atlas: &TextureAtlas, size: f32) -> egui::Response {
	ui.add(image(texture_atlas, DNA_SPRITE, [size, size]))
}

fn recipe(
	ui: &mut Ui,
	texture_atlas: &TextureAtlas,
	recipe: &Recipe,
	current: bool,
	current_ingredients: &[usize],
) {
	egui::Frame {
		margin: Vec2::new(10., 10.),
		fill: Color32::WHITE,
		..Default::default()
	}
	.show(ui, |ui| {
		ui.label(recipe.name);
		ui.horizontal_wrapped(|ui| {
			ui.style_mut().spacing.item_spacing.x = 2.;
			for (index, ingredient) in recipe.ingredients.iter().enumerate() {
				for i in 0..*ingredient {
					let mut image = image(texture_atlas, ASTEROID_SPRITES[index], [32., 32.]);
					if !current || i >= current_ingredients[index] {
						image = image.tint(Color32::from_white_alpha(127));
					}
					ui.add(image);
				}
			}
		});
		ui.horizontal(|ui| {
			ui.style_mut().spacing.item_spacing.x = 2.;
			if current {
				draw_dna(ui, texture_atlas, 20.0);
				ui.label(Label::new(format!("+{}", recipe.reward)).text_style(TextStyle::Small));
			} else {
				draw_dna(ui, texture_atlas, 20.0);
				ui.label(
					Label::new(format!("≥{}", recipe.requirement)).text_style(TextStyle::Small),
				);
			}
		});
	})
}

pub fn side_panel(
	mut egui_context: ResMut<EguiContext>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	biomass: Res<Biomass>,
	current_ingredients: Res<CurrentIngredients>,
) {
	let ctx = &mut egui_context.ctx;
	let texture_atlas = texture_atlases.iter().next().unwrap().1;

	egui::SidePanel::left("side_panel", MENU_WIDTH).show(ctx, |ui| {
		ui.vertical(|ui| {
			ui.style_mut().spacing.item_spacing.y = 32.;

			ui.with_layout(Layout::top_down(Align::Max), |ui| {
				ui.style_mut().spacing.item_spacing.y = 2.;

				ui.label("Biomass");

				ui.horizontal(|ui| {
					ui.style_mut().spacing.item_spacing.x = 2.;

					draw_dna(ui, texture_atlas, 32.0);
					ui.label(Label::new(format!("{}", biomass.0)).text_style(TextStyle::Heading));
				});
			});

			ui.with_layout(Layout::top_down(Align::Max), |ui| {
				ui.style_mut().spacing.item_spacing.y = 8.;

				ui.label(Label::new("Current:").text_style(TextStyle::Small));
				let index = current_recipe(biomass.0);
				recipe(
					ui,
					texture_atlas,
					&RECIPES[index],
					true,
					&current_ingredients.0,
				);
				if index + 1 < RECIPES.len() {
					ui.label(Label::new("Next:").text_style(TextStyle::Small));
					recipe(ui, texture_atlas, &RECIPES[index + 1], false, &[]);
				}
			});
		});
	});
}
