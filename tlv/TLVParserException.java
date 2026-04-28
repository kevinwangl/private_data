/**
 *
 * @brief <p>
 *        <b>TLVParserException解析器异常信息</b>
 *        </p>
 *
 *        &nbsp;&nbsp;&nbsp;&nbsp;TLVParserException解析器异常信息
 *
 * @author wangl
 * @version eJPos SDK1.0
 * @see
 * @since 2008-6-23
 */

public class TLVParserException extends Exception {
	private static final long serialVersionUID = 1L;

	private String mExtMessage;

	/**
	 * @brief TLV解析异常类构造函数
	 *
	 * @param s
	 *            错误信息
	 */
	public TLVParserException(String s) {
		super(s);
	}

	/**
	 * @brief TLV解析异常类构造函数（带原因）
	 *
	 * @param s
	 *            错误信息
	 * @param cause
	 *            原始异常
	 */
	public TLVParserException(String s, Throwable cause) {
		super(s, cause);
	}

	/**
	 * TLV解析异常类构造函数
	 *
	 * @param s
	 *            错误信息
	 * @param extMessage
	 *            扩展信息
	 */
	public TLVParserException(String s, String extMessage) {
		super(s);
		this.mExtMessage = extMessage;
	}

	/**
	 * @return 返回 扩展信息。
	 */
	public String getExtMessage() {
		return mExtMessage;
	}

	public void printStackTrace() {
		java.io.PrintStream err = System.err;
		err.println("TLVParser extMessage " + mExtMessage);
		super.printStackTrace();

	}
}
