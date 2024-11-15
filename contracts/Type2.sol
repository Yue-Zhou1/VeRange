pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

contract Type2 {
    struct Point {
        uint256 x;
        uint256 y;
    }

    struct Part1 {
        Point[] cys;
        Point[] cts;
        Point[] cws;
        Point[] cms;
        Point[] cvs;
        Point bigS;
        Point bigR;
        uint256 eta1;
        uint256 eta2;
        uint256 eta3;
        uint256[][] vs;
        uint256[][] us;
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



    // uint256[] einverse = [17809065204268837970966994315047722988502264046101232976071578269104755116602,12054485186898008393284206020130475786799422697225037406356499066507434714057, 1539115220062195988246022468872989257463825283816411314942754256717978781735, 6282232039555864577334268550121993023406034053211810770941906573736426659689, 11185806933410005586042277664511745277012922371976721676879866699965559466359, 11185806933410005586042277664511745277012922371976721676879866699965559466359, 11185806933410005586042277664511745277012922371976721676879866699965559466359, 17809065204268837970966994315047722988502264046101232976071578269104755116602,17809065204268837970966994315047722988502264046101232976071578269104755116602,17809065204268837970966994315047722988502264046101232976071578269104755116602,17809065204268837970966994315047722988502264046101232976071578269104755116602];
    // uint256[] alphac_1 = [5550180277690528207856717745072138322374346214029977712479326786088622108114,8602056393745822828810900486284363287640637717901613617132724268249020118644, 5494113431316987163137090137496534419383083688613851930549720770694093691697,5111633420753249620132231268381065582706905696709441820229858643715625187317,12103905405397911357896740673648724565126579263829802347895336612721145856424,12103905405397911357896740673648724565126579263829802347895336612721145856424, 5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114,5550180277690528207856717745072138322374346214029977712479326786088622108114];
    
    uint256[] einverse = [3365088986663205768765383595511592812019946708050536029091954850130653085931, 5691060676627603837048379547483348437142527750252328371677136163876211777479, 17736554512503597527083500584146370905747485582179039184758796073281469001293, 4231177393715698714168644793479969344133116046245141971465125033865735427421, 11104064222650389898946302240479911887186191021923314686318105173154073381271, 19793166473621288482676860749151918871872426247139896009655407255278657395640, 942342492839062770636595423980720727367877557524865407163585095915146125760, 645385359095710375618099556642531175582603613850549205998458315164754997971, 19415307115350052823844160940767395153738756333534889277799615315537181418335, 11472082732492054883397588652265120430930980903371433065130065589877251935186, 10665521777186371854787268300764862791854429038769608830854903410840753746365, 6440814252712788342405889136734382103520618774143072272494218817685162629539];
    uint256[] alphac_1 = [12260876186876982405456758625475171805592108000831809901291060534395078488621, 7145715402997682992115709143948493086378421585058060005355022379190684394401, 1953826107791151999400203619405478320464029460119205867948177161973926398590, 13118627379584166653839204218561691427672864445685254949520527889049768905515, 224867684540777752729739222047649266371593879184390336028188121431461220831, 10960153415374973537800046563256921193841018449683490747563722439187501578282, 4868078058674781593842116224104776240467190889823015028286118582617734236297, 7239399272488151191403784727387820107903520006300513205889428281103690263701, 17574530370321306421650240679278654520700773258042495258819497247172160195927, 4535116424452858123086214660955147997689577822595899304818687107443279060038, 7454162298295462330284040331158530411783859628308489354324106260669649559537, 16152819895345957686841444956670630570596737999894059471967359392867344539041, 15822710163143594116558778423215153094166127556088504170602717631412736070997, 20562096528975541450854981554029443302126777709204422720023078965734720030187, 11204392888918284146604716600068522070593981001546405950459708108125291099468];

    uint256 L = 11; 
    uint256 K = 12;
    uint256 B = 15; 
    uint256 nbits = 132;


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

        uint256[] memory uv_sum;
        if (part1.cys.length > 1){
            uv_sum = computeVhat_agg(part1, alpha, betas, gamma);
        }else{
            uv_sum = computeVhat(part1.vs, part1.us, alpha, betas);
        }

        checkCond1(uv_sum, betas, chalPow2Sum, part1);

        checkCond2(alpha, part1, betas);

        checkCond3(betas, part1.cws, part1.vs, part1.eta3, part1.bigR);

        checkCond4(part1.cys, part1.cws, gamma);


    }

    function computeVhat(uint256[][] memory vs, uint256[][] memory us, uint256 alpha, uint256[] memory betas)
        internal
        returns (uint256[] memory x)
    {
        // uint256[][] memory vhats = new uint256[][](L);
        // uint256[][] memory uhats = new uint256[][](L);
        uint256[] memory uv_sum = new uint256[](L);

        for(uint i=0;i<L;i++) {
            uint256[] memory fvs = new uint256[](K);
            uint256[] memory fus = new uint256[](K);
            uint256 fuvs = 0;
            uint256 f_v = 0;
            // uint base = i * K;
            for(uint j=0;j<K;j++) {
                uint idx = i*K + j;
                if(idx < nbits) {
                    uint256 alpha_beta = mulmod(alpha, betas[j] , NN);
                    f_v = addmod(f_v, mulmod(alpha_beta, B ** (idx), NN), NN);
                    fvs[j] = addmod(f_v, vs[i][j], NN);
                    fus[j] = us[i][j];
                    fuvs = addmod(fuvs, mulmod(fvs[j], fus[j], NN), NN);
                }else{
                    f_v = betas[j];
                    fvs[j] = f_v;
                    fus[j] = f_v;
                    fuvs = addmod(fuvs, mulmod(fvs[j], fus[j], NN), NN);
                }
            }
        // vhats[i] = fvs;
        // uhats[i] = fus;
            uv_sum[i] = fuvs;
        }

        return uv_sum;
    }

    function computeVhat_agg(Part1 memory part1, uint256 alpha, uint256[] memory betas, uint256[] memory gamma)
        internal
        returns (uint256[] memory x)
    {
        uint256[] memory uv_sum = new uint256[](11);
        uint tt = 0;
        for(uint i=0;i<11;i++) {
            uint256[] memory fvs = new uint256[](12);
            uint256[] memory fus = new uint256[](12);
            uint256 fuvs = 0;
            uint256 f_v = 0;
            for(uint j=0;j<12;j++) {
                uint idx = i*12 + j;
                // uint mod_agg = idx % 36;
                if(idx % 33 == 0){
                    tt+=1;
                }
                if(idx < nbits) {
                    uint256 alpha_beta = mulmod(alpha, betas[j] , NN);
                    f_v = addmod(f_v, mulmod(alpha_beta, mulmod(gamma[tt-1], B ** (idx % 33), NN), NN), NN);
                    fvs[j] = addmod(f_v, part1.vs[i][j], NN);
                    fus[j] = part1.us[i][j];
                    fuvs = addmod(fuvs, mulmod(fvs[j], fus[j], NN), NN);
                }else{
                    f_v = betas[j];
                    fvs[j] = f_v;
                    fus[j] = f_v;
                    fuvs = addmod(fuvs, mulmod(fvs[j], fus[j], NN), NN);
                }
            }
            uv_sum[i] = fuvs;
        }

        return uv_sum;
    }

    function computeUprime(uint256[][] memory us, uint256 alpha, uint256[] memory betas)
        internal
        returns (uint256 x)
    {
        uint256 uprimes;
        // uint256[][] memory uprimes = new uint256[][](L);
        for(uint i=0;i<L;i++) {
            // uint256[] memory fuprimes = new uint256[](K);
            uint256 f_uprimes = 0;
            uint base = i * K;
            for(uint j=0;j<K;j++) {
                uint idx = base + j;
                if(idx < nbits) {
                    f_uprimes = mulmod(mulmod(B << (idx), einverse[j], NN), us[i][j], NN);
                    // fuprimes[j] = f_uprimes;
                    uprimes = addmod(uprimes, f_uprimes, NN);
                }
            }
            // uprimes[i] = fuprimes;
        }
        return uprimes;

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

    function computeChallenges(Part1 memory part1, uint K)
        internal
        pure
        returns (uint256[] memory x, uint256 y, uint256 z)
    {

        (uint256 alpha, uint256 challenge) = computeBaseChallenge(part1.cws, part1.cms, part1.cvs, part1.cts);

        uint256[] memory challenges = new uint256[](K);
        uint256[] memory challengesPow2 = new uint256[](K);

        uint256 challenge2 = mulmod(challenge, challenge, NN);
        uint256 challenge3 = mulmod(challenge2, challenge, NN);
        uint256 challenge4 = mulmod(challenge2, challenge2, NN);
        uint256 challenge6 = mulmod(challenge3, challenge3, NN);

        uint256 challenge8 = mulmod(challenge4, challenge4, NN);

        challenges[0] = challenge;
        challenges[1] = challenge2;
        challenges[2] = challenge3;
        challenges[3] = challenge4;
        challengesPow2[0] = challenge2;
        challengesPow2[1] = challenge4;
        challengesPow2[2] = challenge6;
        challengesPow2[3] = challenge8;

        if (K == 5) {
            uint256 challenge5 = mulmod(challenge4, challenge, NN);
            uint256 challenge10 = mulmod(challenge5, challenge5, NN);
            challenges[4] = challenge5;
            challengesPow2[4] = challenge10;
        }

        if (K == 7) {
            uint256 challenge5 = mulmod(challenge4, challenge, NN);
            uint256 challenge6 = mulmod(challenge4, challenge2, NN);
            uint256 challenge7 = mulmod(challenge4, challenge3, NN);
            uint256 challenge10 = mulmod(challenge5, challenge5, NN);
            uint256 challenge12 = mulmod(challenge6, challenge6, NN);
            uint256 challenge14 = mulmod(challenge7, challenge7, NN);
            challenges[4] = challenge5;
            challenges[5] = challenge6;
            challenges[6] = challenge7;
            challengesPow2[4] = challenge10;
            challengesPow2[5] = challenge12;
            challengesPow2[6] = challenge14;
        }
        if (K == 8) {
            uint256 challenge5 = mulmod(challenge4, challenge, NN);
            uint256 challenge6 = mulmod(challenge4, challenge2, NN);
            uint256 challenge7 = mulmod(challenge4, challenge3, NN);
            uint256 challenge10 = mulmod(challenge5, challenge5, NN);
            uint256 challenge12 = mulmod(challenge6, challenge6, NN);
            uint256 challenge14 = mulmod(challenge7, challenge7, NN);
            uint256 challenge16 = mulmod(challenge8, challenge8, NN);
            challenges[4] = challenge5;
            challenges[5] = challenge6;
            challenges[6] = challenge7;
            challenges[7] = challenge8;
            challengesPow2[4] = challenge10;
            challengesPow2[5] = challenge12;
            challengesPow2[6] = challenge14;
            challengesPow2[7] = challenge16;
        }

        if (K == 11) {
            uint256 challenge5 = mulmod(challenge4, challenge, NN);
            uint256 challenge6 = mulmod(challenge4, challenge2, NN);
            uint256 challenge7 = mulmod(challenge4, challenge3, NN);
            uint256 challenge9 = mulmod(challenge4, challenge5, NN);
            uint256 challenge10 = mulmod(challenge5, challenge5, NN);
            uint256 challenge11 = mulmod(challenge5, challenge6, NN);
            // uint256 challenge12 = mulmod(challenge6, challenge6, NN);
            // uint256 challenge14 = mulmod(challenge7, challenge7, NN);
            // uint256 challenge16 = mulmod(challenge8, challenge8, NN);
            // uint256 challenge18 = mulmod(challenge9, challenge9, NN);
            // uint256 challenge20 = mulmod(challenge10, challenge10, NN);
            // uint256 challenge22 = mulmod(challenge11, challenge11, NN);
            challenges[4] = challenge5;
            challenges[5] = challenge6;
            challenges[6] = challenge7;
            challenges[7] = challenge8;
            challenges[8] = challenge9;
            challenges[9] = challenge10;
            challenges[10] = challenge11;
            challengesPow2[4] = challenge10;
            // challengesPow2[5] = challenge12;
            // challengesPow2[6] = challenge14;
            // challengesPow2[7] = challenge16;
            // challengesPow2[8] = challenge18;
            // challengesPow2[9] = challenge20;
            // challengesPow2[10] = challenge22;

        }

        if (K == 12) {
            uint256 challenge5 = mulmod(challenge4, challenge, NN);
            // uint256 challenge6 = mulmod(challenge4, challenge2, NN);
            // uint256 challenge7 = mulmod(challenge4, challenge3, NN);
            // uint256 challenge9 = mulmod(challenge4, challenge5, NN);
            // uint256 challenge10 = mulmod(challenge5, challenge5, NN);
            // uint256 challenge11 = mulmod(challenge5, challenge6, NN);
            // uint256 challenge12 = mulmod(challenge6, challenge6, NN);
            // uint256 challenge14 = mulmod(challenge7, challenge7, NN);
            // uint256 challenge16 = mulmod(challenge8, challenge8, NN);
            // uint256 challenge18 = mulmod(challenge9, challenge9, NN);
            // uint256 challenge20 = mulmod(challenge10, challenge10, NN);
            // uint256 challenge22 = mulmod(challenge11, challenge11, NN);
            challenges[4] = challenge5;
            challenges[5] = mulmod(challenge4, challenge2, NN);
            challenges[6] = mulmod(challenge4, challenge3, NN);
            challenges[7] = challenge8;
            challenges[8] = mulmod(challenge8, challenge, NN);
            challenges[9] = mulmod(challenge8, challenge2, NN);
            challenges[10] = mulmod(challenge8, challenge3, NN);
            challenges[11] = mulmod(challenge8, challenge4, NN);
            challengesPow2[4] = challenges[9];
            challengesPow2[5] = challenges[11];
            challengesPow2[6] = mulmod(challenges[6], challenges[6], NN);
            challengesPow2[7] = mulmod(challenge8, challenge8, NN);
            challengesPow2[8] = mulmod(challenges[8], challenges[8], NN);
            challengesPow2[9] = mulmod(challenges[9], challenges[9], NN);


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

    function checkCond3(uint256[] memory challenges, Point[] memory cws, uint256[][] memory vs, uint256 eta3, Point memory bigR)
        internal
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
        mul_input[2] = uint(vsum);
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = eta3;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }

        uint256 ret2x = add_input[0];
        uint256 ret2y = add_input[1];

        (add_input[0], add_input[1]) = computeCWS(challenges, cws, bigR);

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

    function checkCond2(uint256 alpha, Part1 memory part1, uint256[] memory betas)
        internal
    {
        uint256 uprime_sum = computeUprime(part1.vs, alpha, betas);


        uint256[3] memory mul_input;
        uint256[4] memory add_input;
        bool success;

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = uprime_sum;
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

        (add_input[0], add_input[1]) = computeCMS(alpha, part1, betas);

        
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
        // mul_input[2] = alpha;
        // mul_input[2] = modInv(alpha, NN);
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<size;i++){
            Point memory p = part1.cms[i];
            mul_input[0] = p.x;
            mul_input[1] = p.y;
            mul_input[2] = alphac_1[i];
            // mul_input[2] = alpha;
            // mul_input[2] = modInv(addmod(alpha, i, NN), NN);

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }
        }


        uint sizev = part1.cvs.length;

        uint256[3] memory mul_input1;
        uint256[4] memory add_input1;

        Point memory v = part1.cvs[0];
        mul_input1[0] = v.x;
        mul_input1[1] = v.y;
        // mul_input1[2] = modInv(betas[0], NN);
        mul_input1[2] = einverse[0];
        // mul_input1[2] = betas[0];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x0), 0x60)
        }

        for(uint i=1;i<sizev;i++){
            Point memory v = part1.cvs[i];
            mul_input1[0] = v.x;
            mul_input1[1] = v.y;
            // mul_input1[2] = modInv(betas[i], NN);
            mul_input1[2] = einverse[i];
            // mul_input1[2] = betas[i];

            assembly {
                success := call(not(0), 7, 0, mul_input1, 0x80, add(add_input1, 0x40), 0x60)
            }

            assembly {
                success := call(not(0), 6, 0, add_input1, 0x80, add_input1, 0x60)
            }
        }

        add_input[2] = add_input1[0];
        add_input[3] = add_input1[1];

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }


        return (add_input[0], add_input[1]);
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

    function computeBaseChallenge(Point[] memory cws, Point[] memory cms, Point[] memory cvs, Point[] memory cts)
        internal
        pure
        returns (uint256 x, uint256 y)
    {
        uint cwslength = cws.length;
        uint cmslength = cms.length;
        uint cvslength = cvs.length;
        uint ctslength = cts.length;

        uint256 size = 2 * (cwslength + cmslength + cvslength + ctslength);
        uint256[] memory cs = new uint256[](size);
        uint offset = 0;
        for(uint i=0;i<cwslength;i++) {
            Point memory p = cws[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        for(uint i=0;i<cmslength;i++) {
            Point memory p = cms[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        for(uint i=0;i<cvslength;i++) {
            Point memory p = cvs[i];
            cs[offset] = p.x;
            offset = offset + 1;
            cs[offset] = p.y;
            offset = offset + 1;
        }
        uint256 alpha = mod(uint256(keccak256(abi.encodePacked(cs))), NN);
        for(uint i=0;i<ctslength;i++) {
            Point memory p = cts[i];
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
        // if (L == 6){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        // }
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
        //     gs[7] = Point(G9X, G9Y);
        // }
        if (L==11){
            gs[4] = Point(G5X, G5Y);
            gs[5] = Point(G6X, G6Y);
            gs[6] = Point(G7X, G7Y);
            gs[7] = Point(G8X, G8Y);
            gs[8] = Point(G9X, G9Y);
            gs[9] = Point(G10X, G10Y);
            gs[10] = Point(G11X, G11Y);
        }
        // if (L==12){
        //     gs[4] = Point(G5X, G5Y);
        //     gs[5] = Point(G6X, G6Y);
        //     gs[6] = Point(G7X, G7Y);
        //     gs[7] = Point(G8X, G8Y);
        //     gs[8] = Point(G9X, G9Y);
        //     gs[9] = Point(G10X, G10Y);
        //     gs[10] = Point(G11X, G11Y);
        //     gs[11] = Point(G12X, G12Y);
        // }

        return gs;
    }

}