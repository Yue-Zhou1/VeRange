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

public class Type3 extends PedersenZKP{
    private static final long serialVersionUID = 1L;

    private final List<Commitment> cD = new LinkedList<>();
    private final List<Commitment> cW = new LinkedList<>();
    private final List<Commitment> cmS = new LinkedList<>();
    private final List<Commitment> cmB = new LinkedList<>();
    private final List<Commitment> Ws = new LinkedList<>();
    private final Commitment cR1;
    private final Commitment cR2;

    private final List<BigInteger> pi_Bx = new LinkedList<>();
    private final List<BigInteger> pi_Sx = new LinkedList<>();

    private final BigInteger Bx_deg;
    private final BigInteger Sx_deg;
    private final BigInteger yB;
    private final BigInteger yS;
    private final BigInteger eta1;
    private final BigInteger eta2;

    private final List<BigInteger> zs = new LinkedList<>();
    private final List<BigInteger> rds = new LinkedList<>();
    private final List<BigInteger> rDs = new LinkedList<>();
    private final List<BigInteger> rws = new LinkedList<>();
    private final List<BigInteger> djx = new LinkedList<>();
    private final List<BigInteger> Bjx = new LinkedList<>();
    private final List<Polynomial> BjX = new LinkedList<>();

    private final VectorP gs;
    private final int nbits;
    private final int U;
    private final int V;
    private final BigInteger B;
    private final BigInteger X;
    private int m_row_x_m;
    private int m_row_x_n;
    private int m_row_b_m;
    private int m_row_b_n;

    List<BigInteger> lkx_eval_p = new LinkedList<>();
    BigInteger l0_value_p;

    // aggregated parameters
    private final int TT;

    public static int counter = 0;
    public static long ptime = 0;
    public static long vtime = 0;

    public Type3(List<BigInteger> y, int nbits, int b_ary, VectorP gs, int TT, boolean aggre){
        this.gs = gs;
        this.nbits = nbits;
        this.U = this.gs.getList().size();
        this.V = this.U;
        this.B = BigInteger.valueOf(b_ary);
        this.TT = TT;

        List<BigInteger> Bs = Lists.newLinkedList();
        for(int i = 0; i < nbits; i++){
            BigInteger value = this.B.pow(i);
            Bs.add(value);
        }

        List<BigInteger> rcw = new LinkedList<>();
        if(aggre){
            for(int i = 0; i < y.size(); i++){
                BigInteger rcw_i = this.commiter.rand();
                rcw.add(rcw_i);
                this.Ws.add(this.commiter.commitTo(y.get(i), rcw_i));
            }
        }else{
            BigInteger rcw_i = this.commiter.rand();
            rcw.add(rcw_i);
            this.Ws.add(this.commiter.commitTo(y.get(0), rcw_i));
        }
        // aggregated parameter
        BigInteger gamma = HashUtils.hash(this.Ws).mod(this.n);
//        BigInteger gamma = BigInteger.ONE;
//        BigInteger rcw = this.commiter.rand();
//        this.W = this.commiter.commitTo(y, rcw);

        for(int i = 0; i < this.V; i++){
            zs.add(this.commiter.rand());
            rws.add(this.commiter.rand());
            rDs.add(this.commiter.rand());
        }
        zs.add(this.commiter.rand());
        X = this.commiter.rand();
        System.out.println("X: " + X);

        BigInteger rw_sum = BigInteger.ZERO;
        for(int i = 0; i < this.U; i++){
            rds.add(this.commiter.rand());
        }

        rws.remove(V-1);
//        BigInteger gamma_WK = rcw.subtract(VectorB.from(rws, this.n).sum()).mod(this.n);
//        rws.add(gamma_WK);

        BigInteger gamma_WK;
        if(aggre){
            BigInteger gamma_rcw_sum = BigInteger.ZERO;
            for(int i = 0; i < this.TT; i++){
                gamma_rcw_sum = gamma_rcw_sum.add(gamma.modPow(BigInteger.valueOf(i+1), this.n).multiply(rcw.get(i)));
            }
            gamma_rcw_sum = gamma_rcw_sum.mod(this.n);
            gamma_WK = gamma_rcw_sum.subtract(VectorB.from(rws, this.n).sum()).mod(this.n);
        }else{
            gamma_WK = rcw.get(0).subtract(VectorB.from(rws, this.n).sum()).mod(this.n);
        }

        rws.add(gamma_WK);

        BigInteger sU = this.commiter.rand();
        BigInteger r1 = this.commiter.rand();
        BigInteger r2 = this.commiter.rand();

        List<List<BigInteger>> ds = Lists.newLinkedList();
        List<List<BigInteger>> bs = Lists.newLinkedList();
        List<List<BigInteger>> bjk = Lists.newLinkedList();
        List<List<BigInteger>> bbs = Lists.newLinkedList();

        List<List<BigInteger>> dds = Lists.newLinkedList();
        for (int i = 0; i < this.V; i++) {
            ds.add(new LinkedList<>());
            bs.add(new LinkedList<>());
            bbs.add(new LinkedList<>());
            dds.add(new LinkedList<>());
        }

        for (int i = 0; i < this.U; i++) {
            bjk.add(new LinkedList<>());
        }
        PolynomialCommitment.setup(V+1);
        long s = System.nanoTime();

        if(!aggre){
            List<BigInteger> narys = BigIntegerUtils.decomposeToNary(y.get(0), B);
            for (int i = narys.size(); i < this.U*(this.V); i++){
                narys.add(BigInteger.ZERO);
            }
            for (int i = 0; i < this.U * (this.V); i++) {
                int idx = i / this.V;
                int mod = i % this.V;
                if (i < nbits) {
                    ds.get(mod).add(narys.get(i).multiply(Bs.get(i)));
                    bs.get(idx).add(narys.get(i));
                    bjk.get(idx).add(Bs.get(i));
                    dds.get(mod).add(narys.get(i));
                    bbs.get(mod).add(Bs.get(i));
                }
                else {
                    ds.get(mod).add(BigInteger.ZERO);
                    bjk.get(idx).add(BigInteger.ZERO);
                    bs.get(idx).add(BigInteger.ZERO);
                    bbs.get(mod).add(BigInteger.ZERO);
                    dds.get(mod).add(BigInteger.ZERO);
                }
            }

        }else{
            int tt = 0;
//            int actual_mod = (this.V%TT==0?this.V/TT:(this.V/TT+1))*this.U;
            int actual_mod = this.V/TT*this.U;
            List<List<BigInteger>> narys_T = new LinkedList<>();
            List<BigInteger> narys = new LinkedList<>();
            for (int i = 0; i < this.U * (this.V); i++) {
                int idx = i / (this.V);
                int mod = i % (this.V);
                int mod_agg = i % actual_mod;
                if(mod_agg == 0){
                    if(tt<TT){
                        narys = BigIntegerUtils.decomposeToNary(y.get(tt), B);
                        for (int j = narys.size(); j < (this.U * this.V)/TT; j++){
                            narys.add(BigInteger.ZERO);
                        }
                        narys_T.add(narys);
                        tt+=1;
                    }
                }
                if (i < (this.U * this.V)/TT+(tt-1)*actual_mod && i >= (tt-1)*actual_mod) {
                    if(TT == 4 && i >= (this.U*(this.V-1))){
                        ds.get(mod).add(BigInteger.ZERO);
                        bjk.get(idx).add(BigInteger.ZERO);
                        bs.get(idx).add(BigInteger.ZERO);
                        bbs.get(mod).add(BigInteger.ZERO);
                        dds.get(mod).add(BigInteger.ZERO);
                    }else{
                        BigInteger bpowers = gamma.modPow(BigInteger.valueOf(tt), this.n).multiply(Bs.get(mod_agg)).mod(this.n);
                        BigInteger w = narys.get(mod_agg).multiply(bpowers);

                        ds.get(mod).add(w);
                        bs.get(idx).add(narys.get(mod_agg));
                        bjk.get(idx).add(bpowers);
                        dds.get(mod).add(narys.get(mod_agg));
                        bbs.get(mod).add(bpowers);
                    }
                }else{
                    ds.get(mod).add(BigInteger.ZERO);
                    bjk.get(idx).add(BigInteger.ZERO);
                    bs.get(idx).add(BigInteger.ZERO);
                    bbs.get(mod).add(BigInteger.ZERO);
                    dds.get(mod).add(BigInteger.ZERO);
                }
            }
        }

        // set up Lo[X]

        Polynomial l0 = Polynomial.l0X(zs);

        List<Polynomial> dX = new LinkedList<>();
        List<Polynomial> Bj_hat_X = new LinkedList<>();
        List<BigInteger> lkx_eval = new LinkedList<>();

        for (int j = 0; j < this.U; j++){
            Polynomial duv_lx = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
            Polynomial bjk_lk = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
            for(int i = 0; i < this.V; i++){
                Polynomial lux = Polynomial.LagrangePolynomial(zs, i+1);
                lkx_eval.add(lux.evaluate(X));
                Polynomial duv_lx1 = lux.times(new Polynomial(bs.get(j).get(i), BigInteger.ZERO));
                duv_lx = duv_lx.plus(duv_lx1);

                Polynomial bjk_cons = new Polynomial(bjk.get(j).get(i), BigInteger.ZERO);
                bjk_lk = bjk_lk.plus(bjk_cons.times(lux));
            }
            Polynomial dvX = l0.times(new Polynomial(rds.get(j), BigInteger.ZERO)).plus(duv_lx);
            dX.add(dvX);
            Bj_hat_X.add(bjk_lk);
        }

        Polynomial BvX;
        for (int j = 0; j < this.U; j++){
            Polynomial BvX_term1 = dX.get(j);
            for(int i = 1; i < this.B.intValue(); i++){
                Polynomial poly_cons = new Polynomial(BigInteger.valueOf(i),BigInteger.ZERO);
                BvX_term1 = BvX_term1.times(dX.get(j).minus(poly_cons));
            }
//            BvX = BvX_term1.divides(new Polynomial(l0.evaluate(X), BigInteger.ZERO));
            BvX = BvX_term1.longdivide(l0)[0];
            BjX.add(BvX);
        }

        List<BigInteger> wk = new LinkedList<>();
        for(int i = 0; i < this.V; i++){
            cD.add(this.gs.mulBAndSum(VectorB.from(dds.get(i), this.n)).add(this.commiter.mulH(rDs.get(i))));
            BigInteger wk1 = ds.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n);
            cW.add(this.commiter.commitTo(wk1, rws.get(i)));
            wk.add(wk1);
        }

        Polynomial dvbv = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
        for(int i = 0; i < this.U; i++){
            dvbv = dvbv.plus(dX.get(i).times(Bj_hat_X.get(i)));
        }
        Polynomial wulu = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
        for(int i = 0; i < this.V; i++){
            Polynomial lux = Polynomial.LagrangePolynomial(zs, i+1);
            Polynomial wkx = new Polynomial(wk.get(i), BigInteger.ZERO);
            wulu = wulu.plus(wkx.times(lux));
        }
        Polynomial Sx = new Polynomial(sU, BigInteger.ZERO);
        Polynomial wklk_djbj = wulu.minus(dvbv);
//        Sx = Sx.plus(wklk_djbj.divides(new Polynomial(l0.evaluate(X), BigInteger.ZERO)));
        Sx = Sx.plus(wklk_djbj.longdivide(l0)[0]);

        Sx_deg = Sx.degree();
        m_row_x_m = (int) Math.ceil(Math.sqrt(Sx_deg.intValue()));
        m_row_x_n = m_row_x_m;
        if(Sx_deg.intValue()-m_row_x_m*m_row_x_n<0){
            m_row_x_n--;
            if(Sx_deg.intValue()-m_row_x_m*m_row_x_n<0){
                m_row_x_m--;
            }
        }

        cmS.addAll(PolynomialCommitment.PolyCommit(Sx, m_row_x_m, m_row_x_n));

        cR1 = this.gs.mulBAndSum(VectorB.from(rds, this.n)).add(this.commiter.mulH(r1));
        cR2 = this.commiter.commitTo(sU, r2);

        for(int i = 0; i < this.V; i++){
            djx.add(dX.get(i).evaluate(X));
        }

        BigInteger l0_value = l0.evaluate(X);
        for(int i = 0; i < U; i++){
            BigInteger BjkLk = Bj_hat_X.get(i).evaluate(X);
            Bjx.add(BjkLk);
        }

        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cD);
        cs.addAll(this.cW);
        cs.add(cR1);
        cs.add(cR2);

        BigInteger beta = HashUtils.hash(cs).mod(this.n);

        Polynomial Bx = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
        for(int i = 0; i < U; i++){
            Polynomial betaj = new Polynomial(beta.pow(i+1), BigInteger.ZERO);
            Polynomial betaj_BjX = betaj.times(BjX.get(i));
            Bx = Bx.plus(betaj_BjX);
        }

        this.yB = Bx.evaluate(X);

        Bx_deg = Bx.degree();
        m_row_b_m = (int) Math.ceil(Math.sqrt(Bx_deg.intValue()));
        m_row_b_n = m_row_b_m;
        if(Bx_deg.intValue()-m_row_b_m*m_row_b_n<0){
            m_row_b_n--;
            if(Bx_deg.intValue()-m_row_b_m*m_row_b_n<0){
                m_row_b_m--;
            }
        }

        cmB.addAll(PolynomialCommitment.PolyCommit(Bx, m_row_b_m, m_row_b_n));

        this.yS = Sx.evaluate(X);

        pi_Sx.addAll(PolynomialCommitment.PolyEval(Sx, X, m_row_x_m, m_row_x_n));
        pi_Bx.addAll(PolynomialCommitment.PolyEval(Bx, X, m_row_b_m, m_row_b_n));

        BigInteger rDlu = BigInteger.ZERO;
        BigInteger rwlu = BigInteger.ZERO;
        for(int i = 0; i < this.V; i++){
            rDlu = rDlu.add(lkx_eval.get(i).multiply(rDs.get(i)));
            rwlu = rwlu.add(lkx_eval.get(i).multiply(rws.get(i)));
        }
        eta1 = rDlu.add(l0_value.multiply(r1).mod(this.n)).mod(this.n);
        eta2 = rwlu.add(l0_value.multiply(r2).mod(this.n)).mod(this.n);
        l0_value_p = l0_value;

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            ptime += (e - s);
        }

        counter++;
        for (int i = 0; i < this.V; i++) {
            BigInteger lux = Polynomial.LagrangePolynomial(zs, i + 1).evaluate(X).mod(this.n);
            lkx_eval_p.add(lux);
        }

        string();

    }

    public void string() {

        List<String> total1 = new LinkedList<>();

        String cyStr = "[" + String.join(",",
                this.cW.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cyStr);

        String ctsStr = "[" + String.join(",",
                this.cD.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(ctsStr);

        String cwsStr = "[" + String.join(",",
                this.cW.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cwsStr);

        String bigSStr = BigIntegerUtils.toString(this.cR1.getCoordList());
        total1.add(bigSStr);

        String bigRStr = BigIntegerUtils.toString(this.cR2.getCoordList());
        total1.add(bigRStr);


        String cmsStr = "[" + String.join(",",
                this.cmS.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cmsStr);

        String cmbStr = "[" + String.join(",",
                this.cmB.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cmbStr);

        String djxStr = "[" + String.join(",", this.djx.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";

        total1.add(djxStr);

        String pi_sxStr = "[" + String.join(",", this.pi_Sx.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";

        total1.add(pi_sxStr);

        String pi_bxStr = "[" + String.join(",", this.pi_Bx.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";

        total1.add(pi_bxStr);

        total1.add("\"" + this.yS + "\"");

        total1.add("\"" + this.yB + "\"");

        total1.add("\"" + this.eta1 + "\"");

        total1.add("\"" + this.eta2 + "\"");

        total1.add("\"" + l0_value_p + "\"");

        String lkxStr = "[" + String.join(",", this.lkx_eval_p.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";

        total1.add(lkxStr);


        System.out.println("str1:" + String.join(",", total1));

        String gsStr = "[" + String.join(",", this.gs.getList().stream()
                .map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList())) + "]";
        System.out.println("gsStr:" + String.join(",", gsStr));
    }

    @Override
    public boolean verify() {
        BigInteger gamma = HashUtils.hash(this.Ws).mod(this.n);

        // set up L0[x]
        Polynomial l0 = Polynomial.l0X(zs);

        BigInteger l0_value = l0.evaluate(X);

        // set up Lk[x]
        List<BigInteger> lkx_eval = new LinkedList<>();
        for (int i = 0; i < this.V; i++) {
            BigInteger lux = Polynomial.LagrangePolynomial(zs, i + 1).evaluate(X).mod(this.n);
            lkx_eval.add(lux);
        }

        // Compute challenge beta
        long s = System.nanoTime();

        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cD);
        cs.addAll(this.cW);
        cs.add(this.cR1);
        cs.add(this.cR2);
        BigInteger beta = HashUtils.hash(cs).mod(this.n);
        List<BigInteger> betav = new LinkedList<>();
        betav.add(beta);
        for(int i = 1; i < this.V; i++){
            betav.add(beta.pow(i+1).mod(this.n));
        }

        // Check PolyVf of P and U

//        boolean bB = PolynomialCommitment.PolyVerify(cmB, X, yB, pi_Bx,m_row_b_m,m_row_b_n,Bx_deg.intValue());
//        boolean bS = PolynomialCommitment.PolyVerify(cmS, X, yS, pi_Sx,m_row_x_m,m_row_x_n,Sx_deg.intValue());
        System.out.println("bB parameters: ");
        boolean bB = PolynomialCommitment.PolyVerify(cmB, X, yB, pi_Bx,Bx_deg.intValue());
        System.out.println("bS parameters: ");
        boolean bS = PolynomialCommitment.PolyVerify(cmS, X, yS, pi_Sx,Sx_deg.intValue());

        if(bB == false || bS == false){
            return false;
        }

        // check equation 1;

        Commitment eqn1_1 = this.gs.mulBAndSum(VectorB.from(this.djx, this.n)).add(this.commiter.mulH(this.eta1.mod(this.n)));
        Commitment eqn1_2 = this.commiter.getIdentity();

        for (int i = 0; i < this.V; i++){
            eqn1_2 = eqn1_2.add(this.cD.get(i).mul(lkx_eval.get(i)));
        }
        eqn1_2 = eqn1_2.add(cR1.mul(l0_value));

        boolean b1 = eqn1_1.equals(eqn1_2);

        // check equation 2;

        BigInteger Bx = BigInteger.ZERO;
        for(int i = 0; i < U; i++){
            BigInteger term = BigInteger.ONE;
            for(int j = 0; j < B.intValue(); j++){
                term = term.multiply(djx.get(i).subtract(BigInteger.valueOf(j)));
            }
            Bx = Bx.add(betav.get(i).multiply(term).mod(this.n));
        }

        BigInteger eqn2_1 = Bx.mod(this.n);

        BigInteger eqn2_2 = yB.multiply(l0_value).mod(this.n);

        boolean b2 = eqn2_1.equals(eqn2_2);

        // check equation 3;

        BigInteger djBj = VectorB.from(djx, this.n).innerProd(VectorB.from(Bjx, this.n));
        BigInteger G_1_exp = yS.multiply(l0_value).add(djBj).mod(this.n);
        Commitment eqn3_1 = this.commiter.commitTo(G_1_exp, eta2);

        Commitment eqn3_2 = this.commiter.getIdentity();
        eqn3_2 = eqn3_2.add(cR2.mul(l0_value));
        for (int i = 0; i < this.V; i++){
            eqn3_2 = eqn3_2.add(this.cW.get(i).mul(lkx_eval.get(i)));
        }

        boolean b3 = eqn3_1.equals(eqn3_2);

        // check equation 4;
//        Commitment eqn4_1 = this.cW.stream()
//                .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2));
//        Commitment eqn4_2 = this.Ws.get(0);
//
//        boolean b4 = eqn4_1.equals(eqn4_2);

        Commitment eqn4_1 = this.commiter.getIdentity();
        Commitment eqn4_2 = this.cW.stream()
                .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2));
        if(this.TT == 1){
            eqn4_1 = this.Ws.get(0);
        }else{
            for (int i = 0; i < this.TT; i++){
                eqn4_1 = eqn4_1.add(this.Ws.get(i).mul(gamma.modPow(BigInteger.valueOf(i+1), this.n)));
            }
        }
        boolean b4 = eqn4_1.equals(eqn4_2);
        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            vtime += (e - s);
        }
        return b1 && b2 && b3 && b4;
    }
}
