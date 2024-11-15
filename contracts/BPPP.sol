// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

contract BPPP {
    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Part1 {
        Point cy;
        Point cd;
        Point cm;
        Point cf;
        Point cs1;
        Point cs2;
        Point cs3;
        Point ct1;
        Point ct2;
        Point cw1;
        Point cw2;
    }

    struct Part2 {
        uint256 rw1;
        uint256 rw2;
        uint256 t1hat;
        uint256 t2hat;
        uint256 tau_rho;
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

    uint256 B = 4; 
    uint256 nbits = 16;

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

    function tryModExp(uint256 b, uint256 e, uint256 m) internal view returns (bool success, uint256 result) {
        if (m == 0) return (false, 0);
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, 0x20)
            mstore(add(ptr, 0x20), 0x20)
            mstore(add(ptr, 0x40), 0x20)
            mstore(add(ptr, 0x60), b)
            mstore(add(ptr, 0x80), e)
            mstore(add(ptr, 0xa0), m)
            success := staticcall(gas(), 0x05, ptr, 0xc0, 0x00, 0x20)
            result := mload(0x00)
        }
    }

    function modExp(uint256 b, uint256 e, uint256 m) internal view returns (uint256) {
        (bool success, uint256 result) = tryModExp(b, e, m);
        return result;
    }

    function invMod(uint256 a, uint256 n) internal pure returns (uint256) {
        unchecked {
            if (n == 0) return 0;

            // The inverse modulo is calculated using the Extended Euclidean Algorithm (iterative version)
            // Used to compute integers x and y such that: ax + ny = gcd(a, n).
            // When the gcd is 1, then the inverse of a modulo n exists and it's x.
            // ax + ny = 1
            // ax = 1 + (-y)n
            // ax ≡ 1 (mod n) # x is the inverse of a modulo n

            // If the remainder is 0 the gcd is n right away.
            uint256 remainder = a % n;
            uint256 gcd = n;

            // Therefore the initial coefficients are:
            // ax + ny = gcd(a, n) = n
            // 0a + 1n = n
            int256 x = 0;
            int256 y = 1;

            while (remainder != 0) {
                uint256 quotient = gcd / remainder;

                (gcd, remainder) = (
                    // The old remainder is the next gcd to try.
                    remainder,
                    // Compute the next remainder.
                    // Can't overflow given that (a % gcd) * (gcd // (a % gcd)) <= gcd
                    // where gcd is at most n (capped to type(uint256).max)
                    gcd - remainder * quotient
                );

                (x, y) = (
                    // Increment the coefficient of a.
                    y,
                    // Decrement the coefficient of n.
                    // Can overflow, but the result is casted to uint256 so that the
                    // next value of y is "wrapped around" to a value between 0 and n - 1.
                    x - y * int256(quotient)
                );
            }

            if (gcd != 1) return 0; // No inverse exists.
            return x < 0 ? (n - uint256(-x)) : uint256(x); // Wrap the result if it's negative.
        }
    }

    function verifyBPPP(Part1 memory part1, Part2 memory part2)
        external
    {
        uint256[] memory betas = computeBetas(part1);
        (uint256 deltayz, uint256[] memory BsYs) = computeDeltayz(betas);

        assertCond1(betas, deltayz, part1, part2);

        assertCond2(betas, BsYs, part2.rw1, part1);

        assertCond3(part1, part2, betas);


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
        if (L == 16){
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


        return gs;
    }

    function computeBaseBeta(Part1 memory part1)
        internal
        returns (uint256 a, uint256 b, uint256 c, uint256 d)
    {

        uint256 size = 2 * 5;
        uint256[] memory cs = new uint256[](size);
        uint offset = 0;

        cs[offset] = part1.cd.x;
        offset = offset + 1;
        cs[offset] = part1.cd.y;
        offset = offset + 1;
        
        cs[offset] = part1.cm.x;
        offset = offset + 1;
        cs[offset] = part1.cm.y;
        offset = offset + 1;
        
        cs[offset] = part1.cs1.x;
        offset = offset + 1;
        cs[offset] = part1.cs1.y;

        cs[offset] = part1.cs2.x;
        offset = offset + 1;
        cs[offset] = part1.cs2.y;
        offset = offset + 1;

        cs[offset] = part1.cs3.x;
        offset = offset + 1;
        cs[offset] = part1.cs3.y;
        offset = offset + 1;

        uint256[] memory cy = new uint256[](2);
        cy[0] = part1.cy.x;
        cy[1] = part1.cy.y;

        uint256 alpha = mod(uint256(keccak256(abi.encodePacked(cs,cy))), NN);

        uint[] memory cfs = new uint256[](2);
        cfs[0] = part1.cf.x;
        cfs[1] = part1.cf.y;

        uint256 y = mod(uint256(keccak256(abi.encodePacked(cfs, alpha))), NN);
        uint256 z = mod(uint256(keccak256(abi.encodePacked(cfs, y))), NN);

        uint256[] memory cts = new uint256[](4);
        uint offset_cts = 0;
        cts[offset_cts] = part1.ct1.x;
        offset_cts = offset_cts + 1;
        cts[offset_cts] = part1.ct1.y;
        offset_cts = offset_cts + 1;
        
        cts[offset_cts] = part1.ct2.x;
        offset_cts = offset_cts + 1;
        cts[offset_cts] = part1.ct2.y;
        offset_cts = offset_cts + 1;
        uint256 rho = mod(uint256(keccak256(abi.encodePacked(cts))), NN);

        return (alpha, y, z, rho);
    }

    function computeBetas(Part1 memory part1)
        internal
        returns (uint256[] memory)
    {
        // uint256 betaN1 = zkp1.eInvs[0];

        (uint256 alpha, uint256 y, uint256 z, uint256 rho) = computeBaseBeta(part1);

        uint256[] memory betas = new uint256[](4);
        betas[0] = alpha;
        betas[1] = y;
        betas[2] = z;
        betas[3] = rho;
        return betas;
    }

    function assertCond1(uint256[] memory betas, uint256 deltayz, Part1 memory part1, Part2 memory part2)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = part2.t1hat;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part2.tau_rho;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret1x = add_input[0];
        uint256 ret1y = add_input[1];

        (add_input[0], add_input[1]) = computeCond1Right(betas, deltayz, part1, part2);
        
    }

    function computeCond1Right(uint256[] memory betas, uint256 deltayz, Part1 memory part1, Part2 memory part2)
        internal
        returns (uint256 x, uint256 y)
    {

    
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = deltayz;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = mulmod(betas[2], part2.t2hat, NN);

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }



        mul_input[0] = part1.ct1.x;
        mul_input[1] = part1.ct1.y;
        mul_input[2] = betas[3];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }



        mul_input[0] = part1.ct2.x;
        mul_input[1] = part1.ct2.y;
        mul_input[2] = mulmod(betas[3], betas[3], NN);

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        add_input[2] = part1.cy.x;
        add_input[3] = part1.cy.y;

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        return (add_input[0], add_input[1]);
    }

    function computeDeltayz(uint256[] memory betas)
        internal
        returns (uint256 x, uint256[] memory y)
    {
        uint256 z2 = mulmod(betas[2], betas[2], NN);
        uint256 z2yn = 0;
        uint256[] memory bsys = new uint256[](nbits*2);
        // uint256[] memory z_inv_yn = new uint256[](nbits);
        uint256 bsys_alphayn = 0;
        uint256 z_inv = modExp(betas[2], NN-2, NN);

        for(uint i = 0; i<nbits; i++){
            uint256 yn_i = modExp(betas[1], i, NN);
            uint256 y_n_i = modExp(yn_i, NN-2, NN);
            z2yn = addmod(z2yn, mulmod(z2, yn_i, NN), NN);
            bsys[i] = mulmod(B**i, y_n_i, NN);
            uint256 alpha_yn = addmod(z_inv, mulmod(betas[0], yn_i, NN), NN);
            bsys_alphayn = addmod(bsys_alphayn, addmod(bsys[i], alpha_yn, NN), NN);
            bsys[i+nbits] = addmod(betas[0], mulmod(z_inv, y_n_i, NN), NN);
        }
        
        return (addmod(z2yn, bsys_alphayn, NN), bsys);

    }

    function assertCond2(uint256[] memory betas, uint256[] memory BsYs, uint256 rw1, Part1 memory part1)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        add_input[0] = part1.cw1.x;
        add_input[1] = part1.cw1.y;

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = rw1;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret2x = add_input[0];
        uint256 ret2y = add_input[1];

        (add_input[0], add_input[1]) = computeCond2Right(betas, BsYs, part1);
    }

    function computeCond2Right(uint256[] memory betas, uint256[] memory BsYs, Part1 memory part1)
        internal
        returns (uint256 x, uint256 y)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = part1.cs1.x;
        mul_input[1] = part1.cs1.y;
        mul_input[2] = betas[3];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        add_input[2] = part1.cf.x;
        add_input[3] = part1.cf.y; 

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = add_input[0];
        mul_input[1] = add_input[1];
        mul_input[2] = mulmod(betas[2], betas[2], NN);

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = part1.cs2.x;
        mul_input[1] = part1.cs2.y;
        mul_input[2] = betas[3];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        Point[] memory gs = generateGS(nbits);

        for(uint i = 0; i < nbits; i++){
            Point memory g = gs[i];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = BsYs[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = BsYs[i+nbits];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }


        add_input[2] = part1.cd.x;
        add_input[3] = part1.cd.y;

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        return (add_input[0], add_input[1]);
    }
    
    function assertCond3(Part1 memory part1, Part2 memory part2, uint256[] memory betas)
        internal
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        add_input[0] = part1.cw2.x;
        add_input[1] = part1.cw2.y;


        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part2.rw2;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret3x = add_input[0];
        uint256 ret3y = add_input[1];

        add_input[0] = part1.cm.x;
        add_input[1] = part1.cm.y;


        mul_input[0] = part1.cs3.x;
        mul_input[1] = part1.cs3.y;
        mul_input[2] = betas[3];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        Point[] memory gs = generateGS(B);
        // uint256 alpha = 9330853914485710125743323572397414606204019257102224000268035405357803439060;
        // uint256[4] memory alpha_c = [2037395176991239285614814413375709542664134433849815639071910061492110683071, 18043389897498666580981330167140088990429749953662296104095494002300683721693, 17414306728534044308173004380703738714489557446455156537539337104575424925636, 1844436842100513789482354218990977349340590579913990263662478888507980148347];
        for(uint i = 0; i < B; i++){
            Point memory g = gs[i];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            // mul_input[2] = invMod(addmod(alpha, i, NN), NN);
            mul_input[2] = invMod(addmod(betas[0], i, NN), NN);
            // mul_input[2] = alpha_c[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }


    }

    function verifyN1(Point memory z, uint256 ls, uint256 rs)
        internal
        returns (bool b)
    {   
        uint256 t_hat = mulmod(ls, rs, NN);

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = ls;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = rs;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = t_hat;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        return true;

    }

    function verifyip(uint256 nbits, Point[] memory gsprime, Point[] memory hsprime, Point memory zprime, uint256[] memory rsprime, uint256[] memory lsprime)
        internal
        returns (bool b)
    {
        if(nbits == 1){
            verifyN1(zprime, lsprime[0], rsprime[0]);
        }else{
            (Point memory L, Point memory R) = computeLR(gsprime, hsprime, lsprime, rsprime);
            uint256 beta = computeLRbeta(L, R);
            Point memory zprime1 = computeZprime(beta, L, R, zprime);
            (Point[] memory gsprime1, Point[] memory hsprime1) = computeGHprime(beta, gsprime, hsprime);
            (uint256[] memory lsprime1, uint256[] memory rsprime1) = computeLRprime(beta, lsprime, rsprime);
            return verifyip(nbits/2, gsprime1, hsprime1, zprime1, rsprime1, lsprime1);
        }
    }

    function computeZprime(uint256 beta, Point memory L, Point memory R, Point memory zprime)
        internal
        returns (Point memory zprime1)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = L.x;
        mul_input[1] = L.y;
        mul_input[2] = mulmod(beta, beta, NN);

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        mul_input[0] = zprime.x;
        mul_input[1] = zprime.y;
        mul_input[2] = beta;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        } 


        add_input[0] = R.x;
        add_input[1] = R.y;
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }
        return Point(add_input[0], add_input[1]);

    }

    function computeLRbeta(Point memory L, Point memory R)
        internal
        returns(uint256 beta)
    {

        uint256[] memory clr = new uint256[](4);
        clr[0] = L.x;
        clr[1] = L.y;
        clr[2] = R.x;
        clr[3] = R.y;
        return mod(uint256(keccak256(abi.encodePacked(clr))), NN);
    }

    function computeThatprime(uint256[] memory lsprime, uint256[] memory rsprime)
        internal
        returns(uint256 t1hat, uint256 t2hat)
    {
        uint size = lsprime.length;
        uint halfSize = size / 2;

        bool success;

        uint256 t1_hat = 0;
        uint256 t2_hat = 0;
        for(uint i =0; i < halfSize; i++){
            t1_hat = addmod(t1_hat, mulmod(lsprime[i+halfSize], rsprime[i], NN), NN);
            t2_hat = addmod(t1_hat, mulmod(lsprime[i], rsprime[i+halfSize], NN), NN);
        }

        return(t1hat, t2hat);
    }

    function computeLR(Point[] memory gsprime, Point[] memory hsprime, uint256[] memory lsprime, uint256[] memory rsprime)
        internal
        returns (Point memory L, Point memory R)
    
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        uint256[4] memory add_input1;
        bool success;

        // uint size = gsprime.length;
        uint halfSize = gsprime.length / 2;

        (uint256 t1hat, uint256 t2hat) = computeThatprime(lsprime, rsprime);

        for(uint i = 0; i < halfSize; i++){
            Point memory g = gsprime[i];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = lsprime[i+halfSize];

            if(i==0){
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
                }
            }else{
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
                }
                assembly {
                    success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
                }
            }

            Point memory h = hsprime[i+halfSize];    
            mul_input[0] = h.x;
            mul_input[1] = h.y;
            mul_input[2] = rsprime[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }


            mul_input[0] = GX;
            mul_input[1] = GY;
            mul_input[2] = t1hat;

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }


            g = gsprime[i+halfSize];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = lsprime[i];

            if(i==0){
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input1, 0x0), 0x60)
                }
            }else{
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input1, 0x40), 0x60)
                }
                assembly {
                    success := call(not(0), 6, 0, add_input1, 0x80, add_input1, 0x60)
                }
            }

            h = hsprime[i];    
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = rsprime[i+halfSize];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input1, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input1, 0x80, add_input1, 0x60)
            }


            mul_input[0] = GX;
            mul_input[1] = GY;
            mul_input[2] = t2hat;

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input1, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input1, 0x80, add_input1, 0x60)
            }


        }

        Point memory L = Point(add_input[0], add_input[1]);
        Point memory R = Point(add_input1[0], add_input1[1]);
        return (L, R);
    }

    function computeLRprime(uint256 beta, uint256[] memory lsprime, uint256[] memory rsprime)
        internal
        returns (uint256[] memory x, uint256[] memory y)
    {           
        uint size = lsprime.length;
        uint halfSize = size / 2;

        uint256[] memory lprime = new uint256[](halfSize);
        uint256[] memory rprime = new uint256[](halfSize);

        for(uint i = 0; i < halfSize; i++){
            lprime[i] = addmod(lsprime[i], mulmod(beta, lsprime[i+halfSize], NN), NN);
            rprime[i] = addmod(rsprime[i+halfSize], mulmod(beta, rsprime[i], NN), NN);
        }

        return (lprime, rprime);
    }


    function computeGHprime(uint256 beta, Point[] memory gsprimes, Point[] memory hsprimes)
        internal
        returns (Point[] memory gsprime, Point[] memory hsprime)
    {
        uint size = gsprimes.length;
        uint halfSize = size / 2;

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        uint256[4] memory add_input1;
        bool success;

        Point[] memory gsprime = new Point[](halfSize);
        Point[] memory hsprime = new Point[](halfSize);

        for(uint i = 0; i < halfSize; i++){
            Point memory g = gsprimes[i];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = beta;

            if(i==0){
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
                }
            }else{
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
                }
                assembly {
                    success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
                }
            }

            add_input[2] = gsprimes[i+halfSize].x;
            add_input[3] = gsprimes[i+halfSize].y;
            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            gsprime[i] = Point(add_input[0], add_input[1]);


            g = hsprimes[i+halfSize];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = beta;

            if(i==0){
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input1, 0x0), 0x60)
                }
            }else{
                assembly {
                    success := call(not(0), 7, 0, mul_input, 0x80, add(add_input1, 0x40), 0x60)
                }
                assembly {
                    success := call(not(0), 6, 0, add_input1, 0x80, add_input1, 0x60)
                }
            }

            add_input1[2] = hsprimes[i].x;
            add_input1[3] = hsprimes[i].y;
            assembly {
                success := call(not(0), 6, 0, add_input1, 0x80, add_input1, 0x60)
            }


            hsprime[i] = Point(add_input1[0], add_input1[1]);
        }

        return (gsprime, hsprime);

    }

}