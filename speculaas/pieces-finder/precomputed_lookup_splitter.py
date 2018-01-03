import glob
import json
import re

from schema import Piece, BitmapImage, Place, SpriteOffset, Sprite


class PreComputedLookupSplitter:
    def __init__(self, dir_name, place_ids):
        self.dir_name = dir_name
        self.place_ids = place_ids

    @classmethod
    def from_dir(cls, dir_name):
        return PreComputedLookupSplitter(dir_name,
                                         list(cls.place_ids_in_dir(dir_name)))

    def split(self, place_id):
        if place_id in self.place_ids:
            pieces = self.filter_out_background(
                self.load_pieces_from_file(place_id))
            sprite = self.load_sprite_from_file(place_id)
            return Place(id=place_id,
                         sprite=sprite,
                         pieces=list([Piece(id="{}_{}".format(place_id, index),
                                            bitmap_image=BitmapImage(
                                                x=entry['x'],
                                                y=entry['y'],
                                                width=entry['width'],
                                                height=entry['height'],
                                                sprite_offset=SpriteOffset(
                                                    x=entry['sprite_offset'],
                                                    y=0
                                                ))
                                            ) for index, entry in
                                      enumerate(pieces)]))
        else:
            None

    def load_pieces_from_file(self, place_id):
        with open("{}/{}.labels.json".format(self.dir_name, place_id)) as file:
            return json.load(file)

    def load_sprite_from_file(self, place_id):
        with open("{}/{}.label_sprites.png".format(self.dir_name, place_id),
                  "rb") as file:
            return Sprite.from_byte_stream(file.read())

    @classmethod
    def place_ids_in_dir(cls, dir_name):
        for file in glob.glob("./{}/*.labels.json".format(dir_name)):
            match = re.search("./{}/(.+).labels.json".format(dir_name), file)
            if match:
                yield match.group(1)

    def filter_out_background(self, pieces):
        def is_background(piece):
            return piece['x'] == 0 and piece['sprite_offset'] == 0
        return list(filter(lambda p: not is_background(p), pieces))
