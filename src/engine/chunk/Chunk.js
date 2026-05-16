export default class Chunk {
  constructor(x, z, size = 16) {
    this.x = x;
    this.z = z;
    this.size = size;
    this.blocks = [];
  }
  addBlock(block) {
    this.blocks.push(block);
  }
  removeBlock(x, y, z) {
    this.blocks = this.blocks.filter(
      block =>
        !(
          block.x === x &&
          block.y === y &&
          block.z === z
        )
    );
  }
}
