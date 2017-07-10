package com.risetek.nfc.tlv;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.Vector;

/**
 *
 * @brief <p>
 *        <b>TLVParser解析器</b>
 *        </p>
 *
 *        &nbsp;&nbsp;&nbsp;&nbsp;TLVParser解析器
 *
 *        <p>
 *        <center>COPYRIGHT (C) 2000-2006,CoreTek Systems Inc.All Rights
 *        Reserved.</center>
 *        </p>
 * @author wangl
 * @version eJPos SDK1.0
 * @see
 * @since 2008-6-23
 */
public class TLVParser {
	private InputStream mInputStream;

	/**
	 *
	 * @brief 构造函数
	 *
	 * @param in
	 *            输入数据流
	 */
	public TLVParser(InputStream input) {
		this.mInputStream = input;
	}

	/**
	 * 获取输入数据流
	 *
	 * @return 输入数据流
	 */
	public InputStream getInput() {
		return mInputStream;
	}

	/**
	 * 设置输入数据流
	 *
	 * @param input
	 *            输入数据流
	 */
	public void setInput(InputStream input) {
		this.mInputStream = input;
	}

	/**
	 *
	 * @brief 解析Tag
	 *
	 * @param in
	 *            输入数据流
	 * @param out
	 *            Tag输出数据流
	 * @return 是否解析成功
	 * @throws TLVParserException
	 *             TLV解析失败
	 */
	public boolean parseTag(InputStream in, OutputStream out)
			throws TLVParserException {
		try {
			byte value = TLVUtils.readByte(in);
			if (available() > 0) {
				if (value == (byte) 0x00 || value == (byte) 0xFF) {
					return false;
				} else {
					return parseTagImpl(in, out, value);
				}
			} else {
				return false;
			}
		} catch (IOException e) {
			e.printStackTrace();
			throw new TLVParserException("TLV parseTag Parse Fail");
		}
	}

	/**
	 * @brief 解析TAG
	 *
	 * @param in
	 *            输入数据流
	 * @param out
	 *            TAG输出数据流
	 * @param value
	 *            TAG第一个值
	 * @return 解析是否成功
	 * @throws TLVParserException
	 *             解析失败
	 * @throws IOException
	 *             一般不出现除非内存不足
	 */
	private boolean parseTagImpl(InputStream in, OutputStream out, byte value)
			throws TLVParserException {
		try {
			TLVUtils.writeByte(out, value);
			// 判断当前字节标记的后五位是否都为'1',如果为1,那么就继续解析tag的后续
			if (TLVUtils.lastFiveEqualsOne(value)) {
				parseTagRemaining(in, out);
			} else {
				return true;
			}
		} catch (IOException e) {
			e.printStackTrace();
			throw new TLVParserException("TLV parseTagImpl write byte is Fail");
		} catch (TLVParserException e) {
			e.printStackTrace();
			throw new TLVParserException("TLV parseTagImpl Parse Fail");
		}
		return true;
	}

	/**
	 *
	 * @brief 解析Tag的后续字节
	 *
	 * @param in
	 *            输入数据流
	 * @param out
	 *            Tag输出数据流
	 */
	void parseTagRemaining(InputStream in, OutputStream out)
			throws TLVParserException {
		byte value = -1;
		try {
			value = TLVUtils.readByte(in);
			if (TLVUtils.highBitEqualsOne(value)) {
				parseTagRemaining(in, out);
			} else {
				TLVUtils.writeByte(out, value);
				return;
			}
		} catch (IOException e) {
			e.printStackTrace();
			throw new TLVParserException("TLV TagRemaining Parse Fail");
		}
	}

	/**
	 *
	 * @brief 解析长度
	 *
	 * @param in
	 *            输入数据流
	 * @param out
	 *            输出数据流
	 * @throws TLVParserException
	 *             解析失败
	 */
	public void parseLength(InputStream in, OutputStream out)
			throws TLVParserException {
		if (available() > 0) {
			try {
				byte value = TLVUtils.readByte(in);
				int highBit = value & TLVUtils.FIRST_MASK;
				int lowBit = value & TLVUtils.LASTSEVEN_MASK;
				if (highBit == TLVUtils.FIRST_MASK) {
					byte[] lenBuff = new byte[lowBit];
					in.read(lenBuff);
					out.write(lenBuff);
				} else {
					out.write(lowBit);
				}
			} catch (IOException e) {
				e.printStackTrace();
				throw new TLVParserException("TLV Length Parse Fail");
			}
		}
	}

	/**
	 *
	 * @brief 解析Value值
	 *
	 * @param in
	 *            输入数据流
	 * @return
	 * @throws TLVParserException
	 *             解析失败
	 */
	public void parseValue(long length, InputStream in, OutputStream out)
			throws TLVParserException {
		if (available() > 0) {
			byte[] valueBuffer = new byte[(int) length];
			try {
				in.read(valueBuffer);
				out.write(valueBuffer);
			} catch (IOException e) {
				e.printStackTrace();
				throw new TLVParserException("TLV Value Parse Fail");
			}
		}
	}

	/**
	 * @brief 解析TLVElement
	 *
	 * @param in
	 *            输入数据流
	 * @throws TLVParserException
	 *             解析失败
	 */
	public TLVElement parseTLVElement() throws TLVParserException {
		if (available() > 0) {
			ByteArrayOutputStream tagBuffer = new ByteArrayOutputStream();
			ByteArrayOutputStream lenBuffer = new ByteArrayOutputStream();
			ByteArrayOutputStream valueBuffer = new ByteArrayOutputStream();
			if (parseTag(mInputStream, tagBuffer)) {
				parseLength(mInputStream, lenBuffer);
				byte[] tag = tagBuffer.toByteArray();
				byte[] len = lenBuffer.toByteArray();
				long length = TLVUtils.convertBytesToNumber(len);
				TLVElement element = new TLVElement(
						TLVUtils.convertBytesToString(tag), length);
				// 判断当前的标记是否为结构化对象
				if (TLVUtils.thirdBitEqualsOne(tag[0])) {
					element.setConstruct(true);
					TLVParser childParser = new TLVParser(mInputStream);
					while (childParser.available() > 0) {
						TLVElement child = childParser.parseTLVElement();
						if (child != null) {
							element.addChild(child);
						}
					}
				} else {
					parseValue(length, mInputStream, valueBuffer);
					byte[] bytesValue = valueBuffer.toByteArray();
					element.setConstruct(false);
					element.setValue(bytesValue);
				}
				return element;
			} else {
				return parseTLVElement();
			}
		} else {
			return null;
		}
	}

	public Vector<TLVElement> parseAllTLVElement() throws TLVParserException{
		Vector<TLVElement> tlv = new Vector<TLVElement>();
		while(available() > 0){
			TLVElement e = parseTLVElement();
			if(null != e){
				tlv.add(e);
			}else{
				break;
			}
		}
		return tlv;
	}

	/**
	 *
	 * @brief 判断当前解析是否完毕
	 *
	 * @return 未解析数据字节数
	 * @throws IOException
	 *             一般不会出现除非内存不足
	 */
	public int available() {
		try {
			return mInputStream.available();
		} catch (IOException e) {
			e.printStackTrace();
			return -1;
		}
	}
}
