use vexide::controller::Controller;

pub fn has_interaction(controller: &Controller) -> bool {
    let state = controller.state().unwrap_or_default();
    state.button_a.is_pressed()
        || state.button_b.is_pressed()
        || state.button_x.is_pressed()
        || state.button_y.is_pressed()
        || state.button_up.is_pressed()
        || state.button_down.is_pressed()
        || state.button_left.is_pressed()
        || state.button_right.is_pressed()
        || state.button_l1.is_pressed()
        || state.button_l2.is_pressed()
        || state.button_r1.is_pressed()
        || state.button_r2.is_pressed()
        || state.left_stick.x() > 0.05
        || state.left_stick.x() < -0.05
        || state.left_stick.y() > 0.05
        || state.left_stick.y() < -0.05
        || state.right_stick.x() > 0.05
        || state.right_stick.x() < -0.05
        || state.right_stick.y() > 0.05
        || state.right_stick.y() < -0.05
}
