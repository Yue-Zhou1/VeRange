pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

contract Swiftrange {
    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Part1 {
        Point X;
        Point T;
        Point Q;
        uint256 s;
        uint256[] zs;
        Point[][] ws;
    }


    uint256 public constant GX = 1;
    uint256 public constant HX = 9727523064272218541460723335320998459488975639302513747055235660443850046724;
    uint256 public constant GY = 2;
    uint256 public constant HY = 5031696974169251245229961296941447383441169981934237515842977230762345915487;

    // uint256 internal constant AA = 0;
    // uint256 internal constant BB = 3;
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

    uint256 private constant G13X = 10634783317254738331264664985092831821376621286957316213969233667217643505748;
    uint256 private constant G13Y = 3593894136463563836927847830567934263138852783181300960240960063050894828762;
    uint256 private constant G14X = 15079475582167754912076991092891442787377018931665830331839902737242049589194;
    uint256 private constant G14Y = 2994786716555744767228318303490139874612995833883024157075109374536749910797;
    uint256 private constant G15X = 5998026607105146355656696147154635391947958755485950806629481563238770197831;
    uint256 private constant G15Y = 9071495078062910622295597859043028737780972268539387614597098676522874141228;
    uint256 private constant G16X = 16523801944179829840198169326704340952379740918805059924608153703458536682146;
    uint256 private constant G16Y = 6038302089791721424956127465108246322480323157317821739578233683122154868638;
    uint256 private constant G17X = 12118397309736405059200255390820856272335184322136103314456373515523090868384;
    uint256 private constant G17Y = 3299068400210186031537495205944296158829957843773210327992629444571521214447;
    uint256 private constant G18X = 13569683714700597868720693802606454625784876902328159148425602640793284360016;
    uint256 private constant G18Y = 6196366122510722436252226379612342063576138922717891371699758504638443909810;
    uint256 private constant G19X = 6931749079582407422004463406148477362357661472338328270508373724106071518625;
    uint256 private constant G19Y = 21191302843410952823242789471696212271196775028224076743387772065534892367978;
    uint256 private constant G20X = 3818775040669594200577093816348913853129511932617816368937601194351220417610;
    uint256 private constant G20Y = 11457440080399767781624111424667375708460721196045291561100911147854033186883;
    uint256 private constant G21X = 16856378328816595579144346936425326333732976124866714942794663389927527942729;
    uint256 private constant G21Y = 8615147240200783032869516315617482186645983361490736606108099797176368213214;
    uint256 private constant G22X = 12329774238033643601677353420174969544762152665590909463734924154613306217386;
    uint256 private constant G22Y = 9349845555551207787110530432977444099219845456919646677037398830926083607248;
    uint256 private constant G23X = 15819385188090994752881147747997045863063544769706761541586812145860428725372;
    uint256 private constant G23Y = 17100833187243109040711678632824159418785961748506921615441797818807296019294;
    uint256 private constant G24X = 6331024371890361853402322881256862237801465545397987055303359505174258940787;
    uint256 private constant G24Y = 18571650698865115063332538959090545189492566919425356834234934402133902141366;

    uint256 private constant G25X = 10634783317254738331264664985092831821376621286957316213969233667217643505748;
    uint256 private constant G25Y = 3593894136463563836927847830567934263138852783181300960240960063050894828762;
    uint256 private constant G26X = 15079475582167754912076991092891442787377018931665830331839902737242049589194;
    uint256 private constant G26Y = 2994786716555744767228318303490139874612995833883024157075109374536749910797;
    uint256 private constant G27X = 5998026607105146355656696147154635391947958755485950806629481563238770197831;
    uint256 private constant G27Y = 9071495078062910622295597859043028737780972268539387614597098676522874141228;
    uint256 private constant G28X = 16523801944179829840198169326704340952379740918805059924608153703458536682146;
    uint256 private constant G28Y = 6038302089791721424956127465108246322480323157317821739578233683122154868638;
    uint256 private constant G29X = 12118397309736405059200255390820856272335184322136103314456373515523090868384;
    uint256 private constant G29Y = 3299068400210186031537495205944296158829957843773210327992629444571521214447;
    uint256 private constant G30X = 13569683714700597868720693802606454625784876902328159148425602640793284360016;
    uint256 private constant G30Y = 6196366122510722436252226379612342063576138922717891371699758504638443909810;
    uint256 private constant G31X = 6931749079582407422004463406148477362357661472338328270508373724106071518625;
    uint256 private constant G31Y = 21191302843410952823242789471696212271196775028224076743387772065534892367978;
    uint256 private constant G32X = 3818775040669594200577093816348913853129511932617816368937601194351220417610;
    uint256 private constant G32Y = 11457440080399767781624111424667375708460721196045291561100911147854033186883;


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

    function addVector(int256[] memory a, int256[] memory b) internal pure returns (int256[] memory) {
        int256[] memory result = new int256[](4);
        for (uint i=0; i<4; i++) {
            result[i] = int(addmod(uint(a[i]),uint(b[i]),NN));
        }
        return result;
    }

    function subVector(int256[] memory a, int256[] memory b) internal pure returns (int256[] memory) {
        int256[] memory result = new int256[](4);
        for (uint i=0; i<4; i++) {
            if (a[i] >= b[i]){
                result[i] = a[i] - b[i];
            }else{
                result[i] = int(NN - uint(b[i]) + uint(a[i]));
            }
            
        }
        return result;
    }

    function dot(int256[] memory a, int256[] memory b) internal pure returns (int256) {
        uint256 total = 0;
        for (uint i=0; i<4; i++) {
            total = addmod(total, mulmod(uint(a[i]), uint(b[i]), NN), NN);
        }
        return int(total);
    }

    function verifyRangeArgument(Part1 memory part1, uint256 nbits)
        external
        returns (uint256)
    {
        uint256 e = computeE(part1.X, part1.Q);
        uint256 y = computeY(part1.T, e);
        (uint256 Fx, uint256 Fy) = computeF(part1, e, y);

        Point[] memory gs = generateGS(nbits);
        Point[] memory gprimes = computeG(part1.T, gs, e, nbits);
        Point[] memory hprimes = gs;

        // uint256 challenge = y;

        (Fx, Fy, gprimes, hprimes) = computeGHprime(part1, Fx, Fy, gprimes, hprimes, y);


        uint256[] memory z2s = new uint256[](part1.zs.length);
        for(uint i=0;i<part1.zs.length;i++){
            // z2s[i] = -mulmod(part1.zs[i], part1.zs[i], NN);
            z2s[i] = mulmod(part1.zs[i], part1.zs[i], NN);
        }

        uint256[3] memory ret_mul_input;
        uint256[4] memory ret_add_input;
        bool success;

        ret_add_input[0] = 0;
        ret_add_input[1] = 0;

        for(uint i=0;i<part1.zs.length;i++) {
            // Point memory g = gprimes[i];
            ret_mul_input[0] = gprimes[i].x;
            ret_mul_input[1] = gprimes[i].y;
            ret_mul_input[2] = part1.zs[i];

            assembly {
                success := call(not(0), 7, 0, ret_mul_input, 0x80, add(ret_add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, ret_add_input, 0x80, ret_add_input, 0x60)
            }
        }

        uint256[3] memory ret2_mul_input;
        uint256[4] memory ret2_add_input;

        ret2_add_input[0] = 0;
        ret2_add_input[1] = 0;

        for(uint i=0;i<z2s.length;i++) {
            // Point memory h = hprimes[i];
            ret2_mul_input[0] = hprimes[i].x;
            ret2_mul_input[1] = hprimes[i].y;
            ret2_mul_input[2] = z2s[i];

            assembly {
                success := call(not(0), 7, 0, ret2_mul_input, 0x80, add(ret2_add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, ret2_add_input, 0x80, ret2_add_input, 0x60)
            }
        }

        uint256[4] memory ret_add_input_total;
        ret_add_input_total[0]  = ret_add_input[0];
        ret_add_input_total[1]  = ret_add_input[1];
        ret_add_input_total[2]  = ret2_add_input[0];
        ret_add_input_total[3]  = ret2_add_input[1];

        assembly {
            success := call(not(0), 6, 0, ret_add_input_total, 0x80, ret_add_input_total, 0x60)
        }
        
        // assert (ret_add_input_total[0] == Fx && ret_add_input_total[1] == Fy);

    }

    function computeGHprime(Part1 memory part1, uint256 Fx, uint256 Fy, Point[] memory gprimes, Point[] memory hprimes, uint256 challenge)
        internal
        returns(uint256 fx, uint256 fy, Point[] memory gprime, Point[] memory hprime)
    {
        // uint wSize = part1.ws.length;

        uint256[4] memory cs;
        for(uint i=0;i<part1.ws.length;i++) {

            Point[] memory elements = part1.ws[i];
            // Point memory A = elements[0];
            // Point memory B = elements[1];
            // Point memory D = elements[2];
            // Point memory E = elements[3];

            // uint256 ABDEsize = 4*2+1;

            (cs[0], cs[1], cs[2], cs[3]) = computeChallenge(elements, challenge);

            (Fx, Fy) = computeABDE(Fx,Fy,elements,cs);

            (gprimes, hprimes) = computeGHhalf(gprimes, hprimes, cs[2], cs[3]);
        }
        return(Fx, Fy, gprimes, hprimes);
    }

    function computeABDE(uint256 Fx, uint256 Fy, Point[] memory elements,uint256[4] memory cs)
        internal
        returns(uint256 fx, uint256 fy)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;

        bool success;

        mul_input[0] = elements[0].x;
        mul_input[1] = elements[0].y;
        mul_input[2] = cs[3];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = elements[1].x;
        mul_input[1] = elements[1].y;
        mul_input[2] = cs[2];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        add_input[2] = Fx;
        add_input[3] = Fy;

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = elements[2].x;
        mul_input[1] = elements[2].y;
        mul_input[2] = cs[0];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = elements[3].x;
        mul_input[1] = elements[3].y;
        mul_input[2] = cs[1];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }
        // Fx = add_input[0];
        // Fy = add_input[1];
        return (add_input[0], add_input[1]);
    }

    function computeChallenge(Point[] memory elements, uint256 challenge)
        internal
        returns(uint256 c1, uint256 c22, uint256 c3, uint256 c4)
    {
        uint256[] memory cs = new uint256[](4*2+1);
        cs[0] = elements[0].x;
        cs[1] = elements[0].y;
        cs[2] = elements[1].x;
        cs[3] = elements[1].y;
        cs[4] = elements[2].x;
        cs[5] = elements[2].y;
        cs[6] = elements[3].x;
        cs[7] = elements[3].y;
        cs[8] = challenge;

        uint256 c= mod(uint256(keccak256(abi.encodePacked(cs))), NN);
        challenge = c;
        uint256 c2 = mulmod(c, c, NN);
        uint256 cinv = modInv(c, NN);
        uint256 c2inv = mulmod(cinv, cinv, NN);
        
        return(c, c2, cinv, c2inv);
    }

    function computeGHhalf(Point[] memory gprimes, Point[] memory hprimes, uint256 cinv, uint256 c2inv)
        internal
        returns(Point[] memory gprime, Point[] memory hprime)
    {
            uint size = gprimes.length;
            uint halfSize = size / 2;

            bool success;
            // Point[] memory gL = gprimes[0:halfSize];
            // Point[] memory gR = gprimes[halfSize:size];

            // Point[] memory hL = hprimes[0:halfSize];
            // Point[] memory hR = hprimes[halfSize:size];

            Point[] memory gL = new Point[](halfSize);
            Point[] memory gR = new Point[](size-halfSize);
            Point[] memory hL = new Point[](halfSize);
            Point[] memory hR = new Point[](size-halfSize);


            for(uint i=0;i<halfSize;i++){
                gL[i] = gprimes[i];
                hL[i] = hprimes[i];
            }

            for(uint i=0;i<size-halfSize;i++){
                gR[i] = gprimes[i+halfSize];
                hR[i] = hprimes[i+halfSize];
            }

            gprimes = computeGHhalf1(gprimes, gL, gR, cinv);
            hprimes = computeGHhalf2(hprimes, hL, hR, c2inv);

            return(gprimes, hprimes);
    }

    function computeGHhalf1(Point[] memory gprimes, Point[] memory gL, Point[] memory gR, uint256 cinv)
        internal
        returns(Point[] memory gprime)
    {
        uint size = gprimes.length;
        uint halfSize = size / 2;
        bool success;
        uint256[3] memory gL_mul_input;
        uint256[4] memory gL_add_input;

        
        for(uint i=0;i<halfSize;i++){
            // calculate gprime = gL.addP(gR.mulB(cinv));
            Point memory _gR = gR[i];
            gL_mul_input[0] = _gR.x;
            gL_mul_input[1] = _gR.y;
            gL_mul_input[2] = cinv;

            assembly {
                success := call(not(0), 7, 0, gL_mul_input, 0x80, gL_add_input, 0x60)
            }

            gL_add_input[2] = gL[i].x;
            gL_add_input[3] = gL[i].y;

            assembly {
                success := call(not(0), 6, 0, gL_add_input, 0x80, gL_add_input, 0x60)
            }

            gprimes[i].x = gL_add_input[0];
            gprimes[i].y = gL_add_input[1];

            // calculate hprime = hL.addP(hR.mulB(c2inv));


        }
        return gprimes;
    }

    function computeGHhalf2(Point[] memory hprimes, Point[] memory hL, Point[] memory hR,uint256 c2inv)
        internal
        returns(Point[] memory hprime)
    {   
        uint size = hprimes.length;
        uint halfSize = size / 2;
        bool success;

        uint256[3] memory hL_mul_input;
        uint256[4] memory hL_add_input;

        
        for(uint i=0;i<halfSize;i++){
            Point memory _hR = hR[i];
            hL_mul_input[0] = _hR.x;
            hL_mul_input[1] = _hR.y;
            hL_mul_input[2] = c2inv;

            assembly {
                success := call(not(0), 7, 0, hL_mul_input, 0x80, hL_add_input, 0x60)
            }

            hL_add_input[2] = hL[i].x;
            hL_add_input[3] = hL[i].y;

            assembly {
                success := call(not(0), 6, 0, hL_add_input, 0x80, hL_add_input, 0x60)
            }

            hprimes[i].x = hL_add_input[0];
            hprimes[i].y = hL_add_input[1];
        }

        return hprimes;
    }

    function computeE(Point memory X, Point memory Q)
        internal
        returns (uint256)
    {
        uint256 size = 2 * 2;
        uint256[] memory cs = new uint256[](size);
        cs[0] = X.x;
        cs[1] = X.y;
        cs[2] = Q.x;
        cs[3] = Q.y;

        return mod(uint256(keccak256(abi.encodePacked(cs))), NN);
    }

    function computeY(Point memory T, uint256 e)
        internal
        returns (uint256)
    {
        uint256 size = 3;
        uint256[] memory cs = new uint256[](size);
        cs[0] = T.x;
        cs[1] = T.y;
        cs[2] = e;
        return mod(uint256(keccak256(abi.encodePacked(cs))), NN);
    }

    function computeG(Point memory T, Point[] memory gs, uint256 e, uint256 nbits)
        internal
        returns (Point[] memory)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;
        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = e;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        
        uint256 gex = add_input[0];
        uint256 gey = add_input[1];
        Point memory ge;
        ge.x = gex;
        ge.y = gey;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = e;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }  
        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        Point memory ge2power;
        ge2power.x = add_input[0];
        ge2power.y = add_input[1];

        Point[] memory ge2powers = new Point[](nbits);
        ge2powers[0] = ge;

        for(uint i=0;i<nbits-1;i++) {
            Point memory tmp = ge2powers[i];
            add_input[0] = tmp.x;
            add_input[1] = tmp.y;
            add_input[2] = tmp.x;
            add_input[3] = tmp.y;
            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            ge2powers[i+1].x = add_input[0];
            ge2powers[i+1].y = add_input[1];

        }
        Point[] memory gprimes = new Point[](nbits);
        for(uint i=0;i<nbits;i++) {
            add_input[0] = ge2powers[i].x;
            add_input[1] = ge2powers[i].y;
            add_input[2] = gs[i].x;
            add_input[3] = gs[i].y;
            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
            gprimes[i].x = add_input[0];
            gprimes[i].y = add_input[1];
        }
        return gprimes;
    }


    function generateGS(uint256 nbits)
        internal
        returns (Point[] memory)
    {
        Point[] memory gs = new Point[](nbits);
        gs[0] = Point(G1X, G1Y);
        gs[1] = Point(G2X, G2Y);
        gs[2] = Point(G3X, G3Y);
        gs[3] = Point(G4X, G4Y);
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
        gs[16] = Point(G17X, G17Y);
        gs[17] = Point(G18X, G18Y);
        gs[18] = Point(G19X, G19Y);
        gs[19] = Point(G20X, G20Y);
        gs[20] = Point(G21X, G21Y);
        gs[21] = Point(G22X, G22Y);
        gs[22] = Point(G23X, G23Y);
        gs[23] = Point(G24X, G24Y);
        gs[24] = Point(G25X, G25Y);
        gs[25] = Point(G26X, G26Y);
        gs[26] = Point(G27X, G27Y);
        gs[27] = Point(G28X, G28Y);
        gs[28] = Point(G29X, G29Y);
        gs[29] = Point(G30X, G30Y);
        gs[30] = Point(G31X, G31Y);
        gs[31] = Point(G32X, G32Y);

        gs[32] = Point(G1X, G1Y);
        gs[33] = Point(G2X, G2Y);
        gs[34] = Point(G3X, G3Y);
        gs[35] = Point(G4X, G4Y);
        gs[36] = Point(G5X, G5Y);
        gs[37] = Point(G6X, G6Y);
        gs[38] = Point(G7X, G7Y);
        gs[39] = Point(G8X, G8Y);
        gs[40] = Point(G9X, G9Y);
        gs[41] = Point(G10X, G10Y);
        gs[42] = Point(G11X, G11Y);
        gs[43] = Point(G12X, G12Y);
        gs[44] = Point(G13X, G13Y);
        gs[45] = Point(G14X, G14Y);
        gs[46] = Point(G15X, G15Y);
        gs[47] = Point(G16X, G16Y);
        gs[48] = Point(G17X, G17Y);
        gs[49] = Point(G18X, G18Y);
        gs[50] = Point(G19X, G19Y);
        gs[51] = Point(G20X, G20Y);
        gs[52] = Point(G21X, G21Y);
        gs[53] = Point(G22X, G22Y);
        gs[54] = Point(G23X, G23Y);
        gs[55] = Point(G24X, G24Y);
        gs[56] = Point(G25X, G25Y);
        gs[57] = Point(G26X, G26Y);
        gs[58] = Point(G27X, G27Y);
        gs[59] = Point(G28X, G28Y);
        gs[60] = Point(G29X, G29Y);
        gs[61] = Point(G30X, G30Y);
        gs[62] = Point(G31X, G31Y);
        gs[63] = Point(G32X, G32Y);
        // gs[32] = Point(G33X, G33Y);


        return gs;
    }


    function computeF(Part1 memory part1, uint256 e, uint256 y)
        internal
         returns (uint256 x, uint256 z)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;
        mul_input[0] = HX;
        mul_input[1] = HY;
        // mul_input[2] = -part1.s;
        mul_input[2] = part1.s;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = part1.X.x;
        mul_input[1] = part1.X.y;
        mul_input[2] = e;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = part1.T.x;
        mul_input[1] = part1.T.y;
        mul_input[2] = y;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = part1.Q.x;
        mul_input[1] = part1.Q.y;
        mul_input[2] = mulmod(y, y, NN);

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        return (add_input[0], add_input[1]);

    }

}