from path_builder import build_path

def test_should_return_correct_path():
    example_path = 'C/Home/MeForFun'
    path = build_path('C', 'Home', 'MeForFun')
    assert example_path == path

