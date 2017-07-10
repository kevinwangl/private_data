
import java.util.ArrayList;

/**
 *
 * @brief <p>
 *        <b>TLVElement元素</b>
 *        </p>
 *
 *        &nbsp;&nbsp;&nbsp;&nbsp;TLVElement节点元素
 *
 *        </p>
 * @author wangl
 * @version eJPos SDK1.0
 * @see
 * @since 2008-6-23
 */
public class TLVElement {

	protected String mTag;

	protected long mLength;

	protected byte[] mValue;

	protected boolean mIsConstruct = false;

	protected TLVElement mParent;

	protected ArrayList<TLVElement> mChildren = new ArrayList<TLVElement>();

	/**
	 *
	 * @brief 公开的构造函数
	 *
	 * @param parent
	 *            父元素
	 */
	public TLVElement(TLVElement parent) {
		setParentElement(parent);
	}

	/**
	 *
	 * @brief 公开的构造函数
	 *
	 * @param tag
	 *            标签
	 * @param value
	 *            值
	 */
	public TLVElement(String tag, byte[] value) {
		this(tag, value.length, value);
	}

	/**
	 *
	 * @brief 保护的构造函数
	 *
	 * @param tag
	 *            标签
	 * @param length
	 *            长度
	 */
	protected TLVElement(String tag, long length) {
		this(tag, length, null);
	}

	/**
	 *
	 * @brief 公开的构造函数
	 *
	 * @param tag
	 *            标签
	 *
	 */
	public TLVElement(String tag) {
		this(tag, 0, null);
	}

	/**
	 *
	 * @brief 构造函数
	 *
	 */
	protected TLVElement() {

	}

	/**
	 * @brief 赋值当前tag的标记表示是否为结构化对象
	 *
	 * @param tag
	 *            标签
	 */
	protected void evalConstruct(String tag) {
		try {
			byte[] tagValue = TLVUtils.hex2Bin(tag);
			if (TLVUtils.thirdBitEqualsOne(tagValue[0])) {
				setConstruct(true);
			} else {
				setConstruct(false);
			}
		} catch (TLVParserException e) {
			e.printStackTrace();
		}
	}

	/**
	 *
	 * @brief 保护的构造函数
	 *
	 * @param tag
	 *            标签
	 * @param length
	 *            长度
	 * @param value
	 *            值
	 */
	protected TLVElement(String tag, long length, byte[] value) {
		this.mTag = tag;
		this.mLength = length;
		evalConstruct(tag);
		this.mValue = value;
	}

	/**
	 *
	 * @brief 获取长度
	 *
	 * @return 长度
	 */
	public long getLength() {
		return mLength;
	}

	/**
	 *
	 * @brief 设置长度
	 *
	 * @param length
	 *            长度
	 */
	public void setLength(long length) {
		this.mLength = length;
	}

	/**
	 *
	 * @brief 得到父节点
	 *
	 * @return 父节点
	 */
	public TLVElement getParentElement() {
		return mParent;
	}

	/**
	 *
	 * @brief 设置父节点
	 *
	 * @param parent
	 *            父节点
	 */
	public void setParentElement(TLVElement parent) {
		this.mParent = parent;
	}

	/**
	 *
	 * @brief 得到当前节点标记
	 *
	 * @return 节点标记
	 */
	public String getTag() {
		return mTag;
	}

	/**
	 *
	 * @brief 设置当前节点的标记
	 *
	 * @param tag
	 *            节点的标记
	 */
	public void setTag(String tag) {
		this.mTag = tag;
	}

	/**
	 *
	 * @brief 得到当前节点的值
	 *
	 * @return 节点的值
	 */
	public byte[] getValue() {
		return this.mValue;
	}

	/**
	 *
	 * @brief 设置节点的值
	 *
	 * @param value
	 *            节点的值
	 */
	public void setValue(byte[] value) {
		this.mValue = value;
	}

	/**
	 *
	 * @brief 得到所有孩子
	 *
	 * @return 孩子列表
	 */
	public ArrayList<TLVElement> getChildren() {
		return mChildren;
	}

	/**
	 *
	 * @brief 添加一个子
	 *
	 * @param index
	 *            索引
	 * @param child
	 *            子节点
	 */
	public void addChild(int index, TLVElement child) {
		if (child != null) {
			child.setParentElement(this);
			mChildren.add(index, child);
		}
	}

	/**
	 *
	 * @brief 添加一个子
	 *
	 * @param child
	 *            子节点
	 */
	public void addChild(TLVElement child) {
		if (child != null) {
			child.setParentElement(this);
			mChildren.add(child);
		}
	}

	/**
	 *
	 * @brief 得到一个子
	 *
	 * @param index
	 *            索引
	 * @return 子节点
	 */
	public TLVElement getChild(int index) {
		Object element = mChildren.get(index);
		if (element != null) {
			return (TLVElement) element;
		} else {
			return null;
		}

	}

	/**
	 *
	 * @brief 返回当前对象类型
	 *
	 * @return 对象类型
	 */
	public boolean isConstruct() {
		return mIsConstruct;
	}

	/**
	 *
	 * @brief 设置对象是否为结构化对象
	 *
	 * @param isConstruct
	 *            是否为结构化对象
	 */
	public void setConstruct(boolean isConstruct) {
		this.mIsConstruct = isConstruct;
	}

	/**
	 * 设置子节点
	 *
	 * @param children
	 *            子节点
	 */
	public void setChildren(ArrayList<TLVElement> children) {
		this.mChildren = children;
	}

	/**
	 *
	 * @brief 删除一个子节点
	 *
	 * @param element
	 *            要删除的节点
	 */
	public void removeChild(TLVElement element) {
		if (element != null) {
			mChildren.remove(element);
		}
	}

	/**
	 *
	 * @brief 删除一个子节点
	 *
	 * @param index
	 *            要删除子节点索引
	 */
	public void removeChild(int index) {
		mChildren.remove(index);
	}

	/**
	 *
	 * @brief 写入当前节点数据
	 *
	 * @param serializer
	 *            TLV序列号类
	 */
	public void write(TLVSerializer serializer) throws TLVParserException {
		serializer.writeTLVElement(this);
	}

	/**
	 *
	 * @brief 当前对象是否为原始对象
	 *
	 * @return 是否为原始对象
	 */
	public boolean isPrimitive() {
		return !mIsConstruct;

	}

	/**
	 *
	 * @brief 设置当前对象是否为原始对象
	 *
	 * @param flag
	 *            是否为原始对象
	 */
	public void setPrimitive(boolean flag) {
		mIsConstruct = (!flag);
	}

	/**
	 *
	 * @brief 返回根节点
	 *
	 * @return 根节点
	 */
	public TLVElement getRoot() {
		TLVElement current = this;
		while (current.mParent != null) {
			if (!(current.mParent instanceof TLVElement)) {
				return current.mParent;
			}
			current = (TLVElement) current.mParent;
		}

		return current;
	}
}
