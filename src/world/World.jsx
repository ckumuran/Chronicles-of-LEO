import { useEffect, useMemo, useState } from "react";
import ChunkManager from "../engine/chunk/ChunkManager";
import ChunkRenderer from "../engine/chunk/ChunkRenderer";
import { generateWorld } from "./generateWorld";
import { colors } from "../utils/colors";
import useBlockInteraction from "../hooks/useBlockInteraction";
import BlockOutline from "../blocks/BlockOutline";
import WaterSystem from "../engine/water/WaterSystem";
export default function World() {
  const [selected, setSelected] = useState(null);
  const [playerChunk] = useState({
    x: 0,
    z: 0
  });
  const chunkManager = useMemo(() => {
    return new ChunkManager();
  }, []);
  useEffect(() => {
    const generatedBlocks =
      generateWorld();
    for (const block of generatedBlocks) {

      const chunkX =
        Math.floor(block.x / 16);

      const chunkZ =
        Math.floor(block.z / 16);

      const chunk =
        chunkManager.loadChunk(
          chunkX,
          chunkZ
        );

      chunk.addBlock(block);
    }
  }, [chunkManager]);
  const visibleChunks =
    chunkManager.getVisibleChunks(
      playerChunk.x,
      playerChunk.z,
      2
    );
  const {
    breakBlock,
    placeBlock
  } = useBlockInteraction([]);
  return (
    <>
      <WaterSystem />
      {visibleChunks.map((chunk, index) => (
        <group key={index}>
          {chunk.blocks.map((block) => (
            <group
              key={
                `${block.x}-${block.y}-${block.z}`
              }
              onPointerOver={() => {
                setSelected([
                  block.x,
                  block.y,
                  block.z
                ]);
              }}
              onClick={(e) => {
                e.stopPropagation();
                breakBlock(block);
                chunk.removeBlock(
                  block.x,
                  block.y,
                  block.z
                );
              }}
              onContextMenu={(e) => {
                e.stopPropagation();
                const newBlock = {
                  x: block.x,
                  y: block.y + 1,
                  z: block.z,
                  type: "red"
                };
                chunk.addBlock(newBlock);
                placeBlock(block);
              }}
            >
              <ChunkRenderer
                chunk={{
                  blocks: [block]
                }}
                colors={colors}
              />
            </group>
          ))}
        </group>
      ))}
      <BlockOutline
        position={selected}
      />
    </>
  );
}
