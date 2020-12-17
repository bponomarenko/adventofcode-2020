use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone)]
struct Game {
    boxes: HashMap<i8, HashMap<i8, HashMap<i8, HashMap<i8, u8>>>>,
    w_range: RangeInclusive<i8>,
    z_range: RangeInclusive<i8>,
    y_range: RangeInclusive<i8>,
    x_range: RangeInclusive<i8>,
}

impl Game {
    fn active_count(&self) -> u32 {
        self.boxes.values().fold(0, |cube_acc: u32, cube| {
            cube.values().fold(0, |slice_acc: u32, slice| {
                slice
                    .values()
                    .fold(0, |row_acc: u32, row| row_acc + row.values().map(|&v| v as u32).sum::<u32>())
                    + slice_acc
            }) + cube_acc
        })
    }

    fn active_neighbours_count(&self, w: &i8, z: &i8, y: &i8, x: &i8) -> u32 {
        let mut count = 0;
        for dw in (w - 1)..=(w + 1) {
            for dz in (z - 1)..=(z + 1) {
                for dy in (y - 1)..=(y + 1) {
                    for dx in (x - 1)..=(x + 1) {
                        if dw != *w || dz != *z || dy != *y || dx != *x {
                            count += *self.get_value(&dw, &dz, &dy, &dx) as u32;
                        }
                    }
                }
            }
        }
        count
    }

    fn set_value(&mut self, w: &i8, z: &i8, y: &i8, x: &i8, value: u8) {
        self.boxes
            .entry(*w)
            .or_insert(HashMap::new())
            .entry(*z)
            .or_insert(HashMap::new())
            .entry(*y)
            .or_insert(HashMap::new())
            .insert(*x, value);

        // Only update ranges if the value is 1
        if value == 1 {
            self.w_range = ((w - 1) as i8).min(*self.w_range.start())..=((w + 1) as i8).max(*self.w_range.end());
            self.z_range = ((z - 1) as i8).min(*self.z_range.start())..=((z + 1) as i8).max(*self.z_range.end());
            self.y_range = ((y - 1) as i8).min(*self.y_range.start())..=((y + 1) as i8).max(*self.y_range.end());
            self.x_range = ((x - 1) as i8).min(*self.x_range.start())..=((x + 1) as i8).max(*self.x_range.end());
        }
    }

    fn get_value(&self, w: &i8, z: &i8, y: &i8, x: &i8) -> &u8 {
        let cube = self.boxes.get(w);
        if cube.is_none() {
            return &0;
        }
        let slice = cube.unwrap().get(z);
        if slice.is_none() {
            return &0;
        }
        let row = slice.unwrap().get(y);
        if row.is_none() {
            return &0;
        }
        row.unwrap().get(x).unwrap_or(&0)
    }

    fn cycle(&mut self) {
        let clone = self.clone();

        for w in clone.w_range.clone() {
            for z in clone.z_range.clone() {
                for y in clone.y_range.clone() {
                    for x in clone.x_range.clone() {
                        let current_value = *clone.get_value(&w, &z, &y, &x);
                        let active_neighbours_count = clone.active_neighbours_count(&w, &z, &y, &x);
                        // Set new value based on the active neighbours count and current value
                        let new_value = if active_neighbours_count == 3 || (current_value == 1 && active_neighbours_count == 2) {
                            1
                        } else {
                            0
                        };
                        self.set_value(&w, &z, &y, &x, new_value);
                    }
                }
            }
        }
    }
}

pub fn run(input: String) -> u32 {
    let mut game = Game {
        boxes: HashMap::new(),
        w_range: 0..=0,
        z_range: 0..=0,
        y_range: 0..=0,
        x_range: 0..=0,
    };

    // Populate game with initial data
    input.split_ascii_whitespace().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, char)| {
            game.set_value(&0, &0, &(y as i8), &(x as i8), if char == '#' { 1 } else { 0 });
        });
    });

    // Run game iterations
    for _ in 0..6 {
        game.cycle();
    }
    game.active_count()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from(".#.\n..#\n###");
        assert_eq!(run(input), 848);
    }
}
