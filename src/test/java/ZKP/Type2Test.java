package ZKP;

import static org.junit.Assert.assertTrue;

import java.math.BigInteger;
import java.util.LinkedList;
import java.util.List;

import org.junit.Before;
import org.junit.Test;

import commitment.Commiter;
import config.BouncyKey;
import config.Config;
import structure.VectorP;
import zkp.TestConstants;
import zkp.Type1;
import zkp.Type2;

public class Type2Test {
    private Commiter commiter;

    private int instances = 1;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }

    @Test
    public void testType2_nTest() {
//        int nbits = 256;
//        int nprimebits = 72;
//        int L = 9;
//        int K = 8;
//        int B = 12;
        int nbits = 512;
        int nprimebits = 132;
        int L = 11;
        int K = 12;
        int B = 15;

        // aggregated parameters
//        int nbits = 256;
//        int L = 16;
//        int K = 16;
        int TT = 4;
        boolean aggre = true;

        BigInteger n = Config.getInstance().getKey().getN();

//        VectorP gs = VectorP.from(IntStream.range(0, L).mapToObj(
//                        i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
//                .collect(Collectors.toList()));
        VectorP gs = VectorP.from(this.commiter.getGs().subList(0, L));

        for (int i = 0; i < instances; i++) {
//            BigInteger x = this.commiter.randWithBits(nbits);
//            BigInteger x = BigInteger.valueOf(9999);
//            Type2 zkp = new Type2(x, nprimebits, B, gs, K, L);

//			assertTrue(zkp.verify());

            List<BigInteger> xs = new LinkedList<>();
            BigInteger x = this.commiter.randWithBits(nbits/TT);
            BigInteger x1 = this.commiter.randWithBits(nbits/TT);
            BigInteger x2 = this.commiter.randWithBits(nbits/TT);
            BigInteger x3 = this.commiter.randWithBits(nbits/TT);
//            BigInteger x = BigInteger.valueOf(100);
//            BigInteger x1 = BigInteger.valueOf(200);
            xs.add(x);
            xs.add(x1);
            xs.add(x2);
            xs.add(x3);
            Type2 zkp = new Type2(xs, nprimebits, B, gs, K, L, TT, aggre);

            assertTrue(zkp.verify());
        }
        System.out.println("Type-2 Range Prove Time:" + Type2.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("Type-2 Range Verify Time:" + Type2.vtime / (instances - TestConstants.WARMUPS));
    }
}
