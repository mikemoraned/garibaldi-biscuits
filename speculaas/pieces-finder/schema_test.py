import unittest
from collections import OrderedDict

from graphene.test import Client

from dummy_splitter import DummySplitter
from schema import schema

unittest.util._MAX_LENGTH = 2000


class TestSchema(unittest.TestCase):
    def __init__(self, method_name):
        super().__init__(method_name)
        self.maxDiff = None
        self.dummy_pieces = {
            'edinburgh': [
                {
                    "x": 10,
                    "y": 20,
                    "height": 882,
                    "width": 1350
                }
            ]
        }

    def test_piecesByPlaceId_min_fields_for_unknown_id(self):
        client = Client(schema)
        executed = client.execute('''
                                  { 
                                    piecesByPlaceId(id: "glasgow") { 
                                      id
                                    } 
                                  }''',
                                  root_value=DummySplitter(self.dummy_pieces))
        expected = {
            'data': OrderedDict([
                ('piecesByPlaceId', None)
            ])
        }
        self.assertEqual(executed, expected)


    def test_piecesByPlaceId_all_fields_for_known_id(self):
        client = Client(schema)
        executed = client.execute('''
                                  { 
                                    piecesByPlaceId(id: "edinburgh") { 
                                      id
                                      bitmapImage {
                                        data
                                        x
                                        y
                                        width
                                        height
                                      }
                                    } 
                                  }''',
                                  root_value=DummySplitter(self.dummy_pieces))
        expected = {
            'data': OrderedDict([
                ('piecesByPlaceId', [
                    OrderedDict([
                        ('id', '0'),
                        ('bitmapImage', OrderedDict([
                            (
                            'data', "data:image/png;base64,iVBORw0KGgoAAAANSUh"
                                    "EUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk"
                                    "YPhfDwAChwGA60e6kgAAAABJRU5ErkJggg=="),
                            ('x', 10),
                            ('y', 20),
                            ('width', 1350),
                            ('height', 882)]
                        ))
                    ])
                ])
            ])
        }
        self.assertEqual(executed, expected)


if __name__ == '__main__':
    unittest.main()
