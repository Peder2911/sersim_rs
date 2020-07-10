
from matplotlib import pyplot as plt
import pandas as pd

dat = pd.read_csv("out/dat.csv")
plt.plot(dat.b,dat.a)
plt.savefig("out/d.png")
