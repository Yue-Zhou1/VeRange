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
import zkp.Type4_batch;

public class Type4_batchTest {
    private Commiter commiter;

    private int instances = 1;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }

    @Test
    public void testType4_batch() {
        int nbits = 32;
        int nprimebits = 16;

        int B = 4;

        BigInteger n = Config.getInstance().getKey().getN();


        VectorP gs = VectorP.from(IntStream.range(0, nbits).mapToObj(
                        i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
                .collect(Collectors.toList()));
//        VectorP gs = VectorP.from(this.commiter.getGs().subList(0, U));

        for (int i = 0; i < instances; i++) {
            BigInteger x = this.commiter.randWithBits(nbits);
//            System.out.println("input x is: " + x);
//            BigInteger x = BigInteger.valueOf(13);
            Type4_batch zkp = new Type4_batch(x, nprimebits, B, gs);
//            zkp.verify();
            assertTrue(zkp.verify());
        }
        System.out.println("Type-4 Range Prove Time:" + Type4_batch.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("Type-4 Range Verify Time:" + Type4_batch.vtime / (instances - TestConstants.WARMUPS));
    }
}
