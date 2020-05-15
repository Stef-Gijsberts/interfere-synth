import numpy as np

names_independent = ["One", "OscA", "OscB"]
names_dependent = ["OscAVolume", "OscBVolume", "OscAPitch", "OscBPitch"]

independent = np.array([1.0, 0.8, 0.0])

weights = np.array([
    [1, 0, 0],
    [0.1, 0, 0],
    [0.5, 0.4, 0],
    [0.7, 0, 0],
])

dependent = independent @ weights.T

for name, val in zip(names_independent, independent):
    print(name, val)

print()

print(weights)

print()

for name, val in zip(names_dependent, dependent):
    print(name, val)
