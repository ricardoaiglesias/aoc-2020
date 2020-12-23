import matplotlib.pyplot as plt

with open("out.txt") as f:
    l = list(map(lambda x: int(x), f))
    plt.scatter(l)
    plt.show()
