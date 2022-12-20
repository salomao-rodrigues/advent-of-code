import parseLinesToHeightMap from './parseLinesToHeightMap'

describe('parseLinesToHeightMap', () => {
  it('parses lines to a matrix', () => {
    const lines = ['Sa', 'bc']
    const expected = [
      [
        { name: 'S', links: [] },
        { name: 'a', links: [] }
      ],
      [
        { name: 'b', links: [] },
        { name: 'c', links: [] }
      ]
    ]

    expect(parseLinesToHeightMap(lines)).toEqual(expected)
  })
})
