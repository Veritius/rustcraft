use bevy::{prelude::*, text::TextSection};

pub struct DebugMenuPlugin;
impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AppendDebugMenuMessage>();

        app.add_startup_system(debug_menu_startup_system);
        app.add_system(debug_menu_system
            .label(SystemLabels::DebugMenuDisplaySystem));
    }
}

#[derive(SystemLabel)]
pub enum SystemLabels {
    DebugMenuDisplaySystem,
}

#[derive(Component)]
struct DebugMenuTextMarker;

#[derive(Resource)]
pub struct DebugMenuOpen;

pub struct AppendDebugMenuMessage {
    pub(crate) text: TextSection
}

impl AppendDebugMenuMessage {
    pub fn new(text: TextSection) -> Self {
        Self { text }
    }
}

fn debug_menu_startup_system(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("No text", Default::default()),
            visibility: Visibility::INVISIBLE,
            ..default()
        }).insert(DebugMenuTextMarker);
    });
}

fn debug_menu_system(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    debug_menu: Option<Res<DebugMenuOpen>>,
    mut text_events: EventReader<AppendDebugMenuMessage>,
    mut debug_text: Query<(&mut Text, &mut Visibility), With<DebugMenuTextMarker>>,
) {
    let (mut text_content, mut text_visibility) = debug_text.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::F3) {
        match debug_menu {
            Some(_) => {
                commands.remove_resource::<DebugMenuOpen>();
                text_visibility.is_visible = false;
            },
            None => {
                commands.insert_resource(DebugMenuOpen);
                text_visibility.is_visible = true;
            },
        }
    }

    if debug_menu.is_none() { return; }
    
    text_content.sections.clear();
    for event in text_events.iter() {
        println!("{}", event.text.value);
        text_content.sections.push(event.text.clone());
    }
}