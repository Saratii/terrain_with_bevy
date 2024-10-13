
// #[derive(Component)]
// pub struct ToolBarTag;

// pub fn spawn_tool_bar(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<GridMaterial>>,
//     mut images: ResMut<Assets<Image>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     let tool_bar_image = grid_to_image(&generate_tool_bar_image(), TOOL_BAR_BOX_SIZE as u32 * NUM_BOXES_IN_TOOL_BAR as u32, TOOL_BAR_BOX_SIZE as u32, None);
//     commands.spawn(ToolBarTag)
//             .insert(MaterialMesh2dBundle {
//                 material: materials.add(GridMaterial {
//                     color_map: images.add(tool_bar_image),
//                     size: Vec2::new((TOOL_BAR_BOX_SIZE * NUM_BOXES_IN_TOOL_BAR) as f32, (TOOL_BAR_BOX_SIZE) as f32),
//                 }),
//                 mesh: meshes
//                 .add(Rectangle {
//                     half_size: Vec2::new((TOOL_BAR_BOX_SIZE * NUM_BOXES_IN_TOOL_BAR / 2) as f32, (TOOL_BAR_BOX_SIZE/2) as f32),
//                 })
//                 .into(),
//                 transform: Transform {
//                     translation: Vec3::new(-250., (450-TOOL_BAR_BOX_SIZE/2) as f32, 2.),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             });
// }

// fn generate_tool_bar_image() -> Vec<u8> {
//     // Calculate the total image dimensions
//     let width = TOOL_BAR_BOX_SIZE * NUM_BOXES_IN_TOOL_BAR; // Total image width
//     let height = TOOL_BAR_BOX_SIZE; // Total image height (same as box height)

//     // Initialize the buffer to hold the image data (single value per pixel)
//     let mut data_buffer = Vec::with_capacity(width * height);

//     // Loop over every pixel in the image
//     for y in 0..height {
//         for x in 0..width {
//             // Determine the local x and y within the current box
//             let local_x = x % TOOL_BAR_BOX_SIZE;
//             let local_y = y % TOOL_BAR_BOX_SIZE;

//             // Check if the pixel is at the border (top, bottom, left, or right edge of the box)
//             if local_x == 0 || local_x == TOOL_BAR_BOX_SIZE - 1 || local_y == 0 || local_y == TOOL_BAR_BOX_SIZE - 1 {
//                 // White border pixel
//                 data_buffer.push(WHITE);
//             } else {
//                 // Transparent pixel (inside the box)
//                 data_buffer.push(CLEAR);
//             }
//         }
//     }
    
//     data_buffer
// }