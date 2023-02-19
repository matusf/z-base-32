from unittest import main, TestCase

import zbase32


class Test(TestCase):
    def test_encode_decode(self):
        self.assertEqual(b'foo', zbase32.decode((zbase32.encode(b'foo'))))
        self.assertEqual('c3zs6', zbase32.encode(b'foo'))
        self.assertEqual(b'foo', zbase32.decode('c3zs6'))

    def test_exception(self):
        with self.assertRaises(zbase32.DecodeError):
            zbase32.decode('invalid@char')


if __name__ == '__main__':
    main()
