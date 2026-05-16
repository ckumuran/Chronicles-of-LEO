export function greedyMesh(blocks) {
  const mergedFaces = [];
  const visited = new Set();
  for (const block of blocks) {
    const key =
      `${block.x}-${block.y}-${block.z}`;
    if (visited.has(key)) continue;
    visited.add(key);
    mergedFaces.push(block);
  }
  return mergedFaces;
}
