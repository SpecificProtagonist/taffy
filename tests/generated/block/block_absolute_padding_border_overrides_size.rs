#[test]
#[allow(non_snake_case)]
fn block_absolute_padding_border_overrides_size__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(12f32),
                height: taffy::style::Dimension::Length(12f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(8f32),
                right: taffy::style::LengthPercentage::Length(4f32),
                top: taffy::style::LengthPercentage::Length(2f32),
                bottom: taffy::style::LengthPercentage::Length(6f32),
            },
            border: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(7f32),
                right: taffy::style::LengthPercentage::Length(3f32),
                top: taffy::style::LengthPercentage::Length(1f32),
                bottom: taffy::style::LengthPercentage::Length(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style { display: taffy::style::Display::Block, ..Default::default() },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 22f32, "width of node {:?}. Expected {}. Actual {}", node0, 22f32, size.width);
    assert_eq!(size.height, 14f32, "height of node {:?}. Expected {}. Actual {}", node0, 14f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn block_absolute_padding_border_overrides_size__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Block,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(12f32),
                height: taffy::style::Dimension::Length(12f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(8f32),
                right: taffy::style::LengthPercentage::Length(4f32),
                top: taffy::style::LengthPercentage::Length(2f32),
                bottom: taffy::style::LengthPercentage::Length(6f32),
            },
            border: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(7f32),
                right: taffy::style::LengthPercentage::Length(3f32),
                top: taffy::style::LengthPercentage::Length(1f32),
                bottom: taffy::style::LengthPercentage::Length(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node, 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node, 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 34f32, "width of node {:?}. Expected {}. Actual {}", node0, 34f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node0, 26f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
