use crate::state::OverlayDroppingPickingState;

use super::*;

pub fn update_credit_position(
    mut scene_query: Query<&mut Style, With<EndingCreditScene>>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<OverlayDroppingPickingState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut scene) = scene_query.get_single_mut() {
        if keyboard_input.any_pressed([KeyCode::Space]) {
            scene.bottom.add(100. * time.delta_seconds());
        } else {
            scene.bottom.add(25. * time.delta_seconds());
        }
        if scene.bottom.get() >= 100. {
            app_state.set(OverlayDroppingPickingState::Opened);
        }
    }
}
