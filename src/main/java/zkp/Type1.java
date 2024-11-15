package zkp;

import java.math.BigInteger;
import java.util.*;
import java.util.stream.Collectors;

import com.google.common.collect.Lists;

import commitment.Commitment;
import structure.VectorB;
import structure.VectorP;
import utils.BigIntegerUtils;
import utils.HashUtils;

public class Type1 extends PedersenZKP{

    private static final long serialVersionUID = 1L;

    private List<Commitment> Ys = new LinkedList<>();
    private final Commitment bigR;
    private final Commitment bigS;
    private final List<Commitment> cws = new LinkedList<>();

    private final List<Commitment> cts = new LinkedList<>();
    private final List<List<BigInteger>> rs = new LinkedList<>();
    private final List<List<BigInteger>> vs = new LinkedList<>();
    private final BigInteger eta1;
    private final BigInteger eta2;


    private final VectorP gs;
    private final int nbits;
    private final int K;
    private final int L;

    // aggregated parameters
    private final int TT;

    public static int counter = 0;
    public static long ptime = 0;
    public static long vtime = 0;

    public Type1(List<BigInteger> y, int nbits, VectorP gs, int K, int TT, boolean aggre){
        this.gs = gs;
        this.nbits = nbits;
        this.L = this.gs.getList().size();
        this.K = K;
        this.TT = TT;

        // prover initial
        List<BigInteger> rcm = new LinkedList<>();
        if(aggre){
            for(int i = 0; i < y.size(); i++){
                BigInteger rcm_i = this.commiter.rand();
                rcm.add(rcm_i);
                this.Ys.add(this.commiter.commitTo(y.get(i), rcm_i));
            }
        }else{
            BigInteger rcm_i = this.commiter.rand();
            rcm.add(rcm_i);
            this.Ys.add(this.commiter.commitTo(y.get(0), rcm_i));
        }

        // aggregated parameter
        BigInteger gamma = HashUtils.hash(this.Ys).mod(this.n);


        List<BigInteger> gamma_W = Lists.newLinkedList();
        List<BigInteger> gamma_T = Lists.newLinkedList();
        List<BigInteger> r = Lists.newLinkedList();
        List<BigInteger> rk_sum = Lists.newLinkedList();
        BigInteger gamma_R = this.commiter.rand();
        BigInteger gamma_S = this.commiter.rand();

        for(int i = 0; i < this.L; i++){
            rs.add(new LinkedList<>());
            r.add(this.commiter.rand());
            BigInteger rk_sum_ele = BigInteger.ZERO;
            for (int j = 0; j < this.K; j++){
                BigInteger r_k = this.commiter.rand();
                rs.get(i).add(r_k);
                rk_sum_ele = rk_sum_ele.add(r_k);
            }
            rk_sum.add(rk_sum_ele.mod(this.n));
        }

        for(int i = 0; i < this.K; i++){
            gamma_W.add(this.commiter.rand());
            gamma_T.add(this.commiter.rand());
        }

        gamma_W.remove(K-1);
        BigInteger gamma_WK;
        if(aggre){
            BigInteger gamma_rcm_sum = BigInteger.ZERO;
            for(int i = 0; i < this.TT; i++){
                gamma_rcm_sum = gamma_rcm_sum.add(gamma.modPow(BigInteger.valueOf(i+1), this.n).multiply(rcm.get(i)));
            }
            gamma_rcm_sum = gamma_rcm_sum.mod(this.n);
            gamma_WK = gamma_rcm_sum.subtract(VectorB.from(gamma_W, this.n).sum()).mod(this.n);
        }else{
            gamma_WK = rcm.get(0).subtract(VectorB.from(gamma_W, this.n).sum()).mod(this.n);
        }

        gamma_W.add(gamma_WK);

        List<BigInteger> TWOS = Lists.newLinkedList();
        for (int i = 0; i < nbits; i++) {
            BigInteger value = BigInteger.TWO.pow(i);
            TWOS.add(value);
        }

        List<List<BigInteger>> bs = Lists.newLinkedList();
        List<List<BigInteger>> bsr = Lists.newLinkedList();
        List<List<BigInteger>> ds = Lists.newLinkedList();
        for (int i = 0; i < this.L; i++) {
            bs.add(new LinkedList<>());
        }
        for (int i = 0; i < this.K; i++) {
            ds.add(new LinkedList<>());
            bsr.add(new LinkedList<>());
        }

        long s = System.nanoTime();

        if(!aggre){
            int LK = this.L * (this.K);
            for (int i = 0; i < LK; i++) {
                BigInteger b = y.get(0).testBit(i) ? BigInteger.ONE : BigInteger.ZERO;
                int idx = i / (this.K);
                int mod = i % (this.K);
                if (i < nbits) {
                    BigInteger twopowers = TWOS.get(i);
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
        }else{
            int LK = this.L * (this.K);
            int tt = 0;
            int actual_mod = (K%TT==0?K/TT:(K/TT+1))*L;
            for (int i = 0; i < LK; i++) {
                int idx = i / (this.K);
                int mod = i % (this.K);

                int mod_agg = i % actual_mod;
                if(mod_agg == 0){
                    tt+=1;
                }
                BigInteger b = y.get(tt-1).testBit(mod_agg) ? BigInteger.ONE : BigInteger.ZERO;
                if (i < nbits/TT+(tt-1)*actual_mod && i >= (tt-1)*actual_mod) {
                    BigInteger twopowers = gamma.modPow(BigInteger.valueOf(tt), this.n).multiply(TWOS.get(mod_agg)).mod(this.n);
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
        }

        List<BigInteger> rjs = Lists.newLinkedList();
        for (int i = 0; i < this.K; i++) {
            List<BigInteger> ts = Lists.newLinkedList();
            for (int j = 0; j < this.L; j++) {
                BigInteger t = rs.get(j).get(i).multiply(bsr.get(i).get(j).subtract(ds.get(i).get(j).multiply(BigInteger.TWO)));
                t = (t.compareTo(this.n) < 0 && t.compareTo(BigInteger.ZERO) >= 0) ? t : t.mod(this.n);
                ts.add(t);
            }
            this.cws.add(this.commiter
                    .commitTo(ds.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n), gamma_W.get(i)));
            this.cts.add(this.gs.mulBAndSum(VectorB.from(ts, this.n)).add(this.commiter.mulH(gamma_T.get(i))));
        }

        for(int i = 0; i < this.L; i++){
            BigInteger rj = BigInteger.ZERO;
            for(int j = 0; j < this.K; j++){
                BigInteger r2 = rs.get(i).get(j).pow(2).mod(this.n);
                rj = rj.add(r2).mod(this.n);
            }
            rjs.add(rj.negate());
        }

        bigR = this.commiter
                .commitTo(rk_sum.stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n), gamma_R);

        bigS = this.gs.mulBAndSum(VectorB.from(rjs, this.n)).add(this.commiter.mulH(gamma_S));

        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cts);
        cs.addAll(this.cws);
        cs.add(bigR);
        cs.add(bigS);

        List<BigInteger> cl_es = Lists.newLinkedList();
        BigInteger beta = HashUtils.hash(cs).mod(this.n);
        cl_es.add(beta);
        for(int i = 0; i < this.K-1; i ++){
            BigInteger betaX = beta.modPow(BigInteger.valueOf(i+2),this.n);
            cl_es.add(betaX);
        }

        eta1 = VectorB.from(gamma_T,this.n).innerProd(VectorB.from(cl_es,this.n)).add(gamma_S).mod(this.n);
        eta2 = VectorB.from(gamma_W,this.n).innerProd(VectorB.from(cl_es,this.n)).add(gamma_R).mod(this.n);

        for(int i = 0; i < this.L; i++){
            vs.add(VectorB.from(bs.get(i), this.n).mul(VectorB.from(cl_es, this.n)).add(VectorB.from(rs.get(i),this.n)).getList());
        }

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            ptime += (e - s);
        }

        counter++;
        string();

    }


    public void string() {

        List<String> total1 = new LinkedList<>();

        String cyStr = "[" + String.join(",",
                this.Ys.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cyStr);

        String ctsStr = "[" + String.join(",",
                this.cts.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(ctsStr);

        String cwsStr = "[" + String.join(",",
                this.cws.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cwsStr);

        String bigSStr = BigIntegerUtils.toString(this.bigS.getCoordList());
        total1.add(bigSStr);

        String bigRStr = BigIntegerUtils.toString(this.bigR.getCoordList());
        total1.add(bigRStr);

        total1.add("\"" + this.eta1 + "\"");

        total1.add("\"" + this.eta2 + "\"");

        total1.add("[");
        for(int i = 0; i < this.vs.size(); i++){
            String vStr = "[" + String.join(",", this.vs.get(i).stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                    + "]";
            total1.add(vStr);
        }
        total1.add("]");

        System.out.println("str1:" + String.join(",", total1));

        String gsStr = "[" + String.join(",", this.gs.getList().stream()
                .map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList())) + "]";
        System.out.println("gsStr:" + String.join(",", gsStr));
    }
    @Override
    public boolean verify() {
        // check equation 1;
        BigInteger gamma = HashUtils.hash(this.Ys).mod(this.n);
//        System.out.println(gamma);
        List<BigInteger> udotvs = new LinkedList<>();
        List<BigInteger> TWOS = Lists.newLinkedList();
        for (int i = 0; i < nbits; i++) {
            BigInteger value = BigInteger.TWO.pow(i);
            TWOS.add(value);
        }
        List<List<BigInteger>> bsr = Lists.newLinkedList();
        for (int i = 0; i < this.L; i++) {
            bsr.add(new LinkedList<>());
        }
        int LK = this.L * (this.K);
        if(this.TT == 1){
            for (int i = 0; i < LK; i++) {
                int idx = i / (this.K);
                if (i < nbits) {
                    BigInteger twopowers = TWOS.get(i);
                    bsr.get(idx).add(twopowers);
                } else {
                    bsr.get(idx).add(BigInteger.ZERO);
                }
            }
        }else{
            int tt = 0;
            int actual_mod = (K%TT==0?K/TT:(K/TT+1))*L;
            for (int i = 0; i < LK; i++) {
                int idx = i / (this.K);
//                int mod = i % (this.K);
                int mod_agg = i % actual_mod;
                if(mod_agg == 0){
                    tt+=1;
                }
                if (i < nbits/TT+(tt-1)*actual_mod && i >= (tt-1)*actual_mod) {
                    BigInteger twopowers = gamma.modPow(BigInteger.valueOf(tt), this.n).multiply(TWOS.get(mod_agg)).mod(this.n);
                    bsr.get(idx).add(twopowers);
                } else {
                    bsr.get(idx).add(BigInteger.ZERO);
                }
            }
        }

        long s = System.nanoTime();

        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cts);
        cs.addAll(this.cws);
        cs.add(bigR);
        cs.add(bigS);

        List<BigInteger> cl_es = Lists.newLinkedList();
        BigInteger beta = HashUtils.hash(cs).mod(this.n);
        cl_es.add(beta);
        for(int i = 0; i < this.K-1; i ++){
            BigInteger betaX = beta.modPow(BigInteger.valueOf(i+2),this.n);
            cl_es.add(betaX);
        }

        List<List<BigInteger>> us = new LinkedList<>();
        for(int i = 0; i < this.L; i++){
            VectorB two_K_cl = VectorB.from(bsr.get(i), this.n).mul(VectorB.from(cl_es, this.n));
            us.add(two_K_cl.sub(VectorB.from(vs.get(i),this.n)).getList());
        }

        for (int i = 0; i < this.L; i++){
            BigInteger udotv = VectorB.from(us.get(i), this.n).innerProd(VectorB.from(this.vs.get(i), this.n));
            udotvs.add(udotv.mod(this.n));
        }

        Commitment eqn1_1 = this.gs.mulBAndSum(VectorB.from(udotvs, this.n)).add(this.commiter.mulH(this.eta1));
        Commitment eqn1_2 = this.commiter.getIdentity();
        for (int i = 0; i < this.K; i++){
            eqn1_2 = eqn1_2.add(this.cts.get(i).mul(cl_es.get(i)));
        }
        eqn1_2 = eqn1_2.add(this.bigS);
        boolean b1 = eqn1_1.equals(eqn1_2);

//        // check equation 2;
        BigInteger vsum = BigInteger.ZERO;
        for (int i = 0; i < this.L; i++){
            vsum = vsum.add(this.vs.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n));
        }
        vsum = vsum.mod(this.n);
        Commitment eqn2_1 =this.commiter.getIdentity();
        eqn2_1 = eqn2_1.add(this.commiter.commitTo(vsum, this.eta2));

        Commitment eqn2_2 = this.commiter.getIdentity();
        for (int i = 0; i < this.K; i++){
            eqn2_2 = eqn2_2.add(this.cws.get(i).mul(cl_es.get(i)));
        }
        eqn2_2 = eqn2_2.add(this.bigR);
        boolean b2 = eqn2_1.equals(eqn2_2);

//        // check equation 3;
        boolean b3 = false;
        if(this.TT == 1){
            b3 = this.Ys.get(0).equals(this.cws.stream()
                    .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));
        }else{
            Commitment eqn3_1 = this.commiter.getIdentity();
            for (int i = 0; i < this.TT; i++){
                eqn3_1 = eqn3_1.add(this.Ys.get(i).mul(gamma.modPow(BigInteger.valueOf(i+1), this.n)));
            }
            b3 = eqn3_1.equals(this.cws.stream()
                    .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));
        }

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            vtime += (e - s);
        }
        return b1 && b2 && b3;
    }
}
