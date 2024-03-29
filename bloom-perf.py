import json
import os
import matplotlib.pyplot as plt 

colors = {
    'probabilistic-collections': 'violet',
    'bloomfilter': 'purple',
    'bloom': 'orange',
    'fastbloom - 512': 'b',
    'fastbloom - 256': 'purple',
    'fastbloom - 128': 'r',
    'fastbloom - 64': 'orange',
    'fastbloom': 'b',
    'fastbloom-rs': 'g',
    'fastbloom - 64 - xxhash': 'darkblue',
    'fastbloom - 512 - xxhash': 'b',
    'sbbf-rs-safe - xxhash': 'red',

    'fastbloom - 512 - ahash': 'b',
    'fastbloom - 256 - ahash': 'mediumturquoise',
    'fastbloom - 128 - ahash': 'green',
    'fastbloom - 64 - ahash': 'purple',
    }
directory = r"target\criterion"

def is_input(x):
    try:
        int(x)
        return True
    except:
        return False

def get_immediate_subdirectories(a_dir):
    return [name for name in os.listdir(a_dir) if os.path.isdir(os.path.join(a_dir, name))]

def get_non_reports(d):
     return [x for x in get_immediate_subdirectories(d) if x != 'report']

for benches_name, title in zip(get_non_reports(directory), [
    'Member Check Speed vs Items in Bloom Filter (262Kb Allocated)',
    'Non-Member Check Speed vs Items in Bloom Filter (262Kb Allocated)']):
    
    for entity in get_non_reports( directory + '\\' + benches_name):
        if is_input(entity):
            continue
        
        x = []
        y = []
        for x_d in get_non_reports( directory + '\\' + benches_name + '\\' + entity):
            if not is_input(x_d): continue
            x.append(int(x_d))
            with open(directory + '\\' + benches_name + '\\' + entity + '\\' + x_d + '\\base\\estimates.json') as f:
                dic = json.load(f)
                y.append(float(dic['mean']['point_estimate']) / 1000.0)
        if len(x) == 0:
            continue
        x, y = zip(*sorted(zip(x,y)))
        # if entity == 'fastbloom - 512': entity = 'fastbloom'
        if entity == 'block_size_512': entity = 'fastbloom - 512 - xxhash'
        plt.plot(x, y, color=colors[entity], label=entity, linewidth=1.0)

                         
    plt.xlabel('Number of Items in Bloom filter') 
    plt.ylabel('Speed (ns)') 
    plt.title(title)
    plt.grid()
    plt.ylim(0, None)
    plt.xlim(0, None)
    plt.legend()
    plt.show() 

