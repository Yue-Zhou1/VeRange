package zkp;

import commitment.Commiter;
import config.Config;

public abstract class PedersenZKP extends ZKP {

	/**
	 * 
	 */
	private static final long serialVersionUID = 1L;

	protected final Commiter commiter = Config.getInstance().getCommiter();
}
