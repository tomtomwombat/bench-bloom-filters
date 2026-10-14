'''
Plots false positive rate rates of various Rust Bloom filters.
The false pos rates are calculated in Rust in main.rs.

```
cargo run --release
python falsepos.py
```
'''

from math import log10, log2, log
import matplotlib.pyplot as plt 
import csv
from matplotlib import colormaps
from matplotlib.ticker import ScalarFormatter
from matplotlib.ticker import FuncFormatter
from matplotlib.ticker import LogLocator
from matplotlib.ticker import MultipleLocator
import math
plt.rcParams['font.size'] = 18

viridis = colormaps['viridis']
magma = colormaps['magma']

alpha = 1
lw = 2.5

cm = [colormaps['tab10'](i / 7) for i in range(7)][::-1]
filters = [    
    ('bloom', cm[0], alpha, lw),
    ('bloomfilter', cm[1], alpha, lw),
    ('sbbf', cm[2], alpha, lw),
    ('probabilistic-collections', cm[3], alpha, lw),
    ('solana-bloom', cm[4], alpha, lw),
    ('datasketches', cm[5], 1, lw),
    
    ('fastbloom', cm[6], 1, lw),
]

fig, ax = plt.subplots()

def custom_format(yy, _):
    if yy >= 1:
        return f"{int(yy)}"
    else:
        return f"{yy:.8f}".rstrip("0").rstrip(".")

for i, (name, color, aa, lw) in enumerate(filters):
    file_name = 'Acc/%s.csv' % name
    print(file_name)
    with open(file_name, 'r') as csvfile:
        data = []
        rows = csv.reader(csvfile, delimiter = ',')
        for row in rows:
            if row[1] == 'NaN':
                continue
            num_items = float(row[0])
            avg_y = float(row[1])*100.0
            min_y = float(row[2])*100.0
            max_y = float(row[3])*100.0

            data.append((num_items, avg_y, min_y, max_y))

        x,avg_y,min_y,max_y = zip(*data)
        
        ax.plot(x, avg_y, color=color, label=name, linewidth=lw, alpha=aa)
        ax.fill_between(x, max_y, min_y, color = color, alpha = aa*0.15)
        ax.set_yscale('log')
        #ax.set_xscale('log')

plt.xlabel('Items Per Bit') 
plt.ylabel('False Positive %') 

plt.title('Bloom Filter False Positives (Lower is Better)')

plt.xlim(left=0)
plt.xlim(right=0.08)
plt.ylim(bottom=0.00000001)
plt.ylim(top=10)

plt.gca().yaxis.set_major_formatter(ScalarFormatter())
plt.gca().yaxis.get_major_formatter().set_scientific(False)
plt.gca().yaxis.set_major_locator(LogLocator(base=10, subs=(1.0,), numticks=100))
plt.gca().yaxis.set_major_formatter(FuncFormatter(custom_format))
plt.gca().xaxis.set_major_locator(MultipleLocator(0.01))
plt.gca().xaxis.set_major_formatter(
    FuncFormatter(lambda xx, _: '0.0' if math.isclose(xx, 0.0, abs_tol=1e-12) else f'{xx:.2f}')
)

handles,labels = ax.get_legend_handles_labels()

handles = [handles.pop()] + handles
labels = [labels.pop()] + labels

plt.grid()
plt.legend(handles, labels, loc='lower right')
plt.show()

