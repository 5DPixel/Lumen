use imgui::{Context};

pub fn ui_theme(imgui: &mut Context) {
    let style = imgui.style_mut();
    #[allow(unused_mut)]
    let mut colors = &mut style.colors;

    // Minimalist rounding
    style.window_rounding = 4.0;
    style.frame_rounding = 2.0;
    style.grab_rounding = 2.0;
    style.scrollbar_rounding = 2.0;

    // Dense, compact layout
    style.frame_padding = [5.0, 3.0];
    style.item_spacing = [6.0, 4.0];
    style.item_inner_spacing = [4.0, 3.0];
    style.window_padding = [8.0, 8.0];
    style.window_border_size = 1.0;
    style.popup_border_size = 1.0;
    style.scrollbar_size = 12.0;
    style.grab_min_size = 8.0;

    use imgui::StyleColor::*;

    // Main theme: flat dark with subtle contrast
    colors[Text as usize] = [0.95, 0.95, 0.95, 1.00];
    colors[TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    colors[WindowBg as usize] = [0.13, 0.13, 0.14, 1.00];
    colors[ChildBg as usize] = [0.13, 0.13, 0.14, 1.00];
    colors[PopupBg as usize] = [0.16, 0.16, 0.17, 1.00];
    colors[Border as usize] = [0.30, 0.30, 0.30, 0.60];

    // Frame (input boxes, sliders)
    colors[FrameBg as usize] = [0.20, 0.20, 0.21, 1.00];
    colors[FrameBgHovered as usize] = [0.25, 0.25, 0.26, 1.00];
    colors[FrameBgActive as usize] = [0.28, 0.28, 0.30, 1.00];

    // Title bars
    colors[TitleBg as usize] = [0.14, 0.14, 0.15, 1.00];
    colors[TitleBgActive as usize] = [0.18, 0.18, 0.19, 1.00];
    colors[TitleBgCollapsed as usize] = [0.10, 0.10, 0.10, 0.60];

    // Scrollbars
    colors[ScrollbarBg as usize] = [0.10, 0.10, 0.10, 0.50];
    colors[ScrollbarGrab as usize] = [0.31, 0.31, 0.33, 1.00];
    colors[ScrollbarGrabHovered as usize] = [0.36, 0.36, 0.38, 1.00];
    colors[ScrollbarGrabActive as usize] = [0.41, 0.41, 0.43, 1.00];

    // Interactive highlights (orange-based)
    colors[CheckMark as usize] = [1.00, 0.55, 0.15, 1.00];
    colors[SliderGrab as usize] = [1.00, 0.55, 0.15, 1.00];
    colors[SliderGrabActive as usize] = [1.00, 0.60, 0.20, 1.00];
    colors[Button as usize] = [0.20, 0.20, 0.22, 1.00];
    colors[ButtonHovered as usize] = [1.00, 0.55, 0.15, 1.00];
    colors[ButtonActive as usize] = [1.00, 0.60, 0.20, 1.00];

    // Tabs & headers
    colors[Header as usize] = [0.25, 0.25, 0.28, 1.00];
    colors[HeaderHovered as usize] = [1.00, 0.55, 0.15, 0.80];
    colors[HeaderActive as usize] = [1.00, 0.60, 0.20, 1.00];

    colors[Tab as usize] = [0.17, 0.17, 0.18, 1.00];
    colors[TabHovered as usize] = [1.00, 0.55, 0.15, 0.80];
    colors[TabActive as usize] = [0.24, 0.24, 0.25, 1.00];
    colors[TabUnfocused as usize] = [0.14, 0.14, 0.15, 1.00];
    colors[TabUnfocusedActive as usize] = [0.20, 0.20, 0.22, 1.00];

    colors[SeparatorHovered as usize] = [1.00, 0.55, 0.15, 0.78];
    colors[SeparatorActive as usize] = [1.00, 0.60, 0.20, 1.00];

    colors[ResizeGrip as usize] = [0.20, 0.20, 0.22, 1.00];
    colors[ResizeGripHovered as usize] = [1.00, 0.55, 0.15, 0.70];
    colors[ResizeGripActive as usize] = [1.00, 0.60, 0.20, 0.90];
}
