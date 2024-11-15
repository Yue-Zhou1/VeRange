package ZKP;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import java.math.BigInteger;
import java.util.ArrayList;
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
import zkp.SwiftRange;

public class SwiftRangeTest {

    private Commiter commiter;

    private int instances = 50;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();
    }


    @Test
    public void testSwiftRange1() {

        int nbits = 512;
//        int L = 2;

        BigInteger n = Config.getInstance().getKey().getN();

        VectorP gs = VectorP.from(IntStream.range(0, nbits).mapToObj(
                i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
                .collect(Collectors.toList()));

        for (int i = 0; i < instances; i++) {
            BigInteger x = this.commiter.randWithBits(nbits);

            SwiftRange zkp = new SwiftRange(x, nbits, gs, false);

            assertTrue(zkp.verify());
//            System.out.println(zkp.numOfElements());
        }

        System.out.println("Swift Prove Time:" + SwiftRange.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("Swift Verify Time:" + SwiftRange.vtime / (instances - TestConstants.WARMUPS));
    }

    @Test
    public void testDecompose(){
        BigInteger A = BigInteger.valueOf(30);
        List<BigInteger> B_nary = new ArrayList<>();
        while (A.intValue() > 0) {
            BigInteger r = A.mod(BigInteger.valueOf(3));
            B_nary.add(r);
            A = A.divide(BigInteger.valueOf(3));
        }

        //  System.out.println(binary.toString());
//        List<Integer> res = new ArrayList<>();
//        int j = 0;
//        for (int i = 0; i < B_nary.size(); i++) {
//            if (B_nary.get(i) != BigInteger.ZERO) {
//                while (j < B_nary.get(i).intValue()) {
//                    res.add((int) Math.pow(3, i));
//                    j++;
//                }
//                j =0;
//            }
//        }
//        System.out.println(res);
        System.out.println(B_nary);
    }
}