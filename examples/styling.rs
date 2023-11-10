//! Example of styling.

use std::sync::Arc;

use bevy::{prelude::*, ui};
use lazy_static::lazy_static;
use quill::{Bind, Cx, Element, If, QuillPlugin, StyleSet, TrackedResources, View, ViewHandle};

fn main() {
    App::new()
        .init_resource::<Counter>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(QuillPlugin)
        .add_systems(Startup, (setup, setup_view_root))
        .add_systems(Update, (bevy::window::close_on_esc, update_counter))
        .run();
}

lazy_static! {
    static ref STYLE_MAIN: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .position(ui::PositionType::Absolute)
        .left(10.)
        .top(10.)
        .bottom(20.)
        .right(10.)
        .border(1)
        .border_color(Some(Color::hex("#888").unwrap()))
        .display(ui::Display::Flex)));
    static ref STYLE_ASIDE: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#222").unwrap()))
        .display(ui::Display::Flex)
        .flex_direction(ui::FlexDirection::Column)
        .width(200)));
    static ref STYLE_VSPLITTER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#181818").unwrap()))
        .align_items(ui::AlignItems::Center)
        .justify_content(ui::JustifyContent::Center)
        .display(ui::Display::Flex)
        .width(7)));
    static ref STYLE_VSPLITTER_INNER: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::hex("#282828").unwrap()))
        .display(ui::Display::Flex)
        .width(3)
        .height(ui::Val::Percent(30.))));
    static ref STYLE_EVEN: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::RED))
        .padding(UiRect::all(Val::Px(2.)))));
    static ref STYLE_ODD: Arc<StyleSet> = Arc::new(StyleSet::build(|ss| ss
        .background_color(Some(Color::GREEN))
        .padding(UiRect::all(Val::Px(2.)))));
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn setup_view_root(mut commands: Commands) {
    commands.spawn((TrackedResources::default(), ViewHandle::new(ui_main, ())));
}

fn ui_main(mut cx: Cx) -> impl View {
    let counter = cx.use_resource::<Counter>();
    Element::new((
        Element::new(()).styled(STYLE_ASIDE.clone()),
        Bind::new(v_splitter, ()),
        If::new(
            counter.count & 1 == 0,
            Bind::new(even, ()),
            Bind::new(odd, ()),
        ),
    ))
    .styled(STYLE_MAIN.clone())
}

fn v_splitter(mut _cx: Cx) -> impl View {
    Element::new(Element::new(()).styled(STYLE_VSPLITTER_INNER.clone()))
        .styled(STYLE_VSPLITTER.clone())
}

fn even(mut _cx: Cx) -> impl View {
    Element::new("even").styled(STYLE_EVEN.clone())
}

fn odd(mut _cx: Cx) -> impl View {
    Element::new("odd").styled(STYLE_ODD.clone())
}

#[derive(Resource, Default)]
pub struct Counter {
    pub count: u32,
    pub foo: usize,
}

fn update_counter(mut counter: ResMut<Counter>, key: Res<Input<KeyCode>>) {
    if key.pressed(KeyCode::Space) {
        counter.count += 1;
    }
}

// Setup 3d shapes
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}