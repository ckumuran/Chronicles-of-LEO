export function greedyMesh(
blocks
){

const visible=[];

const occupied=
new Set();

for(const block of blocks){

occupied.add(
`${block.x},${block.y},${block.z}`
);

}

for(const block of blocks){

const top=
`${block.x},
${block.y+1},
${block.z}`;

if(
!occupied.has(top)
){

visible.push(block);

}

}

return visible;

}
