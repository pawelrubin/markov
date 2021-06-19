from matplotlib import pyplot as plt
import pandas as pd

data = pd.read_csv("result.txt", sep="\s+", header=None)
data = pd.DataFrame(data)

pos = data[0]
neg = data[1]

plt.plot(pos, label="+1")
plt.plot(neg, label="-1")
plt.xlabel("Iterations")
plt.ylabel("Distribution")
plt.legend()
plt.show()

# data = pd.read_csv("trajectory.txt", sep="\s+", header=None)
# data = pd.DataFrame(data)
# plt.plot(data)
# plt.show()
