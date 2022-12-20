import createLinks from './createLinks'
import findPath from './findPath'
import getStartNode from './getStartNode'
import parseLinesToMatrix from './parseLinesToHeightMap'

describe('findPath', () => {
  it('finds the shortest path lenght between S (Start) and E (End) in a HeightMap', () => {
    const map = parseLinesToMatrix([
      'Sabqponm',
      'abcryxxl',
      'accszExk',
      'acctuvwj',
      'abdefghi'
    ])

    createLinks(map, 1)
    const startNode = getStartNode(map)

    expect(findPath(startNode)).toBe(31)
  })
})
