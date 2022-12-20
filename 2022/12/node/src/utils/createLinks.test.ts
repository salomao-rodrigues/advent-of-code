import { inspect } from 'util'
import { HeightMap, Node } from '../types'
import createLinks, {
  convertHeightCharToNum,
  filterValidNeighbour,
  sortByHeight
} from './createLinks'
import createNode from './createNode'

describe('createLinks', () => {
  it('creates links in a node matrix', () => {
    const map: HeightMap = [
      [
        { name: 'S', links: [] },
        { name: 'a', links: [] }
      ],
      [
        { name: 'b', links: [] },
        { name: 'd', links: [] }
      ]
    ]

    createLinks(map, 1)

    const expected: HeightMap = [
      [
        { name: 'S', links: [] },
        { name: 'a', links: [] }
      ],
      [
        { name: 'b', links: [] },
        { name: 'd', links: [] }
      ]
    ]
    expected[0][0].links = [expected[1][0], expected[0][1]]
    expected[0][1].links = [expected[0][0]]
    expected[1][0].links = [expected[0][0]]
    expected[1][1].links = [expected[1][0], expected[0][1]]

    expect(inspect(map, { depth: 4 })).toEqual(inspect(expected, { depth: 4 }))
  })
})

describe('convertHeightCharToNum', () => {
  it('converts height char to number value', () => {
    expect(convertHeightCharToNum('a')).toBe(97)
    expect(convertHeightCharToNum('p')).toBe(112)
    expect(convertHeightCharToNum('z')).toBe(122)
    // Start position should map to 'a'
    expect(convertHeightCharToNum('S')).toBe(97)
    // End position should map to 'z'
    expect(convertHeightCharToNum('E')).toBe(122)
  })
})

describe('filterValidNeighbour', () => {
  it('checks whether a neightbour node is a valid destination', () => {
    let validator = filterValidNeighbour({ name: 'S', links: [] }, 1)
    expect(validator({ name: 'b', links: [] })).toBe(true)
    expect(validator({ name: 'a', links: [] })).toBe(true)
    expect(validator({ name: 'c', links: [] })).toBe(false)

    validator = filterValidNeighbour({ name: 'j', links: [] }, 1)
    expect(validator({ name: 'h', links: [] })).toBe(true)
    expect(validator({ name: 'i', links: [] })).toBe(true)
    expect(validator({ name: 'j', links: [] })).toBe(true)
    expect(validator({ name: 'k', links: [] })).toBe(true)
    expect(validator({ name: 'l', links: [] })).toBe(false)

    validator = filterValidNeighbour({ name: 'E', links: [] }, 1)
    expect(validator({ name: 'x', links: [] })).toBe(true)
    expect(validator({ name: 'z', links: [] })).toBe(true)
    expect(validator({ name: 'a', links: [] })).toBe(true)
    expect(validator({ name: 'S', links: [] })).toBe(true)
  })
})

describe('sortByHeight', () => {
  it('sorts links by height from tallets to shortest', () => {
    let links: Node['links'] = ['a', 'b', 'c'].map(createNode)
    links.sort(sortByHeight)

    expect(links.map((n) => n.name)).toEqual(['c', 'b', 'a'])

    links = ['S', 'a', 'b'].map(createNode)
    links.sort(sortByHeight)

    expect(links.map((n) => n.name)).toEqual(['b', 'S', 'a'])

    links = ['y', 'z', 'E'].map(createNode)
    links.sort(sortByHeight)

    expect(links.map((n) => n.name)).toEqual(['z', 'E', 'y'])
  })
})
