MEASURES = [
    ('Name',                'Size',         'Cycles_min',       'Cycles_mx'),
    ('base',                8128,           1,                  1),
    ('u32 (tfmt)',          8536,           35,                 278),
    ('u32 (fmt)',           8712,           167,                429),
    ('u32 (tfmt) padded',   8624,           285,                407),
    ('u32 (fmt) padded',    8740,           977,                886),
    ('u32-hex (tfmt)',      8256,           126,                238),
    ('u32-hex (fmt)',       9076,           423,                564),
    ('u8 u16 u32 (tfmt)',   8836,           119,                513),
    ('u8 u16 u32 (fmt)',    9068,           771,                1020),
    ('f32 (tfmt)',          8848,           190,                197),
    ('f32 (fmt)',           31548,          1050,               4800),    
]

base_size = MEASURES[1][1]
base_cycles = MEASURES[1][2]

FORMAT_STR = "| {:<20} | {:>12} | {:>12} | {:>12} |"
FORMAT_STR2 = "|{:<22}|{:>14}|{:>14}|{:>14}|"

print(FORMAT_STR.format('Name', 'Size', 'Cycles_min', 'Cycles_max'))
print(FORMAT_STR2.format('-'*22, '-'*14, '-'*14, '-'*14))

for line in MEASURES[2:]:
    size = line[1] - base_size
    cycles_min = line[2] - base_cycles
    cycles_max = line[3] - base_cycles
    print(FORMAT_STR.format(line[0], size, cycles_min, cycles_max))

