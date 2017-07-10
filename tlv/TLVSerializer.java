package com.risetek.nfc.tlv;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.OutputStream;

/**
 *
 * @brief <p>
 *        <b>TLVSerializerTLV编码数据保存</b>
 *        </p>
 *
 *        &nbsp;&nbsp;&nbsp;&nbsp;TLVSerializerTLV编码数据保存
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
public class TLVSerializer {

	private ByteArrayOutputStream mOutputStream;

	private static int mOffset = 0;

	/**
	 * 构造函数
	 *
	 * @param output
	 *            输出流
	 */
	public TLVSerializer(ByteArrayOutputStream output) {
		this.mOutputStream = output;
	}

	/**
	 *
	 * @brief 得到输出流
	 *
	 * @return 输出流
	 */
	public ByteArrayOutputStream getOutput() {
		return mOutputStream;
	}

	/**
	 *
	 * @brief 设置输出流
	 *
	 * @param output
	 *            输出流
	 */
	public void setOutput(ByteArrayOutputStream output) {
		this.mOutputStream = output;
	}

	/**
	 *
	 * @brief 打包tag
	 *
	 * @param tag
	 *            标签
	 * @param out
	 *            输出流
	 * @throws TLVParserException
	 *             TLV解析失败
	 */
	public void writeTag(String tag, OutputStream out)
			throws TLVParserException {
		try {
			byte[] value = TLVUtils.hex2Bin(tag);
			TLVSerializer.mOffset += value.length;
			out.write(value);
		} catch (IOException e) {
			e.printStackTrace();
			throw new TLVParserException("writeTag lenHeadBuffer is overflow!");
		}
	}

	/**
	 *
	 * @brief 写入一个原始数据对象的长度
	 *
	 * @param len
	 *            长度
	 * @param out
	 *            输出流
	 * @throws TLVParserException
	 *             TLV解析失败
	 */
	public void writePrimitiveLength(long len, OutputStream out)
			throws TLVParserException {
		ByteArrayOutputStream buffer = writeLengthImpl(len);
		if (buffer != null) {
			byte[] lengthBuffer = buffer.toByteArray();
			TLVSerializer.mOffset += lengthBuffer.length;
			try {
				out.write(lengthBuffer);
			} catch (IOException e) {
				e.printStackTrace();
				throw new TLVParserException(
						"writePrimitiveLength buffer is error");
			}
		}
	}

	/**
	 *
	 * @brief 写入一个结构化数据对象的长度
	 *
	 * @param constructLen
	 *            长度
	 * @param outPut
	 *            要插入的流
	 * @param start
	 *            要插入的长度的起始位置
	 * @throws TLVParserException
	 *             TLV解析失败
	 */
	public void writeConstructLength(long constructLen,
			ByteArrayOutputStream outPut, int start) throws TLVParserException {
		ByteArrayOutputStream buffer = writeLengthImpl(constructLen);
		if (buffer != null) {
			byte[] lengthBuffer = buffer.toByteArray();
			TLVSerializer.mOffset += lengthBuffer.length;
			TLVUtils.insertBytes(lengthBuffer, outPut, start);
		}
	}

	/**
	 * @brief 写入数据长度
	 *
	 * @param constructLen
	 *            结构化长度
	 * @return 输出byte数组流
	 * @throws TLVParserException
	 *             TLV解析失败
	 */
	private ByteArrayOutputStream writeLengthImpl(long constructLen)
			throws TLVParserException {
		if (constructLen == 0) {
			return null;
		}

		ByteArrayOutputStream buffer = new ByteArrayOutputStream();
		byte[] lenArray = TLVUtils.convertNumberToBytes(constructLen);
		byte[] lenHeadBuffer = TLVUtils.convertNumberToBytes(lenArray.length);
		if (constructLen <= 127) {
			lenArray[0] = (byte) (lenArray[0] & TLVUtils.LASTSEVEN_MASK);
			try {
				buffer.write(lenArray);
			} catch (IOException e) {
				e.printStackTrace();
				throw new TLVParserException(
						"writeLengthImpl less 127 write is error");
			}
		} else {
			byte lenHead = (byte) (lenHeadBuffer[0] & TLVUtils.FIRST_MASK);
			try {
				buffer.write(lenHead);
				buffer.write(lenArray);
			} catch (IOException e) {
				e.printStackTrace();
				throw new TLVParserException("writeLengthImpl buffer is error");
			}
		}
		return buffer;
	}

	/**
	 *
	 * @brief 打包数据
	 *
	 * @param value
	 *            值
	 * @param out
	 *            输出流
	 * @throws IOException
	 *             一般不会出现，除非内存不足
	 */
	public void writeValue(byte[] value, OutputStream out) throws IOException {
		TLVSerializer.mOffset += value.length;
		out.write(value);
	}

	/**
	 *
	 * @brief 打包一个TLVElement
	 *
	 * @param element
	 *            节点
	 * @throws TLVParserException
	 *             TLV解析失败
	 * @throws IOException
	 *             除非内存不足，否则不会出现
	 */
	public void writeTLVElement(TLVElement element) throws TLVParserException {
		writeTag(element.getTag(), mOutputStream);
		if (element.isPrimitive()) {
			byte[] value = element.getValue();
			if (value != null) {
				element.setLength(value.length);
				writePrimitiveLength(value.length, mOutputStream);
				try {
					writeValue(value, mOutputStream);
				} catch (IOException e) {
					e.printStackTrace();
				}
			}
		} else {
			long constructLen = 0;

			// 当前结构化对象写入字节起始位置
			int startPos = size();
			int endPos = 0;
			TLVSerializer childSerializer = new TLVSerializer(mOutputStream);
			for (int i = 0; i < element.getChildren().size(); i++) {
				TLVElement child = element.getChild(i);
				child.write(childSerializer);
			}
			endPos = size();
			constructLen = endPos - startPos;
			writeConstructLength(constructLen, mOutputStream, startPos);
		}
	}

	/**
	 *
	 * @brief 当前写入字节数
	 *
	 * @return 当前写入字节数
	 * @throws IOException
	 *             除非内存不足，否则不会出现
	 */
	public int size() {
		return TLVSerializer.mOffset;
	}

	/**
	 *
	 * @brief 将数据转换为byte数组
	 *
	 * @return byte数组
	 */
	public byte[] toByteArray() {
		byte[] value = mOutputStream.toByteArray();
		try {
			flush();
		} catch (TLVParserException e) {
			e.printStackTrace();
		}
		return value;
	}

	/**
	 *
	 * @brief 清除当前写入字节位置，清除缓冲
	 *
	 * @throws TLVParserException
	 *             TLV解析失败
	 */
	void flush() throws TLVParserException {
		TLVSerializer.mOffset = 0;
		try {
			mOutputStream.flush();
		} catch (IOException e) {
			e.printStackTrace();
			throw new TLVParserException("flush is error");
		}
	}
}
