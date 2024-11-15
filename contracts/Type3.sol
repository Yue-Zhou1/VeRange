pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

contract Type3 {
    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Part1 {
        Point[] cys;
        Point[] cDs;
        Point[] cWs;
        Point cR1;
        Point cR2;
        Point[] cmS;
        Point[] cmB;
        uint256[] dv_value;
        uint256[] pi_sx;
        uint256[] pi_bx;
        uint256 yS;
        uint256 yB;
        uint256 eta1;
        uint256 eta2;
        uint256 l0;
        uint256[] luxs;
    }


    uint256 public constant GX = 1;
    uint256 public constant HX = 9727523064272218541460723335320998459488975639302513747055235660443850046724;
    uint256 public constant GY = 2;
    uint256 public constant HY = 5031696974169251245229961296941447383441169981934237515842977230762345915487;

    uint256 public constant PP = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47;
    uint256 internal constant NN = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001;

    uint256 private constant G1X = 10634783317254738331264664985092831821376621286957316213969233667217643505748;
    uint256 private constant G1Y = 3593894136463563836927847830567934263138852783181300960240960063050894828762;
    uint256 private constant G2X = 15079475582167754912076991092891442787377018931665830331839902737242049589194;
    uint256 private constant G2Y = 2994786716555744767228318303490139874612995833883024157075109374536749910797;
    uint256 private constant G3X = 5998026607105146355656696147154635391947958755485950806629481563238770197831;
    uint256 private constant G3Y = 9071495078062910622295597859043028737780972268539387614597098676522874141228;
    uint256 private constant G4X = 16523801944179829840198169326704340952379740918805059924608153703458536682146;
    uint256 private constant G4Y = 6038302089791721424956127465108246322480323157317821739578233683122154868638;
    uint256 private constant G5X = 12118397309736405059200255390820856272335184322136103314456373515523090868384;
    uint256 private constant G5Y = 3299068400210186031537495205944296158829957843773210327992629444571521214447;
    uint256 private constant G6X = 13569683714700597868720693802606454625784876902328159148425602640793284360016;
    uint256 private constant G6Y = 6196366122510722436252226379612342063576138922717891371699758504638443909810;
    uint256 private constant G7X = 6931749079582407422004463406148477362357661472338328270508373724106071518625;
    uint256 private constant G7Y = 21191302843410952823242789471696212271196775028224076743387772065534892367978;
    uint256 private constant G8X = 3818775040669594200577093816348913853129511932617816368937601194351220417610;
    uint256 private constant G8Y = 11457440080399767781624111424667375708460721196045291561100911147854033186883;
    uint256 private constant G9X = 16856378328816595579144346936425326333732976124866714942794663389927527942729;
    uint256 private constant G9Y = 8615147240200783032869516315617482186645983361490736606108099797176368213214;
    uint256 private constant G10X = 12329774238033643601677353420174969544762152665590909463734924154613306217386;
    uint256 private constant G10Y = 9349845555551207787110530432977444099219845456919646677037398830926083607248;
    uint256 private constant G11X = 15819385188090994752881147747997045863063544769706761541586812145860428725372;
    uint256 private constant G11Y = 17100833187243109040711678632824159418785961748506921615441797818807296019294;
    uint256 private constant G12X = 6331024371890361853402322881256862237801465545397987055303359505174258940787;
    uint256 private constant G12Y = 18571650698865115063332538959090545189492566919425356834234934402133902141366;
    uint256 private constant G13X = 7105985838448748955209439322478761515339875326287256355331470413548705695561;
    uint256 private constant G13Y = 18438173537644615561966541251456414954420156418562019789648878978333737520283;
    uint256 private constant G14X = 19021747983670190184630514322277256890031751525022704695742421493992173130370;
    uint256 private constant G14Y = 14483250175521306424840452131620408095676413044004913447506078614599443973015;
    uint256 private constant G15X = 12939943061190834182361325225540663473273714337397365880767331596410557127634;
    uint256 private constant G15Y = 21625625012093072106792496546825573499928322833210005959334070047102115545386;
    uint256 private constant G16X = 6590246720507402136630326950618777059699581927789913511934337513344857421023;
    uint256 private constant G16Y = 889573739332574916127485194837452655567667481149713859993862188238306657777;
    uint256 private constant G17X = 10634783317254738331264664985092831821376621286957316213969233667217643505748;
    uint256 private constant G17Y = 3593894136463563836927847830567934263138852783181300960240960063050894828762;
    uint256 private constant G18X = 15079475582167754912076991092891442787377018931665830331839902737242049589194;
    uint256 private constant G18Y = 2994786716555744767228318303490139874612995833883024157075109374536749910797;
    uint256 private constant G19X = 5998026607105146355656696147154635391947958755485950806629481563238770197831;
    uint256 private constant G19Y = 9071495078062910622295597859043028737780972268539387614597098676522874141228;
    uint256 private constant G20X = 16523801944179829840198169326704340952379740918805059924608153703458536682146;
    uint256 private constant G20Y = 6038302089791721424956127465108246322480323157317821739578233683122154868638;
    uint256 private constant G21X = 12118397309736405059200255390820856272335184322136103314456373515523090868384;
    uint256 private constant G21Y = 3299068400210186031537495205944296158829957843773210327992629444571521214447;
    uint256 private constant G22X = 13569683714700597868720693802606454625784876902328159148425602640793284360016;
    uint256 private constant G22Y = 6196366122510722436252226379612342063576138922717891371699758504638443909810;
    uint256 private constant G23X = 6931749079582407422004463406148477362357661472338328270508373724106071518625;
    uint256 private constant G23Y = 21191302843410952823242789471696212271196775028224076743387772065534892367978;
    uint256 private constant G24X = 3818775040669594200577093816348913853129511932617816368937601194351220417610;
    uint256 private constant G24Y = 11457440080399767781624111424667375708460721196045291561100911147854033186883;
    uint256 private constant G25X = 16856378328816595579144346936425326333732976124866714942794663389927527942729;
    uint256 private constant G25Y = 8615147240200783032869516315617482186645983361490736606108099797176368213214;
    uint256 private constant G26X = 12329774238033643601677353420174969544762152665590909463734924154613306217386;
    uint256 private constant G26Y = 9349845555551207787110530432977444099219845456919646677037398830926083607248;
    uint256 private constant G27X = 15819385188090994752881147747997045863063544769706761541586812145860428725372;
    uint256 private constant G27Y = 17100833187243109040711678632824159418785961748506921615441797818807296019294;
    uint256 private constant G28X = 6331024371890361853402322881256862237801465545397987055303359505174258940787;
    uint256 private constant G28Y = 18571650698865115063332538959090545189492566919425356834234934402133902141366;
    uint256 private constant G29X = 7105985838448748955209439322478761515339875326287256355331470413548705695561;
    uint256 private constant G29Y = 18438173537644615561966541251456414954420156418562019789648878978333737520283;
    uint256 private constant G30X = 19021747983670190184630514322277256890031751525022704695742421493992173130370;
    uint256 private constant G30Y = 14483250175521306424840452131620408095676413044004913447506078614599443973015;
    uint256 private constant G31X = 12939943061190834182361325225540663473273714337397365880767331596410557127634;
    uint256 private constant G31Y = 21625625012093072106792496546825573499928322833210005959334070047102115545386;
    uint256 private constant G32X = 6590246720507402136630326950618777059699581927789913511934337513344857421023;
    uint256 private constant G32Y = 889573739332574916127485194837452655567667481149713859993862188238306657777;

    uint256 private constant x = 2468447469808117898496964287025326671865182153500622601644710641121442226513;

    // uint256 L = 13; 
    // uint256 K = 13;
    // uint256 B = 10; 
    // uint256 nbits = 162;
    // uint256 mb = 11; 
    // uint256 nb = 11;
    // uint256 Nb = 126; 

    // uint256 ms = 4; 
    // uint256 ns = 3;
    // uint256 Ns = 13; 

    uint256 L = 10; 
    uint256 K = 10;
    uint256 B = 6; 
    uint256 nbits = 100;
    uint256 mb = 7; 
    uint256 nb = 7;
    uint256 Nb = 55; 

    uint256 ms = 3; 
    uint256 ns = 3;
    uint256 Ns = 10; 

    function mod(uint256 a, uint256 b)
        public
        pure
        returns (uint256)
    {
        require(b != 0, "SafeMath: modulo by zero");

        uint256 c;
        assembly {
            c := mod(a, b)
        }

        return c;
    }

    function pow(uint256 x, uint256 n) internal pure returns (uint256) {
        uint256 result = 1;
        while (n > 0) {
            if (n % 2 != 0) {
                result = mulmod(result, x, NN);
            }
            x = mulmod(x, x, NN);
            n /= 2;
        }
        return result;
    }

    function verifyRangeArgument(Part1 memory part1)
        external
        returns (uint256)
    {
        uint256[] memory betas = computeChallenges(part1);
        uint256[] memory gamma = computeGamma(part1.cys);

        checkPolyVerify(part1.cmB, part1.pi_bx, part1.yB, x, mb, nb, Nb);
        checkPolyVerify(part1.cmS, part1.pi_sx, part1.yS, x, ms, ns, Ns);

        checkCond1(part1);

        checkCond2(betas, part1);

        checkCond3(part1, gamma);

        checkCond4(part1.cys, part1.cWs, gamma);

    }

    function computeGamma(Point[] memory cys)
        internal
        returns(uint256[] memory)
    {
        uint ysize = cys.length;
        if (ysize == 1){
            return new uint256[](1);
        }
        uint256[] memory cs = new uint256[](2*ysize);
        uint offset = 0;
        for(uint i=0;i<ysize;i++) {
            Point memory p = cys[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        uint256[] memory gammas = new uint256[](ysize);
        uint256 gamma1 = mod(uint256(keccak256(abi.encodePacked(cs))), NN);
        uint256 gamma2 = mulmod(gamma1, gamma1, NN);
        gammas[0] = gamma1;
        gammas[1] = gamma2;
        if(ysize == 4){
            uint256 gamma3 = mulmod(gamma1, gamma2, NN);
            uint256 gamma4 = mulmod(gamma2, gamma2, NN);
            gammas[2] = gamma3;
            gammas[3] = gamma4;
        }

        return gammas;
    }

    function checkPolyVerify(Point[] memory cmp, uint256[] memory pi_px, uint256 y, uint256 x, uint256 m, uint256 n, uint256 N)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        uint256 size_G = n+1;
        Point[] memory gs = generateGS(size_G);
        Point memory p = gs[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = pi_px[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size_G;i++){
            Point memory p = gs[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = pi_px[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }
        checkPoly1(cmp, x, m);
        checkPoly2(pi_px,y,x, m, n, N);


    }

    function checkPoly1(Point[] memory cmp, uint256 x, uint256 m)
        internal
    {        
        
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        uint256 size_H = m+1;

        Point memory ph = cmp[0];
        mul_input[0] = ph.x;
        mul_input[1] = ph.y;
        mul_input[2] = 1;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size_H;i++){
            Point memory ph = cmp[i];
            mul_input[0] = ph.x;
            mul_input[1] = ph.y;
            mul_input[2] = pow(x, i);

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }
    }

    function checkPoly2(uint256[] memory pi_px, uint256 y, uint256 x, uint256 m, uint256 n, uint256 N)
        internal
    {
        uint256 eqn2_1 = pi_px[0];
        uint256 eta = N - m*n;
        for(uint256 i = 1; i < n+1; i++){
            uint256 exp = addmod(eta, (i-1)*m, NN);
            uint256 pix = addmod(pi_px[i], pow(x, exp), NN);
            eqn2_1 = addmod(eqn2_1, pix, NN);
        }
    }

    function computeChallenges(Part1 memory part1)
        internal
        view
        returns (uint256[] memory x)
    {

        uint256 challenge = computeBaseChallenge(part1.cDs, part1.cWs, part1.cR1, part1.cR2);

        uint256[] memory challenges = new uint256[](L);

        uint256 challenge2 = mulmod(challenge, challenge, NN);
        uint256 challenge3 = mulmod(challenge2, challenge, NN);
        uint256 challenge4 = mulmod(challenge2, challenge2, NN);
        uint256 challenge5 = mulmod(challenge2, challenge3, NN);
 

        challenges[0] = challenge;
        challenges[1] = challenge2;
        challenges[2] = challenge3;
        challenges[3] = challenge4;
        challenges[4] = challenge5;

        // if (L == 6){
        //      uint256 challenge6 = mulmod(challenge3, challenge3, NN);
        //     challenges[5] = challenge6;
        // }
        // if (L == 8){
        //      uint256 challenge6 = mulmod(challenge3, challenge3, NN);
        //      uint256 challenge7 = mulmod(challenge3, challenge4, NN);
        //      uint256 challenge8 = mulmod(challenge4, challenge4, NN);
        //     challenges[5] = challenge6;
        //     challenges[6] = challenge7;
        //     challenges[7] = challenge8;
        // }
        if (L == 10){
             uint256 challenge6 = mulmod(challenge3, challenge3, NN);
             uint256 challenge7 = mulmod(challenge3, challenge4, NN);
             uint256 challenge8 = mulmod(challenge4, challenge4, NN);
            challenges[5] = challenge6;
            challenges[6] = challenge7;
            challenges[7] = challenge8;
            challenges[8] = mulmod(challenge8, challenge, NN);
            challenges[9] = mulmod(challenge8, challenge2, NN);
        }
        // if (L == 13){
        //      uint256 challenge6 = mulmod(challenge3, challenge3, NN);
        //      uint256 challenge7 = mulmod(challenge3, challenge4, NN);
        //      uint256 challenge8 = mulmod(challenge4, challenge4, NN);
        //     challenges[5] = challenge6;
        //     challenges[6] = challenge7;
        //     challenges[7] = challenge8;
        //     challenges[8] = mulmod(challenge8, challenge, NN);
        //     challenges[9] = mulmod(challenge8, challenge2, NN);
        //     challenges[10] = mulmod(challenge8, challenge3, NN);
        //     challenges[11] = mulmod(challenge8, challenge4, NN);
        //     challenges[12] = mulmod(challenge8, challenge5, NN);
        // }

        return challenges;
    }

    function checkCond1(Part1 memory part1)
        internal
    {
        (uint256 dvpx, uint256 dvpy) = computeDVPoint(part1.dv_value);

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part1.eta1;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        add_input[0] = dvpx;
        add_input[1] = dvpy;
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret1x = add_input[0];
        uint256 ret1y = add_input[1];

        (add_input[0], add_input[1]) = computeCDWS(part1.cDs, part1.cR1, part1.l0, part1.luxs);

    }

    function checkCond2(uint256[] memory challenges, Part1 memory part1)
        internal
    {
        uint256 betadv = 1;
        for(uint j = 0; j < uint(L); j++){
            for(uint i = 0; i < uint(B); i++){
                uint256 dvi = part1.dv_value[i];
                betadv = mulmod(betadv, dvi - uint256(i), NN);
            }
            betadv = mulmod(betadv, challenges[j], NN);
        }

        uint256 eqn2_1 = betadv;

        uint256 eqn2_2 = mulmod(part1.yB, part1.l0, NN);


    }

    function computeCDWS(Point[] memory cs, Point memory cR, uint256 l0_value, uint256[] memory luxs)
        internal
        returns (uint256 x, uint256 y)
    {
       uint size = cs.length;

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        Point memory p = cs[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = luxs[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size;i++){
            Point memory p = cs[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = luxs[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        mul_input[0] = cR.x;
        mul_input[1] = cR.y;
        mul_input[2] = l0_value;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }


        return (add_input[0], add_input[1]);
    }

    function checkCond3(Part1 memory part1, uint256[] memory gamma)
        internal
    {

        uint256[] memory bv_value = computeBvValue(part1.luxs, gamma);

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        
        mul_input[0] = GX;
        mul_input[1] = GY;
        // mul_input[2] = uint(part1.Bkeu);
        mul_input[2] = computeGexp(part1.dv_value, part1.l0, part1.yS, bv_value);
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }
        
        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part1.eta2;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret3x = add_input[0];
        uint256 ret3y = add_input[1];

        (add_input[0], add_input[1]) = computeCDWS(part1.cWs, part1.cR2, part1.l0, part1.luxs);


    }

    function computeBvValue(uint256[] memory lkx, uint256[] memory gamma)
        internal
        returns (uint256[] memory x)
    {
        uint256[] memory BjX = new uint256[](L);
        uint tt = 0;
        for(uint i=0;i<L;i++) {
            uint256 f_v = 0;
            uint base = i * K;
            for(uint j=0;j<K;j++) {
                uint idx = base + j;
                uint mod_agg = idx % 50;
                if(mod_agg == 0){
                    tt+=1; 
                }
                if(idx < nbits) {
                    f_v = addmod(f_v, mulmod(lkx[j], mulmod(gamma[tt-1], B ** mod_agg, NN), NN), NN);
                }else{
                    f_v = f_v;
                }
            }
            BjX[i] = f_v;
        }
        // for(uint i=0;i<L;i++) {
        //     uint256 f_v = 0;
        //     uint base = i * K;
        //     for(uint j=0;j<K;j++) {
        //         uint idx = base + j;
        //         if(idx < nbits) {
        //             f_v = addmod(f_v, mulmod(lkx[j], B ** idx, NN), NN);
        //         }else{
        //             f_v = f_v;
        //         }
        //     }
        //     BjX[i] = f_v;
        // }

        return BjX;
    }

    function computeGexp(uint256[] memory dv_value, uint256 l0_value, uint256 yU, uint256[] memory bv_value)
        internal
        pure
        returns (uint256 x)
    {
        uint256 G1_exp = mulmod(yU, l0_value, NN);
        uint256 dvsize = dv_value.length;
        for (uint i = 0; i < dvsize; i++){
            uint256 dvbv = mulmod(dv_value[i], bv_value[i], NN);
            G1_exp = addmod(G1_exp, dvbv, NN);
        }

        return G1_exp;
    }


    function checkCond4(Point[] memory cys, Point[] memory cws, uint256[] memory gamma)
        internal
    {
        uint wsize = cws.length;
        uint ysize = cys.length;
        uint256[4] memory add_input;
        bool success;

        Point memory p = cws[0];
        if(ysize > 1){
            uint256[3] memory mul_input_cys;
            uint256[4] memory add_input_cys;

            // uint256 _gamma = gamma;

            p = cys[0];
            mul_input_cys[0] = p.x;
            mul_input_cys[1] = p.y;
            mul_input_cys[2] = gamma[0];
            assembly {
                success := call(not(0), 7, 0, mul_input_cys, 0x80, add_input_cys, 0x60)
            }

            for(uint i=1;i<ysize;i++) {
                // _gamma = mulmod(_gamma, gamma, NN);
                p = cys[i];
                mul_input_cys[0] = p.x;
                mul_input_cys[1] = p.y;
                mul_input_cys[2] = gamma[i];

                assembly {
                    success := call(not(0), 7, 0, mul_input_cys, 0x80, add(add_input_cys, 0x40), 0x60)
                }

                assembly {
                    success := call(not(0), 6, 0, add_input_cys, 0x80, add_input_cys, 0x60)
                }
            }
        }else{
            add_input[0] = p.x;
            add_input[1] = p.y;
            for(uint i=1;i<wsize;i++) {
                Point memory p = cws[i];
                add_input[2] = p.x;
                add_input[3] = p.y;
                assembly {
                    success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
                }
            }
        }
    }

    function computeBaseChallenge(Point[] memory cDs, Point[] memory cWs, Point memory cR1, Point memory cR2)
        internal
        pure
        returns (uint256 x)
    {
        uint cwslength = cDs.length;
        uint cmslength = cWs.length;

        uint256 size = 2 * (cwslength + cmslength + 2);
        uint256[] memory cs = new uint256[](size);
        uint offset = 0;
        for(uint i=0;i<cwslength;i++) {
            Point memory p = cDs[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        for(uint i=0;i<cmslength;i++) {
            Point memory p = cWs[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        cs[offset] = cR1.x;
        offset = offset + 1;
        cs[offset] = cR1.y;
        offset = offset + 1;

        cs[offset] = cR2.x;
        offset = offset + 1;
        cs[offset] = cR2.y;
        offset = offset + 1;

        uint256 baseChal = mod(uint256(keccak256(abi.encodePacked(cs))), NN);

        return baseChal;
    }

    function computeDVPoint(uint256[] memory dv_value)
        internal
        returns (uint256 x, uint256 y)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        uint size = dv_value.length;

        Point[] memory gs = generateGS(size);
        Point memory p = gs[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = dv_value[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size;i++){
            Point memory p = gs[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = dv_value[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        return (add_input[0], add_input[1]);
    }

    function generateGS(uint L)
        internal
        pure
        returns (Point[] memory)
    {
        Point[] memory gs = new Point[](L);
        
        gs[0] = Point(G1X, G1Y);
        gs[1] = Point(G2X, G2Y);
        gs[2] = Point(G3X, G3Y);
        
        // if(L == 4){
        //     gs[3] = Point(G4X, G4Y);
        // }

        // if (L == 5){
        //     gs[3] = Point(G4X, G4Y);
        //     gs[4] = Point(G5X, G5Y);
        // }
        // if (L == 6){
        //     gs[3] = Point(G4X, G4Y);
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        // }
        // if (L == 7){
        //     gs[3] = Point(G4X, G4Y);
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);

        // }
        // if (L == 8){
        //     gs[3] = Point(G4X, G4Y);
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        // }
        if (L==10){
            gs[4] = Point(G5X, G5Y);
            gs[5] = Point(G6X, G6Y);
            gs[6] = Point(G7X, G7Y);
            gs[7] = Point(G8X, G8Y);
            gs[8] = Point(G9X, G9Y);
            gs[9] = Point(G10X, G10Y);
        }
        // if (L==13){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        //     gs[8] = Point(G9X, G9Y);
        //     gs[9] = Point(G10X, G10Y);
        //     gs[10] = Point(G11X, G11Y);
        //     gs[11] = Point(G12X, G12Y);
        //     gs[12] = Point(G13X, G13Y);
        // }

        return gs;
    }

}