from sportgems import example_fn

result = example_fn([1,2,3], [(1.2, 5.2), (0.7, 5.3)])
print(f"got result: {result}")
assert result == [1,2,3]
