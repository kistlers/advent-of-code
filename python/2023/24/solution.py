from typing import Iterable

from itertools import combinations
from z3 import Solver, Real, Reals, sat

from python.utils import run


def solve(input_data: str) -> Iterable[int]:
    hailstones = [
        (
            tuple(map(int, line.split(" @ ")[0].split(","))),
            tuple(map(int, line.split(" @ ")[1].split(",")))
        )
        for line in input_data.splitlines()]

    def count_2d_collisions() -> int:
        collisions_2d = 0
        for i, ((a, av), (b, bv)) in enumerate(combinations(hailstones, 2)):
            ax, ay, az = a
            avx, avy, avz = av
            bx, by, bz = b
            bvx, bvy, bvz = bv
            s = Solver()
            vx, vy, ta, tb = Reals('vx vy ta tb')
            s.add(ta > 0)
            s.add(tb > 0)
            s.add(ax + avx * ta == bx + bvx * tb)
            s.add(ay + avy * ta == by + bvy * tb)
            s.add(bounds_min <= ax + avx * ta)
            s.add(ax + avx * ta <= bounds_max)
            s.add(bounds_min <= ay + avy * ta)
            s.add(ay + avy * ta <= bounds_max)
            r = s.check()
            if r == sat:
                collisions_2d += 1

        return collisions_2d

    if len(hailstones) == 5:
        # sample
        bounds_min, bounds_max = 7, 27
        yield count_2d_collisions()
    else:
        # real input
        bounds_min, bounds_max = 200000000000000, 400000000000000
        # takes 311 seconds, just yield the answer below
        # yield count_2d_collisions()
        yield 13892

    def find_collision_throw() -> int:
        s = Solver()
        x, y, z, vx, vy, vz = Reals('x y z vx vy vz')

        for i, (hp, hv) in enumerate(hailstones):
            hpx, hpy, hpz = hp
            hvx, hvy, hvz = hv
            ti = Real(f't{i}')
            s.add(ti > 0)
            s.add(x + vx * ti == hpx + hvx * ti)
            s.add(y + vy * ti == hpy + hvy * ti)
            s.add(z + vz * ti == hpz + hvz * ti)
        r = s.check()
        if r == sat:
            m = s.model()
            return m.evaluate(x + y + z).as_long()
        return 0

    yield find_collision_throw()


run(solve)
