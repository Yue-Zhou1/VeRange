// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

// import "prb-math/contracts/PRBMathSD59x18.sol";


library SafeMath {

    function add(uint256 a, uint8 sign_a, uint256 b, uint8 sign_b)
        public
        pure
        returns (uint256, uint8)
    {
        uint256 c;
        uint8 s;
        if (sign_a == 1 && sign_b == 1) {
            c = a + b;
            s = 1;
        } else if (sign_a == 1 && sign_b == 0) {
            if (a > b) {
                c = a - b;
                s = 1;
            } else {
                c = b - a;
                s = 0;
            }
        } else if (sign_a == 0 && sign_b == 1) {
            if (a > b) {
                c = a - b;
                s = 0;
            } else {
                c = b - a;
                s = 1;
            }
        } else {
            c = a + b;
            s = 0;
        }

        return (c, s);
    }

    function sub(uint256 a, uint8 sign_a, uint256 b, uint8 sign_b)
        public
        pure
        returns (uint256, uint8)
    {
        uint256 c;
        uint8 s;
        if (sign_a == 1 && sign_b == 1) {
            if (a > b) {
                c = a - b;
                s = 1;
            } else {
                c = b - a;
                s = 0;
            }
        } else if (sign_a == 1 && sign_b == 0) {
            c = a + b;
            s = 1;
        } else if (sign_a == 0 && sign_b == 1) {
            c = a + b;
            s = 0;
        } else {
            if (a > b) {
                c = a - b;
                s = 0;
            } else {
                c = b - a;
                s = 1;
            }
        }

        return (c, s);
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

    function mod(uint256 a, uint8 sign, uint256 p)
        public
        pure
        returns (uint256)
    {
        require(p != 0, "SafeMath: modulo by zero");

        uint256 ret = mod(a, p);

        if(sign == 1) {
            return ret;
        } else if (ret == 0) {
            return ret;
        } else {
            return p - mod(a, p);
        }
    }

    function inv(uint256 x) public pure returns (uint) {
        uint256 p = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
        uint256 a = x;
        if (a == 0)
            return 0;
        if (a > p)
            a = a % p;
        int t1;
        int t2 = 1;
        uint r1 = p;
        uint r2 = a;
        uint q;
        while (r2 != 0) {
            q = r1 / r2;
            (t1, t2, r1, r2) = (t2, t1 - int(q) * t2, r2, r1 - q * r2);
        }
        if (t1 < 0)
            return (p - uint(-t1));
        return uint(t1);
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

    function modInv(uint256 a, uint8 sign, uint256 p)
        public
        pure
        returns (uint256)
    {
        uint256 c;
        if(sign == 0) {
            c = mod(a, p);
        }

        return modInv(c, p);
    }

    // a - b = c;
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

    function submod(uint256 a, uint8 sign_a, uint256 b, uint8 sign_b, uint256 q)
        public
        pure
        returns (uint256)
    {
        if(sign_a == 1 && sign_b == 1) {
            return submod(a, b, q);
        } else if (sign_a == 1 && sign_b == 0) {
            return addmod(a, b, q);
        } else if (sign_a == 0 && sign_b == 1) {
            return q - addmod(a, b, q);
        } else {
            if (a > b) {
                return q - submod(a, b, q);
            } else {
                return submod(b, a, q);
            }
        }
    }

    function createMatrix(uint dim1, uint dim2) internal pure returns (int256[][] memory) {
        int256[][] memory result = new int256[][](dim1);
        for (uint i=0; i<dim1; i++) {
            result[i] = new int256[](dim2);
        }
        return result;
    }

    function mul(int256[][] memory a, int256 b) internal pure returns (int256[][] memory) {
        int256[][] memory result = createMatrix(a.length, a[0].length);
        for (uint i=0; i<a.length; i++) {
            for (uint j=0; j<a[0].length; j++) {
                result[i][j] = a[i][j] * b;
            }
        }
        return result;
    }

    function dot(int256[][] memory a, int256[][] memory b) internal pure returns (int256[][] memory) {
        uint l1 = a.length;
        uint l2 = b[0].length;
        uint zipsize = b.length;
        int256[][] memory c = new int256[][](l1);
        for (uint fi=0; fi<l1; fi++) {
            c[fi] = new int256[](l2);
            for (uint fj=0; fj<l2; fj++) {
                int256 entry = 0e18;
                for (uint i=0; i<zipsize; i++) {
                    entry += a[fi][i] * b[i][fj];
                }
                c[fi][fj] = entry;
            }
        }
        return c;
    }

    function add(int256[][] memory a, int256[][] memory b) internal pure returns (int256[][] memory) {
        int256[][] memory result = createMatrix(a.length, a[0].length);
        for (uint i=0; i<a.length; i++) {
            for (uint j=0; j<a[0].length; j++) {
                result[i][j] = a[i][j] + b[i][j];
            }
        }
        return result;
    }

    function dot(int256[] memory a, int256[] memory b) internal pure returns (int256) {
        int256 total = 0e18;
        for (uint i=0; i<a.length; i++) {
            total += a[i] * b[i];
        }
        return total;
    }

    function add(int256[] memory a, int256[] memory b) internal pure returns (int256[] memory) {
        int256[] memory result = new int256[](a.length);
        for (uint i=0; i<a.length; i++) {
            result[i] = a[i] + b[i];
        }
        return result;
    }

    function mul(int256[] memory a, int256 b) internal pure returns (int256[] memory) {
        int256[] memory result = new int256[](a.length);
        for (uint i=0; i<a.length; i++) {
            result[i] = a[i] * b;
        }
        return result;
    }

}
