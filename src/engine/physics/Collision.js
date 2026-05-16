export function checkCollision(
player,
blocks
){
for(const block of blocks){
const dx=Math.abs(
player.x-block.x
);
const dy=Math.abs(
player.y-block.y
);
const dz=Math.abs(
player.z-block.z
);
if(
dx<0.8 &&
dy<1.5 &&
dz<0.8
){
return true;
}
}
return false;
}
