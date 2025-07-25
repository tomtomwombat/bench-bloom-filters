'''
This script plot false positive rate rates of various Rust Bloom filters.
The false pos rates are calculated in Rust in main.rs.

1.
```
cargo run --release
```

2. run this file
'''

from math import log10, log2, log
import matplotlib.pyplot as plt 
import csv
from matplotlib import colormaps

plt.rcParams['font.size'] = 18

viridis = colormaps['viridis']
magma = colormaps['magma']

filters = [
    
    ('sbbf', magma(1 / 5)),
    #('fastbloom-rs', magma(1/ 5)),
    ('bloom',  magma(2 / 5)),
    ('bloomfilter', magma(3 / 5)),
    ('probabilistic-collections', magma(4 /5)),
    
    #('fastbloom-old', viridis(4 / 4)),
    ('fastbloom', viridis(2 / 4)),
    ]

fig, ax = plt.subplots()

size = 16384
for i, (name, color) in enumerate(filters):
    data = []
    with open('BloomFilter-False-Positives/%s.csv' % name,'r') as csvfile: 
        rows = csv.reader(csvfile, delimiter = ',')
        for row in rows:
            num_items = int(row[0])
            fp = float(row[1])*100.0

            data.append((num_items, fp))

        x,y = zip(*sorted(data))
        
        r = 2

        y = [0.00000000001 + u for u in y]
        min_y = [min(y[i-r:i+r+1])for i in range(r, len(y) - r)]
        max_y = [max(y[i-r:i+r+1])for i in range(r, len(y) - r)]
        smooth_y = [sum(y[i-r:i+r+1]) / (1 + 2*r) for i in range(r, len(y) - r)]
        x = x[r:len(x) - r]
        ax.plot(x, smooth_y, color=color, label=name, linewidth=2.5)
        ax.fill_between(x, max_y, min_y, color = color, alpha = 0.15)
        ax.set_yscale('log')

plt.xlabel('Number of Items in Bloom Filter') 
plt.ylabel('False Positive %')
plt.title('Bloom Filter False Positive Rate (%d bytes size)' % size)
'''
micro = False
if micro:
    # plt.ylim(0, 0.0004)
    plt.ylim(0, 0.001)
    plt.xlim(0, 20000)
else:
    plt.xlim(0, 65000)
'''
plt.grid()
plt.legend(loc='upper left') 
plt.show()