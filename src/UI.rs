use bevy::a11y::{
    AccessibilityNode,
    accesskit::{NodeBuilder, Role},
};
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.)),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.)),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: default(),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });
        });
}