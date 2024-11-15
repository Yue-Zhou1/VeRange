package zkp;

import commitment.Commitment;
import structure.VectorB;
import structure.VectorP;
import utils.BigIntegerUtils;
import utils.HashUtils;
import utils.NTT;

import java.math.BigInteger;
import java.util.LinkedList;
import java.util.List;
import java.util.stream.Collectors;

public class Type4_batch extends PedersenZKP{

    private int nbits;
//    private static BigInteger pp = new BigInteger("21888242871839275222246405745257275088548364400416034343698204186575808495617");
//    private static BigInteger root = new BigInteger("5");

    private int B;
    private BigInteger b;
    private BigInteger eprime;
    private BigInteger g_eval_x;
    private BigInteger g_eval_wx;
    private BigInteger f_eval_x;
    private BigInteger fprime_eval_x;
    private BigInteger p_eval_x;
    private BigInteger x;
    private BigInteger rprime_1;
    private BigInteger rprime_2;
    private BigInteger rprime_3;
    private BigInteger rprime_4;
    private BigInteger align_deg;

    Commitment cm_a;
    Commitment cm_e;
    Commitment cm_aprime;
    Commitment cm_eprime;
    List<Commitment> com_g;
    List<Commitment> com_p;
    List<Commitment> com_q;
    private Commitment W;

    private List<BigInteger> qv = new LinkedList<>();
    private VectorP gs;

    public static int counter = 0;
    public static long ptime = 0;
    public static long vtime = 0;

    public Type4_batch(BigInteger y, int nbits, int B, VectorP gs){
        prove(y, nbits, B, gs);
    }

    public void prove(BigInteger y, int nbits, int B, VectorP gs){
        this.nbits = nbits;
        this.B = B;
        this.gs = gs;

        PolynomialCommitment.setup(2*nbits);
        List<BigInteger> domain = NTT.domain(nbits);

        long start = System.nanoTime();

        BigInteger r_cw = this.commiter.rand();
        this.W = this.commiter.commitTo(y, r_cw);

        BigInteger alpha = commiter.rand();
        BigInteger beta = commiter.rand();

        Polynomial g = Polynomial.computeG(domain, y, alpha, beta, BigInteger.valueOf(B));

        BigInteger a = this.commiter.rand();
        BigInteger e = this.commiter.rand();
        BigInteger r_a = this.commiter.rand();
        BigInteger r_aprime = this.commiter.rand();
        BigInteger r_e = this.commiter.rand();
        BigInteger r_eprime = this.commiter.rand();

        BigInteger g_deg = g.degree();
        // extend the degree of g to B*g_deg-1 to make it align with p[X] below.
        align_deg = g_deg.multiply(BigInteger.valueOf(B)).subtract(BigInteger.ONE);

        List<Integer> align_mn = Polynomial.CalculateMatrix_mn(align_deg);
        List<BigInteger> bs = new LinkedList<>();
        for(int i = 0; i < align_mn.get(1); i++){
            bs.add(commiter.rand());
        }

        com_g = PolynomialCommitment.PolyCommit1(g,align_mn.get(0), align_mn.get(1), align_deg.intValue(), bs);

        cm_a = this.commiter.commitTo(a, r_a);
        cm_e = this.commiter.commitTo(e, r_e);
        cm_aprime = this.commiter.getGs().get(0).mul(a).add(this.commiter.mulH(r_aprime));
        cm_eprime = this.commiter.getGs().get(0).mul(e).add(this.commiter.mulH(r_eprime));

        List<Commitment> cs = new LinkedList<>();
//        cs.add(cm_a);
//        cs.add(cm_e);
//        cs.add(cm_aprime);
//        cs.add(cm_eprime);
        cs.addAll(com_g);
        BigInteger gamma = HashUtils.hash(cs).mod(this.n);
        BigInteger theta = gamma.modPow(BigInteger.TWO, this.n);

        BigInteger gamma_a = gamma.multiply(a).mod(this.n);
        Polynomial poly_gamma_a = new Polynomial(gamma_a, BigInteger.ZERO);
        Polynomial gprime = g.plus(poly_gamma_a);

        Polynomial f = Polynomial.computeF_batch(domain, g, B);
        Polynomial fprime = Polynomial.computeFprime(g, B);
        Polynomial p = Polynomial.computeP(domain, f, fprime, theta);

        b = gprime.evaluate(BigInteger.ONE);
        eprime = a.add(gamma.multiply(e)).mod(this.n);

//        p_deg = p.degree();
//        List<Integer> p_mn = Polynomial.CalculateMatrix_mn(p_deg);

        com_p = PolynomialCommitment.PolyCommit1(p, align_mn.get(0), align_mn.get(1), align_deg.intValue(), bs);

        cs.addAll(com_p);
        x = HashUtils.hash(cs).mod(this.n);
        BigInteger x_omega = x.multiply(domain.get(1)).mod(this.n);

        g_eval_x = g.evaluate(x);
        g_eval_wx = g.evaluate(x_omega);
        f_eval_x = f.evaluate(x);
        fprime_eval_x = fprime.evaluate(x);
        p_eval_x = p.evaluate(x);

        List<Polynomial> rs = Polynomial.computeRs(g_eval_x, g_eval_wx, b, p_eval_x, x, x_omega);

        BigInteger rho = x.modPow(BigInteger.TWO, this.n);

        Polynomial q = Polynomial.computeQ_batch(rs, g, gprime, p, rho, x, x_omega);

        //        extend the degree of g to 2*g_deg-1 to make it align with p[X] below.
//        BigInteger q_deg = q.degree();
//        List<Integer> q_mn = Polynomial.CalculateMatrix_mn(q_deg);
        com_q = PolynomialCommitment.PolyCommit1(q, align_mn.get(0), align_mn.get(1), align_deg.intValue(), bs);

        cs.addAll(com_q);
        BigInteger z = HashUtils.hash(cs).mod(this.n);

        qv = new LinkedList<>();

        BigInteger[][] cuv_g = Polynomial.setupHijs(g, align_mn.get(0), align_mn.get(1), align_deg.intValue(), bs);
        BigInteger[][] cuv_p = Polynomial.setupHijs(p, align_mn.get(0), align_mn.get(1), align_deg.intValue(), bs);
        BigInteger[][] cuv_q = Polynomial.setupHijs(q, align_mn.get(0), align_mn.get(1), align_deg.intValue(), bs);

        BigInteger z_minus_1 = z.subtract(BigInteger.ONE);
        BigInteger z_minus_x = z.subtract(x);
        BigInteger z_minus_wx = z.subtract(x_omega);
        BigInteger rho2 = rho.multiply(rho).mod(this.n);
        for(int i = 0; i < align_mn.get(1)+1;i++){
            BigInteger qv_i = BigInteger.ZERO;
            for(int j = 0; j < align_mn.get(0)+1; j++){
                BigInteger part_g = cuv_g[j][i].multiply(
                        z_minus_1.add(
                                rho.multiply(z_minus_x).multiply(z_minus_wx)
                        )
                ).mod(this.n);
                BigInteger part_p = cuv_p[j][i].multiply(rho2).multiply(z_minus_1).multiply(z_minus_wx).mod(this.n);
                BigInteger part_q = cuv_q[j][i].multiply(z_minus_1).multiply(z_minus_x).multiply(z_minus_wx).mod(this.n);
                BigInteger qv_i_zu = (part_g.add(part_p).subtract(part_q)).multiply(z.modPow(BigInteger.valueOf(j), this.n)).mod(this.n);
                qv_i = qv_i.add(qv_i_zu);
            }
            if(i == 0){
                BigInteger part_rho = rho.multiply(gamma).multiply(a).multiply(z_minus_x).multiply(z_minus_wx).mod(this.n);
                qv.add(qv_i.add(part_rho).mod(this.n));
            }else{
                qv.add(qv_i.mod(this.n));
            }
        }

        rprime_1 = rho.multiply(gamma)
                .multiply(z.subtract(x))
                .multiply(z.subtract(x_omega))
                .multiply(r_aprime).mod(this.n);
        rprime_2 = r_cw.add(gamma.multiply(r_a)).mod(this.n);
        rprime_3 = r_a.add(gamma.multiply(r_e)).mod(this.n);
        rprime_4 = r_aprime.add(gamma.multiply(r_eprime)).mod(this.n);

        long end = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            ptime += (end - start);
        }

        counter++;
//
        System.out.println(align_mn.get(0));
        System.out.println(align_mn.get(1));
//        System.out.println(q.degree());
//        System.out.println(align_deg);

//        string();
        System.out.println("domain paras: --------------------------------");
//        System.out.println(domain.get(1));
//        System.out.println(domain.get(domain.size()-1));
    }

    public void string() {

        List<String> total1 = new LinkedList<>();

        String cyStr = BigIntegerUtils.toString(this.W.getCoordList());
        total1.add(cyStr);

        String cgsStr = "[" + String.join(",",
                this.com_g.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cgsStr);

        String cpsStr = "[" + String.join(",",
                this.com_p.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cpsStr);

        String cqsStr = "[" + String.join(",",
                this.com_q.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cqsStr);

        List<Commitment> rae = new LinkedList<>();
        rae.add(cm_a);
        rae.add(cm_e);
        rae.add(cm_aprime);
        rae.add(cm_eprime);

        String raeStr = "[" + String.join(",",
                rae.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(raeStr);

        List<BigInteger> fieldEle = new LinkedList<>();
        fieldEle.add(this.b);
        fieldEle.add(this.eprime);
        fieldEle.add(this.g_eval_x);
        fieldEle.add(this.g_eval_wx);
        fieldEle.add(this.f_eval_x);
        fieldEle.add(this.fprime_eval_x);
        fieldEle.add(this.p_eval_x);
        fieldEle.add(this.rprime_1);
        fieldEle.add(this.rprime_2);
        fieldEle.add(this.rprime_3);
        fieldEle.add(this.rprime_4);

        String fieldEleStr = "[" + String.join(",", fieldEle.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";

        total1.add(fieldEleStr);

        String qvStr = "[" + String.join(",", this.qv.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";

        total1.add(qvStr);

        System.out.println("str1:" + String.join(",", total1));

        String gsStr = "[" + String.join(",", this.gs.getList().stream()
                .map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList())) + "]";
        System.out.println("gsStr:" + String.join(",", gsStr));
    }

    @Override
    public boolean verify() {
//        System.out.println("Verify begins: ---------------------------------");
        List<BigInteger> domain = NTT.domain(this.nbits);
        long start = System.nanoTime();
        List<Commitment> cs = new LinkedList<>();

//        cs.add(cm_a);
//        cs.add(cm_e);
//        cs.add(cm_aprime);
//        cs.add(cm_eprime);
        cs.addAll(com_g);
        BigInteger gamma = HashUtils.hash(cs).mod(this.n);
        BigInteger theta = gamma.modPow(BigInteger.TWO, this.n);
        cs.addAll(com_p);
        x = HashUtils.hash(cs).mod(this.n);
        BigInteger rho = x.modPow(BigInteger.TWO, this.n);
        cs.addAll(com_q);
        BigInteger z = HashUtils.hash(cs).mod(this.n);

        BigInteger x_omega = x.multiply(domain.get(1)).mod(this.n);
        BigInteger z_minus_1 = z.subtract(BigInteger.ONE);
        BigInteger z_minus_x = z.subtract(x);
        BigInteger z_minus_wx = z.subtract(x_omega);
        BigInteger rho2 = rho.multiply(rho).mod(this.n);

        BigInteger rho_z_x_z_wx = rho.multiply(z_minus_x).multiply(z_minus_wx).mod(this.n);
        BigInteger rho2_z_1_z_wx = rho2.multiply(z_minus_1).multiply(z_minus_wx).mod(this.n);
        BigInteger z_1_z_x_z_wx_negate = z_minus_1.multiply(z_minus_x).multiply(z_minus_wx).mod(this.n).negate();
//        System.out.println("negate value is: " + z_1_z_x_z_wx_negate);

        int m = com_p.size()-1;
        int n = qv.size()-1;
        int eta = align_deg.intValue()-m*n;

        Commitment eqn1_1 = this.gs.mulBAndSum(VectorB.from(qv, this.n));
        eqn1_1 = eqn1_1.add(this.commiter.mulH(rprime_1));
        Commitment eqn1_2 = commiter.getIdentity();

        for(int i = 0; i <= m; i++){
            BigInteger z_u = z.modPow(BigInteger.valueOf(i), this.n);
            Commitment h_gpq = com_g.get(i).mul(z_minus_1.add(rho_z_x_z_wx).multiply(z_u).mod(this.n))
                    .add(com_p.get(i).mul(rho2_z_1_z_wx.multiply(z_u).mod(this.n)))
                    .add(com_q.get(i).mul(z_1_z_x_z_wx_negate.multiply(z_u).mod(this.n)));
//            h_gpq = h_gpq.mul(z.modPow(BigInteger.valueOf(i), this.n));
            eqn1_2 = eqn1_2.add(h_gpq);
        }
        eqn1_2 = eqn1_2.add(cm_aprime.mul(rho_z_x_z_wx.multiply(gamma).mod(this.n)));
        boolean b1 = eqn1_1.equals(eqn1_2);

        List<Polynomial> rs = Polynomial.computeRs(g_eval_x, g_eval_wx, b, p_eval_x, x, x_omega);
        BigInteger eqn2_2_part1 = rs.get(0).evaluate(z).multiply(z_minus_1).mod(this.n);
        BigInteger eqn2_2_part2 = rs.get(1).evaluate(z).multiply(rho_z_x_z_wx).mod(this.n);
        BigInteger eqn2_2_part3 = rs.get(2).evaluate(z).multiply(rho2_z_1_z_wx).mod(this.n);
        BigInteger eqn2_2 = eqn2_2_part1.add(eqn2_2_part2).add(eqn2_2_part3).mod(this.n);

        BigInteger eqn2_1 = qv.get(0);
        for(int i = 1; i < n+1; i++){
            BigInteger exp = BigInteger.valueOf((i-1)*m + eta);
            eqn2_1 = eqn2_1.add(qv.get(i).multiply(z.modPow(exp, this.n))).mod(this.n);
        }

        eqn2_1 = eqn2_1.mod(this.n);
        boolean b2 = eqn2_1.equals(eqn2_2);

        BigInteger gx_Bgwx = g_eval_x.subtract(BigInteger.valueOf(B).multiply(g_eval_wx));
        BigInteger eqn3_2 = gx_Bgwx;
        BigInteger gx = g_eval_x;
        BigInteger eqn4_2 = gx;
        for(int i = 1; i < B; i++){
            eqn3_2 = eqn3_2.multiply(BigInteger.valueOf(i).subtract(gx_Bgwx)).mod(this.n);
            eqn4_2 = eqn4_2.multiply(BigInteger.valueOf(i).subtract(gx)).mod(this.n);
        }
        eqn3_2 = eqn3_2.mod(this.n);
        eqn4_2 = eqn4_2.mod(this.n);
        boolean b3 = f_eval_x.equals(eqn3_2);
        boolean b4 = fprime_eval_x.equals(eqn4_2);

        BigInteger omega_N_1 = domain.get(domain.size()-1);
        BigInteger eqn5_2_part1 = f_eval_x.multiply(x.subtract(omega_N_1)).multiply(x.modPow(BigInteger.valueOf(nbits), this.n).subtract(BigInteger.ONE).modInverse(this.n));
        BigInteger eqn5_2_part2 = theta.multiply(fprime_eval_x).multiply(x.subtract(omega_N_1).modInverse(this.n));

        BigInteger eqn5_2 = eqn5_2_part1.add(eqn5_2_part2).mod(this.n);
        boolean b5 = p_eval_x.equals(eqn5_2);

        Commitment eqn6_1 = this.commiter.commitTo(b, rprime_2);
        Commitment eqn6_2 = this.W.add(cm_a.mul(gamma));

        boolean b6 = eqn6_1.equals(eqn6_2);

        Commitment eqn7_1 = this.commiter.commitTo(eprime, rprime_3);
        Commitment eqn7_2 = cm_a.add(cm_e.mul(gamma));

        boolean b7 = eqn7_1.equals(eqn7_2);

        Commitment eqn8_1 = this.commiter.getGs().get(0).mul(eprime)
                .add(this.commiter.mulH(rprime_4));
        Commitment eqn8_2 = cm_aprime.add(cm_eprime.mul(gamma));

        boolean b8 = eqn8_1.equals(eqn8_2);

        long end = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            vtime += (end - start);
        }

//        System.out.println("b1 is: " + b1);
//        System.out.println("b2 is: " + b2);
//        System.out.println("b3 is: " + b3);
//        System.out.println("b4 is: " + b4);
//        System.out.println("b5 is: " + b5);
//        System.out.println("b6 is: " + b6);
//        System.out.println("b7 is: " + b7);
//        System.out.println("b8 is: " + b8);
//        System.out.println(x.modPow(BigInteger.valueOf(nbits), this.n));
//        System.out.println(x.modPow(BigInteger.valueOf(nbits), this.n).subtract(BigInteger.ONE).modInverse(this.n));
//        System.out.println(x.subtract(omega_N_1).modInverse(this.n));
//
//        System.out.println("betas: -----------------------");
//        System.out.println(x);
//        System.out.println(rho);
//
//        System.out.println(rs.get(0).evaluate(z));
//        System.out.println(rs.get(1).evaluate(z));
//        System.out.println(rs.get(2).evaluate(z));
//
//        System.out.println(eqn2_1);
//        System.out.println(eqn2_2);
//        System.out.println(eta);
//        System.out.println(m);
//        System.out.println(n);
//        System.out.println(eqn1_1.getCoordList());
//        System.out.println(eqn1_2.getCoordList());
//        System.out.println(m);
//        System.out.println(n);
//        System.out.println(align_deg);

        return b1 && b2 && b3 && b4 && b5 && b6 && b7 && b8;
    }


}
