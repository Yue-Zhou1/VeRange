package zkp;

import java.math.BigInteger;
import java.util.LinkedList;
import java.util.List;
import utils.HashUtils;
import utils.SquareDecompUtils;
import commitment.Commitment;

public class BIC extends PedersenZKP{

    private static final long serialVersionUID = 1L;

    private final Commitment c;
    private final Commitment d;
    private final int Lambda = 80;
    private final int nbits;
    private final BigInteger x;
    private final BigInteger tao;
    private final BigInteger gamma;

    private final List<BigInteger> xList = new LinkedList<>();
    private final List<Commitment> cList = new LinkedList<>();
    private final List<BigInteger> rList = new LinkedList<>();
    private final List<BigInteger> mList = new LinkedList<>();
    private final List<BigInteger> sList = new LinkedList<>();
    private final List<Commitment> dList = new LinkedList<>();

    private final BigInteger hashRes;

    // Resp parameters
    private final List<BigInteger> zList = new LinkedList<>();
    private final List<BigInteger> tList = new LinkedList<>();

    public static int counter = 0;
    public static long ptime = 0;
    public static long vtime = 0;

    public BIC(BigInteger x, int nbits){
        this.nbits = nbits;
        this.x = x;

        long s = System.nanoTime();

        BigInteger B = BigInteger.TWO.pow(nbits);

        BigInteger r = this.commiter.randWithBits(2*Lambda);

        this.c = this.commiter.mulG(this.x).add(this.commiter.mulH(r));

        // 4x(B-x)+1
        BigInteger BsubX = B.subtract(this.x);
        BigInteger IntegerDecomposed = this.x.multiply(BsubX).multiply(BigInteger.valueOf(4)).add(BigInteger.ONE);

        //x1, x2, x3
        BigInteger[] decompResult = SquareDecompUtils.decompose(IntegerDecomposed);
        xList.add(BsubX);

        BigInteger r0 = r.negate();
        rList.add(r0);
        BigInteger x0 = BsubX;
        Commitment c0 = c.mul(BigInteger.ONE.negate()).add(this.commiter.mulG(B));
        cList.add(c0);

        for(int i=0;i<3; i++){
            xList.add(decompResult[i+1]);
            BigInteger ri = this.commiter.randWithBits(2*Lambda);
            rList.add(ri);
            cList.add(this.commiter.mulG(decompResult[i+1]).add(this.commiter.mulH(ri)));
        }

        for(int i=0; i<4; i++){
            // mi: [0, BCL], s_i: [0, SCL]
            BigInteger mi = this.commiter.randWithBits(2*Lambda+nbits);
            BigInteger si = this.commiter.randWithBits(4*Lambda);
            mList.add(mi);
            sList.add(si);
            Commitment di = this.commiter.mulG(mi).add(this.commiter.mulH(si));
            dList.add(di);
        }

        // sigma: [0, 4SBCL]
        BigInteger sigma = this.commiter.randWithBits(4*Lambda+34);
        Commitment prodCm = this.commiter.getIdentity();
        for(int i=1; i<4; i++){
            prodCm = prodCm.add(cList.get(i).mul(mList.get(i).negate()));
        }

        // init challenge
        this.d = this.commiter.mulH(sigma).add(c.mul(BigInteger.valueOf(4).multiply(mList.get(0)))).add(prodCm);

        gamma = this.commiter.randWithBits(Lambda);
        //sum of (x_i*r_i) + 4x_0*r_0
        BigInteger sumXr = BigInteger.valueOf(4).multiply(x0.multiply(r0)).mod(this.n);
        // Resp

        for (int i=0; i<4; i++){
            zList.add(mList.get(i).add(gamma.multiply(xList.get(i))).mod(this.n));
            tList.add(sList.get(i).add(gamma.multiply(rList.get(i))).mod(this.n));
        }
        hashRes = HashUtils.hash(dList, d);
        for(int i=1; i<4; i++){
            sumXr = sumXr.add(xList.get(i).multiply(rList.get(i)).mod(this.n));
        }
        tao = sigma.add(gamma.multiply(sumXr)).mod(this.n);

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            ptime += (e - s);
        }

        counter++;

//        for(Commitment c : cList){
//            System.out.println(c.getCoordList());
//        }

    }


    @Override
    public boolean verify(){
        long s = System.nanoTime();

        BigInteger B = BigInteger.TWO.pow(nbits);
        Commitment c0 = c.mul(BigInteger.ONE.negate()).add(this.commiter.mulG(B));
        List<Commitment> fList = new LinkedList<>();

//        ArrayList<String> data = new ArrayList<>();
        for(int i=0; i<4; i++){
            Commitment fi = this.commiter.mulG(zList.get(i)).add(this.commiter.mulH(tList.get(i))).add(cList.get(i).mul(gamma.negate()));
            fList.add(fi);
//            data.add(fi.getSum().toString());
        }
        Commitment prodCz = this.commiter.getIdentity();

        for(int i=1; i<4; i++){
            prodCz = prodCz.add(cList.get(i).mul(zList.get(i).negate()));
        }
        Commitment f = this.commiter.mulH(tao).add(this.commiter.mulG(gamma)).add(c.mul(BigInteger.valueOf(4).multiply(zList.get(0)))).add(prodCz);

        Boolean con1 = true;
        Boolean con2 = false;

//        String key = f.getSum().toString();

//        List<String> delta = new LinkedList<>();
        BigInteger delta_value;

        for(int i=0; i<4; i++){
//            delta.add(HashUtils.HMAC("HmacSHA512", data.get(i), key));
            BigInteger upperBound = BigInteger.valueOf(2).pow(nbits+2*Lambda).add(BigInteger.valueOf(2).pow(nbits+Lambda));
            if(zList.get(i).subtract(upperBound).signum() == 1){
                con1 = false;
            }
            if(zList.get(i).signum() == -1){
                con1 = false;
            }
        }

        delta_value = HashUtils.hash(fList, f);
        if(delta_value.equals(hashRes)) con2 = true;

        long e = System.nanoTime();

        if (counter >= TestConstants.WARMUPS) {
            vtime += (e - s);
        }
        return con1 && con2;
    }

}
