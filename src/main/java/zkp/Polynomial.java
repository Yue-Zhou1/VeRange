package zkp;

import java.math.BigInteger;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;

import utils.BigIntegerUtils;
import utils.NTT;

public class Polynomial{
    private BigInteger[] coef;  // coefficients
    private BigInteger deg;     // degree of polynomial (0 for the zero polynomial)
    private BigInteger x[]; // Interpolating points;
    private static BigInteger thisn = new BigInteger("21888242871839275222246405745257275088548364400416034343698204186575808495617");
    private static BigInteger pp = new BigInteger("21888242871839275222246405745257275088548364400416034343698204186575808495617");
    private static BigInteger root = new BigInteger("5");

    // a * x^b
    public Polynomial( BigInteger a, BigInteger b ){
        coef = new BigInteger[ b.add(BigInteger.ONE).intValue() ];
        for(int i = 0; i < coef.length; i++){
            coef[i] = BigInteger.ZERO;
        }
        coef[ b.intValue() ] = a;
        deg = degree();
    }

    public Polynomial( Polynomial p ){
        coef = new BigInteger[ p.coef.length ];
        for( int i = 0; i < p.coef.length; i++ ){
            coef[ i ] = p.coef[ i ];
        }
        deg = p.degree();
    }

    public Polynomial( List<BigInteger> coeffs ){
        coef = new BigInteger[ coeffs.size() ];
        for( int i = 0; i < coeffs.size(); i++ ){
            coef[ i ] = coeffs.get(i);
        }
        deg = degree();
    }

    public static Polynomial LagrangePolynomial(List<BigInteger> xcoord, int n){
//        BigInteger modn = new BigInteger(thisn);
        Polynomial term1 = new Polynomial(BigInteger.ONE, BigInteger.ZERO);
        BigInteger term_cons = BigInteger.ONE;
        for (int i = 0; i < xcoord.size(); i++)
        {
            if(!xcoord.get(n).equals(xcoord.get(i))){
                Polynomial frac1_1 = new Polynomial(BigInteger.ONE, BigInteger.ONE);
                Polynomial frac1_2 = new Polynomial(xcoord.get(i), BigInteger.ZERO);
                Polynomial frac1_sum = frac1_1.minus(frac1_2);
                term1 = term1.times(frac1_sum);
                BigInteger term_cons1 = xcoord.get(n).subtract(xcoord.get(i)).mod(thisn);
                term_cons = term_cons.multiply(term_cons1).mod(thisn);
            }

        }

        Polynomial term2 = new Polynomial(term_cons, BigInteger.ZERO);

        return term1.divides(term2);
    }

    public static Polynomial l0X(List<BigInteger> zi){
        Polynomial term1_1 = new Polynomial(BigInteger.ONE, BigInteger.ONE);

        Polynomial term1_2 = new Polynomial(zi.get(0), BigInteger.ZERO);

        Polynomial term_sum = term1_1.minus(term1_2);
        for(int i = 1; i < zi.size(); i++){
            Polynomial term2_2 = new Polynomial(zi.get(i), BigInteger.ZERO);
            Polynomial term2 = term1_1.minus(term2_2);
            term_sum = term_sum.times(term2);
        }
        return term_sum;
    }

    // return the degree of this polynomial (0 for the zero polynomial)
    public BigInteger degree(){
        BigInteger d = BigInteger.ZERO;
        for( int i = 0; i < coef.length; i++ ) {
            if (coef[i].intValue() != 0) {
                d = BigInteger.valueOf(i);
            }
        }
        return d;
    }

    // return c = a + b
    public Polynomial plus( Polynomial b){
//        BigInteger modn = new BigInteger(thisn);
        Polynomial a = this;
        Polynomial c = new Polynomial( BigInteger.ZERO, BigInteger.valueOf(Math.max( a.deg.intValue(), b.deg.intValue() ) ));
        for( int i = 0; i <= a.deg.intValue(); i++ ) c.coef[ i ] = c.coef[i].add(a.coef[ i ]).mod(thisn);
        for( int i = 0; i <= b.deg.intValue(); i++ ) c.coef[ i ] = c.coef[i].add(b.coef[ i ]).mod(thisn);
        c.deg = c.degree();
        return c;
    }

    // return (a - b)
    public Polynomial minus( Polynomial b){
//        BigInteger modn = new BigInteger(thisn);
        Polynomial a = this;
        Polynomial c = new Polynomial( BigInteger.ZERO, BigInteger.valueOf(Math.max( a.deg.intValue(), b.deg.intValue() )) );
        for( int i = 0; i <= a.deg.intValue(); i++ ) c.coef[ i ] = c.coef[i].add(a.coef[ i ]);
        for( int i = 0; i <= b.deg.intValue(); i++ ) c.coef[ i ] = c.coef[i].subtract(b.coef[ i ]);
        c.deg = c.degree();
        return c;
    }

    // return (a * b)
    public Polynomial times( Polynomial b){
//        BigInteger modn = new BigInteger(thisn);
        Polynomial a = this;
        Polynomial c = new Polynomial( BigInteger.ZERO, a.deg.add(b.deg) );
        for( int i = 0; i <= a.deg.intValue(); i++ )
            for( int j = 0; j <= b.deg.intValue(); j++ )
                c.coef[i+j] =c.coef[i+j].add(( a.coef[i].multiply(b.coef[j]) )).mod(thisn);
        c.deg = c.degree();
        return c;
    }

    // get the coefficient for the highest degree
    public BigInteger coeff(){return coeff( degree() ); }

    // get the coefficient for degree d
    public BigInteger coeff( BigInteger degree ){
        if( degree.compareTo(this.degree())>0 ) throw new RuntimeException( "bad degree" );
        return coef[ degree.intValue() ];
    }

    public List<BigInteger> getCoeffs(){
        List<BigInteger> coefs = new LinkedList<>();
        for(int i = 0; i <coef.length ; i++){
            coefs.add(coef[i]);
        }
        return coefs;
    }

    public Polynomial divides(Polynomial b) {
        Polynomial a = this;
//        BigInteger modn = new BigInteger(thisn);
        if ((b.deg.compareTo(BigInteger.ZERO) == 0) && (b.coef[0].compareTo(BigInteger.ZERO) == 0))
            throw new RuntimeException("Divide by zero polynomial");
        List<BigInteger> coefficients = new LinkedList<>();
        if(b.deg.compareTo(BigInteger.ZERO) == 0){
            for(int i = 0; i <= a.deg.intValue(); i++){
                coefficients.add(a.coef[i].multiply(b.coef[0].modInverse(thisn)).mod(thisn));
            }
            Polynomial c = new Polynomial(coefficients);
            return c;
        }
        if (a.deg.compareTo(b.deg) < 0) return new Polynomial(BigInteger.ZERO, BigInteger.ZERO);

        BigInteger coefficient = a.coef[a.deg.intValue()].multiply(b.coef[b.deg.intValue()].modInverse(thisn)).mod(thisn);
        BigInteger exponent = a.deg.subtract(b.deg);
        Polynomial c = new Polynomial(coefficient, exponent);
        return c.plus( (a.minus(b.times(c)).divides(b)));
    }

    public Polynomial[] longdivide(Polynomial v) {
        Polynomial q = new Polynomial(BigInteger.ZERO, BigInteger.ZERO);
        Polynomial r = this;
        BigInteger lcv = v.getCoeffs().get(v.getCoeffs().size()-1);
        BigInteger dv = v.deg;

        while ( r.deg.intValue() >= dv.intValue() ) {
            BigInteger lcr = r.getCoeffs().get(r.getCoeffs().size()-1);

            BigInteger s = lcr.multiply(lcv.modInverse(thisn));

            Polynomial term = new Polynomial(s, r.deg.subtract(dv).mod(thisn));
            q = q.plus(term);
            Polynomial term_ne = new Polynomial(s.negate(), r.deg.subtract(dv).mod(thisn));
            r = r.plus(v.times(term_ne));
        }
        return new Polynomial[] {q, r};
    }

    // test wether or not this polynomial is zero
    public boolean isZero(){
        for( BigInteger i : coef ){
            if( i.intValue() != 0 ) return false;
        }//end for
        return true;
    }

    // use Horner's method to compute and return the polynomial evaluated at x
    public BigInteger evaluate( BigInteger x){
        BigInteger p = BigInteger.ZERO;

        for( int i = deg.intValue(); i >= 0; i-- ) {
            p = coef[i].add((x.multiply(p)));
        }
        return p.mod(thisn);
    }

    // convert to string representation
    public String toString(){
        if( deg.compareTo(BigInteger.ZERO) == 0 ) return "" + coef[ 0 ];
        if( deg.compareTo(BigInteger.ONE) == 0 ) return coef[ 1 ] + "x + " + coef[ 0 ];
        String s = coef[ deg.intValue() ] + "x^" + deg;
        for( int i = deg.intValue() - 1; i >= 0; i-- ){
            if( coef[ i ].compareTo(BigInteger.ZERO) == 0 ){
                continue;
//                s = s + "x";
            }else if( coef[ i ].compareTo(BigInteger.ZERO) > 0 ){
                s = s + " + " + ( coef[ i ] );
            }else if( coef[ i ].compareTo(BigInteger.ZERO) < 0 ) s = s + " - " + ( coef[ i ].negate() );
            if( i == 1 ){
                s = s + "x";
            }else if( i > 1 ) s = s + "x^" + i;
        }
        return s;
    }


    // Type-4 related polynomial operations below:
    public static Polynomial computeF(List<BigInteger> domain, BigInteger z, BigInteger r){
        List<BigInteger> coeffs = new LinkedList<>();
        coeffs.add(z);
        coeffs.add(r);
        for(int i = 0; i < domain.size()-2; i++){
            coeffs.add(BigInteger.ZERO);
        }

        NTT.ntt(coeffs, true, pp, root);
        Polynomial f = new Polynomial(coeffs);
        return f;
    }

    public static Polynomial computeG(List<BigInteger> domain, BigInteger z, BigInteger alpha, BigInteger beta, BigInteger B){
        List<BigInteger> narys = BigIntegerUtils.decomposeToNary(z, B);
        List<BigInteger> evaluations = new LinkedList<>();
        BigInteger z_n_minus_1 = narys.get(narys.size()-1);
        evaluations.add(z_n_minus_1);

        BigInteger pre_eval = z_n_minus_1;
        for(int i = narys.size()-2; i>=0; i--){
            BigInteger eval = B.multiply(pre_eval).add(narys.get(i));
            evaluations.add(eval);
            pre_eval = eval;
        }
        Collections.reverse(evaluations);
        while(evaluations.size()<domain.size()){
            evaluations.add(BigInteger.ZERO);
        }
        List<BigInteger> g_evals = evaluations;

//        List<BigInteger> g_evals = new LinkedList<>();
//        int extendedDomainExp = log2(domain.size())+1;
//        BigInteger exDomainSize = BigInteger.TWO.pow(extendedDomainExp);
//        int j = 0;
//        for(int i = 0; i<exDomainSize.intValue(); i++){
//            if(i == 1){
//                g_evals.add(alpha);
//            }else if(i==3){
//                g_evals.add(beta);
//            }else if(i % 2 == 0){
//                g_evals.add(evaluations.get(j));
//                j++;
//            }else{
//                g_evals.add(BigInteger.ZERO);
//            }
//        }
        NTT.ntt(g_evals, true, pp, root);
        return new Polynomial(g_evals);

    }

    public static Polynomial computeW1(List<BigInteger> domain, Polynomial g, Polynomial f){
        // polynomial: P(x) = x^n - 1
        Polynomial x_n_minus_1 = vanishPoly(domain);

        // polynomial: P(x) = x - 1
        List<BigInteger> coeffs_x_minus_1 = new LinkedList<>();
        coeffs_x_minus_1.add(BigInteger.ONE.negate());
        coeffs_x_minus_1.add(BigInteger.ONE);
        Polynomial x_minus_1 = new Polynomial(coeffs_x_minus_1);

        Polynomial g_minus_f = g.minus(f);

        return g_minus_f.times(x_n_minus_1).longdivide(x_minus_1)[0];
    }

    public static Polynomial computeW2(List<BigInteger> domain, Polynomial g, BigInteger B){
        // polynomial: P(x) = x^n - 1
        Polynomial x_n_minus_1 = vanishPoly(domain);

        BigInteger w_n_minus_1 = domain.get(domain.size()-1);
        // polynomial: P(x) = x - w^(n-1)
        List<BigInteger> coeffs_x_minus = new LinkedList<>();
        coeffs_x_minus.add(w_n_minus_1.negate());
        coeffs_x_minus.add(BigInteger.ONE);
        Polynomial x_minus_w_n_minus_1 = new Polynomial(coeffs_x_minus);

        // polynomial: P(x) = 1
        Polynomial poly_temp = g;
        for(int i = 1; i < B.intValue(); i++){
            Polynomial poly_i = new Polynomial(BigInteger.valueOf(i), BigInteger.ZERO);
            poly_temp = poly_temp.times(poly_i.minus(g));
        }
        return poly_temp.times(x_n_minus_1).longdivide(x_minus_w_n_minus_1)[0];
    }

    public static Polynomial computeW3(List<BigInteger> domain, List<BigInteger> domain_2n, Polynomial g){

        List<BigInteger> g_coeffs = g.getCoeffs();
        NTT.ntt(g_coeffs, false, pp, root);
        List<BigInteger> g_evals = new LinkedList<>();
        int extendedDomainSize = 2*domain_2n.size();
        List<BigInteger> domain_4n = NTT.domain(extendedDomainSize);

        List<BigInteger> w3_evals = new LinkedList<>();
        BigInteger w_n_minus_1 = domain.get(domain.size()-1);
        BigInteger w_root = domain.get(1);
        for(int i = 0; i < domain_4n.size(); i++){
            BigInteger g_x = g.evaluate(domain_4n.get(i));
            BigInteger g_xw = g.evaluate(domain_4n.get(i).multiply(w_root));
            BigInteger part_a = g_x.subtract(BigInteger.TWO.multiply(g_xw));
            BigInteger part_b = BigInteger.ONE.subtract(part_a);
            BigInteger part_c = domain_4n.get(i).subtract(w_n_minus_1);
            w3_evals.add(part_a.multiply(part_b).multiply(part_c).mod(pp));
        }

        NTT.ntt(w3_evals, true, pp, root);
        return new Polynomial(w3_evals);
    }


    public static Polynomial computeQ(List<BigInteger> domain, Polynomial w1, Polynomial w2, Polynomial w3, BigInteger tau){
        Polynomial poly_tau = new Polynomial(tau, BigInteger.ZERO);
        Polynomial poly_tau_2 = new Polynomial(tau.pow(2), BigInteger.ZERO);

        // linear combinatin of w1, w2, w3

        Polynomial poly_lc = w1.plus(w2.times(poly_tau)).plus(w3.times(poly_tau_2));

        Polynomial x_n_minus_1 = vanishPoly(domain);

        return poly_lc.longdivide(x_n_minus_1)[0];
    }

    public static Polynomial computeWCap(List<BigInteger> domain, Polynomial f, Polynomial q, BigInteger rho){
        BigInteger rho_n_minus_1 = rho.modPow(BigInteger.valueOf(domain.size()),thisn).subtract(BigInteger.ONE);
        BigInteger rho_n_minus_1_by_rho_minus_1 = rho_n_minus_1.multiply(rho.subtract(BigInteger.ONE).modInverse(thisn)).mod(thisn);

        Polynomial rho_poly_1 = new Polynomial(rho_n_minus_1_by_rho_minus_1, BigInteger.ZERO);
        Polynomial rho_poly_2 = new Polynomial(rho_n_minus_1, BigInteger.ZERO);

        return f.times(rho_poly_1).plus(q.times(rho_poly_2));
    }

    // batch verification below:
    public static Polynomial computeFprime(Polynomial g, int B){
        Polynomial poly_temp = g;
        for(int i = 1; i < B; i++){
            Polynomial poly_i = new Polynomial(BigInteger.valueOf(i), BigInteger.ZERO);
            poly_temp = poly_temp.times(poly_i.minus(g));
        }
        return poly_temp;
    }

    public static Polynomial computeP(List<BigInteger> domain, Polynomial fx, Polynomial fprimex, BigInteger theta){
        // polynomial: P(x) = x^n - 1
        Polynomial x_n_minus_1 = vanishPoly(domain);

        BigInteger w_n_minus_1 = domain.get(domain.size()-1);
        // polynomial: P(x) = x - w^(n-1)
        List<BigInteger> coeffs_x_minus = new LinkedList<>();
        coeffs_x_minus.add(w_n_minus_1.negate());
        coeffs_x_minus.add(BigInteger.ONE);
        Polynomial x_minus_w_n_minus_1 = new Polynomial(coeffs_x_minus);
        Polynomial poly_cons = new Polynomial(theta, BigInteger.ZERO);

        Polynomial fx_part = fx.times(x_minus_w_n_minus_1).longdivide(x_n_minus_1)[0];
        Polynomial fprimex_part = fprimex.times(poly_cons).longdivide(x_minus_w_n_minus_1)[0];

        return fx_part.plus(fprimex_part);
    }

    public static List<Polynomial> computeRs(BigInteger g_x, BigInteger g_wx, BigInteger b, BigInteger p_x, BigInteger x, BigInteger wx){
        BigInteger one = BigInteger.ONE;
        // polynomial: P(X) = X - 1
        List<BigInteger> coeffs_x_minus_1 = new LinkedList<>();
        coeffs_x_minus_1.add(one.negate());
        coeffs_x_minus_1.add(one);
        Polynomial x_minus_1 = new Polynomial(coeffs_x_minus_1);

        // polynomial: P(X) = X - wx
        List<BigInteger> coeffs_x_minus_wx = new LinkedList<>();
        coeffs_x_minus_wx.add(wx.negate());
        coeffs_x_minus_wx.add(one);
        Polynomial x_minus_wx = new Polynomial(coeffs_x_minus_wx);

        // polynomial: P(X) = X - x
        List<BigInteger> coeffs_x_minus_x = new LinkedList<>();
        coeffs_x_minus_x.add(x.negate());
        coeffs_x_minus_x.add(one);
        Polynomial x_minus_x = new Polynomial(coeffs_x_minus_x);

        // constant polynomial: g_x/(x-1)(x-wx)
//        BigInteger x_1_x_wx = x.subtract(one).multiply(x.subtract(wx)).mod(thisn);
        BigInteger x_1_x_wx = x.subtract(wx).mod(thisn);
        BigInteger gx_divide_x_1_x_wx = g_x.multiply(x_1_x_wx.modInverse(thisn));
//        Polynomial cons_gx = new Polynomial(gx_divide_x_1_x_wx, BigInteger.ZERO);
        Polynomial cons_gx = new Polynomial(gx_divide_x_1_x_wx, BigInteger.ZERO);

        // constant polynomial: g_wx/(wx-1)(wx-x)
//        BigInteger wx_1_wx_x = wx.subtract(one).multiply(wx.subtract(x)).mod(thisn);
        BigInteger wx_1_wx_x = wx.subtract(x).mod(thisn);
        BigInteger gwx_divide_wx_1_wx_x = g_wx.multiply(wx_1_wx_x.modInverse(thisn));
//        Polynomial cons_gwx = new Polynomial(gwx_divide_wx_1_wx_x, BigInteger.ZERO);
        Polynomial cons_gwx = new Polynomial(gwx_divide_wx_1_wx_x, BigInteger.ZERO);

        // constant polynomial: b/(1-x)(1-wx)
        BigInteger x_1_wx_1 = one.subtract(x).multiply(one.subtract(wx)).mod(thisn);
        BigInteger b_divide_x_1_wx_1 = b.multiply(x_1_wx_1.modInverse(thisn));
//        Polynomial cons_b = new Polynomial(b_divide_x_1_wx_1, BigInteger.ZERO);
        Polynomial cons_b = new Polynomial(b, BigInteger.ZERO);

        // constant polynomial: p_x/(x-1)(x-wx)
        BigInteger px_divide_x_1_wx_1 = p_x.multiply(x_1_x_wx.modInverse(thisn));
//        Polynomial cons_px = new Polynomial(px_divide_x_1_wx_1, BigInteger.ZERO);
        Polynomial cons_px = new Polynomial(p_x, BigInteger.ZERO);
//        Polynomial r0_part1 = cons_gx.times(x_minus_1).times(x_minus_wx);
//        Polynomial r0_part2 = cons_gwx.times(x_minus_1).times(x_minus_x);
//        Polynomial r0 = r0_part1.plus(r0_part2);
//        Polynomial r1 = cons_b.times(x_minus_x).times(x_minus_wx);
//        Polynomial r2 = cons_px.times(x_minus_1).times(x_minus_wx);

        Polynomial r0_part1 = cons_gx.times(x_minus_wx);
        Polynomial r0_part2 = cons_gwx.times(x_minus_x);
        Polynomial r0 = r0_part1.plus(r0_part2);
        Polynomial r1 = cons_b;
        Polynomial r2 = cons_px;

        List<Polynomial> rs = new LinkedList<>(List.of(r0, r1, r2));

        return rs;

    }

//    public static List<Polynomial> computeRs(BigInteger g_x, BigInteger g_wx, BigInteger b, BigInteger p_x, BigInteger x, BigInteger wx){
//        BigInteger one = BigInteger.ONE;
//        // polynomial: P(X) = X - 1
//        List<BigInteger> coeffs_x_minus_1 = new LinkedList<>();
//        coeffs_x_minus_1.add(one.negate());
//        coeffs_x_minus_1.add(one);
//        Polynomial x_minus_1 = new Polynomial(coeffs_x_minus_1);
//
//        // polynomial: P(X) = X - wx
//        List<BigInteger> coeffs_x_minus_wx = new LinkedList<>();
//        coeffs_x_minus_wx.add(wx.negate());
//        coeffs_x_minus_wx.add(one);
//        Polynomial x_minus_wx = new Polynomial(coeffs_x_minus_wx);
//
//        // polynomial: P(X) = X - x
//        List<BigInteger> coeffs_x_minus_x = new LinkedList<>();
//        coeffs_x_minus_x.add(x.negate());
//        coeffs_x_minus_x.add(one);
//        Polynomial x_minus_x = new Polynomial(coeffs_x_minus_x);
//
//        // constant polynomial: g_x/(x-1)(x-wx)
//        BigInteger x_1_x_wx = x.subtract(one).multiply(x.subtract(wx)).mod(thisn);
//        BigInteger gx_divide_x_1_x_wx = g_x.multiply(x_1_x_wx.modInverse(thisn));
//        Polynomial cons_gx = new Polynomial(gx_divide_x_1_x_wx, BigInteger.ZERO);
//
//        // constant polynomial: g_wx/(wx-1)(wx-x)
//        BigInteger wx_1_wx_x = wx.subtract(one).multiply(wx.subtract(x)).mod(thisn);
//        BigInteger gwx_divide_wx_1_wx_x = g_wx.multiply(wx_1_wx_x.modInverse(thisn));
//        Polynomial cons_gwx = new Polynomial(gwx_divide_wx_1_wx_x, BigInteger.ZERO);
//
//        // constant polynomial: b/(1-x)(1-wx)
//        BigInteger x_1_wx_1 = one.subtract(x).multiply(one.subtract(wx)).mod(thisn);
//        BigInteger b_divide_x_1_wx_1 = b.multiply(x_1_wx_1.modInverse(thisn));
//        Polynomial cons_b = new Polynomial(b_divide_x_1_wx_1, BigInteger.ZERO);
//
//        // constant polynomial: p_x/(x-1)(x-wx)
//        BigInteger px_divide_x_1_wx_1 = p_x.multiply(x_1_x_wx.modInverse(thisn));
//        Polynomial cons_px = new Polynomial(px_divide_x_1_wx_1, BigInteger.ZERO);
//
//        Polynomial r0_part1 = cons_gx.times(x_minus_1).times(x_minus_wx);
//        Polynomial r0_part2 = cons_gwx.times(x_minus_1).times(x_minus_x);
//        Polynomial r0 = r0_part1.plus(r0_part2);
//        Polynomial r1 = cons_b.times(x_minus_x).times(x_minus_wx);
//        Polynomial r2 = cons_px.times(x_minus_1).times(x_minus_wx);
//
//
//        List<Polynomial> rs = new LinkedList<>(List.of(r0, r1, r2));
//
//        return rs;
//
//    }

    public static Polynomial computeF_batch(List<BigInteger> domain, Polynomial g, int B){
        int extendedDomainSize = B*domain.size();
//        int extendedDomainSize = B*2*domain.size();
        List<BigInteger> domain_Bn = NTT.domain(extendedDomainSize);

        List<BigInteger> w3_evals = new LinkedList<>();
        BigInteger w_root = domain.get(1);
        for(int i = 0; i < domain_Bn.size(); i++){
            BigInteger g_x = g.evaluate(domain_Bn.get(i));
            BigInteger g_xw = g.evaluate(domain_Bn.get(i).multiply(w_root));
            BigInteger part_a = g_x.subtract(BigInteger.valueOf(B).multiply(g_xw));
            BigInteger part_b = BigInteger.ONE;
            for(int j = 1; j < B; j++){
                part_b = part_b.multiply(BigInteger.valueOf(j).subtract(part_a));
            }
            w3_evals.add(part_a.multiply(part_b).mod(pp));
        }

        NTT.ntt(w3_evals, true, pp, root);
        return new Polynomial(w3_evals);
    }

    public static Polynomial computeQ_batch(List<Polynomial> rs, Polynomial g, Polynomial gprime, Polynomial p, BigInteger rho, BigInteger x, BigInteger wx){
        BigInteger one = BigInteger.ONE;
        Polynomial poly_rho = new Polynomial(rho, BigInteger.ZERO);
        Polynomial poly_rho_2 = new Polynomial(rho.multiply(rho).mod(thisn), BigInteger.ZERO);

        // polynomial: P(X) = X - x
        List<BigInteger> coeffs_x_minus_x = new LinkedList<>();
        coeffs_x_minus_x.add(x.negate());
        coeffs_x_minus_x.add(one);
        Polynomial x_minus_x = new Polynomial(coeffs_x_minus_x);

        // polynomial: P(X) = X - wx
        List<BigInteger> coeffs_x_minus_wx = new LinkedList<>();
        coeffs_x_minus_wx.add(wx.negate());
        coeffs_x_minus_wx.add(one);
        Polynomial x_minus_wx = new Polynomial(coeffs_x_minus_wx);

        // polynomial: P(X) = X - 1
        List<BigInteger> coeffs_x_minus_1 = new LinkedList<>();
        coeffs_x_minus_1.add(one.negate());
        coeffs_x_minus_1.add(one);
        Polynomial x_minus_1 = new Polynomial(coeffs_x_minus_1);

        Polynomial X_minus_x_X_minus_wx = x_minus_x.times(x_minus_wx);

        Polynomial part1 = g.minus(rs.get(0)).longdivide(X_minus_x_X_minus_wx)[0];
        Polynomial part2 = poly_rho.times(gprime.minus(rs.get(1))).longdivide(x_minus_1)[0];
        Polynomial part3 = poly_rho_2.times(p.minus(rs.get(2))).longdivide(x_minus_x)[0];

        return part1.plus(part2).plus(part3);

    }

    public static int log2(int n){
        if(n <= 0) throw new IllegalArgumentException();
        return 31 - Integer.numberOfLeadingZeros(n);
    }

    public static Polynomial vanishPoly(List<BigInteger> domain){
        // polynomial: P(x) = x^n - 1
        List<BigInteger> vanishing_coeffs = new LinkedList<>();
        for(int i = 0; i < domain.size()+1;i++){
            vanishing_coeffs.add(BigInteger.ZERO);
        }
//        vanishing_coeffs.set(0, pp.subtract(BigInteger.ONE));
        vanishing_coeffs.set(0, BigInteger.ONE.negate());
        vanishing_coeffs.set(domain.size(), BigInteger.ONE);
        Polynomial x_n_minus_1 = new Polynomial(vanishing_coeffs);
        return x_n_minus_1;
    }

    public static List<Integer> CalculateMatrix_mn(BigInteger degree){
//        int m = (int) Math.ceil(Math.sqrt(degree.intValue()));
//        int n = m;
//        if(degree.intValue()-m*n<0){
//            n--;
//            if(degree.intValue()-m*n<0){
//                m--;
//            }
//        }
        int m = (int) Math.ceil(Math.sqrt(degree.intValue()/3));
        int n = 3*m;
        if(degree.intValue()-m*n<0){
            m--;
            while(degree.intValue()-m*n<0){
                n--;
            }
        }
        List<Integer> mn = new LinkedList<>();
        mn.add(m);
        mn.add(n);
        return mn;
    }

    public static BigInteger[][] setupHijs(Polynomial F, int m, int n, int degree, List<BigInteger> bs){
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

    public static void main(String[] args) {
        List<Integer> mn = CalculateMatrix_mn(BigInteger.valueOf(59));
        System.out.println(mn);
    }
}