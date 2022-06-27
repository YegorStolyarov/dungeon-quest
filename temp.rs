fn button_bundle(
    main_menu_scene_button: MainMenuSceneButton,
    asset_server: &Res<AssetServer>,
) -> ButtonBundle {
    let size = Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT));

    let possition: [f32; 2] = match main_menu_scene_button {
        MainMenuSceneButton::Play => BUTTON_POSITIONS[0],
    };

    ButtonBundle {
        style: Style {
            size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position: Rect {
                left: Val::Px(possition[0]),
                top: Val::Px(possition[1]),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        image: UiImage(asset_server.load("images/panel_Example1.png")),
        ..Default::default()
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &MainMenuSceneButton, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<State<ApplicationScene>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, mut color, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => {
                text.sections[0].style.color = Color::WHITE.into();
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::GREEN.into();
            }
            Interaction::Clicked => {
                text.sections[0].style.color = Color::RED.into();
                match button {
                    MainMenuSceneButton::Play => state
                        .set(ApplicationScene::LoadingScene)
                        .expect("Couldn't switch state to Loading Screen"),
                }
            }
        }
    }
}

fn text_bundle(
    main_menu_scene_button: MainMenuSceneButton,
    asset_server: &Res<AssetServer>,
) -> TextBundle {
    let text: &str = match main_menu_scene_button {
        MainMenuSceneButton::Play => "PLAY",
    };

    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/DungeonFont.ttf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    }
}
