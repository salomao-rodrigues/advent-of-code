import { HeightMap, Node } from '../types'

export enum ORIENTATION {
  ASC = 1,
  DESC = -1
}

export const convertHeightCharToNum = (height: string) => {
  switch (height) {
    case 'S':
      return 'a'.charCodeAt(0)
    case 'E':
      return 'z'.charCodeAt(0)
    default:
      return height.charCodeAt(0)
  }
}

export const filterValidNeighbour =
  (current: Node, maxStepDiff: number, orientation = ORIENTATION.ASC) =>
  (neighbour: Node): boolean => {
    if (!neighbour) return false
    const stepDiff =
      convertHeightCharToNum(neighbour.name) -
      convertHeightCharToNum(current.name)
    return orientation * stepDiff <= maxStepDiff
  }

export const sortByHeight = (a: Node, b: Node): number => {
  return convertHeightCharToNum(b.name) - convertHeightCharToNum(a.name)
}

/* Mutates map argument */
const createLinks = (
  map: HeightMap,
  maxStepDiff: number,
  orientation = ORIENTATION.ASC
) => {
  for (let y = 0; y < map.length; y += 1) {
    for (let x = 0; x < map[y].length; x += 1) {
      const up = map?.[y - 1]?.[x]
      const right = map?.[y]?.[x + 1]
      const down = map?.[y + 1]?.[x]
      const left = map?.[y]?.[x - 1]

      map[y][x].links = [up, right, down, left]
        .filter(filterValidNeighbour(map[y][x], maxStepDiff, orientation))
        .sort(sortByHeight)
    }
  }
}

export default createLinks
