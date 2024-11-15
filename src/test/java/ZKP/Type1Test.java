package ZKP;

import static org.junit.Assert.assertTrue;

import java.math.BigInteger;
import java.util.LinkedList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import com.google.common.collect.Lists;
import org.junit.Before;
import org.junit.Test;

import commitment.Commiter;
import config.BouncyKey;
import config.Config;
import structure.VectorP;
import utils.HashUtils;
import zkp.TestConstants;
import zkp.Type1;

public class Type1Test {
    private Commiter commiter;

    private int instances = 1;

    @Before
    public void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        this.commiter = Config.getInstance().getCommiter();

    }

    @Test
    public void testType1() {
//        int nbits = 512;
//        int L = 22;
//        int K = 24;

        // aggregated parameters
        int nbits = 256;
        int L = 16;
        int K = 16;
        int TT = 2;
        boolean aggre = true;


        BigInteger n = Config.getInstance().getKey().getN();

        VectorP gs = VectorP.from(IntStream.range(0, L).mapToObj(
                        i -> this.commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(n)))
                .collect(Collectors.toList()));
//        VectorP gs = VectorP.from(this.commiter.getGs().subList(0, L));

        for (int i = 0; i < instances; i++) {
            List<BigInteger> xs = new LinkedList<>();
            BigInteger x = this.commiter.randWithBits(nbits/TT);
            BigInteger x1 = this.commiter.randWithBits(nbits/TT);
//            BigInteger x2 = this.commiter.randWithBits(nbits/TT);
//            BigInteger x3 = this.commiter.randWithBits(nbits/TT);
//            BigInteger x = BigInteger.valueOf(100);
//            BigInteger x1 = BigInteger.valueOf(200);


            xs.add(x);
            xs.add(x1);
//            xs.add(x2);
//            xs.add(x3);
            Type1 zkp = new Type1(xs, nbits, gs, K, TT, aggre);

			assertTrue(zkp.verify());
        }
        System.out.println("Type-1 Range Prove Time:" + Type1.ptime / (instances - TestConstants.WARMUPS));
        System.out.println("Type-1 Range Verify Time:" + Type1.vtime / (instances - TestConstants.WARMUPS));
    }

    @Test
    public void testMatrix(){
        int nbits = 16;
        int L = 4;
        int K = 4;
        int TT = 2;

        List<BigInteger> y = new LinkedList<>();
//            BigInteger x = this.commiter.randWithBits(nbits);
        BigInteger x = BigInteger.valueOf(100);
        BigInteger x1 = BigInteger.valueOf(200);
        BigInteger gamma = BigInteger.valueOf(2);

        List<List<BigInteger>> bs = Lists.newLinkedList();
        List<List<BigInteger>> bsr = Lists.newLinkedList();
        List<List<BigInteger>> ds = Lists.newLinkedList();
        for (int i = 0; i < L; i++) {
            bs.add(new LinkedList<>());
        }
        for (int i = 0; i < K; i++) {
            ds.add(new LinkedList<>());
            bsr.add(new LinkedList<>());
        }

        List<BigInteger> TWOS = Lists.newLinkedList();
        for (int i = 0; i < nbits; i++) {
            BigInteger value = BigInteger.TWO.pow(i);
            TWOS.add(value);
        }

        y.add(x);
        y.add(x1);

        int LK = L * K;
        int tt = 0;
        for (int i = 0; i < LK; i++) {
            int idx = i / (K);
            int mod = i % (K);
            int mod_agg = i % (TT*K);
            if(mod_agg == 0){
                tt+=1;
            }
            System.out.println(tt);
            System.out.println(mod_agg);
            BigInteger b = y.get(tt-1).testBit(mod_agg) ? BigInteger.ONE : BigInteger.ZERO;
            if (i < nbits) {

                BigInteger twopowers = gamma.pow(tt).multiply(TWOS.get(mod_agg));
                BigInteger w = b.multiply(twopowers);
                bs.get(idx).add(w);
                bsr.get(mod).add(twopowers);
                ds.get(mod).add(w);
            } else {
                bs.get(idx).add(BigInteger.ZERO);
                bsr.get(mod).add(BigInteger.ZERO);
                ds.get(mod).add(BigInteger.ZERO);
            }
        }

        System.out.println(bs);
        System.out.println(bsr);
        System.out.println(ds);
    }
}
