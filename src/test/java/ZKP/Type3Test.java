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
import zkp.Type3;

public class Type3Test {
    private Commiter commiter;

    private int instances = 1;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }

    @Test
    public void testType3() {
//        int nbits = 512;
//        int nprimebits = 162;
//        int U = 13;
//        int V = 13;
//        int B = 10;


        // aggregated parameters
        int nbits = 256;
        int nprimebits = 100;
        int U = 10;
        int V = U;
        int B = 6;

        int TT = 2;
        boolean aggre = true;

        BigInteger n = Config.getInstance().getKey().getN();


//        VectorP gs = VectorP.from(IntStream.range(0, V).mapToObj(
//                        i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
//                .collect(Collectors.toList()));
        VectorP gs = VectorP.from(this.commiter.getGs().subList(0, U));

        for (int i = 0; i < instances; i++) {
//            BigInteger x = this.commiter.randWithBits(nbits);

            List<BigInteger> xs = new LinkedList<>();
            BigInteger x = this.commiter.randWithBits(nbits/TT);
            BigInteger x1 = this.commiter.randWithBits(nbits/TT);
//            BigInteger x2 = this.commiter.randWithBits(nbits/TT);
//            BigInteger x3 = this.commiter.randWithBits(nbits/TT);
//            BigInteger x = BigInteger.valueOf(100);
//            BigInteger x1 = BigInteger.valueOf(200);
//            BigInteger x2 = BigInteger.valueOf(300);
//            BigInteger x3 = BigInteger.valueOf(400);


            xs.add(x);
            xs.add(x1);
//            xs.add(x2);
//            xs.add(x3);

            Type3 zkp = new Type3(xs, nprimebits, B, gs, TT, aggre);
//            Type3 zkp = new Type3(xs, nbits, gs, K, TT, aggre);

			assertTrue(zkp.verify());
        }
        System.out.println("Type-3 Range Prove Time:" + Type3.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("Type-3 Range Verify Time:" + Type3.vtime / (instances - TestConstants.WARMUPS));
    }
}
