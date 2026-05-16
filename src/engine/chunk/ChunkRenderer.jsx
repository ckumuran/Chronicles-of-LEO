import BrickBlock from "../../blocks/BrickBlock";
export default function ChunkRenderer({
  chunk,
  colors
}) {
  return (
    <>
      {chunk.blocks.map((block, index) => (
        <BrickBlock
          key={index}
          position={[
            block.x,
            block.y,
            block.z
          ]}
          color={colors[block.type]}
        />
      ))}
    </>
  );
}
