import { HeightMap } from '../types'
import createNode from './createNode'
import getStartNode from './getStartNode'

describe('getStartNode', () => {
  it('finds the start point', () => {
    const startNode = createNode('S')
    const map: HeightMap = [
      [createNode('z'), createNode('a'), createNode('g')],
      [createNode('b'), createNode('d'), startNode, createNode('d')]
    ]

    // We use toBe instead of toEqual because we definitely need the correct
    // reference to the node rather than a copy
    expect(getStartNode(map)).toBe(startNode)
  })
})
