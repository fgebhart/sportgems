import folium
from pathlib import Path
from sportgems import parse_fit_data, find_fastest_section

# desired fastest sections to parse, note larges must come first in
# order to be able to render the smaller sections on top of the larger ones
sections = [5000, 3000, 2000, 1000]
colors = ["yellow", "blue", "green", "red"]

if __name__ == "__main__":
    fit_file = Path(".").parent / "tests" / "data" / "2019-09-14-17-22-05.fit"
    fit_data = parse_fit_data(str(fit_file))
    coords = []
    for coordinate in fit_data.coordinates:
        if coordinate[0] > 0 and coordinate[1] > 0:
            coords.append((coordinate[0], coordinate[1]))

    trace = folium.PolyLine(coords, color="black")
    map = folium.Map(location=fit_data.coordinates[300], zoom_start=15)
    trace.add_to(map)

    for i in range(len(sections)):
        fs = find_fastest_section(sections[i], fit_data.times, fit_data.coordinates)
        fs_coords = coords[fs.start:fs.end]
        fs_poly = folium.PolyLine(fs_coords, color=colors[i])
        fs_poly.add_to(map)

    output_file = "map.html"
    map.save(output_file)
    print(f"saved map to {output_file}, can be viewed in browser")