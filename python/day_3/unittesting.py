from day_3.main import input_to_matix



def test_read_matrix():
    matrix = input_to_matix('./test_input.txt')
    print(matrix)
    assert 1==11