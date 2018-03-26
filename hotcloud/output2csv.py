#!/usr/bin/env python

import json
import csv

with open("output.json") as input, open("output.csv", "w") as output:
    writer = csv.writer(output, dialect='excel')
    writer.writerow(['node', 'metric',  'query', 'hour', 'value', 'disruption'])
    for line in input:
        obj = json.loads(line)
        writer.writerow(obj.values())
