package zkp;

import com.google.common.collect.Lists;
import commitment.Commitment;
import structure.VectorB;
import structure.VectorP;
import utils.BigIntegerUtils;
import utils.HashUtils;

import java.math.BigInteger;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;
import java.util.stream.Collectors;

public class Type2P extends PedersenZKP{
    private List<Commitment> cYs = new LinkedList<>();
    private final Commitment cR;
    private final Commitment cS;
    private final Commitment cU;
    private final List<Commitment> cws = new LinkedList<>();
    private final List<Commitment> cms = new LinkedList<>();
    private final List<Commitment> ctk = new LinkedList<>();
    private final List<Commitment> cfk = new LinkedList<>();
    private final List<Commitment> ctk_kprime = new LinkedList<>();

    private final List<BigInteger> vs = new LinkedList<>();
    private final List<BigInteger> us = new LinkedList<>();

    private BigInteger eta1;
    private final BigInteger eta2;
    private final BigInteger eta3;
    private final BigInteger eta4;

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

    public Type2P(List<BigInteger> y, int nbits, int b_ary, VectorP gs, int K, int L, int TT, boolean aggre){
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
//        BigInteger gamma = HashUtils.hash(this.cYs).mod(this.n);
        BigInteger gamma = BigInteger.ONE;

        // make a list to contain B^0, B^1,..., B^nbits
        List<BigInteger> Bs = Lists.newLinkedList();
        for(int i = 0; i < nbits; i++){
            BigInteger value = this.B.pow(i);
            Bs.add(value);
        }

        // initialize random elements r_mu, r_v
        List<BigInteger> rmu = Lists.newLinkedList();
        List<BigInteger> rv = Lists.newLinkedList();

        for(int i = 0; i < this.L*TT; i++){
            rmu.add(this.commiter.rand());
            rv.add(this.commiter.rand());
        }

        // initialize parameters
        List<BigInteger> romega = Lists.newLinkedList();
        List<BigInteger> rbigt = Lists.newLinkedList();
        List<BigInteger> rbigm = Lists.newLinkedList();
        List<BigInteger> rbigf = Lists.newLinkedList();

        List<List<BigInteger>> B_k_inverse = Lists.newLinkedList();
        List<List<BigInteger>> Bk_inverse = Lists.newLinkedList();
        List<List<BigInteger>> dk_inverse = Lists.newLinkedList();

        BigInteger rbigr = this.commiter.rand();
        BigInteger rbigs = this.commiter.rand();
        BigInteger rbigu = this.commiter.rand();

        for(int i =0; i < this.K; i++){
            romega.add(this.commiter.rand());
            rbigt.add(this.commiter.rand());
            rbigf.add(this.commiter.rand());

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

        int LK = this.L * this.K;
        long s = System.nanoTime();
        List<BigInteger> mc1 = Lists.newLinkedList();
        int eta = nbits - L*(K-1);
        if(!aggre) {
            // decompose the witness y according to the base B, padding 0 if needed.
            List<BigInteger> narys = BigIntegerUtils.decomposeToNary(y.get(0), B);

            for (int i = narys.size(); i < nbits; i++){
                narys.add(BigInteger.ZERO);
            }

            for(int i = 0; i < L; i++){
                List<BigInteger> B_k_inverse_list = new LinkedList<>();
                List<BigInteger> Bk_inverse_list = new LinkedList<>();
                List<BigInteger> dk_inverse_list = new LinkedList<>();
                if(i >= eta){
                    if(K == 2 ){
                        B_k_inverse_list.add(BigInteger.ZERO);
                        B_k_inverse_list.add(Bs.get(i+eta).modInverse(this.n));
                        B_k_inverse.add(B_k_inverse_list);

                        Bk_inverse_list.add(BigInteger.ZERO);
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse.add(Bk_inverse_list);

                        dk_inverse_list.add(BigInteger.ZERO);
                        dk_inverse_list.add(narys.get(i+eta));
                        dk_inverse.add(dk_inverse_list);
                    }else if(K==3){
                        B_k_inverse_list.add(BigInteger.ZERO);
                        B_k_inverse_list.add(Bs.get(i+eta).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta+L).modInverse(this.n));
                        B_k_inverse.add(B_k_inverse_list);

                        Bk_inverse_list.add(BigInteger.ZERO);
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse.add(Bk_inverse_list);

                        dk_inverse_list.add(BigInteger.ZERO);
                        dk_inverse_list.add(narys.get(i+eta));
                        dk_inverse_list.add(narys.get(i+eta+L));
                        dk_inverse.add(dk_inverse_list);
                    }else if(K==4){
                        B_k_inverse_list.add(BigInteger.ZERO);
                        B_k_inverse_list.add(Bs.get(i+eta).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta+L).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta+L*2).modInverse(this.n));
                        B_k_inverse.add(B_k_inverse_list);

                        Bk_inverse_list.add(BigInteger.ZERO);
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse_list.add(Bs.get(i+eta+L*2));
                        Bk_inverse.add(Bk_inverse_list);

                        dk_inverse_list.add(BigInteger.ZERO);
                        dk_inverse_list.add(narys.get(i+eta));
                        dk_inverse_list.add(narys.get(i+eta+L));
                        dk_inverse_list.add(narys.get(i+eta+L*2));
                        dk_inverse.add(dk_inverse_list);
                    }


                }else{
                    if(K == 2 ){
                        B_k_inverse_list.add(Bs.get(i).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta).modInverse(this.n));
                        B_k_inverse.add(B_k_inverse_list);

                        Bk_inverse_list.add(Bs.get(i));
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse.add(Bk_inverse_list);

                        dk_inverse_list.add(narys.get(i));
                        dk_inverse_list.add(narys.get(i+eta));
                        dk_inverse.add(dk_inverse_list);
                    }else if(K==3){
                        B_k_inverse_list.add(Bs.get(i).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta+L).modInverse(this.n));
                        B_k_inverse.add(B_k_inverse_list);

                        Bk_inverse_list.add(Bs.get(i));
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse.add(Bk_inverse_list);

                        dk_inverse_list.add(narys.get(i));
                        dk_inverse_list.add(narys.get(i+eta));
                        dk_inverse_list.add(narys.get(i+eta+L));
                        dk_inverse.add(dk_inverse_list);
                    }else if(K==4){
                        B_k_inverse_list.add(Bs.get(i).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta+L).modInverse(this.n));
                        B_k_inverse_list.add(Bs.get(i+eta+L*2).modInverse(this.n));
                        B_k_inverse.add(B_k_inverse_list);

                        Bk_inverse_list.add(Bs.get(i));
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse_list.add(Bs.get(i+eta+L*2));
                        Bk_inverse.add(Bk_inverse_list);

                        dk_inverse_list.add(narys.get(i));
                        dk_inverse_list.add(narys.get(i+eta));
                        dk_inverse_list.add(narys.get(i+eta+L));
                        dk_inverse_list.add(narys.get(i+eta+L*2));
                        dk_inverse.add(dk_inverse_list);
                    }
                }
            }
            for (int i = 0; i < B.intValue(); i++) {
                int occurrences = Collections.frequency(narys, BigInteger.valueOf(i));
                mc1.add(BigInteger.valueOf(occurrences));
            }

        }else{
            List<List<BigInteger>> narys_T = new LinkedList<>();
            List<BigInteger> narys = new LinkedList<>();
            int tt = 0;
            for(int i = 0; i < TT*L; i++){
                List<BigInteger> B_k_inverse_list = new LinkedList<>();
                List<BigInteger> Bk_inverse_list = new LinkedList<>();
                List<BigInteger> dk_inverse_list = new LinkedList<>();
                if(i == L*tt){
                    narys = BigIntegerUtils.decomposeToNary(y.get(tt), B);
                    for (int j = narys.size(); j < nbits; j++){
                        narys.add(BigInteger.ZERO);
                    }
                    narys_T.add(narys);
                    tt+=1;
                }
                if(i < tt*L && i >= tt*eta){

                    // if k == 3
                    int _i = i;
                    if (i >= L) _i = i-L*(tt-1);
                    BigInteger gamma_t = gamma.modPow(BigInteger.valueOf(tt), this.n);
                    B_k_inverse_list.add(BigInteger.ZERO);
                    B_k_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta)).modInverse(this.n));
                    B_k_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta + L)).modInverse(this.n));
//                    B_k_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta + L * 2)).modInverse(this.n));
                    B_k_inverse.add(B_k_inverse_list);

                    Bk_inverse_list.add(BigInteger.ZERO);
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta)));
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta + L)));
//                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta + L * 2)));
                    Bk_inverse.add(Bk_inverse_list);

                    dk_inverse_list.add(BigInteger.ZERO);
                    dk_inverse_list.add(narys.get(_i + eta));
                    dk_inverse_list.add(narys.get(_i + eta + L));
//                    dk_inverse_list.add(narys.get(_i + eta + L * 2));
                    dk_inverse.add(dk_inverse_list);

                }else{
                    // if k == 3
                    int _i = i;
                    if (i >= L) _i = i-L*(tt-1);
                    BigInteger gamma_t = gamma.modPow(BigInteger.valueOf(tt), this.n);
                    B_k_inverse_list.add(gamma_t.multiply(Bs.get(_i)).modInverse(this.n));
                    B_k_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta)).modInverse(this.n));
                    B_k_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta+L)).modInverse(this.n));
//                    B_k_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta+L*2)).modInverse(this.n));
                    B_k_inverse.add(B_k_inverse_list);

                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i)));
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta)));
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta+L)));
//                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta+L*2)));
                    Bk_inverse.add(Bk_inverse_list);

                    dk_inverse_list.add(narys.get(_i));
                    dk_inverse_list.add(narys.get(_i+eta));
                    dk_inverse_list.add(narys.get(_i+eta+L));
//                    dk_inverse_list.add(narys.get(_i+eta+L*2));
                    dk_inverse.add(dk_inverse_list);
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
        List<List<BigInteger>> wjk = new LinkedList<>();
        for(int i = 0; i < K; i++){
            List<BigInteger> wjs = new LinkedList<>();
            for(int j = 0; j < L*TT; j++){
                wjs.add(dk_inverse.get(j).get(i).multiply(Bk_inverse.get(j).get(i)));
            }
            wjk.add(wjs);
        }
        for (int i = 0; i < this.K; i++) {
            this.cws.add(this.commiter
                    .commitTo(wjk.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n), romega.get(i)));
        }

        // the commitments of mc1
        for (int i = 0; i < B.intValue(); i++){
            BigInteger rM = this.commiter.rand();
            rbigm.add(rM);
            this.cms.add(this.commiter.commitTo(mc1.get(i), rM));
        }

        // calculate the commitments of R~, S~, and U~
        BigInteger cRconstant = rv.stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n);
        VectorB cSconstants = VectorB.from(rmu, this.n).mul(VectorB.from(rv, this.n));
        BigInteger cUconstant = BigInteger.ZERO;
        int tt = 0;
        for(int i = 0; i < L*TT; i++){
            if(i == L*tt){
                tt+=1;
            }
            int _i = i;
            if (i >= L) _i = i-L*(tt-1);
            cUconstant = cUconstant.add(B.pow(_i).multiply(rmu.get(i)).mod(this.n));
        }

        cR = this.commiter.commitTo(cRconstant.mod(this.n), rbigr);
        cS = this.gs.mulBAndSum(cSconstants).add(this.commiter.mulH(rbigs));
        cU = this.commiter.commitTo(cUconstant.mod(this.n), rbigu);

        // calculate random alpha using hash
        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cws);
        cs.addAll(this.cms);

        BigInteger alpha = HashUtils.hash(cs).mod(this.n);

        // f_jk and tao_jk
        List<List<BigInteger>> fs = new LinkedList<>();
        List<List<BigInteger>> taos = new LinkedList<>();
        List<BigInteger> taos_hat = new LinkedList<>();

        for(int i = 0; i < this.L*TT; i++){
            List<BigInteger> fk = new LinkedList<>();
            for(int j = 0; j < this.K; j++){
                if (!Bk_inverse.get(i).get(j).equals(BigInteger.ZERO)) {
                    BigInteger fk1 = alpha.add(dk_inverse.get(i).get(j)).mod(this.n);
                    BigInteger fk1inverse = fk1.modInverse(this.n);
                    fk.add(fk1inverse);
                }
            }
            fs.add(fk);
        }
        for(int i = 0; i < this.K; i++){
            List<BigInteger> taok = new LinkedList<>();
            for(int j = 0; j < this.L*TT; j++){
                if (!Bk_inverse.get(j).get(i).equals(BigInteger.ZERO)) {
                    BigInteger f_jk = fs.get(j).get(i);
                    BigInteger B_kf_rv = B_k_inverse.get(j).get(i).multiply(f_jk).multiply(rv.get(j)).mod(this.n);
                    BigInteger w_jk = dk_inverse.get(j).get(i).multiply(Bk_inverse.get(j).get(i)).mod(this.n);
                    BigInteger alpha_Bk_wk = alpha.multiply(Bk_inverse.get(j).get(i)).add(w_jk).mod(this.n);
                    taok.add(B_kf_rv.add(alpha_Bk_wk.multiply(rmu.get(j))).mod(this.n));
                }else{
                    taok.add(BigInteger.ZERO);
                }
            }
            taos.add(taok);
        }

        if(K==2){
            for(int i = 0; i < this.L*TT; i++){
                for(int j = 0; j < this.K; j=j+2){
                    if (!Bk_inverse.get(i).get(j).equals(BigInteger.ZERO)) {

                        BigInteger w_jk_prime = dk_inverse.get(i).get(j + 1).multiply(Bk_inverse.get(i).get(j + 1)).mod(this.n);

                        BigInteger f_jk = fs.get(i).get(j);
                        BigInteger B_k_f = B_k_inverse.get(i).get(j).multiply(f_jk).mod(this.n);
                        BigInteger alpha_Bk_wk = alpha.multiply(Bk_inverse.get(i).get(j + 1)).add(w_jk_prime).mod(this.n);

                        BigInteger w_jk = dk_inverse.get(i).get(j).multiply(Bk_inverse.get(i).get(j)).mod(this.n);

                        BigInteger f_jk_prime = fs.get(i).get(j + 1);
                        BigInteger B_kprime_f = B_k_inverse.get(i).get(j + 1).multiply(f_jk_prime).mod(this.n);
                        BigInteger alpha_Bk_wk_prime = alpha.multiply(Bk_inverse.get(i).get(j)).add(w_jk).mod(this.n);

                        BigInteger tao_hatk1 = B_k_f.multiply(alpha_Bk_wk).add(B_kprime_f.multiply(alpha_Bk_wk_prime)).mod(this.n);

                        taos_hat.add(tao_hatk1);

                    }else{
                        taos_hat.add(BigInteger.ZERO);
                    }
                }
            }
        }

        if(K==3){
            for(int i = 0; i < this.L*TT; i++){
                BigInteger f_jk = fs.get(i).get(0);
                BigInteger B_k_f = B_k_inverse.get(i).get(0).multiply(f_jk).mod(this.n);
                BigInteger w_jk_prime = dk_inverse.get(i).get(1).multiply(Bk_inverse.get(i).get(1)).mod(this.n);
                BigInteger alpha_Bk_wk = alpha.multiply(Bk_inverse.get(i).get(1)).add(w_jk_prime).mod(this.n);

                BigInteger f_jk_prime = fs.get(i).get(1);
                BigInteger B_kprime_f = B_k_inverse.get(i).get(1).multiply(f_jk_prime).mod(this.n);
                BigInteger w_jk = dk_inverse.get(i).get(0).multiply(Bk_inverse.get(i).get(0)).mod(this.n);
                BigInteger alpha_Bk_wk_prime = alpha.multiply(Bk_inverse.get(i).get(0)).add(w_jk).mod(this.n);

                BigInteger tao_hatk1 = B_k_f.multiply(alpha_Bk_wk).add(B_kprime_f.multiply(alpha_Bk_wk_prime)).mod(this.n);
                taos_hat.add(tao_hatk1);


                BigInteger f_jk2 = fs.get(i).get(1);
                BigInteger B_k_f2 = B_k_inverse.get(i).get(1).multiply(f_jk2).mod(this.n);
                BigInteger w_jk_prime2 = dk_inverse.get(i).get(2).multiply(Bk_inverse.get(i).get(2)).mod(this.n);
                BigInteger alpha_Bk_wk2 = alpha.multiply(Bk_inverse.get(i).get(2)).add(w_jk_prime2).mod(this.n);

                BigInteger f_jk_prime2 = fs.get(i).get(2);
                BigInteger B_kprime_f2 = B_k_inverse.get(i).get(2).multiply(f_jk_prime2).mod(this.n);
                BigInteger w_jk2 = dk_inverse.get(i).get(1).multiply(Bk_inverse.get(i).get(1)).mod(this.n);
                BigInteger alpha_Bk_wk_prime2 = alpha.multiply(Bk_inverse.get(i).get(1)).add(w_jk2).mod(this.n);

                BigInteger tao_hatk3 = B_k_f2.multiply(alpha_Bk_wk2).add(B_kprime_f2.multiply(alpha_Bk_wk_prime2)).mod(this.n);

                taos_hat.add(tao_hatk3);


                BigInteger f_jk1 = fs.get(i).get(2);
                BigInteger B_k_f1 = B_k_inverse.get(i).get(2).multiply(f_jk1).mod(this.n);
                BigInteger w_jk_prime1 = dk_inverse.get(i).get(0).multiply(Bk_inverse.get(i).get(0)).mod(this.n);
                BigInteger alpha_Bk_wk1 = alpha.multiply(Bk_inverse.get(i).get(0)).add(w_jk_prime1).mod(this.n);

                BigInteger f_jk_prime1 = fs.get(i).get(0);
                BigInteger B_kprime_f1 = B_k_inverse.get(i).get(0).multiply(f_jk_prime1).mod(this.n);
                BigInteger w_jk1 = dk_inverse.get(i).get(2).multiply(Bk_inverse.get(i).get(2)).mod(this.n);
                BigInteger alpha_Bk_wk_prime1 = alpha.multiply(Bk_inverse.get(i).get(2)).add(w_jk1).mod(this.n);

                BigInteger tao_hatk2 = B_k_f1.multiply(alpha_Bk_wk1).add(B_kprime_f1.multiply(alpha_Bk_wk_prime1)).mod(this.n);

                taos_hat.add(tao_hatk2);
            }

        }

        if(K==4){
            for(int i = 0; i < this.L*TT; i++){
                BigInteger f_jk = fs.get(i).get(0);
                BigInteger B_k_f = B_k_inverse.get(i).get(0).multiply(f_jk).mod(this.n);
                BigInteger w_jk_prime = dk_inverse.get(i).get(1).multiply(Bk_inverse.get(i).get(1)).mod(this.n);
                BigInteger alpha_Bk_wk = alpha.multiply(Bk_inverse.get(i).get(1)).add(w_jk_prime).mod(this.n);

                BigInteger f_jk_prime = fs.get(i).get(1);
                BigInteger B_kprime_f = B_k_inverse.get(i).get(1).multiply(f_jk_prime).mod(this.n);
                BigInteger w_jk = dk_inverse.get(i).get(0).multiply(Bk_inverse.get(i).get(0)).mod(this.n);
                BigInteger alpha_Bk_wk_prime = alpha.multiply(Bk_inverse.get(i).get(0)).add(w_jk).mod(this.n);
                BigInteger tao_hatk1 = B_k_f.multiply(alpha_Bk_wk).add(B_kprime_f.multiply(alpha_Bk_wk_prime)).mod(this.n);
                taos_hat.add(tao_hatk1);


                BigInteger f_jk1 = fs.get(i).get(0);
                BigInteger B_k_f1 = B_k_inverse.get(i).get(0).multiply(f_jk1).mod(this.n);
                BigInteger w_jk_prime1 = dk_inverse.get(i).get(2).multiply(Bk_inverse.get(i).get(2)).mod(this.n);
                BigInteger alpha_Bk_wk1 = alpha.multiply(Bk_inverse.get(i).get(2)).add(w_jk_prime1).mod(this.n);

                BigInteger f_jk_prime1 = fs.get(i).get(2);
                BigInteger B_kprime_f1 = B_k_inverse.get(i).get(2).multiply(f_jk_prime1).mod(this.n);
                BigInteger w_jk1 = dk_inverse.get(i).get(0).multiply(Bk_inverse.get(i).get(0)).mod(this.n);
                BigInteger alpha_Bk_wk_prime1 = alpha.multiply(Bk_inverse.get(i).get(0)).add(w_jk1).mod(this.n);
                BigInteger tao_hatk2 = B_k_f1.multiply(alpha_Bk_wk1).add(B_kprime_f1.multiply(alpha_Bk_wk_prime1)).mod(this.n);
                taos_hat.add(tao_hatk2);


                BigInteger f_jk2 = fs.get(i).get(0);
                BigInteger B_k_f2 = B_k_inverse.get(i).get(0).multiply(f_jk2).mod(this.n);
                BigInteger w_jk_prime2 = dk_inverse.get(i).get(3).multiply(Bk_inverse.get(i).get(3)).mod(this.n);
                BigInteger alpha_Bk_wk2 = alpha.multiply(Bk_inverse.get(i).get(3)).add(w_jk_prime2).mod(this.n);

                BigInteger f_jk_prime2 = fs.get(i).get(3);
                BigInteger B_kprime_f2 = B_k_inverse.get(i).get(3).multiply(f_jk_prime2).mod(this.n);
                BigInteger w_jk2 = dk_inverse.get(i).get(0).multiply(Bk_inverse.get(i).get(0)).mod(this.n);
                BigInteger alpha_Bk_wk_prime2 = alpha.multiply(Bk_inverse.get(i).get(0)).add(w_jk2).mod(this.n);
                BigInteger tao_hatk3 = B_k_f2.multiply(alpha_Bk_wk2).add(B_kprime_f2.multiply(alpha_Bk_wk_prime2)).mod(this.n);
                taos_hat.add(tao_hatk3);


                BigInteger f_jk3 = fs.get(i).get(1);
                BigInteger B_k_f3 = B_k_inverse.get(i).get(1).multiply(f_jk3).mod(this.n);
                BigInteger w_jk_prime3 = dk_inverse.get(i).get(2).multiply(Bk_inverse.get(i).get(2)).mod(this.n);
                BigInteger alpha_Bk_wk3 = alpha.multiply(Bk_inverse.get(i).get(2)).add(w_jk_prime3).mod(this.n);

                BigInteger f_jk_prime3 = fs.get(i).get(2);
                BigInteger B_kprime_f3 = B_k_inverse.get(i).get(2).multiply(f_jk_prime3).mod(this.n);
                BigInteger w_jk3 = dk_inverse.get(i).get(1).multiply(Bk_inverse.get(i).get(1)).mod(this.n);
                BigInteger alpha_Bk_wk_prime3 = alpha.multiply(Bk_inverse.get(i).get(1)).add(w_jk3).mod(this.n);
                BigInteger tao_hatk4 = B_k_f3.multiply(alpha_Bk_wk3).add(B_kprime_f3.multiply(alpha_Bk_wk_prime3)).mod(this.n);
                taos_hat.add(tao_hatk4);


                BigInteger f_jk4 = fs.get(i).get(1);
                BigInteger B_k_f4 = B_k_inverse.get(i).get(1).multiply(f_jk4).mod(this.n);
                BigInteger w_jk_prime4 = dk_inverse.get(i).get(3).multiply(Bk_inverse.get(i).get(3)).mod(this.n);
                BigInteger alpha_Bk_wk4 = alpha.multiply(Bk_inverse.get(i).get(3)).add(w_jk_prime4).mod(this.n);

                BigInteger f_jk_prime4 = fs.get(i).get(3);
                BigInteger B_kprime_f4 = B_k_inverse.get(i).get(3).multiply(f_jk_prime4).mod(this.n);
                BigInteger w_jk4 = dk_inverse.get(i).get(1).multiply(Bk_inverse.get(i).get(1)).mod(this.n);
                BigInteger alpha_Bk_wk_prime4 = alpha.multiply(Bk_inverse.get(i).get(1)).add(w_jk4).mod(this.n);
                BigInteger tao_hatk5 = B_k_f4.multiply(alpha_Bk_wk4).add(B_kprime_f4.multiply(alpha_Bk_wk_prime4)).mod(this.n);
                taos_hat.add(tao_hatk5);


                BigInteger f_jk5 = fs.get(i).get(2);
                BigInteger B_k_f5 = B_k_inverse.get(i).get(2).multiply(f_jk5).mod(this.n);
                BigInteger w_jk_prime5 = dk_inverse.get(i).get(3).multiply(Bk_inverse.get(i).get(3)).mod(this.n);
                BigInteger alpha_Bk_wk5 = alpha.multiply(Bk_inverse.get(i).get(3)).add(w_jk_prime5).mod(this.n);

                BigInteger f_jk_prime5 = fs.get(i).get(3);
                BigInteger B_kprime_f5 = B_k_inverse.get(i).get(3).multiply(f_jk_prime5).mod(this.n);
                BigInteger w_jk5 = dk_inverse.get(i).get(2).multiply(Bk_inverse.get(i).get(2)).mod(this.n);
                BigInteger alpha_Bk_wk_prime5 = alpha.multiply(Bk_inverse.get(i).get(2)).add(w_jk5).mod(this.n);
                BigInteger tao_hatk6 = B_k_f5.multiply(alpha_Bk_wk5).add(B_kprime_f5.multiply(alpha_Bk_wk_prime5)).mod(this.n);
                taos_hat.add(tao_hatk6);
            }
        }

        // the commitments of F_k
        List<List<BigInteger>> fs_kj = new LinkedList<>();
        for(int i = 0; i < K; i++){
            List<BigInteger> fs_kj_col = new LinkedList<>();
            for(int j = 0; j < L*TT; j++){
//                if(i < fs.get(j).size()){
                fs_kj_col.add(fs.get(j).get(i));
//                }
            }
            fs_kj.add(fs_kj_col);
        }

        for(int i = 0; i< this.K;i++){
            this.cfk.add(this.commiter
                    .commitTo(fs_kj.get(i).stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n), rbigf.get(i)));
        }

        // the commitments of T_k
        for(int i = 0; i< this.K;i++){
            this.ctk.add(this.gs.mulBAndSum(VectorB.from(taos.get(i), this.n)).add(this.commiter.mulH(rbigt.get(i))));
        }

        // the commitments of T_hat_k
        BigInteger rt_kprime = this.commiter.rand();
        List<BigInteger> rt_kprime_list = new LinkedList<>();
        if(K == 2){
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat, this.n)).add(this.commiter.mulH(rt_kprime)));
        }else if(K == 3){
            BigInteger rt_kprime1 = this.commiter.rand();
            BigInteger rt_kprime2 = this.commiter.rand();
            BigInteger rt_kprime3 = this.commiter.rand();
            rt_kprime_list.add(rt_kprime1);
            rt_kprime_list.add(rt_kprime2);
            rt_kprime_list.add(rt_kprime3);
            List<BigInteger> taos_hat_1 = new LinkedList<>();
            List<BigInteger> taos_hat_2 = new LinkedList<>();
            List<BigInteger> taos_hat_3 = new LinkedList<>();
            for(int i = 0; i < taos_hat.size(); i=i+3){
                taos_hat_1.add(taos_hat.get(i));
                taos_hat_2.add(taos_hat.get(i+1));
                taos_hat_3.add(taos_hat.get(i+2));
            }
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_1, this.n)).add(this.commiter.mulH(rt_kprime1)));
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_2, this.n)).add(this.commiter.mulH(rt_kprime2)));
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_3, this.n)).add(this.commiter.mulH(rt_kprime3)));
        }else if(K==4){
            for(int i = 0; i < K*(K-1)/2; i++){
                rt_kprime_list.add(this.commiter.rand());
            }
            List<BigInteger> taos_hat_1 = new LinkedList<>();
            List<BigInteger> taos_hat_2 = new LinkedList<>();
            List<BigInteger> taos_hat_3 = new LinkedList<>();
            List<BigInteger> taos_hat_4 = new LinkedList<>();
            List<BigInteger> taos_hat_5 = new LinkedList<>();
            List<BigInteger> taos_hat_6 = new LinkedList<>();
            for(int i = 0; i < taos_hat.size(); i=i+6){
                taos_hat_1.add(taos_hat.get(i));
                taos_hat_2.add(taos_hat.get(i+1));
                taos_hat_3.add(taos_hat.get(i+2));
                taos_hat_4.add(taos_hat.get(i+3));
                taos_hat_5.add(taos_hat.get(i+4));
                taos_hat_6.add(taos_hat.get(i+5));
            }
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_1, this.n)).add(this.commiter.mulH(rt_kprime_list.get(0))));
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_2, this.n)).add(this.commiter.mulH(rt_kprime_list.get(1))));
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_3, this.n)).add(this.commiter.mulH(rt_kprime_list.get(2))));
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_4, this.n)).add(this.commiter.mulH(rt_kprime_list.get(3))));
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_5, this.n)).add(this.commiter.mulH(rt_kprime_list.get(4))));
            this.ctk_kprime.add(this.gs.mulBAndSum(VectorB.from(taos_hat_6, this.n)).add(this.commiter.mulH(rt_kprime_list.get(5))));
        }

        // calculate random epsilon
        cs.addAll(this.cfk);
        cs.addAll(this.ctk);
        cs.addAll(this.ctk_kprime);

        List<BigInteger> cl_es = Lists.newLinkedList();
        BigInteger beta = HashUtils.hash(cs).mod(this.n);
        BigInteger e2s = BigInteger.ZERO;
        List<BigInteger> e2s_k = new LinkedList<>();
        cl_es.add(beta);
        for(int i = 0; i < this.K-1; i ++){
            BigInteger betaX = beta.modPow(BigInteger.valueOf(i+2),this.n);
            cl_es.add(betaX);
        }
        List<BigInteger> einverse = new LinkedList<>();
        for(int i = 0; i < this.K; i++){
            e2s_k.add(cl_es.get(i).pow(2).mod(this.n));
            e2s = e2s.add(cl_es.get(i).pow(2)).mod(this.n);
            einverse.add(cl_es.get(i).modInverse(this.n));
        }

        // calculate vector v_j, u_j
        for (int i =0; i < this.L*TT;i++){
            List<BigInteger> wdote = new LinkedList<>();
            List<BigInteger> Bdotfdote = new LinkedList<>();
            for(int j = 0; j < this.K; j++) {
                if (!Bk_inverse.get(i).get(j).equals(BigInteger.ZERO)) {
//                    if(i<fs.get(j).size()) {
                    BigInteger w_jk = dk_inverse.get(i).get(j).multiply(Bk_inverse.get(i).get(j)).mod(this.n);
                    wdote.add((Bk_inverse.get(i).get(j).multiply(alpha).add(w_jk)).multiply(cl_es.get(j)).mod(this.n));
                    Bdotfdote.add(B_k_inverse.get(i).get(j).multiply(fs.get(i).get(j)).multiply(cl_es.get(j)).mod(this.n));
//                    }
                }
            }
            if(wdote.size()!=0){
                this.vs.add(VectorB.from(wdote, this.n).sum().add(rv.get(i)).mod(this.n));
                this.us.add(VectorB.from(Bdotfdote, this.n).sum().add(rmu.get(i)).mod(this.n));
            }
        }

        // calculate eta1-4
        this.eta1 = rt_kprime.multiply(cl_es.get(0)).multiply(cl_es.get(1))
                .add(VectorB.from(rbigt, this.n).innerProd(VectorB.from(cl_es, this.n)))
                .add(rbigs).mod(this.n);
        if(K == 3){
            this.eta1 = rt_kprime_list.get(0).multiply(cl_es.get(0)).multiply(cl_es.get(1))
                    .add(rt_kprime_list.get(1).multiply(cl_es.get(1)).multiply(cl_es.get(2)))
                    .add(rt_kprime_list.get(2).multiply(cl_es.get(2)).multiply(cl_es.get(0)))
                    .add(VectorB.from(rbigt, this.n).innerProd(VectorB.from(cl_es, this.n)))
                    .add(rbigs).mod(this.n);
        }
        if(K == 4){
            this.eta1 = rt_kprime_list.get(0).multiply(cl_es.get(0)).multiply(cl_es.get(1))
                    .add(rt_kprime_list.get(1).multiply(cl_es.get(0)).multiply(cl_es.get(2)))
                    .add(rt_kprime_list.get(2).multiply(cl_es.get(0)).multiply(cl_es.get(3)))
                    .add(rt_kprime_list.get(3).multiply(cl_es.get(1)).multiply(cl_es.get(2)))
                    .add(rt_kprime_list.get(4).multiply(cl_es.get(1)).multiply(cl_es.get(3)))
                    .add(rt_kprime_list.get(5).multiply(cl_es.get(2)).multiply(cl_es.get(3)))
                    .add(VectorB.from(rbigt, this.n).innerProd(VectorB.from(cl_es, this.n)))
                    .add(rbigs).mod(this.n);
        }

        BigInteger r1_es1 = rbigf.get(0).multiply(cl_es.get(0)).mod(this.n);
        BigInteger rs_es = BigInteger.ZERO;
        for(int i = 1; i < K; i++){
            rs_es = rs_es.add(rbigf.get(i).multiply(cl_es.get(i)).multiply(B.pow(L*(i-1)+eta).modInverse(this.n)));
        }

        this.eta2 = r1_es1.add(rs_es).add(rbigu).mod(this.n);


        List<BigInteger> alpha_c_1 = new LinkedList<>();
        for(int i = 0; i < B.intValue(); i++){
            alpha_c_1.add(alpha.add(BigInteger.valueOf(i)).modInverse(this.n));
        }
        System.out.println("alpha_c_1: " + alpha_c_1);
        BigInteger rbigf_sum = rbigf.stream().reduce(BigInteger.ZERO, (d1, d2) -> d1.add(d2)).mod(this.n);
        BigInteger rbigm_sum = VectorB.from(rbigm, this.n).innerProd(VectorB.from(alpha_c_1, this.n));

        this.eta3 = rbigm_sum.subtract(rbigf_sum).mod(this.n);

        this.eta4 = VectorB.from(romega, this.n).innerProd(VectorB.from(cl_es, this.n)).add(rbigr).mod(this.n);

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            ptime += (e - s);
        }

        counter++;
        string();

    }

    public void string() {
        List<String> total1 = new LinkedList<>();

//        String cyStr = BigIntegerUtils.toString(this.cy.getCoordList());
//        total1.add(cyStr);
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

        String cfsStr = "[" + String.join(",",
                this.cfk.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(cfsStr);

        String ctk_kprimeStr = "[" + String.join(",",
                this.ctk_kprime.stream().map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList()))
                + "]";
        total1.add(ctk_kprimeStr);

        String bigSStr = BigIntegerUtils.toString(this.cS.getCoordList());
        total1.add(bigSStr);

        String bigRStr = BigIntegerUtils.toString(this.cR.getCoordList());
        total1.add(bigRStr);

        String bigUStr = BigIntegerUtils.toString(this.cU.getCoordList());
        total1.add(bigUStr);

        total1.add("\"" + this.eta1 + "\"");

        total1.add("\"" + this.eta2 + "\"");

        total1.add("\"" + this.eta3 + "\"");

        total1.add("\"" + this.eta4 + "\"");

        String vStr = "[" + String.join(",", this.vs.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";
        total1.add(vStr);

        String uStr = "[" + String.join(",", this.us.stream().map(c -> "\"" + c + "\"").collect(Collectors.toList()))
                + "]";
        total1.add(uStr);


        System.out.println("str1:" + String.join(",", total1));

        String gsStr = "[" + String.join(",", this.gs.getList().stream()
                .map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList())) + "]";
        System.out.println("gsStr:" + String.join(",", gsStr));
    }
    @Override
    public boolean verify() {
        // prepare parameters
//        BigInteger gamma = HashUtils.hash(this.cYs).mod(this.n);
        BigInteger gamma = BigInteger.ONE;
        BigInteger e2s = BigInteger.ZERO;
        List<BigInteger> e2s_k = new LinkedList<>();

        List<BigInteger> Bs = Lists.newLinkedList();
        for(int i = 0; i < nbits; i++){
            BigInteger value = this.B.pow(i);
            Bs.add(value);
        }

        List<List<BigInteger>> Bk_inverse = Lists.newLinkedList();

        int eta = nbits - L*(K-1);
        if(this.TT == 1){
            for(int i = 0; i < L; i++){
                List<BigInteger> Bk_inverse_list = new LinkedList<>();
                if(i >= eta){
                    if(K == 2 ){
                        Bk_inverse_list.add(BigInteger.ZERO);
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse.add(Bk_inverse_list);
                    }else if(K==3){
                        Bk_inverse_list.add(BigInteger.ZERO);
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse.add(Bk_inverse_list);

                    }else if(K==4){
                        Bk_inverse_list.add(BigInteger.ZERO);
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse_list.add(Bs.get(i+eta+L*2));
                        Bk_inverse.add(Bk_inverse_list);
                    }
                }else{
                    if(K == 2 ){
                        Bk_inverse_list.add(Bs.get(i));
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse.add(Bk_inverse_list);

                    }else if(K==3){
                        Bk_inverse_list.add(Bs.get(i));
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse.add(Bk_inverse_list);

                    }else if(K==4){
                        Bk_inverse_list.add(Bs.get(i));
                        Bk_inverse_list.add(Bs.get(i+eta));
                        Bk_inverse_list.add(Bs.get(i+eta+L));
                        Bk_inverse_list.add(Bs.get(i+eta+L*2));
                        Bk_inverse.add(Bk_inverse_list);
                    }
                }
            }
        }else{
            int tt = 0;
            for(int i = 0; i < TT*L; i++){
                List<BigInteger> Bk_inverse_list = new LinkedList<>();
                if(i == L*tt){
                    tt+=1;
                }
                if(i < tt*L && i >= tt*eta){
                    // if k == 3
                    int _i = i;
                    if (i >= L) _i = i-L*(tt-1);
                    BigInteger gamma_t = gamma.modPow(BigInteger.valueOf(tt), this.n);
                    Bk_inverse_list.add(BigInteger.ZERO);
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta)));
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta + L)));
//                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i + eta + L * 2)));
                    Bk_inverse.add(Bk_inverse_list);
                }else{
                    // if k == 3
                    int _i = i;
                    if (i >= L) _i = i-L*(tt-1);
                    BigInteger gamma_t = gamma.modPow(BigInteger.valueOf(tt), this.n);
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i)));
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta)));
                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta+L)));
//                    Bk_inverse_list.add(gamma_t.multiply(Bs.get(_i+eta+L*2)));
                    Bk_inverse.add(Bk_inverse_list);
                }
            }
        }


        long s = System.nanoTime();
        // calculate random challenge
        List<Commitment> cs = new LinkedList<>();
        cs.addAll(this.cws);
        cs.addAll(this.cms);

        BigInteger alpha = HashUtils.hash(cs).mod(this.n);

        cs.addAll(this.cfk);
        cs.addAll(this.ctk);
        cs.addAll(this.ctk_kprime);

        List<BigInteger> cl_es = Lists.newLinkedList();
        BigInteger beta = HashUtils.hash(cs).mod(this.n);
        cl_es.add(beta);
        for(int i = 0; i < this.K-1; i ++){
            BigInteger betaX = beta.modPow(BigInteger.valueOf(i+2),this.n);
            cl_es.add(betaX);
        }
        List<BigInteger> einverse = new LinkedList<>();
        for(int i = 0; i < this.K; i++){
            e2s_k.add(cl_es.get(i).pow(2).mod(this.n));
            e2s = e2s.add(cl_es.get(i).pow(2)).mod(this.n);
            einverse.add(cl_es.get(i).modInverse(this.n));
        }

        // check equation 1;
        BigInteger esk2 = BigInteger.ZERO;
        for(int i = 0; i < L*TT; i++){
            for(int j = 0; j < K; j++){
                if (Bk_inverse.get(i).get(j).equals(BigInteger.ZERO)) {
                    esk2 = esk2.add(cl_es.get(j).multiply(cl_es.get(j)));
                }
            }
        }
        System.out.println("esk2: " + esk2);
        List<BigInteger> vj_muj_es2 = new LinkedList<>();
        for(int i = 0; i < L*TT; i++){
            vj_muj_es2.add(this.vs.get(i).multiply(this.us.get(i)).add(esk2).mod(this.n));
        }

        Commitment eqn1_1 = this.gs.mulBAndSum(VectorB.from(vj_muj_es2, this.n)).add(this.commiter.mulH(this.eta1));

        Commitment eqn1_2 = this.ctk_kprime.get(0).mul(cl_es.get(0).multiply(cl_es.get(1)).mod(this.n));
        if(K == 3){
            eqn1_2 = eqn1_2.add(this.ctk_kprime.get(1).mul(cl_es.get(1).multiply(cl_es.get(2)).mod(this.n)));
            eqn1_2 = eqn1_2.add(this.ctk_kprime.get(2).mul(cl_es.get(2).multiply(cl_es.get(0)).mod(this.n)));
        }
        if(K == 4){
            eqn1_2 = eqn1_2.add(this.ctk_kprime.get(1).mul(cl_es.get(0).multiply(cl_es.get(2)).mod(this.n)));
            eqn1_2 = eqn1_2.add(this.ctk_kprime.get(2).mul(cl_es.get(0).multiply(cl_es.get(3)).mod(this.n)));
            eqn1_2 = eqn1_2.add(this.ctk_kprime.get(3).mul(cl_es.get(1).multiply(cl_es.get(2)).mod(this.n)));
            eqn1_2 = eqn1_2.add(this.ctk_kprime.get(4).mul(cl_es.get(1).multiply(cl_es.get(3)).mod(this.n)));
            eqn1_2 = eqn1_2.add(this.ctk_kprime.get(5).mul(cl_es.get(2).multiply(cl_es.get(3)).mod(this.n)));
        }
        for (int i = 0; i < this.K; i++){
            eqn1_2 = eqn1_2.add(this.ctk.get(i).mul(cl_es.get(i)));
        }
        eqn1_2 = eqn1_2.add(cS);


        Commitment Hj = this.commiter.getIdentity();
        for (int i = 0; i < this.L*TT; i++){
            Hj = Hj.add(this.gs.getList().get(i));
        }

        eqn1_2 = eqn1_2.add(Hj.mul(e2s));

        Boolean b1 = eqn1_1.equals(eqn1_2);

        // check equation 2;
        BigInteger Bj_1 = BigInteger.ZERO;
        int tt = 0;
        for(int i = 0; i < L*TT; i++){
            if(i == L*tt){
                tt+=1;
            }
            int _i = i;
            if (i >= L) _i = i-L*(tt-1);
            Bj_1 = Bj_1.add(B.pow(_i).multiply(this.us.get(i)));
        }

        Commitment eqn2_1 = this.commiter.commitTo(Bj_1.mod(this.n), this.eta2);
        Commitment eqn2_2 = this.cfk.get(0).mul(cl_es.get(0));

        for(int i = 1; i < K; i++){
            int exp = L*(i-1)+eta;
            BigInteger fk_exp = cl_es.get(i).multiply(B.pow(exp).modInverse(this.n));
            System.out.println("B pow inverse: " + B.pow(exp).modInverse(this.n));
            eqn2_2 = eqn2_2.add(this.cfk.get(i).mul(fk_exp));
        }
        eqn2_2 = eqn2_2.add(this.cU);

        boolean b2 = eqn2_1.equals(eqn2_2);

        // check equation 3;
        Commitment Fk = this.commiter.getIdentity();
        for (int i = 0; i < this.K; i++){
            Fk = Fk.add(this.cfk.get(i));
        }

        Commitment eqn3_1 = Fk.add(this.commiter.mulH(this.eta3));
        Commitment eqn3_2 = this.commiter.getIdentity();

        for (int i = 0; i < this.B.intValue(); i++){
            eqn3_2 = eqn3_2.add(this.cms.get(i).mul((alpha.add(BigInteger.valueOf(i))).modInverse(this.n)));
//            System.out.println((alpha.add(BigInteger.valueOf(i))).modInverse(this.n));
        }
        boolean b3 = eqn3_1.equals(eqn3_2);

        // check equation 4;
        List<BigInteger> Bk_sum = new LinkedList<>();
        for(int i = 0; i < K; i++){
            BigInteger Bj_sum = BigInteger.ZERO;
            for(int j = 0; j < L*TT; j++){
                Bj_sum = Bj_sum.add(Bk_inverse.get(j).get(i));
            }
            Bk_sum.add(Bj_sum);
        }
        BigInteger alpha_bk_ek = VectorB.from(cl_es, this.n).mulConstant(alpha).innerProd(VectorB.from(Bk_sum, this.n));
        BigInteger vj_sum = VectorB.from(vs, this.n).sum();
        Commitment eqn4_1 = this.commiter.commitTo(vj_sum.subtract(alpha_bk_ek).mod(this.n), this.eta4);

        Commitment eqn4_2 = this.commiter.getIdentity();
        for (int i = 0; i < this.K; i++){
            eqn4_2 = eqn4_2.add(this.cws.get(i).mul(cl_es.get(i)));
        }
        eqn4_2 = eqn4_2.add(this.cR);
        Boolean b4 = eqn4_1.equals(eqn4_2);

        // check equation 5;
//        boolean b5 = this.cy.equals(this.cws.stream()
//                .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));
        boolean b5 = false;
        if(this.TT == 1){
            b5 = this.cYs.get(0).equals(this.cws.stream()
                    .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));
        }else{
            Commitment eqn5_1 = this.commiter.getIdentity();
            for (int i = 0; i < this.TT; i++){
                eqn5_1 = eqn5_1.add(this.cYs.get(i).mul(gamma.modPow(BigInteger.valueOf(i+1), this.n)));
            }
            b5 = eqn5_1.equals(this.cws.stream()
                    .reduce(this.commiter.getIdentity(), (c1, c2) -> c1.add(c2)));
        }

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            vtime += (e - s);
        }
        return b1&b2&b3&b4&&b5;
    }
}

