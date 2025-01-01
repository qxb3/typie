use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub fn center(
    horizontal: Constraint,
    vertical: Constraint,
    area: Rect,
) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

    area
}
