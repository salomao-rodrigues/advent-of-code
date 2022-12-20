import { HeightMap } from '../types'

const getStartNode = (map: HeightMap, startChar = 'S') => {
  for (const line of map) {
    for (const node of line) {
      if (node.name === startChar) {
        return node
      }
    }
  }

  throw Error('Could not find starting point')
}

export default getStartNode
