import sys

start = int(sys.argv[1])
end = int(sys.argv[2])
speed = '5'#sys.argv[3]

files = []

for i in range(start, end + 1):
    with open('data/file_00000' + str(i) + '.obj', 'r') as f:
        files.append(f.readlines())
with open('data/file.mtl', 'r') as f:
    matfile = f.readlines()
points = []
faces = []
colors = []

mats = {}

matname = ''
for line in matfile:
    parts = line.split()
    if len(parts) > 1:
        if parts[0] == 'newmtl':
            matname = parts[1]
        elif parts[0] == 'Kd':
            mats[matname] = [float(parts[1]), float(parts[2]), float(parts[3]), 1]

matnames = list(mats.keys())


def calc_delta(x1, y1, z1, x2, y2, z2):
    return ((x1 - x2) ** 2 + (y1 - y2) ** 2 + (z1 - z2) ** 2) ** 0.5

max_radius = 0
max_y = 0

def add_point(x, y, z, i, curr, point_number):
    global max_radius
    global max_y
    max_radius = max(max_radius, max(abs(x), abs(z)))
    max_y = max(max_y, y)
    if i == 0:
        points[curr].append([[x, y, z]])
    else:
        points[curr][point_number].append([x, y, z])


current_mat = 0
for i in range(len(files)):
    curr = -1
    point_number = 0
    for line in files[i]:
        parts = line.split()
        if len(parts) > 1:
            if parts[0] == 'v':
                add_point(float(parts[1]), float(parts[2]), float(parts[3]), i, curr, point_number)
                point_number += 1
            elif parts[0] in 'og' and i != 0:
                curr += 1
            elif i == 0:
                if parts[0] == 'f':
                    face = [int(a[0]) - 1 for a in (point.split('/') for point in parts[1:])]
                    current_start = 1
                    while current_start < len(face) - 1:
                        faces[curr].append([face[0], face[current_start], face[current_start + 1]])
                        current_start += 1
                elif parts[0] == 'usemtl':
                    global current_mat
                    colors[curr] = parts[1].replace('_', '.')
                elif parts[0] in 'og':
                    curr += 1
                    faces.append([])
                    colors.append([])
                    points.append([])

if colors == [[]]:
    colors = ['Mat']

with open('data/out.3d', 'w') as f:
    f.write(str(max_radius) + '\n')
    f.write(str(max_y) + '\n')
    for i in range(len(faces)):
        data = 'g'
        data += ' ' + str(mats[colors[i]][0])
        data += ' ' + str(mats[colors[i]][1])
        data += ' ' + str(mats[colors[i]][2])
        data += ' 1'
        f.write(data + '\n')
        for point_row in points[i]:
            data = 'v'
            for point in point_row:
                data += ' ' + str(point[0])
                data += ' ' + str(point[1])
                data += ' ' + str(point[2])
            f.write(data + '\n')
        for face in faces[i]:
            data = 'f'
            for point_id in face:
                data += ' ' + str(point_id)
            f.write(data + '\n')
