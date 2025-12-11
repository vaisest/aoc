import sympy

# [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
# counters
#  7: affected by buttons 1, 3, 4
#  5: affected by buttons 4, 5
# 12: affected by buttons 1, 2, 4, 5
#  7: affected by buttons 1, 2, 5
#  2: affected by buttons 1, 5
# so we get matrix
# 1 0 1 1 0 | 7
# 0 0 0 1 1 | 5
# 1 1 0 1 1 | 12
# 1 1 0 0 1 | 7
# 1 0 0 0 1 | 2
M = sympy.Matrix(
    [
        [1, 0, 1, 1, 0, 7],
        [0, 0, 0, 1, 1, 5],
        [1, 1, 0, 1, 1, 12],
        [1, 1, 0, 0, 1, 7],
        [1, 0, 0, 0, 1, 2],
    ]
)
print(M.rref())
# [1, 0, 0, 0, 0, 2],
# [0, 1, 0, 0, 0, 5],
# [0, 0, 1, 0, 0, 0],
# [0, 0, 0, 1, 0, 5],
# [0, 0, 0, 0, 1, 0]]), (0, 1, 2, 3, 4))


# [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
# counters
# 10: affected by buttons 1, 2, 3
# 11: affected by buttons 4, 5
# 11: affected by buttons 1, 3, 4
#  5: affected by buttons 1, 2
# 10: affected by buttons 1, 2, 3
#  5: affected by buttons 3
# so we get matrix
# 1 1 1 0 0 | 10
# 0 0 0 1 1 | 11
# 1 0 1 1 0 | 11
# 1 1 0 0 0 | 5
# 1 1 1 0 0 | 10
# 0 0 1 0 0 | 5
M = sympy.Matrix(
    [
        [1, 1, 1, 0, 0, 10],
        [0, 0, 0, 1, 1, 11],
        [1, 0, 1, 1, 0, 11],
        [1, 1, 0, 0, 0, 5],
        [1, 1, 1, 0, 0, 10],
        [0, 0, 1, 0, 0, 5],
    ]
)
print(M.rref())
# (Matrix([
# [1, 0, 0, 0, -1, -5],
# [0, 1, 0, 0,  1, 10],
# [0, 0, 1, 0,  0,  5],
# [0, 0, 0, 1,  1, 11],
# [0, 0, 0, 0,  0,  0],
# [0, 0, 0, 0,  0,  0]]), (0, 1, 2, 3))
