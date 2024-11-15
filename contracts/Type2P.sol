pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

contract Type2P {
    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Part1 {
        Point[] cys;
        Point[] cts;
        Point[] cws;
        Point[] cms;
        Point[] cfs;
        Point[] ctks;
        Point bigS;
        Point bigR;
        Point bigU;
        uint256 eta1;
        uint256 eta2;
        uint256 eta3;
        uint256 eta4;
        uint256[] vs;
        uint256[] us;
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

    uint256 private constant G17X = 5570485579165854778333882310324710971789817290946179519363431408236767642025;
    uint256 private constant G17Y = 21398188111975436403033815142789284243728162863175574797813644030243111432723;
    uint256 private constant G18X = 15640119229432533056681054993112105042325750717113753578574477955800827371078;
    uint256 private constant G18Y = 2113838805433469456989639886845836342358653034077300075871559986044145709851;
    uint256 private constant G19X = 2035475386035184297973069222966472230400163889919253400102396725774720235601;
    uint256 private constant G19Y = 19394107776903233319882814583649842696771001238099358456066231371632095804852;
    uint256 private constant G20X = 11851661346648436644483213042559466346806759178358673953507381641445400861248;
    uint256 private constant G20Y = 642661234519408718197147398870868488482607439639720037787987068720241930193;
    uint256 private constant G21X = 5347285414875161975481687284958039928210053061957182491238984467702427655612;
    uint256 private constant G21Y = 3099959058991016370419506078481682625918216798352735876784995901174895803790;
    uint256 private constant G22X = 8837298087035723254369996195110674723419292042173425868352331347955564467033;
    uint256 private constant G22Y = 2379890648151669613853318896125023973754490182109761533732505714585380192719;
    uint256 private constant G23X = 17668078296193790616526761332142542689252203291277004341266407069585110496902;
    uint256 private constant G23Y = 16466492846497685284849642183120228339590418108818978601307019716754728496708;
    uint256 private constant G24X = 9899427479713440420962224239555118897455711805592551006408551457704093412326;
    uint256 private constant G24Y = 5869088647977271387896391532814952024426373595464501116000195960539902931310;
    uint256 private constant G25X = 2709065229911660446906168584856121860788968577341783949349168665497372648963;
    uint256 private constant G25Y = 14184193786661602402994784165993244999652318117892719040115199716896857030182;
    uint256 private constant G26X = 13565834010494535499488693738384458593705263750185712009037200934901090859714;
    uint256 private constant G26Y = 6245240937488103784065209194223783209021543334227188901490879199561059367981;
    uint256 private constant G27X = 4092313457683891490413550449586535203209892674425828372308213846211349484074;
    uint256 private constant G27Y = 16872567688624717878032260659892206633390593202181292631644822601413842208633;
    uint256 private constant G28X = 4553004161353060887246824516264925406587319093125777949522400755502402396666;
    uint256 private constant G28Y = 3075382425323426719209279800148558800583339640679153179548697689921838337034;
    uint256 private constant G29X = 311857847357728154159403235153376734323726250203112864617928012022507073573;
    uint256 private constant G29Y = 8688603317982193527229691655072264382885141586094568525048737637397251390890;
    uint256 private constant G30X = 19117553569066493331738151243068391017461422497993408331294089510932019412121;
    uint256 private constant G30Y = 20617633754962769445348211083982423856819190232644058336453572591276996727731;
    uint256 private constant G31X = 21564936991892425538335423624933325531111739269421169886388309873072809102478;
    uint256 private constant G31Y = 10135496737399729704445973248276843753163730340832867462730499408211493967835;
    uint256 private constant G32X = 20156754468088616078451455716887760260740692349173212848087089719966792160449;
    uint256 private constant G32Y = 3552500512111734652168112934833420055478652809218983229941809283680875828109;
    uint256 private constant G33X = 10628536301278644135479422693566851380981051142638279269633607303687141726330;
    uint256 private constant G33Y = 1103755251586637467916412419034971724084742837105502356028153521025485396186;

    uint256[] alphac_1 = [20766681261493590952505260514604317858808106340147380917920307259637232328681, 14315455267034475364186235486838420386674315911382320817580186330494156800880, 4827872744801420301998832152661940008563912615397196077730304664243803538281, 5098761168437123970891355511842619119085570712827417063956112574980761952276, 11510271572274357798659441528861677861659567610030457512020346067337529770472, 15332053560713568618631423070659526151481935939563374313722832448548616796027, 11718228790034310875683360649074051090025459512970934621835406236730875877675, 20706656769714075361821309631957781973934491683417751307668901785469417455236];

    uint256[] Binvs = [14759434278424752211830011276142021483426032630828210921922530447082500778225, 2245877984169635654091320868349535733361322309058628904028363872758577130797];
    uint256 L = 15; 
    uint256 K = 3;
    uint256 B = 8; 
    uint256 nbits = 45;
    uint TT = 4;


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


    function dot(uint256[] memory a, uint256[] memory b) internal pure returns (uint256) {
        uint256 total = 0;
        for (uint i=0; i<4; i++) {
            total = addmod(total, mulmod(uint(a[i]), uint(b[i]), NN), NN);
        }
        return total;
    }

    function mul(uint256[] memory a, uint256 b) internal pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](a.length);
        for (uint i=0; i<a.length; i++) {
            result[i] = mulmod(a[i], b, NN);
        }
        return result;
    }

    function mul(uint256[] memory a, uint256[] memory b) internal pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](a.length);
        for (uint i=0; i<a.length; i++) {
            result[i] = mulmod(a[i], b[i], NN);
        }
        return result;
    }

    function add(uint256[] memory a, uint256[] memory b) internal pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](a.length);
        for (uint i=0; i<a.length; i++) {
            result[i] = addmod(a[i], b[i], NN);
        }
        return result;
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
        (uint256[] memory betas, uint256 alpha, uint256 chalPow2Sum) = computeChallenges(part1, K);
        uint256[] memory gamma = computeGamma(part1.cys);

        uint256[] memory uv_sum = computeVhat(part1.vs, part1.us, betas);

        checkCond1(uv_sum, betas, chalPow2Sum, part1);

        checkCond2(part1, betas);

        checkCond3(alpha, part1, betas);

        checkCond4(alpha, betas, part1, gamma);

        checkCond5(part1.cys, part1.cws, gamma);


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

    function computeVhat(uint256[] memory vs, uint256[] memory us, uint256[] memory betas)
        internal
        returns (uint256[] memory x)
    {
        uint256[] memory uv_sum = new uint256[](L*TT);
        for(uint i = 0; i<L*TT; i++){
            uv_sum[i] = mulmod(vs[i], us[i], NN);
        }

        return uv_sum;
    }

    function computeChallenges(Part1 memory part1, uint K)
        internal
        pure
        returns (uint256[] memory x, uint256 y, uint256 z)
    {

        (uint256 alpha, uint256 challenge) = computeBaseChallenge(part1);

        uint256[] memory challenges = new uint256[](K);
        uint256[] memory challengesPow2 = new uint256[](K);

        uint256 challenge2 = mulmod(challenge, challenge, NN);
        uint256 challenge3 = mulmod(challenge2, challenge, NN);
        uint256 challenge4 = mulmod(challenge2, challenge2, NN);
        
        challenges[0] = challenge;
        challenges[1] = challenge2;
        challengesPow2[0] = challenge2;
        challengesPow2[1] = challenge4;
        if (K == 3){
            uint256 challenge6 = mulmod(challenge3, challenge3, NN);
            challenges[2] = challenge3;
            challengesPow2[2] = challenge6;
        }
        if (K == 4){
            uint256 challenge6 = mulmod(challenge3, challenge3, NN);
            uint256 challenge8 = mulmod(challenge4, challenge4, NN);
            challenges[2] = challenge3;
            challenges[3] = challenge4;
            challengesPow2[2] = challenge6;
            challengesPow2[3] = challenge8;
        }
        uint size = challengesPow2.length;
        uint256 chalPow2Sum = 0;
        for(uint i=0;i<size;i++){
            chalPow2Sum = addmod(challengesPow2[i], chalPow2Sum, NN);
        }

        return (challenges, alpha, chalPow2Sum);
    }

    function checkCond1(uint256[] memory uv_sum, uint256[] memory betas, uint256 e2, Part1 memory part1)
        internal
    {
        // uint256[] memory fvs = computeUV(part1.us, part1.vs);
        (uint256 uvpx, uint256 uvpy, uint256 hbetaspow2px, uint256 hbetaspow2py) = computeUVPoint(uv_sum, e2);

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        uint256[2] memory HbetasPow2;
        bool success;

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part1.eta1;
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

        HbetasPow2[0] = hbetaspow2px;
        HbetasPow2[1] = hbetaspow2py;
        (add_input[0], add_input[1]) = computeCTS(HbetasPow2, betas, part1);

    }

    function checkCond4(uint256 alpha, uint256[] memory challenges, Part1 memory part1, uint256[] memory gamma)
        internal
    {

        uint256 vsum = 0;
        uint vsize = part1.vs.length;
        for(uint i=0;i<vsize;i++) {
            vsum = addmod(vsum, part1.vs[i], NN);
        }
        uint tt = 0;
        uint256 alpha_b_e = 0;
        for(uint i=0;i<K;i++) {
            uint256 alpha_es = mulmod(alpha, challenges[i], NN);
            uint256 Bk_sum = 0;
            uint base = i*L;
            for(uint j = 0; j < L*TT; j++){
                if(i == L*tt){
                    tt+=1;
                }
                uint _i = i;
                if (i >= L) _i = i-L*(tt-1);
                // Bk_sum = addmod(Bk_sum, B**base+j, NN);
                Bk_sum = addmod(Bk_sum, mulmod(gamma[tt-1], B ** (base+_i), NN), NN);
            }
            alpha_b_e = addmod(alpha_b_e, mulmod(alpha_es, Bk_sum, NN), NN);
        }
        // for(uint i=0;i<K;i++) {
        //     uint256 alpha_es = mulmod(alpha, challenges[i], NN);
        //     uint256 Bk_sum = 0;
        //     uint base = i*L;
        //     for(uint j = 0; j < L; j++){
        //         Bk_sum = addmod(Bk_sum, B**base+j, NN);
        //     }
        //     alpha_b_e = addmod(alpha_b_e, mulmod(alpha_es, Bk_sum, NN), NN);
        // }
        
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = submod(vsum, alpha_b_e, NN);
        // mul_input[2] = aa;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part1.eta4;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret2x = add_input[0];
        uint256 ret2y = add_input[1];

        (add_input[0], add_input[1]) = computeCWS(challenges, part1.cws, part1.bigR);
        
        // assert(ret2x==add_input[0] && ret2y==add_input[1]);
    }

    function computeCTS(uint256[2] memory HbetasPow2, uint256[] memory betas, Part1 memory part1)
        internal
        returns (uint256 x, uint256 y)
    {
       uint size = part1.cts.length;

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        Point memory p = part1.cts[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = betas[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size;i++){
            Point memory p = part1.cts[i];
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

        uint sizetk = part1.ctks.length;
        Point memory pt = part1.ctks[0];
        mul_input[0] = pt.x;
        mul_input[1] = pt.y;
        mul_input[2] = mulmod(betas[0], betas[1], NN);
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        if(K==3){
            Point memory pt = part1.ctks[1];
            mul_input[0] = pt.x;
            mul_input[1] = pt.y;
            mul_input[2] = mulmod(betas[1], betas[2], NN);
            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            Point memory pt1 = part1.ctks[2];
            mul_input[0] = pt1.x;
            mul_input[1] = pt1.y;
            mul_input[2] = mulmod(betas[2], betas[0], NN);
            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        // if(K==4){
        //     Point memory pt = part1.ctks[1];
        //     mul_input[0] = pt.x;
        //     mul_input[1] = pt.y;
        //     mul_input[2] = mulmod(betas[0], betas[2], NN);
        //     assembly {
        //         success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        //     }

        //     assembly {
        //         success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        //     }

        //     Point memory pt1 = part1.ctks[2];
        //     mul_input[0] = pt1.x;
        //     mul_input[1] = pt1.y;
        //     mul_input[2] = mulmod(betas[0], betas[3], NN);
        //     assembly {
        //         success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        //     }

        //     assembly {
        //         success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        //     }

        //     mul_input[0] = part1.ctks[3].x;
        //     mul_input[1] = part1.ctks[3].y;
        //     mul_input[2] = mulmod(betas[1], betas[2], NN);
        //     assembly {
        //         success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        //     }

        //     assembly {
        //         success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        //     }

        //     mul_input[0] = part1.ctks[4].x;
        //     mul_input[1] = part1.ctks[4].y;
        //     mul_input[2] = mulmod(betas[1], betas[3], NN);
        //     assembly {
        //         success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        //     }

        //     assembly {
        //         success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        //     }

        //     mul_input[0] = part1.ctks[5].x;
        //     mul_input[1] = part1.ctks[5].y;
        //     mul_input[2] = mulmod(betas[2], betas[3], NN);
        //     assembly {
        //         success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        //     }

        //     assembly {
        //         success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        //     }


        // }

        add_input[2] = part1.bigS.x;
        add_input[3] = part1.bigS.y;

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        add_input[2] = HbetasPow2[0];
        add_input[3] = HbetasPow2[1];

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }
        

        return (add_input[0], add_input[1]);
    }

    function computeCWS(uint256[] memory challenges, Point[] memory cws, Point memory bigR)
        internal
        returns (uint256 x, uint256 y)
    {
        uint size = cws.length;

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        Point memory p = cws[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = challenges[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size;i++){
            Point memory p = cws[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = challenges[i];

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

    function checkCond3(uint256 alpha, Part1 memory part1, uint256[] memory betas)
        internal
    {

        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        uint fsize = part1.cfs.length;
        Point memory p = part1.cfs[0];
        add_input[0] = p.x;
        add_input[1] = p.y;

        for(uint i=1;i<fsize;i++) {
            Point memory p = part1.cfs[i];
            add_input[2] = p.x;
            add_input[3] = p.y;
            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }
        
        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part1.eta3;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret3x = add_input[0];
        uint256 ret3y = add_input[1];

        (add_input[0], add_input[1]) = computeCMS(alpha, part1, betas);
        
        
    }

    function checkCond2(Part1 memory part1, uint256[] memory betas)
        internal
    {

        uint256 uj_B_sum = 0;
        uint256 tt = 0;
        for(uint i = 0; i < part1.us.length; i++){
            if(i == L*tt){
                tt+=1;
            }
            uint _i = i;
            if (i >= L) _i = i-L*(tt-1);
            uj_B_sum = addmod(uj_B_sum, mulmod(B**_i, part1.us[i], NN), NN);
        }
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = uj_B_sum;
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

        (add_input[0], add_input[1]) = computeCFS(part1, betas);
    }

    function computeCFS(Part1 memory part1, uint256[] memory betas)
        internal
        returns (uint256 x, uint256 y)
    {
        uint size = part1.cfs.length;
        
        uint256[4] memory add_input;
        uint256[3] memory mul_input;
        bool success;

        Point memory p = part1.cfs[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = betas[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size;i++){
            Point memory p = part1.cfs[i];
            uint exp = L*(i-1)+nbits-L*(K-1);
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            // mul_input[2] = mulmod(betas[i], modInv(B**exp, NN), NN);
            mul_input[2] = mulmod(betas[i], Binvs[i-1], NN);
            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        add_input[2] = part1.bigU.x;
        add_input[3] = part1.bigU.y;

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        return (add_input[0], add_input[1]);
    }

    function computeCMS(uint256 alpha, Part1 memory part1, uint256[] memory betas)
        internal
        returns (uint256 x, uint256 y)
    {
        uint size = part1.cms.length;
        uint256[4] memory add_input;
        uint256[3] memory mul_input;
        bool success;
        
        Point memory p = part1.cms[0];
        mul_input[0] = p.x;
        mul_input[1] = p.y;
        mul_input[2] = alphac_1[0];
        // mul_input[2] = modInv(alpha, NN);
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size;i++){
            Point memory p = part1.cms[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = alphac_1[i];
            // mul_input[2] = modInv(addmod(alpha, i, NN), NN);
            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }

        return (add_input[0], add_input[1]);
    }

    function checkCond5(Point[] memory cys, Point[] memory cws, uint256[] memory gamma)
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

    function computeBaseChallenge(Part1 memory part1)
        internal
        pure
        returns (uint256 x, uint256 y)
    {
        uint cwslength = part1.cws.length;
        uint cmslength = part1.cms.length;
        uint cfslength = part1.cfs.length;
        uint ctslength = part1.cts.length;
        uint ctkslength = part1.ctks.length;

        uint256 size = 2 * (cwslength + cmslength + cfslength + ctslength + ctkslength);
        uint256[] memory cs = new uint256[](size);
        uint offset = 0;
        for(uint i=0;i<cwslength;i++) {
            Point memory p = part1.cws[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        for(uint i=0;i<cmslength;i++) {
            Point memory p = part1.cms[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }

        uint256 alpha = mod(uint256(keccak256(abi.encodePacked(cs))), NN);
        for(uint i=0;i<cfslength;i++) {
            Point memory p = part1.cfs[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        for(uint i=0;i<ctslength;i++) {
            Point memory p = part1.cts[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        for(uint i=0;i<ctkslength;i++) {
            Point memory p = part1.ctks[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        uint256 baseChal = mod(uint256(keccak256(abi.encodePacked(cs))), NN);

        return (alpha, baseChal);
    }


    function computeUVPoint(uint256[] memory uvs, uint256 e2)
        internal
        returns (uint256 x, uint256 y, uint256 x1, uint256 y1)
    {
        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        uint256[3] memory mul_input1;
        uint256[4] memory add_input1;
        bool success;

        add_input[0] = 0;
        add_input[1] = 0;

        add_input1[0] = 0;
        add_input1[1] = 0;
        uint size = uvs.length;

        Point[] memory gs = generateGS(size);
        for(uint i=0;i<size;i++) {
            Point memory g = gs[i];
            mul_input[0] = g.x;
            mul_input[1] = g.y;
            mul_input[2] = uvs[i];

            add_input1[0] = g.x;
            add_input1[1] = g.y;

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input1, 0x80, add_input1, 0x60)
            }
        }
        mul_input1[0] = add_input1[0];
        mul_input1[1] = add_input1[1];
        mul_input1[2] = e2;
        assembly {
            success := call(not(0), 7, 0, mul_input1, 0x80, add(add_input1, 0x40), 0x60)
        }

        return (add_input[0], add_input[1], add_input1[0], add_input1[1]);
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
        gs[3] = Point(G4X, G4Y);

        // if (L == 8){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        // }
        // if (L == 9){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        //     gs[8] = Point(G9X, G9Y);
        // }
        if (L==15){
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
        }
        // if (L==20){
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
        // }
        // if(L == 33){
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
        //     gs[23] = Point(G24X, G24Y);
        //     gs[24] = Point(G25X, G25Y);
        //     gs[25] = Point(G26X, G26Y);
        //     gs[26] = Point(G27X, G27Y);
        //     gs[27] = Point(G28X, G28Y);
        //     gs[28] = Point(G29X, G29Y);
        //     gs[29] = Point(G30X, G30Y);
        //     gs[30] = Point(G31X, G31Y);
        //     gs[31] = Point(G32X, G32Y);
        //     gs[32] = Point(G33X, G33Y);
        // }

        return gs;
    }

}