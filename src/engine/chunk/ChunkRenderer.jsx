import BrickBlock from "../../blocks/BrickBlock";

import { greedyMesh }
from "./GreedyMesher";

export default function ChunkRenderer({
chunk,
colors
}){

const visibleBlocks=
greedyMesh(chunk.blocks);

return(
<>

{visibleBlocks.map(
(block,index)=>(

<BrickBlock

key={index}

position={[
block.x,
block.y,
block.z
]}

color={
colors[block.type]
}

/>

)
)}

</>
);

}
