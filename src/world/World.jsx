import { useEffect,useMemo,useState } from "react";

import ChunkManager
from "../engine/chunk/ChunkManager";

import ChunkRenderer
from "../engine/chunk/ChunkRenderer";

import WaterSystem
from "../engine/water/WaterSystem";

import BlockOutline
from "../blocks/BlockOutline";

import { generateChunk }
from "./generateWorld";

import { colors }
from "../utils/colors";

import {
loadChunk
}
from "../engine/save/SaveSystem";

export default function World(){

const [selected,setSelected]=
useState(null);

const [visibleChunks,
setVisibleChunks]=
useState([]);

const chunkManager=
useMemo(
()=>new ChunkManager(),
[]
);

useEffect(()=>{

const chunks=[];

for(let x=-2;x<=2;x++){

for(let z=-2;z<=2;z++){

const chunk=
chunkManager.loadChunk(
x,
z
);

const savedChunk=
loadChunk(x,z);

chunk.blocks=
savedChunk ||
generateChunk(x,z);

chunks.push(chunk);

}

}

setVisibleChunks(chunks);

},[chunkManager]);

return(
<>

<WaterSystem />

{visibleChunks.map(
(chunk,index)=>(

<group
key={index}
onPointerMissed={()=>
setSelected(null)
}
>

<ChunkRenderer
chunk={chunk}
colors={colors}
/>

</group>

)
)}

<BlockOutline
position={selected}
/>

</>
);

}
