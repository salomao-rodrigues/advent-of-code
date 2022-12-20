import { Node } from '../types'
import createNode from './createNode'

describe('createNode', () => {
  it('creates a node given a node name', () => {
    const expected: Node = { name: 'a', links: [] }
    expect(createNode('a')).toEqual(expected)
    expected.name = 'z'
    expect(createNode('z')).toEqual(expected)
    expected.name = 'S'
    expect(createNode('S')).toEqual(expected)
    expected.name = 'E'
    expect(createNode('E')).toEqual(expected)
  })

  it('throws if node has an invalid name', () => {
    expect(() => createNode('-')).toThrow()
    expect(() => createNode('A')).toThrow()
    expect(() => createNode('F')).toThrow()
    expect(() => createNode('R')).toThrow()
  })
})
