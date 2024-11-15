package zkp;

import java.math.BigInteger;
import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import com.google.common.collect.Lists;

import commitment.Commitment;
import structure.VectorB;
import structure.VectorP;
import utils.BigIntegerUtils;
import utils.HashUtils;

public class BPPP extends PedersenZKP{
    private static final long serialVersionUID = 1L;

    private final Commitment Y;
    private final Commitment cD;
    private final Commitment cM;
    private final Commitment cF;
    private final Commitment cS1;
    private final Commitment cS2;
    private final Commitment cS3;
    private final Commitment bigw1;
    private final Commitment bigw2;
    private final Commitment bigt1;
    private final Commitment bigt2;

    private final BigInteger rw1;
    private final BigInteger rw2;
    private final BigInteger t1hat;
    private final BigInteger t2hat;
    private final BigInteger tao_rho;
    List<BigInteger> Bs = Lists.newLinkedList();
    VectorB ls;
    VectorB rs;
    VectorB mprimes;
//    List<BigInteger> lamda_alpha = Lists.newLinkedList();
    List<BigInteger> Bk = Lists.newLinkedList();

    private final List<Commitment> cs = new LinkedList<>();

    private final VectorP gs;
    private final int nbits;
    private final BigInteger B;

    public static int counter = 0;
    public static long ptime = 0;
    public static long vtime = 0;

    public BPPP(BigInteger y, int nbits, int b_ary, VectorP gs){
        this.gs = gs;
        this.nbits = nbits;
        this.B = BigInteger.valueOf(b_ary);

        BigInteger rcm = this.commiter.rand();
        this.Y = this.commiter.commitTo(y, rcm);

        List<BigInteger> s1 = Lists.newLinkedList();
        List<BigInteger> s2 = Lists.newLinkedList();
        List<BigInteger> s3 = Lists.newLinkedList();
        List<BigInteger> B_k = Lists.newLinkedList();

        for(int i = 0; i < this.nbits; i++){
            BigInteger value = this.B.pow(i);
            Bs.add(value);
            s1.add(this.commiter.rand());
            s2.add(this.commiter.rand());
        }

        for(int i =0; i < nbits; i++){
            Bk.add(B.modPow(BigInteger.valueOf(i), this.n));
            B_k.add(B.modPow(BigInteger.valueOf(i).negate(), this.n));
        }

        BigInteger rbigd = this.commiter.rand();
        BigInteger rbigm = this.commiter.rand();
        BigInteger rbigf = this.commiter.rand();
        BigInteger rbigs1 = this.commiter.rand();
        BigInteger rbigs2 = this.commiter.rand();
        BigInteger rbigs3 = this.commiter.rand();

        List<BigInteger> betas = Lists.newLinkedList();

        long s = System.nanoTime();

        List<BigInteger> narys = BigIntegerUtils.decomposeToNary(y, B);
        for (int i = narys.size(); i < nbits; i++){
            narys.add(BigInteger.ZERO);
        }

        List<BigInteger> mc1 = Lists.newLinkedList();
        for(int i = 0; i < B.intValue(); i++){
            int occurrences = Collections.frequency(narys, BigInteger.valueOf(i));
            mc1.add(BigInteger.valueOf(occurrences));
            s3.add(this.commiter.rand());
        }

        // 1: p --> v

        cD = this.gs.mulBAndSum(VectorB.from(narys, this.n)).add(this.commiter.mulH(rbigd));
        cM = this.gs.mulBAndSum(VectorB.from(mc1, this.n)).add(this.commiter.mulH(rbigm));
        cS1 = this.gs.mulBAndSum(VectorB.from(s1, this.n)).add(this.commiter.mulH(rbigs1));
        cS2 = this.gs.mulBAndSum(VectorB.from(s2, this.n)).add(this.commiter.mulH(rbigs2));
        cS3 = this.gs.mulBAndSum(VectorB.from(s3, this.n)).add(this.commiter.mulH(rbigs3));

        // 1: v --> p && p
        cs.add(cD);
        cs.add(cM);
        cs.add(cS1);
        cs.add(cS2);
        cs.add(cS3);

        BigInteger cl_alpha = HashUtils.hash(cs, this.Y).mod(this.n);
        List<BigInteger> f = Lists.newLinkedList();
        for(int i = 0; i < nbits; i++){
            f.add(cl_alpha.add(narys.get(i)).modInverse(this.n));
        }

        // 2: p --> v
        cF = this.gs.mulBAndSum(VectorB.from(f, this.n)).add(this.commiter.mulH(rbigf));

        // 2: v --> p && p
        BigInteger cl_y = HashUtils.hash(Arrays.asList(cF), cl_alpha).mod(this.n);
        BigInteger cl_z = HashUtils.hash(Arrays.asList(cF), cl_y).mod(this.n);

        BigInteger cl_z2 = cl_z.multiply(cl_z).mod(this.n);

        List<BigInteger> cl_yn = Lists.newLinkedList();
        List<BigInteger> cl_y_n = Lists.newLinkedList();
        for(int i = 0; i < nbits; i++){
            cl_yn.add(cl_y.modPow(BigInteger.valueOf(i), this.n));
            cl_y_n.add(cl_y.modPow(BigInteger.valueOf(i), this.n).modInverse(this.n));
        }

        BigInteger tao1 = this.commiter.rand();
        BigInteger tao2 = this.commiter.rand();

        VectorB t2_left = VectorB.from(s1, this.n).mulConstant(cl_z2);
        VectorB t2_right = VectorB.from(s2, this.n).mul(VectorB.from(cl_yn, this.n));
        BigInteger t2 = t2_left.innerProd(t2_right);

        List<BigInteger> entityList = Collections.nCopies(nbits, BigInteger.ONE);
        VectorB t1_1_left = VectorB.from(s1, this.n).mulConstant(cl_z2);
        VectorB t1_1_right = VectorB.from(cl_yn,this.n).mulConstant(cl_alpha)
                                    .add(VectorB.from(narys, this.n).mul(VectorB.from(cl_yn, this.n))
                                    .add(VectorB.from(entityList, this.n).mulConstant(cl_z.modInverse(this.n))));
        BigInteger t1_1 = t1_1_left.innerProd(t1_1_right);

        List<BigInteger> lamda_alpha = Lists.newLinkedList();
        for(int i = 0; i < B.intValue(); i++){
            lamda_alpha.add((cl_alpha.add(BigInteger.valueOf(i))).modInverse(this.n));
        }
        BigInteger t1_2 = VectorB.from(s3, this.n).innerProd(VectorB.from(lamda_alpha, this.n)).multiply(cl_z).mod(this.n);

        VectorB t1_3_left = VectorB.from(f, this.n).mulConstant(cl_z2).add(VectorB.from(Bs, this.n).mul(VectorB.from(cl_y_n, this.n))) ;
        VectorB t1_3_right = VectorB.from(s2, this.n).mul(VectorB.from(cl_yn, this.n));
        BigInteger t1_3 = t1_3_left.innerProd(t1_3_right);

        BigInteger t1 = t1_1.subtract(t1_2).add(t1_3);

        // 3: p --> v
        bigt1 = this.commiter.commitTo(t1, tao1);
        bigt2 = this.commiter.commitTo(t2, tao2);

        // 3: v --> p && p
        BigInteger cl_rho = HashUtils.hash(bigt1, bigt2).mod(this.n);
        BigInteger cl_rho2 = cl_rho.multiply(cl_rho).mod(this.n);
        ls = VectorB.from(f, this.n)
                    .add(VectorB.from(s1, this.n).mulConstant(cl_rho))
                    .mulConstant(cl_z2)
                    .add(VectorB.from(Bs, this.n).mul(VectorB.from(cl_y_n, this.n)));
        rs = VectorB.from(cl_yn, this.n).mulConstant(cl_alpha)
                    .add(VectorB.from(cl_yn, this.n)
                            .mul(VectorB.from(narys, this.n)
                                    .add(VectorB.from(s2, this.n)
                                            .mulConstant(cl_rho))
                            )
                    )
                    .add(VectorB.from(entityList, this.n).mulConstant(cl_z.modInverse(this.n)));
        mprimes = VectorB.from(mc1, this.n)
                          .add(VectorB.from(s3, this.n).mulConstant(cl_rho));

        // 4: p --> v
        t1hat = ls.innerProd(rs);
        t2hat = mprimes.innerProd(VectorB.from(lamda_alpha,this.n));
        tao_rho = rcm.add(tao2.multiply(cl_rho2))
                .add(tao1.multiply(cl_rho))
                .mod(this.n);

        bigw1 =this.gs.mulBAndSum(ls).add(this.gs.mulBAndSum(VectorB.from(cl_y_n, this.n).mul(rs)));
        bigw2 =this.gs.mulBAndSum(mprimes).add(this.gs.mulBAndSum(VectorB.from(lamda_alpha, this.n)));

        rw1 = rbigf.multiply(cl_z2)
                         .add(rbigd)
                         .add(cl_rho.multiply(rbigs1.multiply(cl_z2).add(rbigs2)))
                         .mod(this.n);
        rw2 = rbigm.add(rbigs3.multiply(cl_rho)).mod(this.n);

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            ptime += (e - s);
        }
        counter++;
        string();
    }


    public void string() {

        List<String> total1 = new LinkedList<>();
        List<String> total2 = new LinkedList<>();

        String cyStr = BigIntegerUtils.toString(this.Y.getCoordList());
        total1.add(cyStr);

        String cdStr = BigIntegerUtils.toString(this.cD.getCoordList());
        total1.add(cdStr);

        String cmStr = BigIntegerUtils.toString(this.cM.getCoordList());
        total1.add(cmStr);

        String cfStr = BigIntegerUtils.toString(this.cF.getCoordList());
        total1.add(cfStr);

        String cs1Str = BigIntegerUtils.toString(this.cS1.getCoordList());
        total1.add(cs1Str);

        String cs2Str = BigIntegerUtils.toString(this.cS2.getCoordList());
        total1.add(cs2Str);

        String cs3tr = BigIntegerUtils.toString(this.cS3.getCoordList());
        total1.add(cs3tr);

        String ct1Str = BigIntegerUtils.toString(this.bigt1.getCoordList());
        total1.add(ct1Str);

        String ct2tr = BigIntegerUtils.toString(this.bigt2.getCoordList());
        total1.add(ct2tr);

        String cw1Str = BigIntegerUtils.toString(this.bigw1.getCoordList());
        total1.add(cw1Str);

        String cw2tr = BigIntegerUtils.toString(this.bigw2.getCoordList());
        total1.add(cw2tr);

        System.out.println("str1:" + String.join(",", total1));

        total2.add("\"" + this.rw1 + "\"");

        total2.add("\"" + this.rw2 + "\"");

        total2.add("\"" + this.t1hat + "\"");

        total2.add("\"" + this.t2hat + "\"");

        total2.add("\"" + this.tao_rho + "\"");

        System.out.println("str2:" + String.join(",", total2));


        String gsStr = "[" + String.join(",", this.gs.getList().stream()
                .map(c -> BigIntegerUtils.toString(c.getCoordList())).collect(Collectors.toList())) + "]";
        System.out.println("gsStr:" + String.join(",", gsStr));
    }

    @Override
    public boolean verify() {
        BigInteger cl_rho = HashUtils.hash(bigt1, bigt2).mod(this.n);
        BigInteger cl_rho2 = cl_rho.multiply(cl_rho).mod(this.n);
        BigInteger cl_alpha = HashUtils.hash(cs, this.Y).mod(this.n);

        BigInteger cl_y = HashUtils.hash(Arrays.asList(cF), cl_alpha).mod(this.n);
        BigInteger cl_z = HashUtils.hash(Arrays.asList(cF), cl_y).mod(this.n);

        BigInteger cl_z2 = cl_z.multiply(cl_z).mod(this.n);

        List<BigInteger> cl_yn = Lists.newLinkedList();
        List<BigInteger> cl_y_n = Lists.newLinkedList();
        List<BigInteger> entityList = Collections.nCopies(nbits, BigInteger.ONE);

        for(int i = 0; i < nbits; i++){
            cl_yn.add(cl_y.modPow(BigInteger.valueOf(i), this.n));
            cl_y_n.add(cl_y.modPow(BigInteger.valueOf(i), this.n).modInverse(this.n));
        }

        long s = System.nanoTime();

        // check equation 1;

        Commitment eqn1_1 = this.commiter.commitTo(t1hat, tao_rho);

        VectorB BsYs =  VectorB.from(Bs, this.n).mul(VectorB.from(cl_y_n, this.n));
        VectorB alpha_y_z_1 = VectorB.from(cl_yn, this.n).mulConstant(cl_alpha).add(VectorB.from(entityList,this.n).mulConstant(cl_z.modInverse(this.n)));

        BigInteger delta_yz = VectorB.from(cl_yn, this.n).innerProd(VectorB.from(entityList, this.n)).multiply(cl_z2).mod(this.n);
        BigInteger delta_yz_right = BsYs.innerProd(alpha_y_z_1);
        delta_yz = delta_yz.add(delta_yz_right);

        Commitment eqn1_2 = this.Y
                            .add(this.commiter.mulG(delta_yz))
                            .add(this.commiter.mulG(cl_z.multiply(t2hat).mod(this.n)))
                            .add(bigt1.mul(cl_rho))
                            .add(bigt2.mul(cl_rho2));


        Boolean b1 = eqn1_1.equals(eqn1_2);

        Commitment eqn2_1 = bigw1.add(this.commiter.mulH(rw1));
        Commitment eqn2_2 = (cF.add(cS1.mul(cl_rho))).mul(cl_z2)
                            .add(this.gs.mulBAndSum(
                                    BsYs
                            ))
                            .add(cD)
                            .add(cS2.mul(cl_rho))
                            .add(this.gs.mulBAndSum(
                                    VectorB.from(entityList, this.n)
                                            .mulConstant(cl_alpha)
                                            .add(VectorB.from(cl_y_n, this.n)
                                                    .mulConstant(cl_z.modInverse(this.n)
                                                    )
                                            )
                                    )
                            );
        Boolean b2 = eqn2_1.equals(eqn2_2);

        // check equation 3;
        Commitment eqn3_1 = bigw2.add(this.commiter.mulH(rw2));
        List<BigInteger> lamda_alpha = Lists.newLinkedList();
        for(int i = 0; i < B.intValue(); i++){
            lamda_alpha.add((cl_alpha.add(BigInteger.valueOf(i))).modInverse(this.n));
        }
        System.out.println(lamda_alpha);
        Commitment eqn3_2 = cM.add(cS3.mul(cl_rho)).add(this.gs.mulBAndSum(VectorB.from(lamda_alpha, this.n)));
        Boolean b3 = eqn3_1.equals(eqn3_2);


        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            vtime += (e - s);
        }

        return b1 && b2 && b3;
    }


    public boolean verifyip(int nbits, VectorP gs, VectorP hs, Commitment z, VectorB ls, VectorB rs){
        int nbits_prime = nbits;
        VectorP gprime = gs;
        VectorP hprime = hs;
        VectorB lprime = ls;
        VectorB rprime = rs;
        Commitment zprime = z;
        Commitment K = this.gs.getList().get(0);
        if(nbits_prime == 1){
            BigInteger l_final = lprime.sum();
            BigInteger r_final = rprime.sum();
            BigInteger t_hat = l_final.multiply(r_final);
            Commitment G_0H_0 = gprime.getList().get(0).mul(l_final).add(hprime.getList().get(0).mul(r_final));
            Commitment K_t_hat = K.mul(t_hat);
            return (G_0H_0.add(K_t_hat)).equals(zprime);
        }else{
            int size = nbits_prime;
            int halfSize = size / 2;

            VectorP gL = gprime.subVector(0, halfSize);
            VectorP gR = gprime.subVector(halfSize, size);

            VectorP hL = hprime.subVector(0, halfSize);
            VectorP hR = hprime.subVector(halfSize, size);

            VectorB lL = ls.subVector(0, halfSize);
            VectorB lR = ls.subVector(halfSize, size);

            VectorB rL = rs.subVector(0, halfSize);
            VectorB rR = rs.subVector(halfSize, size);

            BigInteger t_1_hat = lR.innerProd(rL);
            BigInteger t_2_hat = lL.innerProd(rR);

            Commitment L = gL.mulBAndSum(lR).add(hR.mulBAndSum(rL)).add(K.mul(t_1_hat));
            Commitment R = gR.mulBAndSum(lL).add(hL.mulBAndSum(rR)).add(K.mul(t_2_hat));

            BigInteger c = HashUtils.hash(Arrays.asList(L, R)).mod(this.n);

            BigInteger c2 = c.multiply(c).mod(this.n);

            lprime = lL.add(lR.mulConstant(c));
            rprime = rL.mulConstant(c).add(rR);

            gprime = gL.mulB(c).addP(gR);
            hprime = hL.addP(hR.mulB(c));
            zprime = L.mul(c2).add(z.mul(c)).add(R);

            return verifyip(nbits_prime/2, gprime, hprime, zprime, lprime, rprime);
        }
    }

}


