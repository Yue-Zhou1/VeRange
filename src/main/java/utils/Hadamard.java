package utils;

import com.google.common.collect.Lists;

import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;

public class Hadamard {

    public static List<List<BigInteger>> hadamard(int K){
        int n = K;

        boolean[][] hadamard = new boolean[n][n];
        List<List<BigInteger>> _hadamard = Lists.newLinkedList();
        for(int m = 0; m < K; m++){
            _hadamard.add(Lists.newLinkedList());
        }

        // initialize Hadamard matrix of order n
        hadamard[0][0] = true;
        for (int k = 1; k < n; k += k) {
            for (int i = 0; i < k; i++) {
                for (int j = 0; j < k; j++) {
                    hadamard[i+k][j]   =  hadamard[i][j];
                    hadamard[i][j+k]   =  hadamard[i][j];
                    hadamard[i+k][j+k] = !hadamard[i][j];
                }
            }
        }

        // print matrix
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (hadamard[i][j]) _hadamard.get(i).add(BigInteger.ONE);
                else                _hadamard.get(i).add(BigInteger.valueOf(-1));
            }
        }
        return _hadamard;
    }

    public static void main(String[] args) {
        List<List<BigInteger>> a = hadamard(2);
        System.out.println(a);
        List<BigInteger> h1 = getHadamardColumn(a, 1);
        System.out.println(h1);
    }

    public static List<BigInteger> getHadamardColumn(List<List<BigInteger>> h, int i){
        List<BigInteger> column = Lists.newLinkedList();
        for (int j = 0; j < h.size(); j++){
            column.add(h.get(j).get(i));
        }
        return column;
    }
}
