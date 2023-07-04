from imdb import Cinemagoer
import sys

ia = Cinemagoer()

media = ia.get_movie(sys.argv[1])
if media["kind"] == "tv series":
    ia.update(media, "episodes")
    print([
        [
            (j, m["title"], round(m["rating"], 2))
            for j, m in media["episodes"][i].items()
        ]
        for i in media["episodes"].keys()
    ])
