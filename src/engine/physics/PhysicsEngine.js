import { applyGravity } from "./Gravity";
export default class PhysicsEngine{
update(
player,
velocity,
delta
){
applyGravity(
velocity,
delta
);
player.position.y+=
velocity.y*delta;
if(player.position.y<2){
player.position.y=2;
velocity.y=0;
}
}
}
