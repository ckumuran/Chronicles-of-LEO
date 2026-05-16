import { getHeight }
from "../utils/terrain";

export function generateChunk(
chunkX,
chunkZ
){

const blocks=[];

for(let x=0;x<16;x++){

for(let z=0;z<16;z++){

const worldX=
chunkX*16+x;

const worldZ=
chunkZ*16+z;

const height=
getHeight(
worldX,
worldZ
);

for(let y=0;y<height;y++){

blocks.push({

x:worldX,
y,
z:worldZ,

type:
y===height-1
?"grass"
:"dirt"

});

}

}

}

return blocks;

}
