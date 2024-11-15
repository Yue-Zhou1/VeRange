package zkp;

import commitment.Commiter;
import commitment.Commitment;
import config.BouncyKey;
import config.Config;
import structure.VectorB;
import structure.VectorP;
import utils.HashUtils;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;


public class PolynomialCommitment {
    private static VectorP gs;
    private static BigInteger nn;
    private static Commiter commiter;
    private static List<BigInteger> bs = new LinkedList<>();

    public static void init() {
        Config.getInstance().init(new BouncyKey("bn128"));
        commiter = Config.getInstance().getCommiter();
        nn = Config.getInstance().getKey().getN();
    }

    public static void setup(int N){
        init();
        gs = VectorP.from(IntStream.range(0, N).mapToObj(
                        i -> commiter.mulG(HashUtils.hash(BigInteger.valueOf(71).add(BigInteger.valueOf(i))).mod(nn)))
                .collect(Collectors.toList()));
//        gs = VectorP.from(commiter.getGs().subList(0, N));
        for(int i = 0; i < 2*N; i++){
            bs.add(commiter.rand());
        }
    }

    public static BigInteger[][] setupHijs(Polynomial F, int m, int n){
        List<BigInteger> coeffs = F.getCoeffs();
        int eta =  (coeffs.size()-1-m*n);
        BigInteger[][] hijs_new = new BigInteger[m+1][n+1];
        if(eta == 0){
            hijs_new[0][0] = coeffs.get(0).subtract(bs.get(0));
        }else{
            hijs_new[0][0] = coeffs.get(0);
        }
        for(int i = 1; i <= n; i++){
            hijs_new[0][i] = bs.get(i-1);
        }

        for(int i = 1; i <= m; i++){
            if(eta == 0){
                hijs_new[i][0] = BigInteger.ZERO;
            }else{
                if (i < eta){
                    hijs_new[i][0] = coeffs.get(i);
                }else if(i == eta){
                    hijs_new[i][0] = coeffs.get(i).subtract(bs.get(0));
                }else{
                    hijs_new[i][0] = BigInteger.ZERO;
                }
            }
        }

        int flag = eta;
        for(int i = 1; i <= n; i++){
            for(int j = 1; j <= m; j++){
                flag++;
                hijs_new[j][i] = coeffs.get(flag);

            }
        }

        for(int i = 1; i <= n; i++){
            if(i != n){
                hijs_new[m][i] = hijs_new[m][i].subtract(bs.get(i));
            }

        }
        return hijs_new;
    }

    public static BigInteger[][] setupHijs_bs(Polynomial F, int m, int n, int degree, List<BigInteger> bs){
        List<BigInteger> coeffs = F.getCoeffs();
        while (coeffs.size()<=degree){
            coeffs.add(BigInteger.ZERO);
        }
        int eta =  (coeffs.size()-1-m*n);
        BigInteger[][] hijs_new = new BigInteger[m+1][n+1];
        if(eta == 0){
            hijs_new[0][0] = coeffs.get(0).subtract(bs.get(0));
        }else{
            hijs_new[0][0] = coeffs.get(0);
        }
        for(int i = 1; i <= n; i++){
            hijs_new[0][i] = bs.get(i-1);
        }

        for(int i = 1; i <= m; i++){
            if(eta == 0){
                hijs_new[i][0] = BigInteger.ZERO;
            }else{
                if (i < eta){
                    hijs_new[i][0] = coeffs.get(i);
                }else if(i == eta){
                    hijs_new[i][0] = coeffs.get(i).subtract(bs.get(0));
                }else{
                    hijs_new[i][0] = BigInteger.ZERO;
                }
            }
        }

        int flag = eta;
        for(int i = 1; i <= n; i++){
            for(int j = 1; j <= m; j++){
                flag++;
                hijs_new[j][i] = coeffs.get(flag);

            }
        }

        for(int i = 1; i <= n; i++){
            if(i != n){
                hijs_new[m][i] = hijs_new[m][i].subtract(bs.get(i));
            }

        }
        return hijs_new;
    }

    public static List<Commitment> PolyCommit(Polynomial F, int m, int n){
        BigInteger[][] hijs_new = setupHijs(F, m, n);
        List<Commitment> Hs = new LinkedList<>();
        for (int i = 0; i <= m; i++){
            List<BigInteger> hij = new LinkedList<>();
            for(int j = 0; j <= n; j++){
                hij.add(hijs_new[i][j]);
            }
            Hs.add(gs.mulBAndSum(VectorB.from(hij, nn)));
        }
        return Hs;
    }

    public static List<Commitment> PolyCommit1(Polynomial F, int m, int n, int degree, List<BigInteger> bs){
        BigInteger[][] hijs_new = setupHijs_bs(F, m, n, degree, bs);

        List<Commitment> Hs = new LinkedList<>();
        List<List<BigInteger>> hijs = new LinkedList<>();
        for (int i = 0; i <= m; i++){
            List<BigInteger> hij = new LinkedList<>();
            for(int j = 0; j <= n; j++){
                hij.add(hijs_new[i][j]);
            }
            hijs.add(hij);
            Hs.add(gs.mulBAndSum(VectorB.from(hij, nn)));
        }
        return Hs;
    }

    public static List<BigInteger> PolyEval(Polynomial F, BigInteger xpoint, int m, int n){
        BigInteger[][] hijs_new = setupHijs(F, m, n);

        List<BigInteger> pi_Fx = new LinkedList<>();
        for (int i = 0; i <= n; i++){
            BigInteger fj = BigInteger.ZERO;
            for(int j = 0; j <= m; j++){
                fj = fj.add(hijs_new[j][i].multiply(xpoint.modPow(BigInteger.valueOf(j), nn)));
            }
            pi_Fx.add(fj.mod(nn));
        }
        return pi_Fx;
    }

    public static List<BigInteger> PolyEvalBatch(List<Polynomial> Fs, BigInteger xpoint, BigInteger rho, int m, int n, int degree, List<List<BigInteger>> bs){
//        List<List<BigInteger>> pi_Fs = new LinkedList<>();
        List<BigInteger> pi_F_batch = new LinkedList<>();
        BigInteger[] pi_Fx = new BigInteger[n+1];
        Arrays.fill(pi_Fx, BigInteger.ZERO);
        for(int i = 0; i < Fs.size(); i++) {
            BigInteger[][] hijs_new = setupHijs_bs(Fs.get(i), m, n, degree, bs.get(i));
            for (int j = 0; j <= n; j++){
                BigInteger fk = BigInteger.ZERO;
                for(int k = 0; k <= m; k++){
                    fk = fk.add(
                            rho.modPow(BigInteger.valueOf(i+1),nn)
                                    .multiply(hijs_new[k][j]
                                                    .multiply(xpoint
                                                            .modPow(BigInteger.valueOf(k), nn)
                                                    )
                                    )
                    );
                }
                pi_Fx[j] = pi_Fx[j].add(fk).mod(nn);
            }
        }
        pi_F_batch = Arrays.asList(pi_Fx);
        return pi_F_batch;
    }

    public static List<BigInteger> PolyEval1(Polynomial F, BigInteger xpoint, int m, int n, int degree, List<BigInteger> bs){
        BigInteger[][] hijs_new = setupHijs_bs(F, m, n, degree, bs);

        List<BigInteger> pi_Fx = new LinkedList<>();
        for (int i = 0; i <= n; i++){
            BigInteger fj = BigInteger.ZERO;
            for(int j = 0; j <= m; j++){
                fj = fj.add(hijs_new[j][i].multiply(xpoint.modPow(BigInteger.valueOf(j), nn)));
            }
            pi_Fx.add(fj.mod(nn));
        }
        return pi_Fx;
    }

    public static boolean PolyVerify(List<Commitment> cmF, BigInteger xpoint, BigInteger yvalue, List<BigInteger> pi, int N){
        int m = cmF.size()-1;
        int n = pi.size()-1;
        Commitment eqn1_1 = gs.mulBAndSum(VectorB.from(pi, nn));
        System.out.println("m: " + m);
        System.out.println("n: " + n);
        System.out.println("N: " + N);

        Commitment eqn1_2 = commiter.getIdentity();
        for(int i = 0; i <= m; i++){
            eqn1_2 = eqn1_2.add(cmF.get(i).mul(xpoint.modPow(BigInteger.valueOf(i), nn)));
        }

        int eta = N - m*n;
        Boolean b1 = eqn1_1.equals(eqn1_2);
        BigInteger eqn2_1 = pi.get(0);

        for(int i = 1; i < n+1; i++){
            BigInteger exp = BigInteger.valueOf((i-1)*m + eta);
            eqn2_1 = eqn2_1.add(pi.get(i).multiply(xpoint.modPow(exp, nn)));
        }
        eqn2_1 = eqn2_1.mod(nn);

        Boolean b2 = eqn2_1.equals(yvalue);

        return b1&&b2;
    }

    public static boolean PolyVerifyBatch(List<List<Commitment>> cmF, BigInteger xpoint, BigInteger rho, BigInteger yvalue, List<BigInteger> pi, int N){
        int m = cmF.get(0).size()-1;
        int n = pi.size()-1;
        Commitment eqn1_1 = gs.mulBAndSum(VectorB.from(pi, nn));

        Commitment eqn1_2 = commiter.getIdentity();
        for(int i = 0; i < cmF.size(); i++){
            for(int j = 0; j <= m; j++){
                eqn1_2 = eqn1_2.add(cmF.get(i).get(j).mul(rho.modPow(BigInteger.valueOf(i+1), nn).multiply(xpoint.modPow(BigInteger.valueOf(j), nn)).mod(nn)));
            }
        }

        int eta = N - m*n;
        Boolean b1 = eqn1_1.equals(eqn1_2);
        BigInteger eqn2_1 = pi.get(0);

        for(int i = 1; i < n+1; i++){
            BigInteger exp = BigInteger.valueOf((i-1)*m + eta);
            eqn2_1 = eqn2_1.add(pi.get(i).multiply(xpoint.modPow(exp, nn)));
        }
        eqn2_1 = eqn2_1.mod(nn);

        Boolean b2 = eqn2_1.equals(yvalue);

        return b1&&b2;
    }

}
