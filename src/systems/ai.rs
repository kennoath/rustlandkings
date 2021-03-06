use crate::systems::command::*;
use crate::simulation_state::*;
use crate::entity::*;

pub fn compute_ai_commands(state: &SimulationState, commands: &mut Vec<Command>) {
    for (enemy_id, enemy) in state.entities.iter() {
        match enemy.variety {
            EntityType::Enemy => {
                // look for a target to shoot
                for (target_id, target) in state.entities.iter().filter(|(_, e)| e.variety == EntityType::Player) {
                    let distance = enemy.aabb.center().sub(target.aabb.center()).magnitude();
                    if distance < 0.5 && state.terrain.raycast(enemy.aabb.center(), target.aabb.center()) == None {
                        commands.push(Command::Look(*enemy_id, target.aabb.center().sub(enemy.aabb.center()).normalize()));
                        commands.push(Command::Shoot(*enemy_id));
                    }
                }
            },
            EntityType::Swarmer => {
                for (target_id, target) in state.entities.iter().filter(|(_, e)| e.variety == EntityType::Player) {
                    let distance = enemy.aabb.center().sub(target.aabb.center()).magnitude();
                    let dir = target.aabb.center().sub(enemy.aabb.center()).normalize();
                    if distance < 1.0 && state.terrain.raycast(enemy.aabb.center(), target.aabb.center()) == None {
                        commands.push(Command::Walk(*enemy_id, dir));
                    }
                }
            },
            EntityType::Sprayer => {
                let engagement_range = 0.5;

                for (target_id, target) in state.entities.iter().filter(|(_, e)| e.variety == EntityType::Player) {
                    let distance = enemy.aabb.center().sub(target.aabb.center()).magnitude();
                    let dir = target.aabb.center().sub(enemy.aabb.center()).normalize();

                    if distance > 1.0 {continue};
                    if state.terrain.raycast(enemy.aabb.center(), target.aabb.center()) != None {continue};

                    if distance > engagement_range * 1.1 {
                        commands.push(Command::Walk(*enemy_id, dir));
                    } else if distance < engagement_range * 0.9 {
                        commands.push(Command::Walk(*enemy_id, dir.mul_scalar(-1.0)));
                    } else {
                        commands.push(Command::Look(*enemy_id, target.aabb.center().sub(enemy.aabb.center()).normalize()));
                        commands.push(Command::Shoot(*enemy_id));
                    }
                }
            },
            EntityType::Bigdog => {
                let engagement_range = 1.0;

                for (target_id, target) in state.entities.iter().filter(|(_, e)| e.variety == EntityType::Player) {
                    let distance = enemy.aabb.center().sub(target.aabb.center()).magnitude();
                    let dir = target.aabb.center().sub(enemy.aabb.center()).normalize();

                    if distance > 1.0 {continue};
                    if state.terrain.raycast(enemy.aabb.center(), target.aabb.center()) != None {continue};

                    if distance > engagement_range * 1.1 {
                        commands.push(Command::Walk(*enemy_id, dir));
                    } else if distance < engagement_range * 0.9 {
                        commands.push(Command::Look(*enemy_id, target.aabb.center().sub(enemy.aabb.center()).normalize()));
                        commands.push(Command::Shoot(*enemy_id));
                        commands.push(Command::Walk(*enemy_id, dir));
                    }
                }
            },
            _ => {},
        }
    }
}