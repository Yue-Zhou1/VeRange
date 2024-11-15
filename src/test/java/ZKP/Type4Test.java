package ZKP;

import static org.junit.Assert.assertTrue;

import java.math.BigInteger;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import org.junit.Before;
import org.junit.Test;

import commitment.Commiter;
import config.BouncyKey;
import config.Config;
import structure.VectorP;
import utils.HashUtils;
import zkp.TestConstants;
import zkp.Type4;

public class Type4Test {
    private Commiter commiter;

    private int instances = 1;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }

    @Test
    public void testType4() {
        int nbits = 8;
        int nprimebits = 8;

        int B = 2;

        BigInteger n = Config.getInstance().getKey().getN();


        VectorP gs = VectorP.from(IntStream.range(0, nbits).mapToObj(
                        i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
                .collect(Collectors.toList()));
//        VectorP gs = VectorP.from(this.commiter.getGs().subList(0, U));

        for (int i = 0; i < instances; i++) {
            BigInteger x = this.commiter.randWithBits(nbits);
//            BigInteger x = BigInteger.valueOf(13);
            Type4 zkp = new Type4(x, nprimebits, B, gs);
//            zkp.verify();
            assertTrue(zkp.verify_batch());
        }
//        System.out.println("Type-3 Range Prove Time:" + Type4.ptime / (instances - TestConstants.WARMUPS));
//        System.out.println("Type-3 Range Verify Time:" + Type4.vtime / (instances - TestConstants.WARMUPS));
    }
}
