package ZKP;

import static org.junit.Assert.assertTrue;

import java.math.BigInteger;
import java.util.LinkedList;
import java.util.List;
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
import zkp.Type2;
import zkp.Type2P;

public class Type2PTest {
    private Commiter commiter;

    private int instances = 1;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }

    @Test
    public void testType2PTest() {
        int nbits = 128;
        int nprimebits = 45;
        int L = 15;
        int K = 3;
        int B = 8;

        // aggregated parameters
        int TT = 4;
        boolean aggre = true;

        BigInteger n = Config.getInstance().getKey().getN();

        VectorP gs = VectorP.from(IntStream.range(0, L*TT).mapToObj(
                        i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
                .collect(Collectors.toList()));
//        VectorP gs = VectorP.from(this.commiter.getGs().subList(0, L));

        for (int i = 0; i < instances; i++) {
//            BigInteger x = this.commiter.randWithBits(nbits);
//            Type2P zkp = new Type2P(x, nprimebits, B, gs, K, L);
//
//            assertTrue(zkp.verify());
            List<BigInteger> xs = new LinkedList<>();
            BigInteger x = this.commiter.randWithBits(nbits);
            BigInteger x1 = this.commiter.randWithBits(nbits);
            BigInteger x2 = this.commiter.randWithBits(nbits);
            BigInteger x3 = this.commiter.randWithBits(nbits);

            xs.add(x);
            xs.add(x1);
            xs.add(x2);
            xs.add(x3);
            Type2P zkp = new Type2P(xs, nprimebits, B, gs, K, L, TT, aggre);
            assertTrue(zkp.verify());
        }
        System.out.println("Type-2P Range Prove Time:" + Type2P.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("Type-2P Range Verify Time:" + Type2P.vtime / (instances - TestConstants.WARMUPS));
    }
}
