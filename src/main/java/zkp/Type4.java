package zkp;

import commitment.Commitment;
import structure.VectorP;
import utils.HashUtils;
import utils.NTT;

import java.math.BigInteger;
import java.util.LinkedList;
import java.util.List;

public class Type4 extends PedersenZKP{

    private int nbits;
    private static BigInteger pp = new BigInteger("21888242871839275222246405745257275088548364400416034343698204186575808495617");
    private static BigInteger root = new BigInteger("5");

    private BigInteger g_eval_rho;
    private BigInteger g_eval_rho_omega;
    private BigInteger w_cap_eval_rho;
    private BigInteger eval_rho_batch;

    private BigInteger w_cap_deg;
    private List<Integer> w_cap_mn;

    List<Commitment> com_f;
    List<Commitment> com_g;
    List<Commitment> com_q;
    private Commitment W;
    private List<BigInteger> pi_g_rho = new LinkedList<>();
    private List<BigInteger> pi_g_rhoomega = new LinkedList<>();
    private List<BigInteger> pi_w_cap_rho = new LinkedList<>();
    private List<BigInteger> pi_rho_batch = new LinkedList<>();

    public static int counter = 0;
    public static long ptime = 0;
    public static long vtime = 0;

    public Type4(BigInteger y, int nbits, int B, VectorP gs){
//        prove(y, nbits, B, gs);
        prove_batch(y, nbits, B, gs);
    }

    @Override
    public boolean verify() {
        System.out.println("Batch verify multiple polynomials at the same point rho begin: --------------------------");
        List<Commitment> cs = new LinkedList<>();
        cs.add(this.W);
        BigInteger tau = HashUtils.hash(cs).mod(pp);

        cs.addAll(com_f);
        cs.addAll(com_q);
        BigInteger rho = HashUtils.hash(cs).mod(this.n);

        List<BigInteger> domain = NTT.domain(this.nbits);
        BigInteger rho_omega = rho.multiply(domain.get(1));

        BigInteger rho_n_minus_1 = rho.modPow(BigInteger.valueOf(domain.size()), pp).subtract(BigInteger.ONE);
        BigInteger rho_minus_1 = rho.subtract(BigInteger.ONE);
        BigInteger w_n_minus_1 = domain.get(domain.size()-1);

        BigInteger w1_eval = g_eval_rho.multiply(rho_n_minus_1).multiply(rho.subtract(BigInteger.ONE).modInverse(pp)).mod(pp);
        BigInteger w2_eval = g_eval_rho.multiply(BigInteger.ONE.subtract(g_eval_rho)).multiply(rho_n_minus_1).multiply(rho.subtract(w_n_minus_1).modInverse(pp)).mod(pp);
        BigInteger part_a = g_eval_rho.subtract(BigInteger.TWO.multiply(g_eval_rho_omega));
        BigInteger part_b = BigInteger.ONE.subtract(part_a);
        BigInteger part_c = rho.subtract(w_n_minus_1);
        BigInteger w3_eval = part_a.multiply(part_b).multiply(part_c).mod(pp);
        BigInteger w_at_rho = w1_eval
                                .add(tau.multiply(w2_eval))
                                .add(tau.multiply(tau).multiply(w3_eval))
                                .subtract(w_cap_eval_rho).mod(pp);

        Boolean b1 = w_at_rho.equals(BigInteger.ZERO);

        BigInteger rho_1 = rho_n_minus_1.multiply(rho_minus_1.modInverse(pp)).mod(pp);
        List<Commitment> com_w_cap = new LinkedList<>();
        for(int i = 0; i < com_f.size(); i++){
            com_w_cap.add(com_f.get(i).mul(rho_1).add(com_q.get(i).mul(rho_n_minus_1)));
        }

        boolean g_rhoomega = PolynomialCommitment.PolyVerify(com_g, rho_omega, g_eval_rho_omega, pi_g_rhoomega,w_cap_deg.intValue());

        System.out.println("w_rho is 0?: " + b1);

        System.out.println("g_rhoomega: " + g_rhoomega);

        List<List<Commitment>> poly_coms = new LinkedList<>();
        poly_coms.add(com_g);
        poly_coms.add(com_w_cap);
        boolean batch_verify = PolynomialCommitment.PolyVerifyBatch(poly_coms, rho, tau, eval_rho_batch, pi_rho_batch,w_cap_deg.intValue());
        System.out.println("batch_verify: " + batch_verify);
        return b1 && batch_verify && g_rhoomega;
    }

    public boolean verify_batch(){

        return true;
    }

    public void prove(BigInteger y, int nbits, int B, VectorP gs){
        this.nbits = nbits;

        PolynomialCommitment.setup(2*nbits);
        List<BigInteger> domain = NTT.domain(nbits);
        List<BigInteger> domain_2n = NTT.domain(2*nbits);

        BigInteger z = y;
        BigInteger rcw = this.commiter.rand();
        this.W = this.commiter.commitTo(y, rcw);

        BigInteger r = commiter.rand();
        BigInteger alpha = commiter.rand();
        BigInteger beta = commiter.rand();

        Polynomial f = Polynomial.computeF(domain, z, r);
        Polynomial g = Polynomial.computeG(domain, z, alpha, beta, BigInteger.valueOf(B));
        Polynomial w1 = Polynomial.computeW1(domain, g, f);
        Polynomial w2 = Polynomial.computeW2(domain, g, BigInteger.valueOf(B));
        Polynomial w3 = Polynomial.computeW3(domain, domain_2n, g);

        List<Commitment> cs = new LinkedList<>();
        cs.add(this.W);
        BigInteger tau = HashUtils.hash(cs).mod(pp);

        Polynomial qq = Polynomial.computeQ(domain, w1, w2, w3, tau);
        BigInteger q_deg = qq.degree();
        List<Integer> q_mn = Polynomial.CalculateMatrix_mn(q_deg);
        List<BigInteger> bs_q = new LinkedList<>();
        for(int i = 0; i < q_mn.get(0); i++){
            bs_q.add(commiter.rand());
        }
        com_q = PolynomialCommitment.PolyCommit1(qq, q_mn.get(0), q_mn.get(1), q_deg.intValue(), bs_q);

        List<Integer> f_extend_mn = Polynomial.CalculateMatrix_mn(q_deg);
        List<BigInteger> bs_f = new LinkedList<>();
        for(int i = 0; i < f_extend_mn.get(0); i++){
            bs_f.add(commiter.rand());
        }
        com_f = PolynomialCommitment.PolyCommit1(f, f_extend_mn.get(0), f_extend_mn.get(1), q_deg.intValue(), bs_f);

        cs.addAll(com_f);
        cs.addAll(com_q);
        BigInteger rho = HashUtils.hash(cs).mod(this.n);

        Polynomial w_cap = Polynomial.computeWCap(domain, f, qq, rho);
        BigInteger rho_n_minus_1 = rho.modPow(BigInteger.valueOf(domain.size()),pp).subtract(BigInteger.ONE);
        BigInteger rho_n_minus_1_by_rho_minus_1 = rho_n_minus_1.multiply(rho.subtract(BigInteger.ONE).modInverse(pp)).mod(pp);
        w_cap_deg = w_cap.degree();
        w_cap_mn = Polynomial.CalculateMatrix_mn(w_cap_deg);
        List<BigInteger> bs_w = new LinkedList<>();
        List<BigInteger> bs_g = new LinkedList<>();
        for(int i = 0; i < w_cap_mn.get(0); i++){
            bs_w.add(bs_f.get(i).multiply(rho_n_minus_1_by_rho_minus_1).add(bs_q.get(i).multiply(rho_n_minus_1)).mod(this.n));
            bs_g.add(commiter.rand());
        }
        com_g = PolynomialCommitment.PolyCommit1(g, q_mn.get(0), q_mn.get(1), q_deg.intValue(), bs_g);

        BigInteger rho_omega = rho.multiply(domain.get(1));

        g_eval_rho = g.evaluate(rho);
        g_eval_rho_omega = g.evaluate(rho_omega);
        w_cap_eval_rho = w_cap.evaluate(rho);

        pi_g_rho.addAll(PolynomialCommitment.PolyEval1(g, rho, w_cap_mn.get(0), w_cap_mn.get(1), w_cap_deg.intValue(), bs_g));
        pi_g_rhoomega.addAll(PolynomialCommitment.PolyEval1(g, rho_omega, w_cap_mn.get(0), w_cap_mn.get(1), w_cap_deg.intValue(), bs_g));
        pi_w_cap_rho.addAll(PolynomialCommitment.PolyEval1(w_cap, rho, w_cap_mn.get(0), w_cap_mn.get(1), w_cap_deg.intValue(), bs_w));

        List<Polynomial> poly_batch = new LinkedList<>();
        List<List<BigInteger>> bs = new LinkedList<>();
        poly_batch.add(g);
        poly_batch.add(w_cap);
        bs.add(bs_g);
        bs.add(bs_w);

        eval_rho_batch = tau.multiply(g_eval_rho).add(tau.modPow(BigInteger.TWO, this.n).multiply(w_cap_eval_rho)).mod(this.n);
        pi_rho_batch.addAll(PolynomialCommitment.PolyEvalBatch(poly_batch, rho, tau, w_cap_mn.get(0), w_cap_mn.get(1), w_cap_deg.intValue(), bs));

    }

    public void prove_batch(BigInteger y, int nbits, int B, VectorP gs){
        this.nbits = nbits;

        PolynomialCommitment.setup(2*nbits);
        List<BigInteger> domain = NTT.domain(nbits);
        List<BigInteger> domain_2n = NTT.domain(2*nbits);

        BigInteger z = y;
        BigInteger rcw = this.commiter.rand();
        this.W = this.commiter.commitTo(y, rcw);

        BigInteger r = commiter.rand();
        BigInteger alpha = commiter.rand();
        BigInteger beta = commiter.rand();

        Polynomial f = Polynomial.computeF(domain, z, r);
        Polynomial g = Polynomial.computeG(domain, z, alpha, beta, BigInteger.valueOf(B));
        Polynomial w1 = Polynomial.computeW1(domain, g, f);
        Polynomial w2 = Polynomial.computeW2(domain, g, BigInteger.valueOf(B));
        Polynomial w3 = Polynomial.computeW3(domain, domain_2n, g);

        List<Commitment> cs = new LinkedList<>();
        cs.add(this.W);
        BigInteger tau = HashUtils.hash(cs).mod(pp);

        Polynomial qq = Polynomial.computeQ(domain, w1, w2, w3, tau);

//        cs.addAll(com_f);
//        cs.addAll(com_q);
//        BigInteger rho = HashUtils.hash(cs).mod(this.n);
        BigInteger rho = BigInteger.TWO;

        Polynomial w_cap = Polynomial.computeWCap(domain, f, qq, rho);
        BigInteger rho_n_minus_1 = rho.modPow(BigInteger.valueOf(domain.size()),pp).subtract(BigInteger.ONE);
        BigInteger rho_n_minus_1_by_rho_minus_1 = rho_n_minus_1.multiply(rho.subtract(BigInteger.ONE).modInverse(pp)).mod(pp);
        w_cap_deg = w_cap.degree();
        w_cap_mn = Polynomial.CalculateMatrix_mn(w_cap_deg);

        BigInteger rho_omega = rho.multiply(domain.get(1));
        g_eval_rho = g.evaluate(rho);
        g_eval_rho_omega = g.evaluate(rho_omega);
        w_cap_eval_rho = w_cap.evaluate(rho);


        List<BigInteger> inputs_Ig = new LinkedList<>();
        List<BigInteger> outputs_Ig = new LinkedList<>();
        Polynomial poly_rg = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
        inputs_Ig.add(rho);
        inputs_Ig.add(rho_omega);
        outputs_Ig.add(g_eval_rho);
        outputs_Ig.add(g_eval_rho_omega);


        List<BigInteger> inputs_Iw = new LinkedList<>();
        List<BigInteger> outputs_Iw = new LinkedList<>();
        Polynomial poly_rw = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
        inputs_Iw.add(rho);
        outputs_Iw.add(w_cap_eval_rho);

        for(int i = 0; i < inputs_Ig.size(); i++){
            Polynomial lagx = Polynomial.LagrangePolynomial(inputs_Ig, i);
            Polynomial outputs_Igx = new Polynomial(outputs_Ig.get(i), BigInteger.ZERO);
            Polynomial r_gx = outputs_Igx.times(lagx);
            poly_rg = poly_rg.plus(r_gx);
        }

        for(int i = 0; i < inputs_Iw.size(); i++){
            Polynomial lagx = Polynomial.LagrangePolynomial(inputs_Iw, i);
            Polynomial outputs_Iwx = new Polynomial(outputs_Iw.get(i), BigInteger.ZERO);
            Polynomial r_wx = outputs_Iwx.times(lagx);
            poly_rw = poly_rw.plus(r_wx);
        }

        System.out.println(poly_rg.evaluate(rho));
        System.out.println(g_eval_rho);
        System.out.println("---------------------------------------");
        System.out.println(poly_rg.evaluate(rho_omega));
        System.out.println(g_eval_rho_omega);
        System.out.println("---------------------------------------");
        System.out.println(poly_rw.evaluate(rho));
        System.out.println(w_cap_eval_rho);

        System.out.println(poly_rw);

    }
}
