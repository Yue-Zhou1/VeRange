package utils;

import java.math.BigInteger;
import java.util.StringTokenizer;
import java.util.Vector;

public class SquareDecompUtils {
    private final static BigInteger ZERO		= BigInteger.valueOf(0L);
    private final static BigInteger ONE			= BigInteger.valueOf(1L);
    private final static BigInteger TWO			= BigInteger.valueOf(2L);
    private final static BigInteger MAXINT		= BigInteger.valueOf(Integer.MAX_VALUE);
    private final static BigInteger ITERBETTER	= ONE.shiftLeft(1024);

    private final static int primeCertainty = 10;

    private final static BigInteger[] specialCasesArray = new BigInteger[]{
            BigInteger.valueOf(9634L),	BigInteger.valueOf(2986L),	BigInteger.valueOf(1906L),
            BigInteger.valueOf(1414L),	BigInteger.valueOf(730L),	BigInteger.valueOf(706L),
            BigInteger.valueOf(526L),	BigInteger.valueOf(370L),	BigInteger.valueOf(226L),
            BigInteger.valueOf(214L),	BigInteger.valueOf(130L),	BigInteger.valueOf(85L),
            BigInteger.valueOf(58L),	BigInteger.valueOf(34L),	BigInteger.valueOf(10L),
            BigInteger.valueOf(3L),		TWO
    };
    private final static int[][] specialCasesDecomposition = new int[][]{
            {56,	57, 57},	{21,	32, 39},	{13,	21, 36},
            { 6,	17, 33},	{ 0,	1,	27},	{15,	15, 16},
            { 6,	7,	21},	{ 8,	9,	15},	{ 8,	9,	9},
            { 3,	6,	13},	{ 0,	3,	11},	{ 0,	6,	7},
            { 0,	3,	7},		{ 3,	3,	4},		{ 0,	1,	3},
            { 1,	1,	1},		{ 0,	1,	1}
    };

    private final static java.util.Hashtable specialCases = new java.util.Hashtable(50);
    static {
        for (int i = 0; i < specialCasesArray.length; i++)
            specialCases.put(specialCasesArray[i], specialCasesDecomposition[i]);
    }

    private final static long magicN = 10080;
    private final static BigInteger bigMagicN = BigInteger.valueOf(magicN);

    private final static java.util.Hashtable squaresModMagicN = new java.util.Hashtable(500);
    static {
        for (long i = 0; i <= (magicN >> 1); i++)
            squaresModMagicN.put(BigInteger.valueOf((i * i) % magicN), ONE);
    }

    private final static boolean isProbableSquare(BigInteger n) {
        return squaresModMagicN.get(n.remainder(bigMagicN)) != null;
    }

    private final static int jacobi(long b, BigInteger p) {
        int s = 1;
        long a = p.mod(BigInteger.valueOf(b)).longValue();
        while (a > 1) {
            if ((a & 3) == 0) a >>= 2;
            else if ((a & 1) == 0) {
                if (((b & 7) == 3) || ((b & 7) == 5)) s = -s;
                a >>= 1;
            }
            else {
                if (((a & 2) == 2) && ((b & 3) == 3)) s = -s;
                long t = b % a; b = a; a = t;
            }
        }
        // Return -1 also in case gcd(a, b) > 1 to ensure termination of /***/ below
        return a == 0 ? -1 : s;
    }

    private final static long[] primes = {
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
            61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
            127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
            191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
            257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
            331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
            401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
            467, 479, 487, 491, 499
    };

    private final static BigInteger lastPrecomputedPrime =
            BigInteger.valueOf(primes[primes.length - 1]);
    private static BigInteger cachePrime = lastPrecomputedPrime;
    private static int cacheN = primes.length; // Invariant: cacheN >= primes.length

    private final static BigInteger nthPrime(int n) {
        BigInteger result;
        if (n < 1) return TWO;
        if (n <= primes.length) return BigInteger.valueOf(primes[n - 1]);
        if (n < cacheN) {
            result = lastPrecomputedPrime;
            for (int i = primes.length; i < n; i++)
                result = nextProbablePrime(result.add(TWO), primeCertainty);
        }
        else {
            result = cachePrime;
            for (int i = cacheN; i < n; i++)
                result = nextProbablePrime(result.add(TWO), primeCertainty);
            cacheN = n;
            cachePrime = result;
        }
        return result;
    }

    private final static BigInteger[] iunit(BigInteger p) {
        BigInteger r = null;
        if (p.testBit(0) && !p.testBit(1) && p.testBit(2)) r = TWO; // p = 5 mod 8
        else {
            int k = 2;
            long q = 3;
    		while (jacobi(q, p) == 1) {
                if (k < primes.length) {
                    q = primes[k++];
                    if ((q == 229) && isProbableSquare(p)) {
                        // reached when decomposing d(1, 4*pp(k), 1) for k > 47
                        BigInteger[] sr = isqrt(p);
                        if (sr[1].signum() == 0)
                            return new BigInteger[]{sr[0], ZERO};
                    }
                }
                else {
                    if (r == null) r = BigInteger.valueOf(q);
                    r = nextProbablePrime(r.add(TWO), 2);
                    q = r.longValue();
                }
            }
            if (r == null) r = BigInteger.valueOf(q);
        }
        return new BigInteger[]{r.modPow(p.shiftRight(2), p), ONE};
    }

    private final static BigInteger[] isqrtInternal(BigInteger n, int log2n) {
        if (n.compareTo(MAXINT) < 1) {
            int ln = n.intValue(), s = (int)java.lang.Math.sqrt(ln);
            return new BigInteger[]{BigInteger.valueOf(s), BigInteger.valueOf(ln - s * s)};
        }
        if (n.compareTo(ITERBETTER) < 1) {
            int d = 7 * (log2n / 14 - 1), q = 7;
            BigInteger s = BigInteger.valueOf((long)java.lang.Math.sqrt(n.shiftRight(d << 1).intValue()));
            while (d > 0) {
                if (q > d) q = d;
                s = s.shiftLeft(q);
                d -= q;
                q <<= 1;
                s = s.add(n.shiftRight(d << 1).divide(s)).shiftRight(1);
            }
            return new BigInteger[]{s, n.subtract(s.multiply(s))};
        }
        int log2b = log2n >> 2;
        BigInteger mask = ONE.shiftLeft(log2b).subtract(ONE);
        BigInteger[] sr = isqrtInternal(n.shiftRight(log2b << 1), log2n - (log2b << 1));
        BigInteger s = sr[0];
        BigInteger[] qu = sr[1].shiftLeft(log2b).add(n.shiftRight(log2b).and(mask)).divideAndRemainder(s.shiftLeft(1));
        BigInteger q = qu[0];
        return new BigInteger[]{s.shiftLeft(log2b).add(q), qu[1].shiftLeft(log2b).add(n.and(mask)).subtract(q.multiply(q))};
    }

    private final static BigInteger[] isqrt(BigInteger n) {
        if (n.compareTo(MAXINT) < 1) {
            long ln = n.longValue();
            long s = (long)java.lang.Math.sqrt(ln);
            return new BigInteger[]{BigInteger.valueOf(s), BigInteger.valueOf(ln - s * s)};
        }
        BigInteger[] sr = isqrtInternal(n, n.bitLength() - 1);
        if (sr[1].signum() < 0) {
            return new BigInteger[]{sr[0].subtract(ONE), sr[1].add(sr[0].shiftLeft(1)).subtract(ONE)};
        }
        return sr;
    }
    private final static BigInteger primeProduct97 =
            new BigInteger("1152783981972759212376551073665878035");
    private final static BigInteger b341 = BigInteger.valueOf(341L);

    private final static boolean isProbablePrime(BigInteger n, int certainty) {
        return ((n.compareTo(b341) < 0) || primeProduct97.gcd(n).equals(ONE)) &&
                TWO.modPow(n.subtract(ONE), n).equals(ONE) &&
                n.isProbablePrime(certainty);
    }


    private final static BigInteger nextProbablePrime(BigInteger n, int certainty) {
        while (!isProbablePrime(n, certainty)) n = n.add(TWO);
        return n;
    }

    private final static BigInteger[] decomposePrime(BigInteger p) {
        BigInteger a = p, b, t, x0 = ZERO, x1 = ONE;
        BigInteger[] sr = iunit(p);
        b = sr[0];
        if (ZERO.equals(sr[1]))
            return new BigInteger[]{ZERO, b, ONE};
        if (b.multiply(b).add(ONE).mod(p).signum() != 0)
            // Failure to compute imaginary unit, p was not a prime
            return new BigInteger[]{ZERO, ZERO, ZERO};
        while (b.multiply(b).compareTo(p) > 0) {
            t = a.remainder(b);
            a = b;
            b = t;
        }
        return new BigInteger[]{a.remainder(b), b, ONE};
    }

    public final static BigInteger[] decompose(BigInteger n) {
        // Check for 0 and 1
        if (n.compareTo(ONE) < 1) return new BigInteger[]{ZERO, ZERO, ZERO, n, ONE};
        BigInteger sq, x, p, delta, v;
        BigInteger[] z, sqp;
        int k = n.getLowestSetBit() >> 1; // n = 4^k*m and (m != 0 mod 4)
        if (k > 0) {
            v = ONE.shiftLeft(k);
            n = n.shiftRight(k << 1);
        }
        else v = ONE;
        // The following two checks are not strictly necessary but the result looks nicer
        // Case 1: Check for perfect square, in this case one square is sufficient
        sqp = isqrt(n);
        sq = sqp[0];
//        if (sqp[1].signum() == 0) // n is a perfect square
//            return new BigInteger[]{ZERO, ZERO, ZERO, v.multiply(sq), ONE};
        // Case 2: Check for prime = 1 mod 4, in this case two squares are sufficient
        if (n.testBit(0) && !n.testBit(1) && isProbablePrime(n, primeCertainty)) {
            z = decomposePrime(n);
//            if (ONE.equals(z[2]))
//                return new BigInteger[]{ZERO, ZERO, v.multiply(z[0]), v.multiply(z[1]), ONE};
            delta = ZERO;
        }
//        else if (n.testBit(0) && n.testBit(1) && n.testBit(2)) {
//			/*	n = 7 mod 8, need four squares
//					Subtract largest square sq1^2 such that n > sq1^2 and sq1^2 != 0 mod 8
//			*/
//            if (sq.testBit(0) || sq.testBit(1)) {
//                delta = v.multiply(sq);
//                n = sqp[1];
//            }
//            else {
//                delta = v.multiply(sq.subtract(ONE));
//                n = sqp[1].add(sq.shiftLeft(1).subtract(ONE));
//            }
//            sqp = isqrt(n); // Recompute sq, n cannot be a perfect square (n(old) = 7 mod 8)
//            sq = sqp[0];
//        }
        else delta = ZERO;
		/*	Postcondition: (sq = isqrt(n)) && (n != 7 mod 8) && (n != 0 mod 4)
			This implies that n is a sum of three squares - now check whether n
			is one of the special cases the rest of the algorithm could not handle.
		*/
        int[] special = (int[])specialCases.get(n); // look up in hash table
        if (special != null)
            return new BigInteger[]{delta,
                    v.multiply(BigInteger.valueOf(special[0])),
                    v.multiply(BigInteger.valueOf(special[1])),
                    v.multiply(BigInteger.valueOf(special[2])), ONE};
		/*	Case n = 3 mod 4 (actually n = 3 mod 8)
			Attempt to represent n = x^2 + 2*p with p = 1 mod 4 and p is prime
			Then we can write p = y^2 + z^2 and get n = x^2 + (y+z)^2 + (y-z)^2
		*/
        if (n.testBit(0) && n.testBit(1)) {
            if	(sq.testBit(0)) {
                x = sq;
                p = sqp[1].shiftRight(1);
            }
            else {
                x = sq.subtract(ONE);
                p = sqp[1].add(sq.shiftLeft(1).subtract(ONE)).shiftRight(1);
            }
            while (true) {
                if (isProbablePrime(p, 2)) {
                    z = decomposePrime(p);
                    if (ONE.equals(z[2])) {
                        return new BigInteger[]{delta, v.multiply(x), v.multiply(z[0].add(z[1])),
                                v.multiply(z[0].subtract(z[1])).abs(), ONE};
                    }
                }
                x = x.subtract(TWO);
                // No case for the following to return is known
                if (x.signum() < 0) return new BigInteger[]{ZERO, ZERO, ZERO, ZERO, ZERO};
                p = p.add(x.add(ONE).shiftLeft(1)); // Proceed to next prime candidate
            }
        }
		/*	Case n = 1 mod 4 or n = 2 mod 4
			Attempt to represent n = x^2 + p with p = 1 mod 4 and p is prime
			Then we can write p = y^2 + z^2 and get n = x^2 + y^2 + z^2
		*/
        if (n.subtract(sq).testBit(0)) {
            x = sq;
            p = sqp[1];
        }
        else {
            x = sq.subtract(ONE);
            p = sqp[1].add(sq.shiftLeft(1).subtract(ONE));
        }
        while (true) {
            if (isProbablePrime(p, 2)) {
                z = decomposePrime(p);
                if (ONE.equals(z[2])) {
                    return new BigInteger[]{delta, v.multiply(x), v.multiply(z[0]), v.multiply(z[1]), ONE};
                }
            }
            x = x.subtract(TWO);
            // No case for the following to return is known
            if (x.signum() < 0) return new BigInteger[]{ZERO, ZERO, ZERO, ZERO, ZERO};
            p = p.add(x.add(ONE).shiftLeft(2)); // Proceed to next prime candidate
        }
    }

    public static void main(String[] args) {
        BigInteger test1 = BigInteger.valueOf(753113122);
        BigInteger[] result = decompose(test1);
        System.out.println(test1.toString() + " " + result[1] + " " + result[2] +
                " " + result[3]);
    }
}
