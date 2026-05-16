import Chunk from "./Chunk";
export default class ChunkManager {
  constructor() {
    this.chunks = new Map();
  }
  getChunkKey(x, z) {
    return `${x},${z}`;
  }
  loadChunk(x, z) {
    const key = this.getChunkKey(x, z);
    if (!this.chunks.has(key)) {
      const chunk = new Chunk(x, z);
      this.chunks.set(key, chunk);
    }
    return this.chunks.get(key);
  }
  unloadChunk(x, z) {
    const key = this.getChunkKey(x, z);
    this.chunks.delete(key);
  }
  getVisibleChunks(playerX, playerZ, distance = 2) {
    const visible = [];
    for (
      let x = playerX - distance;
      x <= playerX + distance;
      x++
    ) {
      for (
        let z = playerZ - distance;
        z <= playerZ + distance;
        z++
      ) {
        visible.push(
          this.loadChunk(x, z)
        );
      }
    }
    return visible;
  }
}
