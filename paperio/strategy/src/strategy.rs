use paperio_proto::{Cell, Direction, World, MAP_SIZE_CELLS};
use std::cmp::{max, min};
////////////////////////////////////////////////////////////////////////////////

pub struct Strategy {
    cur_goal_index: usize,
    plan: Cell,
    goals: Vec<Cell>,
    previous_direction: Direction,
    got_new_territory: bool,
}

impl Strategy {
    const MIN_SCORE: i32 = -10000;
    pub fn new() -> Self {
        Self {
            plan: Cell(-1, -1),
            cur_goal_index: 0,
            previous_direction: Direction::Left, //not specified
            goals: Vec::new(),
            got_new_territory: true,
        }
    }

    fn plan_route(world: &World) -> Cell {
        let x0 = world.me().position.0;
        let y0 = world.me().position.1;

        let mut best_route: Cell = Cell(-1, -1); //to which cell should we move by rectangle direction
        let mut best_route_score: i32 = Self::MIN_SCORE;

        for x in 0..MAP_SIZE_CELLS {
            for y in 0..MAP_SIZE_CELLS {
                let rectangle_square = (x - x0 + 1).abs() * (y - y0 + 1).abs();

                if rectangle_square <= 1 || dist(x, x0) == 0 || dist(y, y0) == 0 {
                    continue;
                }
                let rect_score = count_new_territory(world, Cell(x, y));
                let danger = calculate_danger(x0, y0, x, y, world);

                let mut route_score = 3 * rect_score - danger * danger;
                if rect_score <= 0 {
                    route_score = Self::MIN_SCORE + 1;
                }
                if route_score > best_route_score {
                    best_route = Cell(x, y);
                    best_route_score = route_score;
                }
            }
        }
        best_route
    }
    fn create_route(&mut self, start: Cell) -> Vec<Cell> {
        let x_route = Cell(self.plan.0, start.1);
        let y_route = Cell(start.0, self.plan.1);

        if start.direction_to(x_route).opposite() == self.previous_direction {
            vec![y_route, self.plan, x_route, start]
        } else {
            vec![x_route, self.plan, y_route, start]
        }
    }

    fn make_move(&mut self, position: Cell) -> Direction {
        self.previous_direction = position.direction_to(self.goals[self.cur_goal_index]);
        self.previous_direction
    }

    pub fn on_tick(&mut self, world: World) -> Direction {
        let my_pos = world.me().position;

        if (self.got_new_territory && own_territory(&world))
            || my_pos == *self.goals.last().unwrap()
        {
            //change route
            self.plan = Self::plan_route(&world);
            self.goals = self.create_route(my_pos);
            self.cur_goal_index = 0;
        } else if my_pos == self.goals[self.cur_goal_index] {
            //start to moving along another vector (to another cell)
            self.cur_goal_index += 1;
        }

        self.got_new_territory = !own_territory(&world);
        self.make_move(my_pos)
    }
}
fn own_territory(world: &World) -> bool {
    world.me().territory.contains(&world.me().position)
}

fn between(checkable: i32, first_border: i32, second_border: i32) -> bool {
    checkable >= first_border && checkable <= second_border
        || checkable >= second_border && checkable <= first_border
}

fn dist(x1: i32, x2: i32) -> i32 {
    (x1 - x2).abs()
}

fn calculate_danger(x0: i32, y0: i32, x1: i32, y1: i32, world: &World) -> i32 {
    let number_moves = (dist(x1, x0) + 2 + dist(y1, y0)) * 2;

    let edges = [Cell(x0, y0), Cell(x1, y1), Cell(x0, y1), Cell(x1, y0)];
    let mut moves_to_kill = 31 * 31;

    for (_id, enemy) in world.iter_enemies() {
        let x_e = enemy.position.0;
        let y_e = enemy.position.1;

        if between(x_e, x1, x0) {
            //perpendicular to y1 or y0
            moves_to_kill = min(min(dist(y_e, y1), dist(y_e, y0)), moves_to_kill);
        } else if between(y_e, y1, y0) {
            //perpendicular to x1 or x0
            moves_to_kill = min(min(dist(x_e, x1), dist(x_e, x0)), moves_to_kill);
        } else {
            //to one of edges
            let min_dist_to_edge = *edges
                .map(|edge| edge.distance_to(enemy.position))
                .iter()
                .min()
                .unwrap();
            moves_to_kill = min(min_dist_to_edge, moves_to_kill);
        }
    }
    number_moves - moves_to_kill
}

fn count_new_territory(world: &World, dst: Cell) -> i32 {
    let mut count = 0;
    let player = world.me();

    let x_from = min(player.position.0, dst.0);
    let y_from = min(player.position.1, dst.1);
    let x_to = max(player.position.0, dst.0);
    let y_to = max(player.position.1, dst.1);

    for x in x_from..=x_to {
        for y in y_from..=y_to {
            if world
                .iter_enemies()
                .any(|(_id, enemy)| enemy.lines.contains(&Cell(x, y)))
            {
                count += 5;
            } else if world
                .iter_enemies()
                .any(|(_id, enemy)| enemy.territory.contains(&Cell(x, y)))
            {
                count += 3;
            } else if !player.territory.contains(&Cell(x, y)) {
                count += 1;
            }
        }
    }
    count
}

impl Default for Strategy {
    fn default() -> Self {
        Self::new()
    }
}
