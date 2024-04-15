MEASURES = [
    ('Name',                'Crate',        'Size',         'Cycles_min',       'Cycles_mx'),
    ('base',                '-',            8128,           1,                  1),
    ('u32',                 'tfmt',         8536,           35,                 278),
    ('u32',                 'fmt',          8712,           167,                429),
    ('u32 padded',          'tfmt',         8624,           285,                407),
    ('u32 padded',          'fmt',          9068,           771,                1020),
    ('u32-hex',             'tfmt',         8256,           126,                238),
    ('u32-hex',             'fmt',          9076,           423,                564),
    ('u8 u16 u32',          'tfmt',         8836,           119,                513),
    ('u8 u16 u32',          'fmt',          9068,           771,                1020),
    ('f32',                 'tfmt',         8848,           190,                197),
    ('f32',                 'fmt',          31548,          1050,               4800),    
]

names = []
sizes = {'tfmt': [], 'fmt': []}
min_cycles = {'tfmt': [], 'fmt': []}
max_cycles = {'tfmt': [], 'fmt': []}


base_size = MEASURES[1][2]
base_cycles = MEASURES[1][3]

FORMAT_STR = "| {:<20} | {:>5} | {:>12} | {:>12} | {:>12} |"
FORMAT_STR2 = "|{:<22}|{:>7}|{:>14}|{:>14}|{:>14}|"

print(FORMAT_STR.format('Name', 'Crate','Size', 'Cycles_min', 'Cycles_max'))
print(FORMAT_STR2.format('-'*22, '-'*7, '-'*14, '-'*14, '-'*14))

for line in MEASURES[2:]:
    name = line[0]
    crate = line[1]
    size = line[2] - base_size
    cycles_min = line[3] - base_cycles
    cycles_max = line[4] - base_cycles
    print(FORMAT_STR.format(name, crate, size, cycles_min, cycles_max))

    if name not in names:
        names.append(name)
    sizes[crate].append(size)
    min_cycles[crate].append(cycles_min)
    max_cycles[crate].append(cycles_max-cycles_min)

import matplotlib.pyplot as plt
import numpy as np

x = np.arange(len(names))  # the label locations
width = 0.34  # the width of the bars
multiplier = 0
colors={'tfmt': 'tab:green', 'fmt': 'tab:orange'}

fig, ax = plt.subplots(1, 2, figsize=(12.8, 4.2), dpi=100)

for attribute, min_cs in sizes.items():
    offset = width * multiplier
    color = colors[attribute]
    rects = ax[0].bar(x + offset, min_cs, width, label=attribute, color=color)
    ax[0].bar_label(rects, padding=3)
    multiplier += 1

# Add some text for labels, title and custom x-axis tick labels, etc.
ax[0].set_title("Flash Size for 'tfmt' and 'core::fmt'")
ax[0].title.set_size(15)
ax[0].set_ylabel('Size (bytes)')
ax[0].set_xticks(x + width - 0.17, names)
ax[0].legend(loc='upper left', ncols=len(names))
ax[0].set_ylim(0, 25_000)

multiplier = 0

for attribute, min_cs in min_cycles.items():
    max_cs = max_cycles[attribute]
    offset = width * multiplier
    color = colors[attribute]

    label = attribute + '-min'
    rects = ax[1].bar(x + offset, min_cs, width, label=label, color=color)

    label = attribute + '-max'
    rects = ax[1].bar(x + offset, max_cs, width, bottom=min_cs, label=label, color=color, alpha=0.5)
    ax[1].bar_label(rects, padding=3)
    multiplier += 1

# Add some text for labels, title and custom x-axis tick labels, etc.
ax[1].set_title("Cortex M4 Cycles for 'tfmt' and 'core::fmt'")
ax[1].title.set_size(15)
ax[1].set_ylabel('Cycles')
ax[1].set_xticks(x + width -0.17, names)
ax[1].legend(loc='upper left', ncols=2)
ax[1].set_ylim(0, 6_000)

# plt.show()
plt.savefig("performance.png")
