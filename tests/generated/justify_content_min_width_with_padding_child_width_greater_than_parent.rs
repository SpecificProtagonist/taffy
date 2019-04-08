#[test]
fn justify_content_min_width_with_padding_child_width_greater_than_parent() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(1000f32),
                height: stretch::style::Dimension::Points(1584f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![&stretch::node::Node::new(
            stretch::style::Style { ..Default::default() },
            vec![&stretch::node::Node::new(
                stretch::style::Style {
                    justify_content: stretch::style::JustifyContent::Center,
                    min_size: stretch::geometry::Size {
                        width: stretch::style::Dimension::Points(400f32),
                        ..Default::default()
                    },
                    padding: stretch::geometry::Rect {
                        start: stretch::style::Dimension::Points(100f32),
                        end: stretch::style::Dimension::Points(100f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                vec![&stretch::node::Node::new(
                    stretch::style::Style {
                        size: stretch::geometry::Size {
                            width: stretch::style::Dimension::Points(300f32),
                            height: stretch::style::Dimension::Points(100f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    vec![],
                )],
            )],
        )],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 1000f32);
    assert_eq!(layout.size.height, 1584f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
    assert_eq!(layout.children[0usize].size.width, 1000f32);
    assert_eq!(layout.children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].size.width, 500f32);
    assert_eq!(layout.children[0usize].children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].location.x, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].location.y, 0f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.width, 300f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].size.height, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.x, 100f32);
    assert_eq!(layout.children[0usize].children[0usize].children[0usize].location.y, 0f32);
}
