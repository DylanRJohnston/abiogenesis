use bevy::math::{Rect, Vec2};

#[derive(Debug, Clone)]
pub struct SpatialHashGrid<T> {
    cells: Vec<Vec<(Vec2, T)>>,
    bounds: Rect,
    cell_count: (usize, usize),
    cell_size: Vec2,
}

impl<T> SpatialHashGrid<T> {
    pub fn new(bounds: Rect, (x, y): (usize, usize)) -> Self {
        let mut cells = Vec::with_capacity(x * y);
        cells.resize_with(x * y, Vec::new);

        SpatialHashGrid {
            cells,
            bounds,
            cell_count: (x, y),
            cell_size: Vec2::new(
                (bounds.min.x - bounds.max.x).abs() / x as f32,
                (bounds.min.y - bounds.max.y).abs() / y as f32,
            ),
        }
    }

    pub fn update_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.cell_size = Vec2::new(
            (bounds.min.x - bounds.max.x).abs() / self.cell_count.0 as f32,
            (bounds.min.y - bounds.max.y).abs() / self.cell_count.1 as f32,
        );
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
    }

    pub fn insert(&mut self, pos: Vec2, item: T) {
        let grid_pos = self.world_to_grid(pos);

        let index = self.grid_to_index(grid_pos);
        self.cells.get_mut(index).map(|cell| cell.push((pos, item)));
    }

    pub fn query(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = (Vec2, &T)> {
        self.get_query_cells(pos, radius)
            .map(|cell_index| self.grid_to_index(cell_index))
            .flat_map(|index| self.cells[index].iter())
            .filter_map(move |(item_pos, item)| {
                if self.toroidal_distance(*item_pos, pos) <= radius {
                    Some((*item_pos, item))
                } else {
                    None
                }
            })
    }

    fn toroidal_distance(&self, a: Vec2, b: Vec2) -> f32 {
        let width = self.bounds.max.x - self.bounds.min.x;
        let height = self.bounds.max.y - self.bounds.min.y;

        let mut dx = (a.x - b.x).abs();
        let mut dy = (a.y - b.y).abs();

        if dx > width / 2.0 {
            dx = width - dx;
        }

        if dy > height / 2.0 {
            dy = height - dy;
        }

        (dx * dx + dy * dy).sqrt()
    }

    fn world_to_grid(&self, pos: Vec2) -> (i32, i32) {
        let grid_x = ((pos.x - self.bounds.min.x) / self.cell_size.x).floor() as i32;
        let grid_y = ((pos.y - self.bounds.min.y) / self.cell_size.y).floor() as i32;

        (grid_x, grid_y)
    }

    fn wrap_coordinates(&self, (x, y): (i32, i32)) -> (i32, i32) {
        (
            (x % self.cell_count.0 as i32 + self.cell_count.0 as i32) % self.cell_count.0 as i32,
            (y % self.cell_count.1 as i32 + self.cell_count.1 as i32) % self.cell_count.1 as i32,
        )
    }

    fn grid_to_index(&self, (x, y): (i32, i32)) -> usize {
        (y * self.cell_count.0 as i32 + x) as usize
    }

    fn get_query_cells(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = (i32, i32)> {
        let (grid_x, grid_y) = self.world_to_grid(pos);

        gen move {
            let x_radius = (radius / self.cell_size.x).ceil() as i32;
            let y_radius = (radius / self.cell_size.y).ceil() as i32;

            for x in (grid_x as i32 - x_radius)..=(grid_x as i32 + x_radius) {
                for y in (grid_y as i32 - y_radius)..=(grid_y as i32 + y_radius) {
                    yield self.wrap_coordinates((x, y));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Rect;

    use super::*;

    fn init_tracing() {
        let _ = bevy::log::tracing_subscriber::fmt().try_init();
    }

    fn just_values<T>(iter: impl Iterator<Item = (Vec2, T)>) -> Vec<T> {
        iter.map(|(_, value)| value).collect()
    }

    #[test]
    fn test_query_cells_center() {
        init_tracing();

        let grid = SpatialHashGrid::<i32>::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        assert_eq!(
            grid.get_query_cells(Vec2::new(-40.0, -40.0), 10.0)
                .collect::<Vec<_>>(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
    }

    #[test]
    fn test_query_cells_toroidal() {
        init_tracing();

        let grid = SpatialHashGrid::<i32>::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        assert_eq!(
            grid.get_query_cells(Vec2::new(-50.0, -50.0), 10.0)
                .collect::<Vec<_>>(),
            vec![
                (9, 9),
                (9, 0),
                (9, 1),
                (0, 9),
                (0, 0),
                (0, 1),
                (1, 9),
                (1, 0),
                (1, 1)
            ]
        );
    }

    #[test]
    fn test_grid_to_index_conversion() {
        let grid = SpatialHashGrid::<i32>::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );
        // Test simple hash function: row + col * grid_height
        assert_eq!(grid.grid_to_index((0, 0)), 0);
        assert_eq!(grid.grid_to_index((1, 0)), 1);
        assert_eq!(grid.grid_to_index((0, 1)), 10); // 0 + 1 * 10
        assert_eq!(grid.grid_to_index((5, 5)), 55); // 5 + 5 * 10
        assert_eq!(grid.grid_to_index((9, 9)), 99); // 9 + 9 * 10
    }

    #[test]
    fn test_insert_and_query_basic() {
        let mut grid = SpatialHashGrid::<i32>::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        // Insert an item at origin
        grid.insert(Vec2::ZERO, 42);

        // Query at the same location should find the item
        let results: Vec<&i32> = just_values(grid.query(Vec2::ZERO, 10.0));
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], 42);
    }

    #[test]
    fn test_query_within_radius() {
        let mut grid = SpatialHashGrid::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        // Insert items
        grid.insert(Vec2::ZERO, 1);
        grid.insert(Vec2::new(10.0, 0.0), 2); // Within radius
        grid.insert(Vec2::new(20.0, 0.0), 3); // Outside radius

        let results: Vec<&i32> = just_values(grid.query(Vec2::ZERO, 15.0));
        assert!(results.contains(&&1));
        assert!(results.contains(&&2));
        assert!(!results.contains(&&3));
    }

    #[test]
    fn test_toroidal_wrapping_horizontal() {
        let mut grid = SpatialHashGrid::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        // Insert item at far right edge
        grid.insert(Vec2::new(45.0, 0.0), 1);

        // Query from far left edge - should find the item due to wrapping
        let results: Vec<&i32> = just_values(grid.query(Vec2::new(-45.0, 0.0), 20.0));
        assert_eq!(results, vec![&1]);
    }

    #[test]
    fn test_toroidal_wrapping_vertical() {
        let mut grid = SpatialHashGrid::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        // Insert item at top edge
        grid.insert(Vec2::new(0.0, 45.0), 1);

        // Query from bottom edge - should find the item due to wrapping
        let results: Vec<&i32> = just_values(grid.query(Vec2::new(0.0, -45.0), 20.0));
        assert_eq!(results, vec![&1]);
    }

    #[test]
    fn test_toroidal_wrapping_diagonal() {
        let mut grid = SpatialHashGrid::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        // Insert item at top-right corner
        grid.insert(Vec2::new(45.0, 45.0), 1);

        // Query from bottom-left corner - should find the item due to wrapping
        let results: Vec<&i32> = just_values(grid.query(Vec2::new(-45.0, -45.0), 20.0));
        assert_eq!(results, vec![&1]);
    }

    #[test]
    fn test_multiple_items_same_cell() {
        let mut grid = SpatialHashGrid::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        // Insert multiple items in the same cell
        grid.insert(Vec2::new(1.0, 1.0), 1);
        grid.insert(Vec2::new(2.0, 2.0), 2);
        grid.insert(Vec2::new(3.0, 3.0), 3);

        let results: Vec<&i32> = just_values(grid.query(Vec2::new(1.0, 1.0), 2.0));
        assert_eq!(results.len(), 2);
        assert!(results.contains(&&1));
        assert!(results.contains(&&2));
        assert!(!results.contains(&&3));
    }

    #[test]
    fn test_expanding_query_radius() {
        init_tracing();

        let mut grid = SpatialHashGrid::new(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(50.0)),
            (10, 10),
        );

        grid.insert(Vec2::new(0.0, 0.0), 1);
        grid.insert(Vec2::new(10.0, 10.0), 2);
        grid.insert(Vec2::new(15.0, 15.0), 3);
        grid.insert(Vec2::new(-20.0, -20.0), 4);
        grid.insert(Vec2::new(-30.0, 30.0), 5);

        assert_eq!(just_values(grid.query(Vec2::new(0.0, 0.0), 1.0)), vec![&1]);

        assert_eq!(
            just_values(grid.query(Vec2::new(0.0, 0.0), 15.0)),
            vec![&1, &2]
        );

        assert_eq!(
            just_values(grid.query(Vec2::new(0.0, 0.0), 22.0)),
            vec![&1, &2, &3]
        );

        assert_eq!(
            just_values(grid.query(Vec2::new(0.0, 0.0), 30.0)),
            vec![&4, &1, &2, &3,]
        );

        assert_eq!(
            just_values(grid.query(Vec2::new(0.0, 0.0), 42.0)),
            vec![&4, &1, &2, &3,]
        );
    }
}
