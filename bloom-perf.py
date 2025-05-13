import json
import os
import matplotlib.pyplot as plt
from matplotlib import colormaps

plt.rcParams['font.size'] = 12

viridis = colormaps['viridis']
magma = colormaps['cool']

filters = [
    ('sbbf', magma(0)),
    ('fastbloom-rs', magma(1/ 5)),
    ('bloom',  magma(2/ 5)),
    ('bloomfilter', magma(3 / 5)),
    ('probabilistic-collections', magma(4 /5)),

    ('fastbloom - 64', viridis(0)),
    ('fastbloom - 128', viridis(1/ 3)),
    ('fastbloom - 256', viridis(2 /4)),
    ('fastbloom', viridis(3/ 4)),
    ]

filters = dict(filters)

directory = r"target\criterion"

def is_input(x):
    try:
        int(x)
        return True
    except:
        return False

def add_labels(x, y):
    for i in range(len(x)):
        plt.text(i, y[i], round(y[i], 2), ha='center')

def get_immediate_subdirectories(a_dir):
    return [name for name in os.listdir(a_dir) if os.path.isdir(os.path.join(a_dir, name))]

def get_non_reports(d):
     return [x for x in get_immediate_subdirectories(d) if x != 'report']


for benches_name, title in zip(get_non_reports(directory), [
    'Member Check Speed for Bloom Filters',
    'Non-Member Check Speed for Bloom Filters']):
    avg_y = []
    min_y = []
    max_y = []
    names = []

    for entity in get_non_reports(directory + '\\' + benches_name):
        if is_input(entity):
            continue
        
        if entity not in filters:
            continue
        
        x = []
        y = []
        for x_d in get_non_reports( directory + '\\' + benches_name + '\\' + entity):
            if not is_input(x_d): continue

            if int(x_d) <= 10: continue
            x.append(int(x_d))
            
            with open(directory + '\\' + benches_name + '\\' + entity + '\\' + x_d + '\\base\\estimates.json') as f:
                dic = json.load(f)
                y.append(float(dic['mean']['point_estimate']) / 1000.0)
        if len(x) == 0:
            continue
        x, y = zip(*sorted(zip(x,y)))
        y = [i * 100.0 for i in y]
        avg_y.append(sum(y) / len(y))
        min_y.append(avg_y[-1] - min(y))
        max_y.append(max(y) - avg_y[-1])
        names.append(entity)

    names, avg_y = zip(*sorted(zip(names, avg_y), key=lambda kv: kv[1]))

    fig,ax = plt.subplots(1,1, figsize=(10,10))
    b = []
    for i, (name, latency) in enumerate(zip(names, avg_y)):
        b.append(
            ax.bar(
                name, latency,
                width=1.0, 
                color=filters[name], 
                align='center', 
                edgecolor = 'black', 
                linewidth = 1.0, 
                alpha=0.5)
            )
        
    add_labels(names, avg_y)
    plt.ylabel('Speed (ns)') 
    plt.title(title)
    ax.legend(b, names, ncol = 3, loc = 'best', framealpha = 0.1)
    plt.show() 

