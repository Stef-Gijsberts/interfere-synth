import numpy as np

num_voices = 16

num_independents_global = 5
num_independents_voice = 7

num_dependents_global = 5
num_dependents_voice = 2


independents_global = np.zeros((1, num_independents_global))
independents_voices = np.zeros((num_voices, num_independents_voice))

independents_global[0][0] = 1.0
independents_voices[0] = 1.0

print(independents_global)
print(independents_voices)

connections_global_global = np.zeros((num_independents_global, num_dependents_global))
connections_global_voices = np.zeros((num_independents_global, num_dependents_voice))
connections_voices_voices = np.zeros((num_independents_voice, num_dependents_voice))

dependents_global = independents_global @ connections_global_global
dependents_voices = independents_global @ connections_global_voices + independents_voices @ connections_voices_voices


assert dependents_global.shape == (1, num_dependents_global)
assert dependents_voices.shape == (num_voices, num_dependents_voice)

