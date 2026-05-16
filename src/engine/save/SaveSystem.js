export function saveChunk(
chunk
){

const key=
`chunk_${chunk.x}_${chunk.z}`;

localStorage.setItem(
key,
JSON.stringify(chunk.blocks)
);

}

export function loadChunk(
x,
z
){

const key=
`chunk_${x}_${z}`;

const data=
localStorage.getItem(key);

if(!data)return null;

return JSON.parse(data);

}

export function saveWorld(
chunks
){

chunks.forEach(chunk=>{

saveChunk(chunk);

});

}
