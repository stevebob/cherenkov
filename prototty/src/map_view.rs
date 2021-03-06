use cherenkov::*;
use prototty::*;

pub struct MapView;

const FLOOR: ViewCell = ViewCell::new()
    .with_character('.')
    .with_foreground(colours::WHITE)
    .with_background(colours::BLACK);
const WALL: ViewCell = ViewCell::new()
    .with_character('#')
    .with_foreground(colours::WHITE)
    .with_background(colours::BLACK);
const PLAYER: ViewCell = ViewCell::new()
    .with_character('@')
    .with_bold(true)
    .with_foreground(colours::WHITE)
    .with_background(colours::BLACK);
const TREE: ViewCell = ViewCell::new()
    .with_character('&')
    .with_bold(true)
    .with_foreground(colours::WHITE)
    .with_background(colours::BLACK);

impl View<Cherenkov> for MapView {
    fn view<G: ViewGrid>(
        &mut self,
        game: &Cherenkov,
        offset: Coord,
        depth: i32,
        grid: &mut G,
    ) {
        let to_render = game.to_render();
        for ((coord, cell), visibility) in to_render
            .world
            .grid()
            .enumerate()
            .zip(to_render.visible_area.iter())
        {
            if !visibility.is_discovered() {
                continue;
            }
            let mut view_cell = match cell.background_tile() {
                BackgroundTile::Floor | BackgroundTile::Ground => FLOOR,
                BackgroundTile::Wall => WALL,
            };
            for entity in cell.entity_iter(to_render.world.entities()) {
                let foreground_view_cell =
                    entity.foreground_tile().and_then(|foreground_tile| {
                        match foreground_tile {
                            ForegroundTile::Player => Some(PLAYER),
                            ForegroundTile::Tree => Some(TREE),
                        }
                    });
                if let Some(foreground_view_cell) = foreground_view_cell {
                    view_cell = foreground_view_cell.coalesce(view_cell);
                    break;
                }
            }
            grid.set_cell(offset + coord, depth, view_cell);
        }
    }
}
