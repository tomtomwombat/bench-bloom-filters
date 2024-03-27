'''
This script plot false positive rate rates of various Rust Bloom filters.
The false pos rates are calculated in Rust in lib.rs.

(sorry, super jank):

1. Edit lib.rs for a particular filter

2.
```
cargo run --release > BloomFilter-False-Positives/my_filter.csv
```

3. edit `filters` in this file

4. run this file
'''

from math import log10, log2
import matplotlib.pyplot as plt 
import csv

filters = [
    ('test', 'b'),
    ]

size = 262144
for (name, color) in filters:
    fp = []
    data = []
    with open('BloomFilter-False-Positives/%s.csv' % name,'r') as csvfile: 
        rows = csv.reader(csvfile, delimiter = ',')
          
        for row in rows:
            if int(row[1]) != size: continue
            data.append((int(row[0]), float(row[3]), int(row[2])))
            fp.append(float(row[3]))
        x,y,z = zip(*sorted(data))
        plt.plot(x, y, color=color, label=name, linewidth=2.0)

plt.xlabel('Number of Items') 
plt.ylabel('False Positive Rate') 
plt.title('Bloom Filter False Positives - %d bytes' % size)
plt.ylim(0, 0.1)
plt.legend() 
plt.show() 


