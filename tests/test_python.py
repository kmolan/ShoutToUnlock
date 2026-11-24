import math
import unittest

from shout_to_unlock import ShoutLock, level_dbfs

SAMPLE_RATE = 48_000
LOUD = [0.5] * 4800  # 100 ms of screaming, -6 dBFS
QUIET = [0.01] * 4800  # 100 ms of polite mumbling, -40 dBFS


class ShoutLockTest(unittest.TestCase):
    def test_closed_by_default(self):
        lock = ShoutLock(-20.0, 1.0)
        self.assertFalse(lock.is_unlocked)
        self.assertFalse(lock)
        self.assertEqual(lock.last_level_dbfs, -math.inf)

    def test_loud_time_does_not_need_to_be_continuous(self):
        lock = ShoutLock(-20.0, 0.3)
        for _ in range(2):
            self.assertFalse(lock.process(LOUD, SAMPLE_RATE))
            self.assertFalse(lock.process(QUIET, SAMPLE_RATE))
        self.assertAlmostEqual(lock.loud_seconds, 0.2)
        self.assertAlmostEqual(lock.remaining_seconds, 0.1)
        self.assertAlmostEqual(lock.total_seconds, 0.4)
        self.assertAlmostEqual(lock.progress, 2 / 3)
        self.assertTrue(lock.process(LOUD, SAMPLE_RATE))
        self.assertTrue(lock.process(QUIET, SAMPLE_RATE))
        self.assertAlmostEqual(lock.peak_level_dbfs, -6.0206, places=3)
        self.assertAlmostEqual(lock.last_level_dbfs, -40.0, places=3)

    def test_process_level_and_reset(self):
        lock = ShoutLock(-20.0, 1.0)
        self.assertTrue(lock.process_level(-10.0, 1.0))
        with self.assertRaises(ValueError):
            lock.process_level(-10.0, -1.0)
        lock.reset()
        self.assertFalse(lock)
        self.assertEqual(lock.minimum_level_dbfs, -20.0)
        self.assertEqual(lock.minimum_seconds, 1.0)

    def test_invalid_minimum_duration(self):
        with self.assertRaises(ValueError):
            ShoutLock(-20.0, -1.0)

    def test_level_dbfs(self):
        self.assertEqual(level_dbfs([0.0] * 8), -math.inf)
        self.assertAlmostEqual(level_dbfs([0.1] * 8), -20.0, places=3)


if __name__ == "__main__":
    unittest.main()
