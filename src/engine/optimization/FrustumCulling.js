export function isChunkVisible(
  chunkX,
  chunkZ,
  playerX,
  playerZ,
  distance = 3
) {
  const dx =
    Math.abs(chunkX - playerX);
  const dz =
    Math.abs(chunkZ - playerZ);
  return (
    dx <= distance &&
    dz <= distance
  );
}
