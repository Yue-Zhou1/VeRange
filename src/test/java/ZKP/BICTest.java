package ZKP;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import java.math.BigInteger;

import org.junit.Before;
import org.junit.Test;

import commitment.Commiter;
import config.BouncyKey;
import config.Config;

import zkp.BIC;
import zkp.TestConstants;


public class BICTest {

    private Commiter commiter;

    private int instances = 50;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }

    @Test
    public void testBIC1() {

        int nbits = 128;

        for (int i = 0; i < instances; i++) {
            BigInteger x = this.commiter.randWithBits(nbits);
            BIC zkp = new BIC(x, nbits);
            assertTrue(zkp.verify());
        }

        System.out.println("BIC Prove Time:" + BIC.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("BIC Verify Time:" + BIC.vtime / (instances - TestConstants.WARMUPS));
    }

}
