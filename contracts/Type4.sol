pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

contract Type4 {
    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Part1 {
        Point cy;
        Point[] cgs;
        Point[] cps;
        Point[] cqs;
        Point[] cmae;
        //b, eprime, g_x, g_wx, f_x, f'_x, p_x, r'1, r'2, r'3, r'4
        uint256[] fieldEle;
        uint256[] qvs;
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

    uint256 internal constant omega = 0x30644E72E131A029048B6E193FD841045CEA24F6FD736BEC231204708F703636;
    
    uint256 omega_n1 = 4407920970296243842541313971887945403937097133418418784715;
    uint256 B = 2;


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

    function modInv(uint256 x, uint256 p)
        public
        pure
        returns (uint256)
    {
        require(x != 0 && x != p && p != 0, "Invalid number");
        uint256 q = 0;
        uint256 newT = 1;
        uint256 r = p;
        uint256 t;
        while (x != 0) {
            t = r / x;
            (q, newT) = (newT, addmod(q, (p - mulmod(t, newT, p)), p));
            (r, x) = (x, r - t * x);
        }

        return q;
    }

    function inv(uint256 x, uint256 p)
        internal
        pure
        returns (uint256)
    {
        return pow(x, p-2);
    }

    function div(uint256 x, uint256 y)
        internal
        pure
        returns (uint256)
    {
        return mulmod(x, inv(y, NN), NN);
    }

    function verifyRangeArgument(Part1 memory part1, uint256 nbits)
        external
    {
        uint256[] memory betas = computeChallenges(part1);

        checkCond1(part1, betas);

        checkCond2(part1, betas, nbits);

        checkCond3(part1.fieldEle);

        checkCond4(part1.fieldEle[2], part1.fieldEle[5]);

        checkCond5(part1.fieldEle, betas, nbits);

        checkCond6(part1, betas[0]);

        checkCond7(part1, betas[0]);

        checkCond8(part1, betas[0]);

    }


    function computeChallenges(Part1 memory part1)
        internal
        view
        returns (uint256[] memory x)
    {
        uint256[] memory cs1 = new uint256[](2 * (part1.cgs.length));
        uint256[] memory cs2 = new uint256[](2 * (part1.cgs.length + part1.cps.length));
        uint256[] memory cs3 = new uint256[](2 * (part1.cgs.length + part1.cps.length + part1.cqs.length));
        uint offset = 0;
        uint256[] memory challenges = new uint256[](5);

        for(uint i=0;i<part1.cgs.length;i++) {
            Point memory p = part1.cgs[i];
            cs1[offset] = p.x;
            cs2[offset] = p.x;
            cs3[offset] = p.x;
            offset = offset + 1;
            cs1[offset] = p.y;
            cs2[offset] = p.y;
            cs3[offset] = p.y;
            offset = offset + 1;
        }

        uint256 gamma = mod(uint256(keccak256(abi.encodePacked(cs1))), NN);
        challenges[0] = gamma;
        challenges[1] = mulmod(gamma, gamma, NN);

        for(uint i=0;i<part1.cps.length;i++) {
            Point memory p = part1.cps[i];
            cs2[offset] = p.x;
            cs3[offset] = p.x;
            offset = offset + 1;
            cs2[offset] = p.y;
            cs3[offset] = p.y;
            offset = offset + 1;
        }
        uint256 xpoint = mod(uint256(keccak256(abi.encodePacked(cs2))), NN);
        challenges[2] = xpoint;
        challenges[3] = mulmod(xpoint, xpoint, NN);

        for(uint i=0;i<part1.cqs.length;i++) {
            Point memory p = part1.cqs[i];
            cs3[offset] = p.x;
            offset = offset + 1;
            cs3[offset] = p.y;
            offset = offset + 1;
        }
        uint256 z = mod(uint256(keccak256(abi.encodePacked(cs3))), NN);
        challenges[4] = z;

        return challenges;
    }

    function checkCond1(Part1 memory part1, uint256[] memory beta)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        add_input[0] = 0;
        add_input[1] = 0;
        uint size = part1.qvs.length;

        Point[] memory gs = generateGS(size);
        for(uint i=0;i<size;i++) {
            Point memory g = gs[i];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = part1.qvs[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part1.fieldEle[7];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret1x = add_input[0];
        uint256 ret1y = add_input[1];
        // assert(ret1x == 7174584835309886496400765371963240332059066071404252601722076133947075622999);
        // assert(ret1y == 13598779401185368623811155768309402485558513988169421837145175450379840436556);


        (add_input[0], add_input[1]) = computeCond1Right(part1, beta);

        // assert(ret1x == add_input[0] && ret1y == add_input[1]);
    }

    function computeCond1Right(Part1 memory part1, uint256[] memory beta)
        internal
        returns (uint256 x, uint256 y)
    {   
        uint256 z_1 = submod(beta[4], 1, NN);
        uint256 z_x = submod(beta[4], beta[2], NN);
        uint256 z_wx = submod(beta[4], mulmod(beta[2], omega, NN) , NN);
        uint256 rho_z_x_z_wx = mulmod(mulmod(beta[3], z_x, NN), z_wx, NN);
        uint256 hx_g = addmod(z_1, rho_z_x_z_wx, NN);
        uint256 hx_p = mulmod(mulmod(z_1, z_wx, NN), mulmod(beta[3], beta[3], NN), NN);
        // uint256 hx_q = -mulmod(mulmod(z_1, z_x, NN), z_wx, NN);
        uint256 hx_q = mulmod(mulmod(z_1, z_x, NN), z_wx, NN);

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        for(uint i=0;i<part1.cgs.length;i++) {
            Point memory p = part1.cgs[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = hx_g;

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
            }

            p = part1.cps[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = hx_p;

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            p = part1.cqs[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = hx_q;

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            // if(i != 0){
            //     uint256 x_n = 1;
            //     mul_input[0] = add_input[0];
            //     mul_input[1] = add_input[1];
            //     for(uint j = 1; j < i; j++){
            //         x_n = mulmod(x_n, beta[4], NN);
            //     }
            //     mul_input[2] = x_n;
            //     assembly {
            //         success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
            //     }
            // }
        }
        
        mul_input[0] = part1.cmae[2].x;
        mul_input[1] = part1.cmae[2].y;
        mul_input[2] = mulmod(beta[0], rho_z_x_z_wx, NN);

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        return (add_input[0], add_input[1]);
    }

    function checkCond2(Part1 memory part1, uint256[] memory beta, uint256 nbits)
        internal
    {
        uint256 eta = B*(2*nbits-1)-(part1.cps.length-1)*(part1.qvs.length-1)-1;
        uint256 ret2_1 = part1.qvs[0];
        for(uint i = 1; i < part1.qvs.length; i++){
            uint256 exp = (i-1)*(part1.cps.length-1) + eta;
            uint256 x_n = beta[4];
            for(uint j = 1; j < exp; j++){
                x_n = mulmod(x_n, beta[4], NN);
            }
            uint256 qv_z = mulmod(part1.qvs[i], x_n, NN);
            ret2_1 = addmod(ret2_1, qv_z, NN);
        }
        (uint256 r0, uint256 r1, uint256 r2) = computeRs(part1.fieldEle, beta);

        uint256 wx = mulmod(omega, beta[2], NN);
        uint256 ret2_r0 = mulmod(r0, submod(beta[4], 1, NN), NN);
        uint256 ret2_r1 = mulmod(mulmod(beta[3], r1, NN), mulmod(submod(beta[4], beta[2], NN), submod(beta[4], wx, NN), NN), NN);
        uint256 rho2 = mulmod(beta[3], beta[3], NN);
        uint256 ret2_r2 = mulmod(mulmod(rho2, r2, NN), mulmod(submod(beta[4], 1, NN), submod(beta[4], wx, NN), NN), NN);
        assert(ret2_1==addmod(addmod(ret2_r0, ret2_r1, NN), ret2_r2, NN));
    }

    function checkCond3(uint256[] memory fieldEle)
        internal
    {
        uint256 gx_Bgwx = submod(fieldEle[2], mulmod(B, fieldEle[3], NN), NN);
        uint256 ret3_2 = gx_Bgwx;
        
        for(uint i = 1; i < B; i++){
            ret3_2 = mulmod(ret3_2, submod(i, gx_Bgwx, NN), NN);
        }

        // f_x
        assert(fieldEle[4] == ret3_2);

    }

    function checkCond4(uint256 gx, uint256 fprime_x)
        internal
    {
        uint256 ret4_2 = gx;
        
        for(uint i = 1; i < B; i++){
            ret4_2 = mulmod(ret4_2, submod(i, gx, NN), NN);
        }

        // f'_x
        assert(fprime_x == ret4_2);
    }

    function checkCond5(uint256[] memory fieldEle, uint256[] memory beta, uint256 nbits)
        internal
    {
        // f_x, x, theta
        uint256 x_n = beta[2];
        for(uint i = 1; i < nbits; i++){
            x_n = mulmod(x_n, beta[2], NN);
        }
        
        // uint256 modinv1 = 17631884349014720375937952052632537309271643882229040878517051678378892138569;
        // uint256 modinv2 = 8082994100701360340407521615961276317581926580161273734597883315337190257808;
       

        uint256 part_fx = mulmod(mulmod(fieldEle[4], submod(beta[2], omega_n1, NN), NN), inv(x_n-1, NN), NN);
        uint256 part_fprimex = mulmod(mulmod(fieldEle[5], beta[1], NN), inv(submod(beta[2], omega_n1, NN), NN), NN);
        // uint256 part_fx = mulmod(mulmod(fieldEle[4], submod(beta[2], omega_n1, NN), NN), modinv1, NN);
        // uint256 part_fprimex = mulmod(mulmod(fieldEle[5], beta[1], NN), modinv2, NN);
        // p_x 
        assert(fieldEle[6] == addmod(part_fx, part_fprimex, NN));
    }

    function checkCond6(Part1 memory part1, uint256 gamma)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        // b
        mul_input[2] = part1.fieldEle[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }
        
        mul_input[0] = HX;
        mul_input[1] = HY;
        // r'2
        mul_input[2] = part1.fieldEle[8];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret6x = add_input[0];
        uint256 ret6y = add_input[1];

        // cm_a
        mul_input[0] = part1.cmae[0].x;
        mul_input[1] = part1.cmae[0].y;
        // gamma
        mul_input[2] = gamma;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        // cm_w
        add_input[2] = part1.cy.x;
        add_input[3] = part1.cy.y;
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        assert (add_input[0] == ret6x && add_input[1] == ret6y);
    }

    function checkCond7(Part1 memory part1, uint256 gamma)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        // e'
        mul_input[2] = part1.fieldEle[1];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }
        
        mul_input[0] = HX;
        mul_input[1] = HY;
        // r'3
        mul_input[2] = part1.fieldEle[9];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret7x = add_input[0];
        uint256 ret7y = add_input[1];

        // cm_e
        mul_input[0] = part1.cmae[1].x;
        mul_input[1] = part1.cmae[1].y;
        // gamma
        mul_input[2] = gamma;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        // cm_a
        add_input[2] = part1.cmae[0].x;
        add_input[3] = part1.cmae[0].y;
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        assert (add_input[0] == ret7x && add_input[1] == ret7y);
    }

    function checkCond8(Part1 memory part1, uint256 gamma)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = G1X;
        mul_input[1] = G1Y;
        // e'
        mul_input[2] = part1.fieldEle[1];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }
        
        mul_input[0] = HX;
        mul_input[1] = HY;
        // r'4
        mul_input[2] = part1.fieldEle[10];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret8x = add_input[0];
        uint256 ret8y = add_input[1];

        // cm_e'
        mul_input[0] = part1.cmae[3].x;
        mul_input[1] = part1.cmae[3].y;
        // gamma
        mul_input[2] = gamma;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        // cm_a'
        add_input[2] = part1.cmae[2].x;
        add_input[3] = part1.cmae[2].y;
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        assert (add_input[0] == ret8x && add_input[1] == ret8y);

    }

    function computeRs(uint256[] memory fieldEle, uint256[] memory beta)
        internal
        returns (uint256, uint256, uint256)
    {
        uint256 z = beta[4];
        uint256 x = beta[2];
        uint256 wx = mulmod(omega, x, NN);
        uint256 x_wx_inv = inv(submod(x, wx, NN), NN);
        uint256 r0_1 = mulmod(mulmod(fieldEle[2], submod(z, wx, NN), NN), x_wx_inv, NN);

        uint256 wx_x_inv = inv(submod(wx, x, NN), NN);
        // uint256 r0_2 = fieldEle[3]*(z-x)*wx_x_inv;
        uint256 r0_2 = mulmod(mulmod(fieldEle[3], submod(z, x, NN), NN), wx_x_inv, NN);

        uint256 r1 = fieldEle[0];
        uint256 r2 = fieldEle[6];
        return (addmod(r0_1, r0_2, NN), r1, r2);
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
        
        if(L == 4){
            gs[3] = Point(G4X, G4Y);
        }

        if (L == 5){
            gs[3] = Point(G4X, G4Y);
            gs[4] = Point(G5X, G5Y);
        }
        if (L == 6){
            gs[3] = Point(G4X, G4Y);
            gs[4] = Point(G5X, G5Y);
            gs[5] = Point(G6X, G6Y);
        }
        if (L == 7){
            gs[3] = Point(G4X, G4Y);
            gs[4] = Point(G5X, G5Y);
            gs[5] = Point(G6X, G6Y);
            gs[6] = Point(G7X, G7Y);

        }
        if (L == 8){
            gs[3] = Point(G4X, G4Y);
            gs[4] = Point(G5X, G5Y);
            gs[5] = Point(G6X, G6Y);
            gs[6] = Point(G7X, G7Y);
            gs[7] = Point(G8X, G8Y);
        }

        return gs;
    }

}