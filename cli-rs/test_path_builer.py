import unittest
import path_builder as pathbuilder

def test_should_return_correct_path():
    example_path = 'C/Home/MeForFun'
    path = pathbuilder('C', 'Home', 'MeForFun')

    assertEquals(example_path, path)

