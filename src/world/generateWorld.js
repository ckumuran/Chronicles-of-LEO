import { worldBlocks } from "./worldData";
import { getHeight } from "../utils/terrain";
export function generateWorld() {
  for (let x = -20; x < 20; x++) {
    for (let z = -20; z < 20; z++) {
      const height = getHeight(x, z);
      for (let y = 0; y < height; y++) {
        worldBlocks.push({
          x,
          y,
          z,
          type: y === height - 1 ? "grass" : "dirt"
        });

      }

    }

  }

}
