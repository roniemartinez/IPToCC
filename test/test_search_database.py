import unittest
import iptocc
import ipaddress

class TestSearchDatabase(unittest.TestCase):
    def test_get_country_code(self):
        # Test valid US ipv4
        ipv4String = "5.35.192.0"
        country_code = iptocc.get_country_code(ipv4String)
        self.assertEqual(country_code, "US")

        # Test valid SE ipv4
        ipv4String = "5.35.184.0"
        country_code = iptocc.get_country_code(ipv4String)
        self.assertEqual(country_code, "SE")

        # Test valid US ipv6
        ipv6String = "2a00:5440:0000:0000:0000:ff00:0042:8329"
        country_code = iptocc.get_country_code(ipv6String)
        self.assertEqual(country_code, "US")

        # Test valid GB ipv6
        ipv6String = "2a00:95e0:0000:0000:0000:ff00:0042:8329"
        country_code = iptocc.get_country_code(ipv6String)
        self.assertEqual(country_code, "GB")

        # Testing an invalid IP
        with self.assertRaises(ValueError):
            invalidString = "123456"
            country_code = iptocc.get_country_code(invalidString)

    def test_convert_ip_string(self):
        # Test ipv6
        ipv6String = "2001:0db8:0000:0000:0000:ff00:0042:8329"
        ipv6Object = iptocc.convert_ip_string(ipv6String)
        self.assertEqual(isinstance(ipv6Object, ipaddress.IPv6Address), True)

        # Test ipv4
        ipv4String = "127.0.0.1"
        ipv4Object = iptocc.convert_ip_string(ipv4String)
        self.assertEqual(isinstance(ipv4Object, ipaddress.IPv4Address), True)

        # Testing an invalid ip parameter
        with self.assertRaises(ValueError):
            invalidParam = 123456
            invalidObject = iptocc.convert_ip_string(invalidParam)
        

if __name__ == '__main__':
    unittest.main()
