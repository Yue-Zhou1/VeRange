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

public class Type2 extends PedersenZKP{
    private List<Commitment> cYs = new LinkedList<>();;
    private final Commitment cR;
    private final Commitment cS;
    private final List<Commitment> cws = new LinkedList<>();
    private final List<Commitment> cms = new LinkedList<>();
    private final List<List<Commitment>> cms_agg = new LinkedList<>();
    private final List<Commitment> cvk = new LinkedList<>();
    private final List<Commitment> ctk = new LinkedList<>();
    private final List<List<BigInteger>> vs = new LinkedList<>();
    private final List<List<BigInteger>> us = new LinkedList<>();

    private final BigInteger eta1;
    private final BigInteger eta2;
    private final BigInteger eta3;

    private final VectorP gs;
    private final int nbits;
    private final int K;
    private final int L;
    private final BigInteger B;

    // aggregated parameters
    private final int TT;

    public static int counter = 0;
    public static long ptime = 0;
    public static long vtime = 0;

    public Type2(List<BigInteger> y, int nbits, int b_ary, VectorP gs, int K, int L, int TT, boolean aggre){
        this.gs = gs;
        this.nbits = nbits;
        this.L = L;
        this.K = K;
        this.B = BigInteger.valueOf(b_ary);
        this.TT = TT;

        // make a commitment to the witness y.
//        BigInteger rcm = this.commiter.rand();
//        this.cy = this.commiter.commitTo(y, rcm);

        List<BigInteger> rcm = new LinkedList<>();
        if(aggre){
            for(int i = 0; i < y.size(); i++){
                BigInteger rcm_i = this.commiter.rand();
                rcm.add(rcm_i);
                this.cYs.add(this.commiter.commitTo(y.get(i), rcm_i));
            }
        }else{
            BigInteger rcm_i = this.commiter.rand();
            rcm.add(rcm_i);
            this.cYs.add(this.commiter.commitTo(y.get(0), rcm_i));
        }

        // aggregated parameter
        BigInteger gamma = HashUtils.hash(this.cYs).mod(this.n);

        // make a list to contain B^0, B^1,..., B^nbits
        List<BigInteger> Bs = Lists.newLinkedList();
        for(int i = 0; i < nbits; i++){
            BigInteger value = this.B.pow(i);
            Bs.add(value);
        }

        // initialize random elements r_mu, r_v
        List<List<BigInteger>> rmu = Lists.newLinkedList();
        List<List<BigInteger>> rv = Lists.newLinkedList();

        for(int i = 0; i < this.L; i++){
            rmu.add(new LinkedList<>());
            rv.add(new LinkedList<>());
            for (int j = 0; j < this.K; j++){
                rmu.get(i).add(this.commiter.rand());
                rv.get(i).add(this.commiter.rand());
            }
        }

        // initialize parameters
        List<BigInteger> romega = Lists.newLinkedList();
        List<BigInteger> rbigt = Lists.newLinkedList();
        List<BigInteger> rbigm = Lists.newLinkedList();
        List<BigInteger> rbigv = Lists.newLinkedList();

        List<List<BigInteger>> ds = Lists.newLinkedList();
        List<List<BigInteger>> bs = Lists.newLinkedList();
        List<List<BigInteger>> dds = Lists.newLinkedList();

        List<List<BigInteger>> Bk = Lists.newLinkedList();
        List<List<BigInteger>> B_k = Lists.newLinkedList();

        BigInteger rbigr = this.commiter.rand();
        BigInteger rbigs = this.commiter.rand();

        for(int i =0; i < this.K; i++){
            romega.add(this.commiter.rand());
            rbigt.add(this.commiter.rand());
            rbigv.add(this.commiter.rand());
            dds.add(new LinkedList<>());
        }

        // set romega_K
        romega.remove(K-1);
//        BigInteger gamma_WK = rcm.subtract(VectorB.from(romega, this.n).sum()).mod(this.n);
//        romega.add(gamma_WK);

        BigInteger gamma_WK;
        if(aggre){
            BigInteger gamma_rcm_sum = BigInteger.ZERO;
            for(int i = 0; i < this.TT; i++){
                gamma_rcm_sum = gamma_rcm_sum.add(gamma.modPow(BigInteger.valueOf(i+1), this.n).multiply(rcm.get(i)));
            }
            gamma_rcm_sum = gamma_rcm_sum.mod(this.n);
            gamma_WK = gamma_rcm_sum.subtract(VectorB.from(romega, this.n).sum()).mod(this.n);
        }else{
            gamma_WK = rcm.get(0).subtract(VectorB.from(romega, this.n).sum()).mod(this.n);
        }
        romega.add(gamma_WK);


        int newRows = L;

        for (int i = 0; i < newRows; i++) {
            Bk.add(new LinkedList<>());
            B_k.add(new LinkedList<>());
            ds.add(new LinkedList<>());
            bs.add(new LinkedList<>());
        }

        // decompose the witness y according to the base B, padding 0 if needed.

        int LK = this.L * this.K;

        long s = System.nanoTime();
        List<BigInteger> mc1 = Lists.newLinkedList();

        // set up matrix, d_jk, B_jk, omega_jk
        if(!aggre){
            List<BigInteger> narys = BigIntegerUtils.decomposeToNary(y.get(0), B);
            for (int i = narys.size(); i < nbits; i++){
                narys.add(BigInteger.ZERO);
            }
            for (int i = 0; i < LK; i++) {
                int idx = i / this.K;
                int mod = i % this.K;
                if (i < nbits) {
                    ds.get(idx).add(narys.get(i).multiply(Bs.get(i)));
                    Bk.get(idx).add(Bs.get(i));
                    B_k.get(idx).add(Bs.get(i).modInverse(this.n));
                    bs.get(idx).add(narys.get(i));
                    dds.get(mod).add(narys.get(i).multiply(Bs.get(i)));
                }
                else {
                    ds.get(idx).add(BigInteger.ZERO);
                    Bk.get(idx).add(BigInteger.ZERO);
                    B_k.get(idx).add(BigInteger.ZERO);
                    bs.get(idx).add(BigInteger.ZERO);
                    dds.get(mod).add(BigInteger.ZERO);
                }
            }

            for(int i = 0; i < B.intValue(); i++){
                int occurrences = Collections.frequency(narys, BigInteger.valueOf(i));
                mc1.add(BigInteger.valueOf(occurrences));
            }
        }else{
            int tt = 0;
            int actual_mod = (K%TT==0?K/TT:(K/TT+1))*L;
            List<List<BigInteger>> narys_T = new LinkedList<>();
            List<BigInteger> narys = new LinkedList<>();
            for (int i = 0; i < LK; i++) {
                int idx = i / (this.K);
                int mod = i % (this.K);
                int mod_agg = i % actual_mod;
                if(mod_agg == 0){
                    narys = BigIntegerUtils.decomposeToNary(y.get(tt), B);
                    for (int j = narys.size(); j < nbits/TT; j++){
                        narys.add(BigInteger.ZERO);
                    }
                    narys_T.add(narys);
                    tt+=1;
                }
                if (i < nbits/TT+(tt-1)*actual_mod && i >= (tt-1)*actual_mod) {
                    BigInteger bpowers = gamma.modPow(BigInteger.valueOf(tt), this.n).multiply(Bs.get(mod_agg)).mod(this.n);
                    BigInteger w = narys.get(mod_agg).multiply(bpowers);
                    ds.get(idx).add(w);
                    Bk.get(idx).add(bpowers);
                    B_k.get(idx).add(bpowers.modInverse(this.n));
                    bs.get(idx).add(narys.get(mod_agg));
                    dds.get(mod).add(w);
                } else {
                    ds.get(idx).add(BigInteger.ZERO);
                    Bk.get(idx).add(BigInteger.ZERO);
                    B_k.get(idx).add(BigInteger.ZERO);
                    bs.get(idx).add(BigInteger.ZERO);
                    dds.get(mod).add(BigInteger.ZERO);
                }

            }

            for(int i = 0; i < B.intValue(); i++){
                int occurrences = 0;
                for(int j = 0; j < TT; j++){
                    occurrences += Collections.frequency(narys_T.get(j), BigInteger.valueOf(i));
                }
                mc1.add(BigInteger.valueOf(occurrences));
            }
        }

        // the commitments of omega_k
        for (int i = 0; i < this.K; i++) {
            this.cws.add(this.commiter
                    .commitTo(dds.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n), romega.get(i)));
        }

        // the commitments of mc1
        BigInteger rM_value = BigInteger.ZERO;
        for (int i = 0; i < B.intValue(); i++){
            BigInteger rM = this.commiter.rand();
            rbigm.add(rM);
            rM_value = rM_value.add(rM);
        }
        if(!aggre){
            for (int i = 0; i < B.intValue(); i++){
                this.cms.add(this.commiter.commitTo(mc1.get(i), rbigm.get(i)));
            }
        }
        else{
            for (int i = 0; i < B.intValue(); i++){
                this.cms.add(this.commiter.commitTo(mc1.get(i), rbigm.get(i)));
            }
        }

        List<List<BigInteger>> Bk_rmu = new LinkedList<>();
        for(int i = 0; i < this.K; i++){
            List<BigInteger> column = new LinkedList<>();
            for (int j = 0; j < this.L; j++){
                if (!Bk.get(j).get(i).equals(BigInteger.ZERO)) {
                    column.add((Bk.get(j).get(i).multiply(rmu.get(j).get(i))).mod(this.n));
                }
            }
            Bk_rmu.add(column);
        }

        for (int i = 0; i < this.K; i++) {
            this.cvk.add(this.commiter
                    .commitTo(Bk_rmu.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n), rbigv.get(i)));
        }

        // calculate random alpha using hash
        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cws);
        cs.addAll(this.cms);
        cs.addAll(this.cvk);

        BigInteger alpha = HashUtils.hash(cs).mod(this.n);
        // calculate the commitments of R~, S~, and V_k
        BigInteger cRconstant = BigInteger.ZERO;
        List<BigInteger> cSconstants = new LinkedList<>();
        for(int i = 0; i < this.L; i++){
            BigInteger cSconstant = BigInteger.ZERO;
            for (int j = 0; j < this.K; j++){
                if (!Bk.get(i).get(j).equals(BigInteger.ZERO)){
                    cRconstant = cRconstant.add(rv.get(i).get(j).mod(this.n));
                    cSconstant = cSconstant.add(rmu.get(i).get(j).multiply(rv.get(i).get(j)).mod(this.n));
                }
            }
            cSconstants.add(cSconstant.mod(this.n));
        }
        cR = this.commiter.commitTo(cRconstant.mod(this.n), rbigr);
        cS = this.gs.mulBAndSum(VectorB.from(cSconstants, this.n)).add(this.commiter.mulH(rbigs));

        // f_jk and tau_jk
        List<List<BigInteger>> fs = new LinkedList<>();
        List<List<BigInteger>> taos = new LinkedList<>();
        BigInteger fs_sum = BigInteger.ZERO;

        for(int i = 0; i < this.L; i++){
            List<BigInteger> fk = new LinkedList<>();
            for(int j = 0; j < this.K; j++){
                if (!Bk.get(i).get(j).equals(BigInteger.ZERO)) {
                    BigInteger fk1 = alpha.add(bs.get(i).get(j)).mod(this.n);
                    BigInteger fk1inverse = fk1.modInverse(this.n);
                    fs_sum = fs_sum.add(fk1inverse);
                    fk.add(fk1inverse);
                }
            }
            fs.add(fk);
        }

        for(int i = 0; i < this.K; i++){
            List<BigInteger> taok = new LinkedList<>();
            for(int j = 0; j < this.L; j++){
                if (!Bk.get(j).get(i).equals(BigInteger.ZERO)) {
                    BigInteger f_jk = fs.get(j).get(i);
                    BigInteger B_kf_rv = B_k.get(j).get(i).multiply(f_jk).multiply(rv.get(j).get(i)).mod(this.n);
                    BigInteger alpha_Bk_wk = alpha.multiply(Bk.get(j).get(i)).add(ds.get(j).get(i)).mod(this.n);
                    taok.add(B_kf_rv.add(alpha_Bk_wk.multiply(rmu.get(j).get(i))).mod(this.n));
                }else{
                    taok.add(BigInteger.ZERO);
                }
            }
            taos.add(taok);
        }

        // the commitments of T_k
        for(int i = 0; i< this.K;i++){
            this.ctk.add(this.gs.mulBAndSum(VectorB.from(taos.get(i), this.n)).add(this.commiter.mulH(rbigt.get(i))));
        }

        // calculate random epsilon
        cs.addAll(this.ctk);

        List<BigInteger> cl_es = Lists.newLinkedList();
        BigInteger beta = HashUtils.hash(cs).mod(this.n);
        BigInteger e2s = BigInteger.ZERO;
        cl_es.add(beta);
        for(int i = 0; i < this.K-1; i ++){
            BigInteger betaX = beta.modPow(BigInteger.valueOf(i+2),this.n);
            cl_es.add(betaX);
        }
        List<BigInteger> einverse = new LinkedList<>();
        for(int i = 0; i < this.K; i++){
            e2s = e2s.add(cl_es.get(i).pow(2)).mod(this.n);
            einverse.add(cl_es.get(i).modInverse(this.n));
        }
        System.out.println("einverse: " + einverse);

        // calculate vector v_j, u_j
        for (int i =0; i < this.L;i++){
            List<BigInteger> wdote = new LinkedList<>();
            List<BigInteger> Bdotfdote = new LinkedList<>();
            for(int j = 0; j < this.K; j++) {
                if (!Bk.get(i).get(j).equals(BigInteger.ZERO)) {
                    wdote.add(ds.get(i).get(j).multiply(cl_es.get(j)).add(rv.get(i).get(j)).mod(this.n));
                    Bdotfdote.add(B_k.get(i).get(j).multiply(fs.get(i).get(j)).multiply(cl_es.get(j)).add(rmu.get(i).get(j)).mod(this.n));
                }
            }
            if(wdote.size()!=0){
                this.vs.add(wdote);
                this.us.add(Bdotfdote);
            }
        }

        // calculate eta1-3
        this.eta1 = VectorB.from(rbigt, this.n).innerProd(VectorB.from(cl_es, this.n)).add(rbigs).mod(this.n);

        List<BigInteger> alpha_c_1 = new LinkedList<>();
        for(int i = 0; i < B.intValue(); i++){
            alpha_c_1.add(alpha.add(BigInteger.valueOf(i)).modInverse(this.n));
        }
        System.out.println("alpha_c_1: " + alpha_c_1);

        BigInteger mc_alpha_c = VectorB.from(mc1, this.n).innerProd(VectorB.from(alpha_c_1, this.n));
        BigInteger rbigv_sum = VectorB.from(rbigv, this.n).innerProd(VectorB.from(einverse, this.n));
        BigInteger rbigm_sum = VectorB.from(rbigm, this.n).innerProd(VectorB.from(alpha_c_1, this.n));
        this.eta2 = rbigm_sum.add(rbigv_sum).mod(this.n);

        this.eta3 = VectorB.from(romega, this.n).innerProd(VectorB.from(cl_es, this.n)).add(rbigr).mod(this.n);

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
                this.cYs.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cyStr);

        String ctsStr = "[" + String.join(",",
                this.ctk.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(ctsStr);

        String cwsStr = "[" + String.join(",",
                this.cws.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cwsStr);

        String cmsStr = "[" + String.join(",",
                this.cms.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cmsStr);

        String cvsStr = "[" + String.join(",",
                this.cvk.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cvsStr);

        String bigSStr = BigIntegerUtils.toString(this.cS.getCoordList());
        total1.add(bigSStr);

        String bigRStr = BigIntegerUtils.toString(this.cR.getCoordList());
        total1.add(bigRStr);

        total1.add("\"" + this.eta1 + "\"");

        total1.add("\"" + this.eta2 + "\"");

        total1.add("\"" + this.eta3 + "\"");

//        String vStr = "[" + String.join(",", this.vs.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
//                + "]";
        total1.add("[");
        for(int i = 0; i < this.vs.size(); i++){
            String vStr = "[" + String.join(",", this.vs.get(i).stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                    + "]";
            total1.add(vStr);
        }
        total1.add("]");

//        total1.add(vStr);

//        String uStr = "[" + String.join(",", this.us.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
//                + "]";
//
//        total1.add(uStr);
        total1.add("[");
        for(int i = 0; i < this.vs.size(); i++){
            String uStr = "[" + String.join(",", this.us.get(i).stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                    + "]";
            total1.add(uStr);
        }
        total1.add("]");


        System.out.println("str1:" + String.join(",", total1));

        String gsStr = "[" + String.join(",", this.gs.getList().stream()
                .map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList())) + "]";
        System.out.println("gsStr:" + String.join(",", gsStr));
    }

    @Override
    public boolean verify() {
        // prepare parameters
        BigInteger gamma = HashUtils.hash(this.cYs).mod(this.n);
        List<BigInteger> udotvs = new LinkedList<>();
        BigInteger e2s = BigInteger.ZERO;

        List<BigInteger> Bs = Lists.newLinkedList();
        for(int i = 0; i < nbits; i++){
            BigInteger value = this.B.pow(i);
            Bs.add(value);
        }
        List<List<BigInteger>> Bk = Lists.newLinkedList();
        for (int i = 0; i < this.L; i++) {
            Bk.add(new LinkedList<>());
        }
        int LK = this.L * this.K;
        if(this.TT == 1){
            for (int i = 0; i < LK; i++) {
                int idx = i / this.K;
                if (i < this.nbits) {
                    Bk.get(idx).add(Bs.get(i));
                }
            else{
                    Bk.get(idx).add(BigInteger.ZERO);
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
                    BigInteger twopowers = gamma.modPow(BigInteger.valueOf(tt), this.n).multiply(Bs.get(mod_agg)).mod(this.n);
                    Bk.get(idx).add(twopowers);
                } else {
                    Bk.get(idx).add(BigInteger.ZERO);
                }
            }
        }

        long s = System.nanoTime();
        // calculate random challenge
        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cws);
        cs.addAll(this.cms);
        cs.addAll(this.cvk);

        BigInteger alpha = HashUtils.hash(cs).mod(this.n);

        cs.addAll(this.ctk);

        List<BigInteger> cl_es = Lists.newLinkedList();
        BigInteger beta = HashUtils.hash(cs).mod(this.n);
        cl_es.add(beta);
        for(int i = 0; i < this.K-1; i ++){
            BigInteger betaX = beta.modPow(BigInteger.valueOf(i+2),this.n);
            cl_es.add(betaX);
        }

        List<BigInteger> einverse = new LinkedList<>();
        for(int i = 0; i < this.K; i++){
            e2s = e2s.add(cl_es.get(i).pow(2)).mod(this.n);
            einverse.add(cl_es.get(i).modInverse(this.n));
        }

        // calculate v`, u`, u*v`
        List<List<BigInteger>> muprime_ks = new LinkedList<>();

        for(int i = 0; i < this.L; i++){
            List<BigInteger> muprime_k1 = new LinkedList<>();
            BigInteger udotv = BigInteger.ZERO;
            for (int j = 0; j < this.K; j++){
                BigInteger vprime;
                BigInteger muprime_k;
                if (!Bk.get(i).get(j).equals(BigInteger.ZERO)) {
                    vprime = Bk.get(i).get(j).multiply(alpha).multiply(cl_es.get(j)).add(this.vs.get(i).get(j)).mod(this.n);
                    udotv = udotv.add(this.us.get(i).get(j).multiply(vprime).mod(this.n));
                    muprime_k = Bk.get(i).get(j).multiply(this.us.get(i).get(j)).multiply(einverse.get(j)).mod(this.n);
                    muprime_k1.add(muprime_k);
                }else{
                    vprime = cl_es.get(j);
                    udotv = udotv.add(cl_es.get(j).multiply(vprime));
                }

            }
            udotvs.add(udotv);
            if (muprime_k1.size()!=0){muprime_ks.add(muprime_k1);}

        }


        // check equation 1;
        Commitment eqn1_1 = this.gs.mulBAndSum(VectorB.from(udotvs, this.n)).add(this.commiter.mulH(this.eta1));
        Commitment eqn1_2 = this.commiter.getIdentity();
        Commitment Hj = this.commiter.getIdentity();
        for (int i = 0; i < this.L; i++){
            Hj = Hj.add(this.gs.getList().get(i));
        }

        eqn1_2 = eqn1_2.add(Hj.mul(e2s));
        for (int i = 0; i < this.K; i++){
            eqn1_2 = eqn1_2.add(this.ctk.get(i).mul(cl_es.get(i)));
        }
        eqn1_2 = eqn1_2.add(this.cS);
        boolean b1 = eqn1_1.equals(eqn1_2);

        // check equation 2;
        BigInteger muprime_sum = BigInteger.ZERO;
        for (int i = 0; i < muprime_ks.size(); i++){
            BigInteger row_sum = muprime_ks.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n);
            muprime_sum = muprime_sum.add(row_sum);
        }
        muprime_sum = muprime_sum.mod(this.n);
        Commitment eqn2_1 = this.commiter.commitTo(muprime_sum, this.eta2);
        Commitment eqn2_2 = this.commiter.getIdentity();
//        if(this.TT == 1){
            for (int i = 0; i < this.B.intValue(); i++){
                eqn2_2 = eqn2_2.add(this.cms.get(i).mul((alpha.add(BigInteger.valueOf(i))).modInverse(this.n)));
            }
//        }
//        else{
//            for(int j = 0; j < TT; j++){
//                for (int i = 0; i < this.B.intValue(); i++){
//                    eqn2_2 = eqn2_2.add(this.cms_agg.get(j).get(i).mul((alpha.add(BigInteger.valueOf(i))).modInverse(this.n)));
//                }
//            }
//
//        }


        for (int i = 0; i < this.K; i++){
            eqn2_2 = eqn2_2.add(this.cvk.get(i).mul(einverse.get(i)));
        }

        boolean b2 = eqn2_2.equals(eqn2_1);

        // check equation 3;
        BigInteger vsum = BigInteger.ZERO;
        for (int i = 0; i < this.vs.size(); i++){
            vsum = vsum.add(this.vs.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n));
        }
        vsum = vsum.mod(this.n);
        Commitment eqn3_1 = this.commiter.commitTo(vsum, this.eta3);

        Commitment eqn3_2 = this.commiter.getIdentity();
        for (int i = 0; i < this.K; i++){
            eqn3_2 = eqn3_2.add(this.cws.get(i).mul(cl_es.get(i)));
        }
        eqn3_2 = eqn3_2.add(this.cR);
        boolean b3 = eqn3_1.equals(eqn3_2);

        // check equation 4;
//        boolean b4 = this.cy.equals(this.cws.stream()
//                .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));

        boolean b4 = false;
        if(this.TT == 1){
            b4 = this.cYs.get(0).equals(this.cws.stream()
                    .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));
        }else{
            Commitment eqn4_1 = this.commiter.getIdentity();
            for (int i = 0; i < this.TT; i++){
                eqn4_1 = eqn4_1.add(this.cYs.get(i).mul(gamma.modPow(BigInteger.valueOf(i+1), this.n)));
            }
            b4 = eqn4_1.equals(this.cws.stream()
                    .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));
        }

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            vtime += (e - s);
        }

        return b1&b2&b3&b4;
    }

}
