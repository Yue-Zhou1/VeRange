// // SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

// import "./SafeMath.sol";

contract Type1 {

    // using SafeMath for uint256;

    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Part1 {
        Point[] cys;
        Point[] cts;
        Point[] cws;
        Point bigS;
        Point bigR;
        uint256 eta1;
        uint256 eta2;
        uint256[][] vs;
    }

    uint256 public constant GX = 1;
    uint256 public constant HX = 9727523064272218541460723335320998459488975639302513747055235660443850046724;
    uint256 public constant GY = 2;
    uint256 public constant HY = 5031696974169251245229961296941447383441169981934237515842977230762345915487;

    uint256 internal constant AA = 0;
    uint256 internal constant BB = 3;
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
    uint256 private constant G24X = 6931749079582407422004463406148477362357661472338328270508373724106071518625;
    uint256 private constant G24Y = 21191302843410952823242789471696212271196775028224076743387772065534892367978;



    function multiply(Point memory p, uint256 k)
        public
        returns (Point memory)
    {
        uint256[3] memory input;
        input[0] = p.x;
        input[1] = p.y;
        input[2] = k;

        bool success;
        uint256[2] memory result;

        assembly {
            success := call(not(0), 0x07, 0, input, 96, result, 64)
        }
        require(success, "elliptic curve multiplication failed");

        return Point(result[0], result[1]);
    }
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

    function add(Point memory p1, Point memory p2)
        internal
        returns (Point memory)
    {
        uint256[4] memory input;
        input[0] = p1.x;
        input[1] = p1.y;
        input[2] = p2.x;
        input[3] = p2.y;

        bool success;
        uint256[2] memory result;
        assembly {
            success := call(not(0), 0x06, 0, input, 128, result, 64)
        }

        require(success, "bn256 addition failed");
        return Point(result[0], result[1]);
    }

    function sub(Point memory p1, Point memory p2)
        public
        returns (Point memory)
    {
        uint256[4] memory input;
        input[0] = p1.x;
        input[1] = p1.y;
        input[2] = p2.x;
        input[3] = PP - p2.y;

        bool success;
        uint256[2] memory result;
        assembly {
            success := call(not(0), 0x06, 0, input, 128, result, 64)
        }

        require(success, "bn256 subtraction failed");
        return Point(result[0], result[1]);
    }

    function modExp(uint256 base, uint256 exponent, uint256 modulus)
        internal view returns
        (uint256)
    {
        uint256[6] memory input;
        uint256[1] memory output;
        input[0] = 0x20;  // length_of_BASE
        input[1] = 0x20;  // length_of_EXPONENT
        input[2] = 0x20;  // length_of_MODULUS
        input[3] = base;
        input[4] = exponent;
        input[5] = modulus;
        assembly {
            if iszero(staticcall(not(0), 5, input, 0xc0, output, 0x20)) {
                revert(0, 0)
            }
        }
        return output[0];
    }

     function submod(uint256 a, uint256 b, uint256 q)
        public
        pure
        returns (uint256)
    {
        if(a > b) {
           return addmod(a - b, 0, q);
        } else {
            return q - addmod(b - a, 0, q);
        }
    }

    function verifyRangeArgument(Part1 memory zkp1, uint nbits, uint K, uint L)
        external
        returns (uint256)
    {   
        uint256[] memory betas = computeBetas(zkp1, K);
        uint256[] memory gamma = computeGamma(zkp1.cys);

        assertCond1(betas, gamma, zkp1, nbits, K, L);

        assertCond2(betas, zkp1.cws, zkp1.vs, zkp1.eta2, zkp1.bigR);

        assertCond3(zkp1.cys, zkp1.cws, gamma);
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

    function generateGS(uint L)
        internal
        returns (Point[] memory)
    {
        Point[] memory gs = new Point[](L);
        gs[0] = Point(G1X, G1Y);
        gs[1] = Point(G2X, G2Y);
        gs[2] = Point(G3X, G3Y);
        gs[3] = Point(G4X, G4Y);
        // if(L == 6){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        // }
        // if(L == 8){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        // }
        // if(L == 12){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        //     gs[8] = Point(G9X, G9Y);
        //     gs[9] = Point(G10X, G10Y);
        //     gs[10] = Point(G11X, G11Y);
        //     gs[11] = Point(G12X, G12Y);
        // }
        if(L == 16){
            gs[4] = Point(G5X, G5Y);
            gs[5] = Point(G6X, G6Y);
            gs[6] = Point(G7X, G7Y);
            gs[7] = Point(G8X, G8Y);
            gs[8] = Point(G9X, G9Y);
            gs[9] = Point(G10X, G10Y);
            gs[10] = Point(G11X, G11Y);
            gs[11] = Point(G12X, G12Y);
            gs[12] = Point(G13X, G13Y);
            gs[13] = Point(G14X, G14Y);
            gs[14] = Point(G15X, G15Y);
            gs[15] = Point(G16X, G16Y);
        }
        // if(L == 22){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        //     gs[8] = Point(G9X, G9Y);
        //     gs[9] = Point(G10X, G10Y);
        //     gs[10] = Point(G11X, G11Y);
        //     gs[11] = Point(G12X, G12Y);
        //     gs[12] = Point(G13X, G13Y);
        //     gs[13] = Point(G14X, G14Y);
        //     gs[14] = Point(G15X, G15Y);
        //     gs[15] = Point(G16X, G16Y);
        //     gs[16] = Point(G17X, G17Y);
        //     gs[17] = Point(G18X, G18Y);
        //     gs[18] = Point(G19X, G19Y);
        //     gs[19] = Point(G20X, G20Y);
        //     gs[20] = Point(G21X, G21Y);
        //     gs[21] = Point(G22X, G22Y);
        //     // gs[22] = Point(G23X, G23Y);
        //     // gs[23] = Point(G24X, G24Y);
        // }
        // if(L == 23){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        //     gs[8] = Point(G9X, G9Y);
        //     gs[9] = Point(G10X, G10Y);
        //     gs[10] = Point(G11X, G11Y);
        //     gs[11] = Point(G12X, G12Y);
        //     gs[12] = Point(G13X, G13Y);
        //     gs[13] = Point(G14X, G14Y);
        //     gs[14] = Point(G15X, G15Y);
        //     gs[15] = Point(G16X, G16Y);
        //     gs[16] = Point(G17X, G17Y);
        //     gs[17] = Point(G18X, G18Y);
        //     gs[18] = Point(G19X, G19Y);
        //     gs[19] = Point(G20X, G20Y);
        //     gs[20] = Point(G21X, G21Y);
        //     gs[21] = Point(G22X, G22Y);
        //     gs[22] = Point(G23X, G23Y);
        // }
        return gs;
    }

    function computeBaseBeta(Point[] memory cts, Point[] memory cws, Point memory bigR, Point memory bigS)
        internal
        returns (uint256)
    {
        uint ctlength = cts.length;
        uint cwlength = cws.length;

        uint256 size = 2 * (ctlength + cwlength + 2);
        uint256[] memory cs = new uint256[](size);
        uint offset = 0;
        for(uint i=0;i<ctlength;i++) {
            Point memory p = cts[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        for(uint i=0;i<cwlength;i++) {
            Point memory p = cws[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        Point memory p = bigR;
        cs[offset] = p.x;
        offset = offset + 1;
        cs[offset] = p.y;
        offset = offset + 1;

        Point memory p1 = bigS;
        cs[offset] = p1.x;
        offset = offset + 1;
        cs[offset] = p1.y;
        offset = offset + 1;
        return mod(uint256(keccak256(abi.encodePacked(cs))), NN);
    }

    function computeBetas(Part1 memory zkp1, uint K)
        internal
        returns (uint256[] memory)
    {
        // uint256 betaN1 = zkp1.eInvs[0];

        uint256 beta = computeBaseBeta(zkp1.cts, zkp1.cws, zkp1.bigR, zkp1.bigS);
        // require (mulmod(beta, betaN1, NN) == 1);

        uint256[] memory betas = new uint256[](K);

        uint256 beta2 = mulmod(beta, beta, NN);
        uint256 beta3 = mulmod(beta, beta2, NN);
        uint256 beta4 = mulmod(beta2, beta2, NN);
        uint256 beta5 = mulmod(beta3, beta2, NN);
        uint256 beta6 = mulmod(beta3, beta3, NN);
        betas[0] = beta;
        betas[1] = beta2;
        betas[2] = beta3;
        betas[3] = beta4;
        // if (K==6){
        //     betas[4] = beta5;
        //     betas[5] = beta6;
        // }
        // if(K==8){
        //     betas[4] = beta5;
        //     betas[5] = beta6;
        //     betas[6] = mulmod(beta3, beta4, NN);
        //     betas[7] = mulmod(beta4, beta4, NN);
        // }
        // if(K==11){
        //     betas[4] = beta5;
        //     betas[5] = beta6;
        //     betas[6] = mulmod(beta3, beta4, NN);
        //     betas[7] = mulmod(beta4, beta4, NN);
        //     betas[8] = mulmod(beta4, beta5, NN);
        //     betas[9] = mulmod(beta5, beta5, NN);
        //     betas[10] = mulmod(beta5, beta6, NN);
        // }
        if(K==16){
            uint256 beta7 = mulmod(beta, beta6, NN);
            uint256 beta8 = mulmod(beta4, beta4, NN);
            betas[4] = beta5;
            betas[5] = beta6;
            betas[6] = mulmod(beta3, beta4, NN);
            betas[7] = mulmod(beta4, beta4, NN);
            betas[8] = mulmod(beta4, beta5, NN);
            betas[9] = mulmod(beta5, beta5, NN);
            betas[10] = mulmod(beta5, beta6, NN);
            betas[11] = mulmod(beta6, beta6, NN);
            betas[12] = mulmod(beta6, beta7, NN);
            betas[13] = mulmod(beta7, beta7, NN);
            betas[14] = mulmod(beta7, beta8, NN);
            betas[15] = mulmod(beta8, beta8, NN);
        }
        // if(K==24){
        //     uint256 beta7 = mulmod(beta, beta6, NN);
        //     uint256 beta8 = mulmod(beta4, beta4, NN);
        //     betas[4] = beta5;
        //     betas[5] = beta6;
        //     betas[6] = mulmod(beta3, beta4, NN);
        //     betas[7] = mulmod(beta4, beta4, NN);
        //     betas[8] = mulmod(beta4, beta5, NN);
        //     betas[9] = mulmod(beta5, beta5, NN);
        //     betas[10] = mulmod(beta5, beta6, NN);
        //     betas[11] = mulmod(beta6, beta6, NN);
        //     betas[12] = mulmod(beta6, beta7, NN);
        //     betas[13] = mulmod(beta7, beta7, NN);
        //     betas[14] = mulmod(beta7, beta8, NN);
        //     betas[15] = mulmod(beta8, beta8, NN);
        //     betas[16] = mulmod(betas[6], betas[9], NN);
        //     betas[17] = mulmod(betas[8], betas[8], NN);
        //     betas[18] = mulmod(betas[8], betas[9], NN);
        //     betas[19] = mulmod(betas[9], betas[9], NN);
        //     betas[20] = mulmod(betas[9], betas[10], NN);
        //     betas[21] = mulmod(betas[9], betas[11], NN);
        //     betas[22] = mulmod(betas[9],  betas[12], NN);
        //     betas[23] = mulmod(betas[9],  betas[13], NN);
        // }
        // if(K==23){
        //     uint256 beta7 = mulmod(beta, beta6, NN);
        //     uint256 beta8 = mulmod(beta4, beta4, NN);
        //     betas[4] = beta5;
        //     betas[5] = beta6;
        //     betas[6] = mulmod(beta3, beta4, NN);
        //     betas[7] = mulmod(beta4, beta4, NN);
        //     betas[8] = mulmod(beta4, beta5, NN);
        //     betas[9] = mulmod(beta5, beta5, NN);
        //     betas[10] = mulmod(beta5, beta6, NN);
        //     betas[11] = mulmod(beta6, beta6, NN);
        //     betas[12] = mulmod(beta6, beta7, NN);
        //     betas[13] = mulmod(beta7, beta7, NN);
        //     betas[14] = mulmod(beta7, beta8, NN);
        //     betas[15] = mulmod(beta8, beta8, NN);
        //     betas[16] = mulmod(betas[6], betas[9], NN);
        //     betas[17] = mulmod(betas[8], betas[8], NN);
        //     betas[18] = mulmod(betas[8], betas[9], NN);
        //     betas[19] = mulmod(betas[9], betas[9], NN);
        //     betas[20] = mulmod(betas[9], betas[10], NN);
        //     betas[21] = mulmod(betas[9], betas[11], NN);
        //     betas[22] = mulmod(betas[9],  betas[12], NN);
        // }

        return betas;
    }

    function assertCond1(uint256[] memory betas, uint256[] memory gamma, Part1 memory zkp1, uint256 nbit, uint256 K, uint256 L)
        internal
    {
        uint256[] memory uvs;
        if (gamma.length > 1){
            uvs = computeUVAggre(betas, gamma, zkp1.cys, zkp1.vs);
        }else{
            uvs = computeUV(betas, zkp1.cys, zkp1.vs, nbit, K, L);
        }
        
        (uint256 uvpx, uint256 uvpy) = computeUVPoint(uvs);

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;
        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = zkp1.eta1;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        add_input[0] = uvpx;
        add_input[1] = uvpy;
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret1x = add_input[0];
        uint256 ret1y = add_input[1];

        (add_input[0], add_input[1]) = computeCondRight(betas, zkp1.cts, zkp1.bigS);
        
    }

    function assertCond2(uint256[] memory betas, Point[] memory cws, uint256[][] memory vs, uint256 eta2, Point memory bigR)
        internal
        returns (Point memory)
    {
        uint256 vsum = 0;
        uint vsize = vs.length;
        for(uint i=0;i<vsize;i++) {
            for(uint j=0;j<vs[i].length;j++){

                vsum = addmod(vsum, vs[i][j], NN);
            }
        }

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = vsum;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = eta2;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret1x = add_input[0];
        uint256 ret1y = add_input[1];

        (add_input[0], add_input[1]) = computeCondRight(betas, cws, bigR);
    }

    function computeCondRight(uint256[] memory betas, Point[] memory cws, Point memory bigR)
        internal
        returns (uint256 x, uint256 y)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        Point memory p = cws[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = betas[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        uint wsize = cws.length;
        for(uint i=1;i<wsize;i++) {
            Point memory p = cws[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = betas[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        add_input[2] = bigR.x;
        add_input[3] = bigR.y;

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        return (add_input[0], add_input[1]);
    }

    function assertCond3(Point[] memory cys, Point[] memory cws, uint256[] memory gamma)
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

    function computeUV(uint256[] memory betas, Point[] memory cys, uint256[][] memory vs, uint nbits, uint K, uint L)
        internal
        returns (uint256[] memory)
    {
        uint256[] memory uvs = new uint256[](L);
        for(uint i=0;i<L;i++) {
            uint base = i * K;
            uint256 temp = 0;
            for(uint j=0;j<K;j++) {
                uint256 u = 0;
                uint idx = base + j;
                if(idx < nbits) {
                    if (idx == 0){
                        u = betas[j];
                    }else{
                        u = mulmod(betas[j], 2 << (idx-1), NN);
                    }                  
                    uint256 v = vs[i][j];
                    if(u >= v) {
                        u = u - v;
                    } else {
                        u = NN - v + u;
                    }
                    temp = addmod(temp, mulmod(v, u, NN), NN);
                }
                
            }
            uvs[i] = temp;
        }
        return uvs;
    }

    function computeUVAggre(uint256[] memory betas, uint256[] memory gamma, Point[] memory cys, uint256[][] memory vs)
        internal
        returns (uint256[] memory)
    {
        uint256[] memory uvs = new uint256[](16);
        uint tt = 0;
        for(uint i=0;i<16;i++) {
            uint base = i * 16;
            uint256 temp = 0;
            for(uint j=0;j<16;j++) {
                uint256 u = 0;
                // uint idx = base + j;
                uint mod_agg = (base+j) % 128;
                if(mod_agg == 0){
                    tt+=1;
                }
                // uint256 agg = (tt-1)*132;
                if((base + j) < 256) {
                    if ((base + j) == 0){
                        u = mulmod(betas[j], gamma[0], NN);
                    }else{
                        // uint256 gamma_power = modExp(gamma, tt, NN);
                        // uint256 tt_2power = mulmod(gamma[tt-1], 2 << mod_agg, NN);
                        u = mulmod(betas[j], mulmod(gamma[tt-1], 2 << mod_agg, NN), NN);
                    }                  
                    uint256 v = vs[i][j];
                    if(u >= v) {
                        u = u - v;
                    } else {
                        u = NN - v + u;
                    }
                    temp = addmod(temp, mulmod(v, u, NN), NN);
                }
                
            }
            uvs[i] = temp;
    }
        return uvs;
        
    }
    
    function computeUVPoint(uint256[] memory uvs)
        internal
        returns (uint256 x, uint256 y)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        add_input[0] = 0;
        add_input[1] = 0;
        uint size = uvs.length;

        Point[] memory gs = generateGS(size);
        for(uint i=0;i<size;i++) {
            Point memory g = gs[i];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = uvs[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        return (add_input[0], add_input[1]);
    }
}

