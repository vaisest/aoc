import urllib.request
from urllib.error import HTTPError
from pathlib import Path

api_key = ""
try:
    with open("./api_key.txt", "r") as f:
        api_key = f.read()
except IOError:
    api_key = input(
        "please insert aoc cookie containing the session key (excluding the 'session=' and ';'):"
    )
    with open("./api_key.txt", "w") as f:
        f.write(api_key)


def write_day(year, day, data):
    file = Path(f"input/{year}/day_{day:02}.txt")
    if not file.exists():
        with open(file, "w") as f:
            f.write(data)
    else:
        print(f"{file} already exists")


def fetch(year, day):
    print(f"Fetching day {day}")
    req = urllib.request.Request(f"https://adventofcode.com/{year}/day/{day}/input")
    req.add_header("Cookie", f"session={api_key}")
    try:
        data = urllib.request.urlopen(req, timeout=10).read().decode()
        write_day(year, day, data)
    except HTTPError as e:
        if e.code == 404:
            print(f"Year {year} day {day} not available yet")
            return
        raise e


for day in range(1, 25 + 1):
    fetch(2024, day)

for day in range(1, 12 + 1):
    fetch(2025, day)
