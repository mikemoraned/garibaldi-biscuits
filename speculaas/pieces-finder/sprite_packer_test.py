import tempfile
import unittest

from image_signature import signature
from precomputed_lookup_splitter import PreComputedLookupSplitter
from sprite_packer import SpritePacker

unittest.util._MAX_LENGTH = 2000


class TestSpritePacker(unittest.TestCase):
    def __init__(self, method_name):
        super().__init__(method_name)
        self.maxDiff = None

    def test_sprite_bytes_are_identical(self):
        input_dir_name = 'precomputed_test'
        place_id = 'edinburgh_real'
        with tempfile.TemporaryDirectory() as packed_dir_name:
            packer = SpritePacker(input_dir_name, packed_dir_name,
                                  input_dir_has_background=True)
            packer.pack(place_id)
            input_content = self.sprite_content(place_id, input_dir_name,
                                                has_background=True)
            packed_content = self.sprite_content(place_id, packed_dir_name,
                                                 has_background=False)
            self.assertListEqual(input_content, packed_content)

    def test_sprite_content_equal(self):
        input_dir_name = 'precomputed_test'
        edinburgh = self.sprite_content('edinburgh_real', input_dir_name)
        edinburgh_same = self.sprite_content('edinburgh_real', input_dir_name)
        self.assertEqual(edinburgh, edinburgh_same)
        budapest = self.sprite_content('budapest', input_dir_name)
        self.assertNotEqual(edinburgh, budapest)

    @staticmethod
    def sprite_content(place_id, dir_name, has_background):
        splitter = \
            PreComputedLookupSplitter.from_dir(dir_name,
                                               has_background=has_background)
        place = splitter.split(place_id)

        def summary(id, bitmap_image):
            image = place.sprite.extract(bitmap_image)
            return (id,
                    bitmap_image.x, bitmap_image.y,
                    bitmap_image.width, bitmap_image.height,
                    signature(image))

        return list(sorted([summary(p.id, p.bitmap_image)
                            for p in place.pieces],
                           key=lambda s: s[0]))


if __name__ == '__main__':
    unittest.main()
