// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

contract BIC {
    struct Point {
        int256 x;
        int256 y;
    }

    struct Part1 {
        int256[] zi;
        int256[] ti;
        int256 tao;
    }

    struct Part2 {
        Point[] ci;
        uint256 delta;
        Point C;
        int256 gamma;
    }

    int256 public constant GX = 1;
    int256 public constant HX = 9727523064272218541460723335320998459488975639302513747055235660443850046724;
    int256 public constant GY = 2;
    int256 public constant HY = 5031696974169251245229961296941447383441169981934237515842977230762345915487;

    uint256 public constant PP = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47;
    uint256 internal constant NN = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001;
    int256 public constant B = 2**32;
    // int256 public constant B = 2**64;

    function verifyBIC(Part1 memory part1, Part2 memory part2)
        external
        returns (uint256)
    {
        // uint256 memory gamma = computeChallenge(C);

        // int256 gamma = 0;
        Point memory c0 = computeC0(part2);
        Point[4] memory Fi = computeFi(part1, part2, c0);
        Point memory F = computeF(part1, part2);
        uint256 hash_res = hash(Fi, F);
        bool checkZi = compareZi(part1.zi);

        // assert (part2.delta == hash_res && checkZi);


    }


    function computeFi(Part1 memory part1, Part2 memory part2, Point memory c0)
        internal
        returns(Point[4] memory fi)
    {
        Point[4] memory Fi;

        for (uint i=0;i<4;i++){
            int256[3] memory mul_input;
            int256[6] memory add_input;
            bool success;

            mul_input[0] = GX;
            mul_input[1] = GY;
            mul_input[2] = part1.zi[i];
            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
            }

            mul_input[0] = HX;
            mul_input[1] = HY;
            mul_input[2] = part1.ti[i];

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }
            if (i==0){
                mul_input[0] = c0.x;
                mul_input[1] = c0.y;
            }else{
                mul_input[0] = part2.ci[i].x;
                mul_input[1] = part2.ci[i].y;
            }

            mul_input[2] = -part2.gamma;

            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
            }
            
            assembly {
                success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
            }

            Fi[i].x = add_input[0];
            Fi[i].y = add_input[1];

        }
        return Fi;
    }

    function computeC0(Part2 memory part2)
        internal
        returns(Point memory f)
    {
        Point memory c0;

        int256[3] memory mul_input;
        int256[4] memory add_input;
        bool success;

        mul_input[0] = part2.C.x;
        mul_input[1] = part2.C.y;
        mul_input[2] = -1;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = B;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }
        
        c0.x = add_input[0];
        c0.y = add_input[1];
        return c0;
    }

    function computeF(Part1 memory part1, Part2 memory part2)
        internal
        returns(Point memory f)
    {
        Point memory f;

        int256[3] memory mul_input;
        int256[8] memory add_input;
        bool success;

        mul_input[0] = HX;
        mul_input[1] = HY;
        mul_input[2] = part1.tao;
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input, 0x60)
        }

        mul_input[0] = GX;
        mul_input[1] = GY;
        mul_input[2] = part2.gamma;

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        mul_input[0] = part2.C.x;
        mul_input[1] = part2.C.y;
        mul_input[2] = 4*part1.zi[0];

        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add(add_input, 0x40), 0x60)
        }

        int256[4] memory add_input_cz;

        mul_input[0] = part2.ci[1].x;
        mul_input[1] = part2.ci[1].y;
        mul_input[2] = -part1.zi[1];
        assembly {
            success := call(not(0), 7, 0, mul_input, 0x80, add_input_cz, 0x60)
        }
        for(uint i=2;i<4;i++) {
            mul_input[0] = part2.ci[i].x;
            mul_input[1] = part2.ci[i].y;
            mul_input[2] = -part1.zi[i];
            assembly {
                success := call(not(0), 7, 0, mul_input, 0x80, add(add_input_cz, 0x40), 0x60)
            }
        }
        assembly {
            success := call(not(0), 6, 0, add_input_cz, 0x80, add_input_cz, 0x60)
        }
        add_input[6] = add_input_cz[0];
        add_input[7] = add_input_cz[1];

        assembly {
            success := call(not(0), 6, 0, add_input, 0x80, add_input, 0x60)
        }
        
        f.x = add_input[0];
        f.y = add_input[1];
        return f;
    }

    function hash(Point[4] memory fi , Point memory f)
        internal
        returns(uint256)
    {

        int256[] memory hash_f = new int256[](10);
        hash_f[8] = f.x;
        hash_f[9] = f.y;
        uint offset = 0;
        for(uint i=0;i<4;i++){
            hash_f[offset] = fi[i].x;
            offset = offset+1;
            hash_f[offset] = fi[i].y;
            offset = offset+1;
        }

		return uint256(keccak256(abi.encodePacked(hash_f)));
	}

    function compareZi(int256[] memory zi)
        internal
        returns(bool)
    {
        int256 upperBound = 2**192+2**112;
        for(uint i = 0;i<4;i++){
            if (zi[i] > upperBound) return false;
        }
        return true;
    }

}