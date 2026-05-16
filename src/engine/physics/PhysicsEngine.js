import { applyGravity } from "./Gravity";
import { checkCollision }
from "./Collision";
export default class PhysicsEngine{
update(
player,
velocity,
delta,
blocks
){
applyGravity(
velocity,
delta
);
const oldY=
player.position.y;
player.position.y+=
velocity.y*delta;
if(
checkCollision(
player,
blocks
)
){
player.position.y=oldY;
velocity.y=0;
}
if(player.position.y<2){
player.position.y=2;
velocity.y=0;
}
}
}
