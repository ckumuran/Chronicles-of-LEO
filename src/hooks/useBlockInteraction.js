import { useState } from "react";
export default function useBlockInteraction(initialBlocks) {
  const [blocks, setBlocks] = useState(initialBlocks);
  function breakBlock(target) {
    setBlocks(prev =>
      prev.filter(
        block =>
          !(
            block.x === target.x &&
            block.y === target.y &&
            block.z === target.z
          )
      )
    );
  }
  function placeBlock(target) {
    setBlocks(prev => [
      ...prev,
      {
        x: target.x,
        y: target.y + 1,
        z: target.z,
        type: "red"
      }

    ]);
  }
  return {
    blocks,
    breakBlock,
    placeBlock
  };
}
