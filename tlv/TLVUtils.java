
import java.io.*;
import java.util.ArrayList;

public class TLVUtils {
	public static final int SUFFIX = 1; // 后面补零

	public static final int PREFIX = 2; // 前面补零

	protected static final byte LASTSEVEN_MASK = 0x7F;

	protected static final byte LASTFIVE_MASK = (byte) 0x1F;

	protected static final byte FIRST_MASK = (byte) 0x80;

	protected static final byte THIRD_MASK = 0x20;

	private TLVUtils() {
	}

	/**
	 *
	 * @brief 读取一个byte字节
	 *
	 * @param in
	 *            输入流
	 * @return 一个字节
	 */
	public static byte readByte(InputStream in) throws IOException {
		DataInputStream dataInput = new DataInputStream(in);
		return dataInput.readByte();

	}

	/**
	 *
	 * @brief 写一个byte数据到流中
	 *
	 * @param out
	 *            输出流
	 * @throws IOException
	 *             除非内存不足，否则不会出现
	 */
	public static void writeByte(OutputStream out, byte value)
			throws IOException {
		DataOutputStream dataOutput = new DataOutputStream(out);
		dataOutput.writeByte(value);
	}

	/**
	 *
	 * @brief 判断一个字节的高8位是否为1
	 *
	 * @param value
	 *            待判定值
	 * @return ture 高8位是全为1，false:高8位不全为1
	 */
	public static boolean highBitEqualsOne(byte value) {
		// 判断这个字节的高8位的第一位是否为1
		return TLVUtils.bitMaskResult(value, TLVUtils.FIRST_MASK);
	}

	/**
	 *
	 * @brief 判断这个标记是否为结构化对象
	 *
	 * @param value
	 *            待判定值
	 * @return 是结构化的返回true否则返回false
	 * @throws TLVParserException
	 *             TLV解析失败
	 */
	public static boolean tagIsConstruct(String value)
			throws TLVParserException {
		byte[] tagBuff = hex2Bin(value);
		// 判断这个TLV对象是否是结构化对象 1：代表是结构化对象
		return TLVUtils.bitMaskResult(tagBuff[0], TLVUtils.THIRD_MASK);
	}

	/**
	 *
	 * @brief 判断一个字节的后5位是否为全为1
	 *
	 * @param value
	 *            待判定值
	 * @return true 后5位是全为1, false: 其他
	 */
	public static boolean thirdBitEqualsOne(byte value) {
		// 判断这个TLV对象是否是结构化对象 1：代表是结构化对象
		return TLVUtils.bitMaskResult(value, TLVUtils.THIRD_MASK);
	}

	/**
	 *
	 * @brief 判断一个标记的后5位是否全为1
	 *
	 * @param value
	 *            待判定值
	 * @return true:后5位是全为1, false: 其他
	 */
	public static boolean lastFiveEqualsOne(byte value) {
		return TLVUtils.bitMaskResult(value, TLVUtils.LASTFIVE_MASK);
	}

	/**
	 *
	 * @brief 判断当前字节‘与’一个字节值是否相等
	 *
	 * @param value
	 *            待判定值
	 * @param mask
	 *            掩码
	 * @return true:相等 false:不相等
	 */
	private static boolean bitMaskResult(byte value, byte mask) {
		if ((value & mask) == mask) {
			return true;
		} else {
			return false;
		}
	}

	/**
	 *
	 * @brief 将从byte数组中读取的数值转换为一个长整型number
	 *
	 * @param srcLen
	 *            要转换数据源
	 * @return 返回长度值
	 * @throws TLVParserException
	 *             解析失败
	 */
	public static long convertBytesToNumber(byte[] srcLen)
			throws TLVParserException {
		long ret = 0;
		int index = srcLen.length - 1;
		if (srcLen.length > 8) {
			throw new TLVParserException("TLV Length is overflow");
		} else {
			// 将读取出来的byte数组组成一个长整型number
			for (int i = 0; i < srcLen.length; i++) {
				long element = srcLen[index - i] & 0xFF;
				ret += (element << (8 * i));
			}
			if (ret < 0) {
				throw new TLVParserException("TLV Length is overflow");
			}
		}
		return ret;
	}

	/**
	 *
	 * @brief 将长整型number从数值转换为一个byte数组
	 *
	 * @param srcLen
	 *            要转换数据源
	 * @return 返回长度值
	 * @throws TLVParserException
	 *             解析失败
	 * @throws IOException
	 *             除非内存不足，否则不会出现
	 */
	public static byte[] convertNumberToBytes(long srcLen)
			throws TLVParserException {
		if (srcLen == 0) {
			return null;
		}
		ByteArrayOutputStream out = new ByteArrayOutputStream();
		DataOutputStream dataOutput = new DataOutputStream(out);
		boolean flag = false;
		for (int i = 7; i >= 0; i--) {
			byte element = (byte) (srcLen >>> (8 * i));
			if (element != 0 && !flag) {
				flag = true;
				try {
					dataOutput.writeByte(element);
				} catch (IOException e) {
					e.printStackTrace();
					throw new TLVParserException(
							"converNumberToBytes convert bytes error");
				}
			}
		}
		return out.toByteArray();
	}

	/**
	 *
	 * @brief 将一个byte数组转换为一个字符串
	 *
	 * @param value
	 *            待转换值
	 * @return 转换后字符串
	 */
	public static String convertBytesToString(byte[] value) {
		StringBuffer buffer = new StringBuffer();
		for (int i = 0; i < value.length; i++) {
			buffer.append(bin2Hex(value[i]));
		}
		return buffer.toString();
	}

	/**
	 *
	 * @brief 将一个16进制的byte转换成一个字符
	 *
	 * @param data
	 *            待转换值
	 * @return 转换后数据
	 */
	public static String bin2Hex(byte data) {
		char high;
		char low;
		high = (char) ((data >> 4) & 0xf);
		low = (char) ((data) & 0xf);
		high = (char) ((high >= 10) ? ('A' + (high - 10)) : ('0' + (high)));
		low = (char) ((low >= 10) ? ('A' + (low - 10)) : ('0' + (low)));
		StringBuffer buf = new StringBuffer();
		buf.append(high);
		buf.append(low);
		return buf.toString();
	}

	/**
	 *
	 * @brief 将一个字符数组转换成一个byte数组
	 *
	 * @param value
	 *            待转换值
	 * @return 转换后数据
	 * @throws IOException
	 *             除非内存不足，否则不会出现
	 */
	public static byte[] hex2Bin(String value) throws TLVParserException {
		if (value == null || "".equals(value)) {
			throw new IllegalArgumentException("value is null");
		}
		String buffer = value.toUpperCase();
		int desLength = 0;
		if (buffer.length() % 2 == 0) {
			desLength = buffer.length();
		} else {
			desLength = buffer.length() + 1;
		}
		byte[] src = buffer.getBytes();
		byte[] desBytes = format(src, desLength, TLVUtils.PREFIX, (byte) 0);
		byte[] des = new byte[desLength / 2];
		for (int i = 0; i < des.length; i++) {
			byte high = desBytes[2 * i];
			byte low = desBytes[2 * i + 1];
			if (high >= 'A' && high <= 'F') {
				high = (byte) ((high - 55) & 0xf);
			} else if (high >= '0' && high <= '9') {
				high = (byte) ((high - 48) & 0xf);
			}

			if (low >= 'A' && low <= 'F') {
				low = (byte) ((low - 55) & 0xf);
			} else if (low >= '0' && low <= '9') {
				low = (byte) ((low - 48) & 0xf);
			}
			// 下面语句把它们两个拼成data[]数组的一个元素
			des[i] = (byte) ((high << 4) | low);
		}
		return des;
	}

	/**
	 * 格式化字符串
	 *
	 * @param inBytes
	 *            输入byte数组
	 * @param length
	 *            格式化长度
	 * @param position
	 *            对齐位置
	 * @param stuffing
	 *            填充byte
	 * @return 经过格式化后的byte数组
	 */
	public static byte[] format(byte[] inBytes, int length, int position,
			byte stuffing) {
		if (inBytes == null) {
			return null;
		}
		if (inBytes.length >= length) {
			return inBytes;
		}
		int blankSize = length - inBytes.length;
		byte[] out = new byte[blankSize + inBytes.length];
		for (int i = 0; i < blankSize; i++) {
			out[i] = (stuffing);
		}

		switch (position) {
		case SUFFIX: {
			System.arraycopy(inBytes, 0, out, 0, inBytes.length);
			break;
		}
		case PREFIX:
		default: {
			System.arraycopy(inBytes, 0, out, blankSize, inBytes.length);
		}
		}
		return out;
	}

	/**
	 * 格式化字符串
	 *
	 * @param inStr
	 *            输入字符串
	 * @param length
	 *            格式化长度
	 * @param position
	 *            对齐位置
	 * @param stuffing
	 *            填充字符
	 * @return 经过格式化后的字符串
	 */
	public static String format(String inStr, int length, int position,
			char stuffing) {
		if (inStr == null) {
			inStr = "";
		}
		if (inStr.length() >= length) {
			return inStr;
		}
		int blankSize = length - inStr.length();
		StringBuffer out = new StringBuffer(blankSize);
		for (int i = 0; i < blankSize; i++) {
			out.append(stuffing);
		}

		switch (position) {
		case SUFFIX: {
			out.insert(0, inStr);
			break;
		}
		case PREFIX:
			out.append(inStr);
			break;
		default: {
			out.insert(0, inStr);
			break;
		}
		}
		return out.toString();
	}

	/**
	 *
	 * @brief 插入一段数据到流当中
	 *
	 * @param lengthBuffer
	 *            插入的数据
	 * @param outPut
	 *            要插入输入的流
	 * @param start
	 *            插入数据起始位置
	 * @throws IOException
	 *             除非内存不足，否则不会出现
	 *
	 */
	public static void insertBytes(byte[] lengthBuffer,
			ByteArrayOutputStream outPut, int start) throws TLVParserException {
		byte[] dataBuff = outPut.toByteArray();
		if (start < 0 || dataBuff.length < start) {
			throw new TLVParserException(
					"TLVUtils insertBytes start less zero !");
		}
		byte[] destBuff = new byte[lengthBuffer.length + dataBuff.length];
		byte[] insertBuff = new byte[start];
		byte[] lastBuff = new byte[dataBuff.length - start];
		System.arraycopy(dataBuff, 0, insertBuff, 0, insertBuff.length);
		System.arraycopy(dataBuff, start, lastBuff, 0, lastBuff.length);
		// 将分开的数据拷贝到目标数组
		System.arraycopy(insertBuff, 0, destBuff, 0, insertBuff.length);
		System.arraycopy(lengthBuffer, 0, destBuff, start, lengthBuffer.length);
		System.arraycopy(lastBuff, 0, destBuff, start + lengthBuffer.length,
				lastBuff.length);

		outPut.reset();
		try {
			outPut.flush();
			outPut.write(destBuff);
		} catch (IOException e) {
			e.printStackTrace();
			throw new TLVParserException("TLVUtils insertBytes is error");
		}
	}

	/**
	 *
	 * @brief 遍历查找树得到tag相符的所有节点
	 *
	 * @param findElement
	 *            要查找的元素
	 * @param findTag
	 *            要查找的标记
	 * @param findResult
	 *            查找后的结果
	 */
	public static void findTLVElementByTag(TLVElement findElement,
			String findTag, ArrayList<TLVElement> findResult) {
		if (findTag != null && findTag.equals(findElement.getTag())) {
			findResult.add(findElement);
		}

		if (findElement.getChildren() != null
				&& !findElement.getChildren().isEmpty()) {
			int childNumber = findElement.getChildren().size();
			for (int i = 0; i < childNumber; i++) {
				TLVElement child = (TLVElement) findElement.getChildren()
						.get(i);
				findTLVElementByTag(child, findTag, findResult);
			}
		}
	}

	/**
	 * @brief 遍历查找树查找是否有tag相符的节点
	 *
	 * @param findElement
	 *            要查找的元素
	 * @param findTag
	 *            要查找的标记
	 */
	public static boolean isContainsTag(TLVElement findElement,
			String findTag) {
		if (findTag != null && findTag.equals(findElement.getTag())) {
			return true;
		}
		if (findElement.getChildren() != null
				&& !findElement.getChildren().isEmpty()) {
			int childNumber = findElement.getChildren().size();
			for (int i = 0; i < childNumber; i++) {
				TLVElement child = (TLVElement) findElement.getChildren()
						.get(i);
				if(isContainsTag(child, findTag)){
					return true;
				}
			}
		}
		return false;
	}

	/**
	 * @brief 遍历查找树查找是否有tags相符的任意节点
	 *
	 * @param findElement
	 *            要查找的元素
	 * @param tags
	 *            要查找的标记链表
	 */
	public static boolean isContainsTag(TLVElement findElement,
			ArrayList<String> tags) {
		for(int i = 0; i < tags.size(); i ++){
			String tag = tags.get(i);
			if (tag != null && tag.equals(findElement.getTag())) {
				return true;
			}
		}
		if (findElement.getChildren() != null
				&& !findElement.getChildren().isEmpty()) {
			int childNumber = findElement.getChildren().size();
			for (int i = 0; i < childNumber; i++) {
				TLVElement child = (TLVElement) findElement.getChildren()
						.get(i);
				if(isContainsTag(child, tags)){
					return true;
				}
			}
		}
		return false;
	}

}
