from imdb import Cinemagoer
import sys
import json

ia = Cinemagoer()

movies = ia.search_movie(sys.argv[1], results=5)
print(json.dumps([{"title": i.data["title"], "imdb_id": i.movieID} for i in movies]))

