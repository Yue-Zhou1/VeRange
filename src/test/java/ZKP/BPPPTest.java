package ZKP;

import commitment.Commiter;
import static org.junit.Assert.assertTrue;
import config.BouncyKey;
import config.Config;
import org.junit.Before;
import org.junit.Test;
import structure.VectorP;
import utils.HashUtils;
import zkp.BPPP;
import zkp.TestConstants;

import java.math.BigInteger;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class BPPPTest {
    private Commiter commiter;

    private int instances = 1;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }

    @Test
    public void testBPPP() {
        int nbits = 32;
        int nprimebits = 16;
        int B = 4;

        BigInteger n = Config.getInstance().getKey().getN();

        VectorP gs = VectorP.from(IntStream.range(0, nprimebits).mapToObj(
                i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
                .collect(Collectors.toList()));

        for (int i = 0; i < instances; i++) {
            BigInteger x = this.commiter.randWithBits(nbits);

            BPPP zkp = new BPPP(x, nprimebits, B, gs);

            assertTrue(zkp.verify());
        }
        System.out.println("BPPP Prove Time:" + BPPP.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("BPPP Verify Time:" + BPPP.vtime / (instances - TestConstants.WARMUPS));
    }
}
