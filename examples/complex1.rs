//! Example of a simple UI layout
#![feature(impl_trait_in_assoc_type)]

#[path = "./common/lib.rs"]
mod common;

use bevy::{color::palettes, prelude::*};
use bevy_mod_picking::{debug::DebugPickingMode, DefaultPickingPlugins};
use bevy_mod_stylebuilder::*;
use bevy_quill::{Cx, Element, IntoViewChild, QuillPlugin, View, ViewChild, ViewTemplate};
use bevy_quill_obsidian::ObsidianUiPlugin;
use common::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            DefaultPickingPlugins,
            QuillPlugin,
            ObsidianUiPlugin,
        ))
        .insert_resource(DebugPickingMode::Disabled)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (close_on_esc, rotate))
        .run();
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(
        Element::<NodeBundle>::new()
            .children(NestedParamsTest::new().slot1("Slot-1"))
            .to_root(),
    );
}

fn style_test(ss: &mut StyleBuilder) {
    ss.display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .border_color(palettes::css::LIME)
        .border(3)
        .padding(3);
}

#[derive(Clone, PartialEq, Default)]
struct ChildParamsTest {
    slot1: ViewChild,
    slot2: ViewChild,
}

impl ChildParamsTest {
    fn new() -> Self {
        Self { ..default() }
    }

    fn slot1(mut self, slot1: impl IntoViewChild) -> Self {
        self.slot1 = slot1.into_view_child();
        self
    }

    fn slot2(mut self, slot2: impl IntoViewChild) -> Self {
        self.slot2 = slot2.into_view_child();
        self
    }
}

impl ViewTemplate for ChildParamsTest {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        Element::<NodeBundle>::new()
            .style(style_test)
            .insert(BackgroundColor(palettes::css::DARK_BLUE.into()))
            .children(("Title", self.slot1.clone(), self.slot2.clone()))
    }
}

#[derive(Clone, PartialEq, Default)]
struct NestedParamsTest {
    slot1: ViewChild,
}

impl NestedParamsTest {
    fn new() -> Self {
        Self { ..default() }
    }

    fn slot1(mut self, slot1: impl IntoViewChild) -> Self {
        self.slot1 = slot1.into_view_child();
        self
    }
}

impl ViewTemplate for NestedParamsTest {
    type View = impl View;
    fn create(&self, _cx: &mut Cx) -> Self::View {
        ChildParamsTest::new()
            .slot1(self.slot1.clone())
            .slot2(("Slot-2", "Slot-3"))
    }
}
