export function checkCollision(
player,
blocks
){
for(const block of blocks){
const dx=Math.abs(
player.position.x-block.x
);
const dy=Math.abs(
player.position.y-block.y
);
const dz=Math.abs(
player.position.z-block.z
);
if(
dx<0.9 &&
dy<1.7 &&
dz<0.9
){
return true;
}
}
return false;
}
