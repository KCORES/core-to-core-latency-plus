import requests
import json
from tabulate import tabulate

# Fetch data from API
url = "http://localhost:9009/leaderboard"
response = requests.get(url)
data = response.json()

# Sort by median_latency in ascending order
sorted_data = sorted(data, key=lambda x: x["median_latency"])

# Prepare table data
table_data = [
    [
        item["cpu_name"],
        round(item["min_latency"], 4),
        round(item["median_latency"], 4),
        round(item["max_latency"], 4),
    ]
    for item in sorted_data
]

# Generate markdown table
headers = ["CPU Name", "Min Latency", "Median Latency", "Max Latency"]
markdown_table = tabulate(table_data, headers=headers, tablefmt="pipe", floatfmt=".4f")

# Print markdown table
print(markdown_table)
