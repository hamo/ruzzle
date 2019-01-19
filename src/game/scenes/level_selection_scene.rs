// use web_sys::console::log_1;
use wasm_bindgen::prelude::JsValue;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::game::World;
use crate::game::level::LEVELS;
use crate::game::constants::{
    TILE_SIZE,
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
    LEVEL_BUTTON_WIDTH,
    LEVEL_BUTTON_HEIGHT,
    LEVEL_BUTTON_MARGIN,
    LEVELS_PER_PAGE,
    ROW_PER_PAGE,
    PAGE_BUTTON_WIDTH,
    PAGE_BUTTON_HEIGHT,
};

pub struct LevelSelectionScene {
    scene_type: SceneType,
    width: f64,
    height: f64,
    next_scene_type: Option<SceneType>,
    current_page: usize,
    horizontal_padding: f64,
    vertical_padding: f64,
    mouse_down_coordinate: Option<(f64, f64)>,
}

impl Scene for LevelSelectionScene {
    fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }
    fn render(&self, renderer: &Renderer, _world: &World, completed_levels: &Vec<bool>) {
        renderer.clear_screen();
        renderer.draw_rectangle(0f64, 0f64, self.width, self.height, &JsValue::from_str("#0d9263"));
        let levels = self.get_levels_by_page();
        for (index, level) in levels.iter().enumerate() {
            let x = self.horizontal_padding + (index % (LEVELS_PER_PAGE / ROW_PER_PAGE)) as f64 * (LEVEL_BUTTON_WIDTH + LEVEL_BUTTON_MARGIN);
            let y = self.vertical_padding + (index as isize / (LEVELS_PER_PAGE / ROW_PER_PAGE) as isize) as f64 * (LEVEL_BUTTON_HEIGHT + LEVEL_BUTTON_MARGIN);
            let mut level_fill_color = JsValue::from_str("#d4ce46");
            if completed_levels[*level] {
                level_fill_color = JsValue::from_str("#cc0000");
            }
            renderer.draw_rectangle(x, y, LEVEL_BUTTON_WIDTH, LEVEL_BUTTON_HEIGHT, &level_fill_color);
        }
        if self.current_page > 0 {
            self.render_last_page_button(renderer);
        }
        if LEVELS.len() > (self.current_page + 1) * LEVELS_PER_PAGE {
            self.render_next_page_button(renderer);
        }
    }
    fn on_mouse_up(&mut self, mouse_x: f64, mouse_y: f64, world: &mut World, current_level_page: &mut usize) {
        let mut mouse_down_level = None;
        let is_next_page_rendered = LEVELS.len() > (self.current_page + 1) * LEVELS_PER_PAGE;
        let is_last_page_rendered = self.current_page > 0;
        if let Some((x, y)) = self.mouse_down_coordinate {
            mouse_down_level = self.calculate_level_from_mouse_position(x, y);
        }
        let mouse_up_level = self.calculate_level_from_mouse_position(mouse_x, mouse_y);
        if let Some(mouse_up_level) = mouse_up_level {
            if let Some(mouse_down_level) = mouse_down_level {
                if mouse_up_level == mouse_down_level {
                    self.set_next_scene_type(SceneType::Game);
                    world.init_level(mouse_up_level);
                    // log_1(&format!("go to level {:?}", mouse_up_level).into());
                }
            }
        }
        if let Some((down_x, down_y)) = self.mouse_down_coordinate {
            if is_next_page_rendered && self.is_next_page_pressed(down_x, down_y, mouse_x, mouse_y) {
                self.current_page += 1;
                *current_level_page += 1;
            }
            if is_last_page_rendered && self.is_last_page_pressed(down_x, down_y, mouse_x, mouse_y) {
                self.current_page -= 1;
                *current_level_page -= 1;
            }
        }
        self.mouse_down_coordinate = None;
    }
    fn on_mouse_down(&mut self, mouse_x: f64, mouse_y: f64, _world: &mut World) {
        if self.mouse_down_coordinate == None {
            self.mouse_down_coordinate = Some((mouse_x, mouse_y));
        }
    }
    fn next_scene_type(&self) -> &Option<SceneType> {
        &self.next_scene_type
    }
    fn set_next_scene_type(&mut self, scene_type: SceneType) {
        self.next_scene_type = Some(scene_type);
    }
}

impl LevelSelectionScene {
    pub fn new(current_page: usize) -> LevelSelectionScene {
        let width = WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
        let height = WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE;
        let horizontal_padding = (width - (LEVELS_PER_PAGE as f64 / ROW_PER_PAGE as f64) * (LEVEL_BUTTON_WIDTH + LEVEL_BUTTON_MARGIN)) / 2f64;
        let vertical_padding = (height - (ROW_PER_PAGE as f64) * (LEVEL_BUTTON_HEIGHT + LEVEL_BUTTON_MARGIN)) / 2f64;
        LevelSelectionScene {
            scene_type: SceneType::LevelSelection,
            width,
            height,
            next_scene_type: None,
            current_page,
            horizontal_padding,
            vertical_padding,
            mouse_down_coordinate: None
        }
    }

    fn get_levels_by_page(&self) -> Vec<usize> {
        let start = self.current_page * LEVELS_PER_PAGE;
        let mut end = start + LEVELS_PER_PAGE - 1;
        if end >= LEVELS.len() {
            end = LEVELS.len() - 1;
        }
        (start..=end).collect()
    }

    fn calculate_level_from_mouse_position(&self, x: f64, y: f64) -> Option<usize> {
        let mut result = None;
        let levels_per_row = (LEVELS_PER_PAGE / ROW_PER_PAGE) as f64;
        let level_button_total_width = LEVEL_BUTTON_WIDTH + LEVEL_BUTTON_MARGIN;
        let level_button_total_height = LEVEL_BUTTON_HEIGHT + LEVEL_BUTTON_MARGIN;

        if x >= self.horizontal_padding && x <= (self.width - self.horizontal_padding) && y >= self.vertical_padding && y <= (self.height - self.vertical_padding) {
            let x = x - self.horizontal_padding;
            let y = y - self.vertical_padding;
            let x_floor = (x / level_button_total_width) as isize;
            let y_floor = (y / level_button_total_height) as isize;
            let x_offset = x - (x_floor as f64 * level_button_total_width);
            let y_offset = y - (y_floor as f64 * level_button_total_height);
            if x_offset <= LEVEL_BUTTON_WIDTH && y_offset <= LEVEL_BUTTON_HEIGHT {
                let clicked_level = y_floor as usize * levels_per_row as usize + x_floor as usize;
                let final_level = self.current_page * LEVELS_PER_PAGE as usize + clicked_level;
                if final_level < LEVELS.len() {
                    result = Some(final_level);
                }

            }
        }
        result
    }

    fn render_last_page_button(&self, renderer: &Renderer) {
        let x = self.horizontal_padding / 2f64 - PAGE_BUTTON_WIDTH;
        let y = self.height / 2f64 - PAGE_BUTTON_HEIGHT;
        renderer.draw_rectangle(x, y, PAGE_BUTTON_WIDTH, PAGE_BUTTON_HEIGHT, &JsValue::from_str("#ffffff"));
    }

    fn render_next_page_button(&self, renderer: &Renderer) {
        let x = self.width - (self.horizontal_padding / 2f64) - PAGE_BUTTON_WIDTH;
        let y = self.height / 2f64 - PAGE_BUTTON_HEIGHT;
        renderer.draw_rectangle(x, y, PAGE_BUTTON_WIDTH, PAGE_BUTTON_HEIGHT, &JsValue::from_str("#ffffff"));
    }

    fn is_next_page_pressed (&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64) -> bool {
        let x0 = self.width - (self.horizontal_padding / 2f64) - PAGE_BUTTON_WIDTH;
        let x1 = x0 + PAGE_BUTTON_WIDTH;
        let y0 = self.height / 2f64 - PAGE_BUTTON_HEIGHT;
        let y1 = y0 + PAGE_BUTTON_HEIGHT;
        let mut result = false;

        if down_x >= x0 && down_x <= x1 && up_x >= x0 && up_x <= x1 &&
            down_y >= y0 && down_y <= y1 && up_y >= y0 && up_y <= y1 {
            result = true;
        }
        result
    }
    fn is_last_page_pressed (&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64) -> bool {
        let x0 = self.horizontal_padding / 2f64 - PAGE_BUTTON_WIDTH;
        let x1 = x0 + PAGE_BUTTON_WIDTH;
        let y0 = self.height / 2f64 - PAGE_BUTTON_HEIGHT;
        let y1 = y0 + PAGE_BUTTON_HEIGHT;
        let mut result = false;

        if down_x >= x0 && down_x <= x1 && up_x >= x0 && up_x <= x1 &&
            down_y >= y0 && down_y <= y1 && up_y >= y0 && up_y <= y1 {
            result = true;
        }
        result
    }
}