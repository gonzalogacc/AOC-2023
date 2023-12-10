from typing import List


def input_to_matix(filename: str) -> List[List]:
    matrix = []
    for line in open(filename):
        matrix.append(line.strip())
    return matrix