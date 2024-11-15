package zkp;

import java.io.Serializable;
import java.math.BigInteger;

import config.Config;
import utils.SerializationUtils;

public abstract class ZKP implements Serializable {

	/**
	 * 
	 */
	private static final long serialVersionUID = 1L;

	protected final BigInteger n = Config.getInstance().getKey().getN();

	/**
	 * Verifies that the rehash of the particular variables is indeed the specified
	 * hash.
	 * 
	 * @return Description of this proof
	 */
	public abstract boolean verify();

	public byte[] toByteArray() {
		return SerializationUtils.toByteArray(this);
	}
}
