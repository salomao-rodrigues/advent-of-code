import sys


def is_tree_visible_from_left(matrix: list[list[int]], row: int, col: int):
    tree = matrix[row][col]

    for neighbour_tree in matrix[row][0:col]:
        if neighbour_tree >= tree:
            return False

    return True


def is_tree_visible_from_right(matrix: list[list[int]], row: int, col: int):
    tree = matrix[row][col]

    for neighbour_tree in matrix[row][col+1:]:
        if neighbour_tree >= tree:
            return False

    return True


def is_tree_visible_from_up(matrix: list[list[int]], row: int, col: int):
    tree = matrix[row][col]
    for row_idx in range(row):
        if matrix[row_idx][col] >= tree:
            return False

    return True


def is_tree_visible_from_down(matrix: list[list[int]], row: int, col: int):
    tree = matrix[row][col]
    for row_idx in range(row+1, len(matrix)):
        if matrix[row_idx][col] >= tree:
            return False

    return True


def is_tree_visible(matrix: list[list[int]], row: int, col: int):
    if row == 0 or row == len(matrix) - 1:
        return True
    elif col == 0 or col == len(matrix[row]) - 1:
        return True
    elif is_tree_visible_from_left(matrix, row, col):
        return True
    elif is_tree_visible_from_right(matrix, row, col):
        return True
    elif is_tree_visible_from_up(matrix, row, col):
        return True
    elif is_tree_visible_from_down(matrix, row, col):
        return True
    else:
        return False


file = open(sys.argv[1], "r")
matrix = []
visible_trees = 0

for line in file:
    row = list(line.strip())
    matrix.append(list(map(int, row)))

for row_idx in range(len(matrix)):
    for col_idx in range(len(matrix[row_idx])):
        if is_tree_visible(matrix, row_idx, col_idx):
            visible_trees += 1

print(visible_trees)
