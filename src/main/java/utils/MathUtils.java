package utils;

import java.math.BigInteger;

public class MathUtils {
	
	private MathUtils() {
	}
	
	public static BigInteger addmod(BigInteger a, BigInteger b, BigInteger q) {
		return a.add(b).mod(q);
	}
	
	public static BigInteger mulmod(BigInteger a, BigInteger b, BigInteger q) {
		return a.multiply(b).mod(q);
	}
	
	
}
