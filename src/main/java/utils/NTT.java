package utils;

import java.math.BigInteger;
import java.util.*;

import static java.math.BigInteger.ONE;

public class NTT {
    private static BigInteger mod = new BigInteger("21888242871839275222246405745257275088548364400416034343698204186575808495617");
    private static BigInteger root = BigInteger.valueOf(5);

    // a.length == b.length == 2^x
    public static void ntt(List<BigInteger> a, boolean invert, BigInteger mod, BigInteger root) {
        int n = a.size();
        int shift = 32 - Integer.numberOfTrailingZeros(n);
        for (int i = 1; i < n; i++) {
            int j = Integer.reverse(i << shift);
            if (i < j) {
                BigInteger temp = a.get(i);
                a.set(i, a.get(j));
                a.set(j, temp);
            }
        }

        BigInteger root_inv = root.modPow(mod.subtract(BigInteger.TWO), mod);

        for (int len = 1; len < n; len <<= 1) {
            BigInteger wlen = BigInteger.ZERO;
            if(invert){
                wlen = root_inv.modPow(mod.subtract(BigInteger.ONE).divide(BigInteger.valueOf(2*len)), mod);
            }else{
                wlen = root.modPow(mod.subtract(BigInteger.ONE).divide(BigInteger.valueOf(2*len)), mod);
            }
//            int wlen = pow(invert ? root_inv : root, (mod - 1) / (2 * len), mod);
            for (int i = 0; i < n; i += 2 * len){
                BigInteger w = BigInteger.ONE;
                for (int j = 0; j < len; ++j) {
                    BigInteger u = a.get(i + j);
                    BigInteger v = a.get(i + j + len).multiply(w).mod(mod);
                    a.set(i + j, u.add(v).mod(mod));
                    a.set(i + j + len, u.subtract(v).add(mod).mod(mod));
                    w = w.multiply(wlen).mod(mod);
                }
            }

        }
        if (invert) {
            BigInteger nrev = BigInteger.valueOf(n).modPow(mod.subtract(BigInteger.TWO), mod);
            for (int i = 0; i < n; ++i) a.set(i, a.get(i).multiply(nrev).mod(mod));
        }
    }

    public static List<BigInteger> multiply(List<BigInteger> a, List<BigInteger> b) {
        int need = a.size() + b.size();
        int n = Integer.highestOneBit(need - 1) << 1;
        List<BigInteger> A = new LinkedList<>();
        List<BigInteger> B = new LinkedList<>();
        for (int i = 0; i < a.size(); i++) A.add(a.get(i));
        for (int i = 0; i < b.size(); i++) B.add(b.get(i));
        for (int i = a.size(); i < need; i ++) A.add(BigInteger.ZERO);
        for (int i = b.size(); i < need; i ++) B.add(BigInteger.ZERO);

//        int mod = 998244353; // 2^23 * 119 + 1


        ntt(A, false, mod, root);
        ntt(B, false, mod, root);
        for (int i = 0; i < n; i++) A.set(i, A.get(i).multiply(B.get(i)).mod(mod));
        ntt(A, true, mod, root);
        BigInteger carry = BigInteger.ZERO;
        for (int i = 0; i < need; i++) {
            BigInteger temp = A.get(i);
            temp = temp.add(carry);
            carry = temp.divide(BigInteger.valueOf(10));
            temp = temp.mod(BigInteger.valueOf(10));
            A.set(i, temp);
        }
        return A;
    }

    public static List<BigInteger> domain(int domainSize){
        BigInteger generator = findPrimitiveRoot(BigInteger.valueOf(domainSize), mod.subtract(ONE), mod);
        List<BigInteger> domains = new LinkedList<>();
        for(int i = 0; i < domainSize; i++){
            domains.add(generator.pow(i).mod(mod));
        }
        return domains;
    }

    // random test
    public static void main(String[] args) {
        BigInteger mod1 = new BigInteger("21888242871839275222246405745257275088548364400416034343698204186575808495617");
        BigInteger root1= findPrimitiveRoot(BigInteger.valueOf(32), mod1.subtract(BigInteger.ONE), mod1);
        System.out.println(root1);

    }

    public static BigInteger findPrimitiveRoot(BigInteger degree, BigInteger totient, BigInteger mod) {
        if (degree.compareTo(ONE) < 0 || degree.compareTo(totient) > 0
                || totient.compareTo(mod) >= 0 || totient.mod(degree).signum() != 0)
            throw new IllegalArgumentException();
//        BigInteger gen = findGenerator(totient, mod);
//        The generator 5 was precomputed.
        BigInteger gen = BigInteger.valueOf(5);
        return gen.modPow(totient.divide(degree), mod);
    }

    public static BigInteger findGenerator(BigInteger totient, BigInteger mod) {
        if (totient.compareTo(ONE) < 0 || totient.compareTo(mod) >= 0)
            throw new IllegalArgumentException();
        for (BigInteger i = ONE; i.compareTo(mod) < 0; i = i.add(ONE)) {
            if (isPrimitiveRoot(i, totient, mod))
                return i;
        }
        throw new ArithmeticException("No generator exists");
    }

    public static boolean isPrimitiveRoot(BigInteger val, BigInteger degree, BigInteger mod) {
        if (val.signum() == -1 || val.compareTo(mod) >= 0)
            throw new IllegalArgumentException();
        if (degree.compareTo(ONE) < 0 || degree.compareTo(mod) >= 0)
            throw new IllegalArgumentException();

        if (!val.modPow(degree, mod).equals(ONE))
            return false;
        for (BigInteger p : uniquePrimeFactors(degree)) {
            if (val.modPow(degree.divide(p), mod).equals(ONE))
                return false;
        }
        return true;
    }

    public static List<BigInteger> uniquePrimeFactors(BigInteger n) {
        if (n.compareTo(ONE) < 0)
            throw new IllegalArgumentException();
        List<BigInteger> result = new ArrayList<>();
        for (BigInteger i = TWO, end = sqrt(n); i.compareTo(end) <= 0; i = i.add(ONE)) {
            if (n.mod(i).signum() == 0) {
                result.add(i);
                do n = n.divide(i);
                while (n.mod(i).signum() == 0);
                end = sqrt(n);
            }
        }
        if (n.compareTo(ONE) > 0)
            result.add(n);
        return result;
    }

    public static boolean isPrime(BigInteger n) {
        if (n.compareTo(ONE) <= 0)
            throw new IllegalArgumentException();
        if (!n.isProbablePrime(10))
            return false;
        if (!n.testBit(0))
            return n.equals(TWO);
        for (BigInteger i = BigInteger.valueOf(3), end = sqrt(n);
             i.compareTo(end) <= 0; i = i.add(TWO)) {
            if (n.mod(i).signum() == 0)
                return false;
        }
        return true;
    }
    public static BigInteger sqrt(BigInteger x) {
        if (x.signum() == -1)
            throw new IllegalArgumentException();
        BigInteger y = BigInteger.ZERO;
        for (int i = (x.bitLength() - 1) / 2; i >= 0; i--) {
            y = y.setBit(i);
            if (y.multiply(y).compareTo(x) > 0)
                y = y.clearBit(i);
        }
        return y;
    }


    private static final BigInteger TWO = BigInteger.valueOf(2);

}