def test_simple_variable_assignment():
    x = 5
    assert x == 5
    
def test_variable_assignment_with_arithmetic_operation():
    x = 5
    y = 10
    x += y
    assert x == 15
    
def test_variable_assignment_with_expression():
    x = 5
    y = 10
    z = x + y
    assert z == 15
    
def test_variable_assignment_with_comparison():
    x = 5
    y = 10
    z = x < y
    assert z == True
    
def test_variable_assignment_with_logical_operation():
    x = True
    y = False
    z = x and y
    assert z == False