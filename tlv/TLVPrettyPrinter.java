import java.util.ArrayList;
import java.util.List;

public class TLVPrettyPrinter {

    private static final String LINE = "\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500";
    private List<String> diagnostics = new ArrayList<String>();

    public void print(byte[] rawData, ArrayList<TLVElement> elements) {
        System.out.println();
        System.out.println("EMV TLV Analysis \u2014 " + elements.size() + " elements, " + rawData.length + " bytes");
        System.out.println(LINE);
        System.out.println();

        // Raw hex dump
        System.out.println("Raw: " + formatHexSpaced(rawData, 0, rawData.length));
        System.out.println();

        // Parsed tree
        for (int i = 0; i < elements.size(); i++) {
            printElement(elements.get(i), "", true);
            if (i < elements.size() - 1) System.out.println();
        }
        System.out.println();

        // Diagnostics
        if (diagnostics.isEmpty()) {
            System.out.println("\u26a0 Diagnostics: (none)");
        } else {
            System.out.println("\u26a0 Diagnostics:");
            for (String d : diagnostics) {
                System.out.println("  - " + d);
            }
        }
        System.out.println(LINE);
    }

    private void printElement(TLVElement elem, String prefix, boolean isLast) {
        String tag = elem.getTag();
        String name = EMVTagDict.getName(tag);
        long length = elem.getLength();
        boolean constructed = elem.isConstruct();

        String lenHex = longToHex(length);

        // 树形连接符
        String connector = prefix.isEmpty() ? "" : (isLast ? "\u2514\u2500 " : "\u251c\u2500 ");
        String childPrefix = prefix.isEmpty() ? "" : prefix + (isLast ? "   " : "\u2502  ");

        if (constructed) {
            // [Tag] [Len] ── Name ── N bytes, Constructed
            System.out.println(prefix + connector
                    + "[" + tag + "] [" + lenHex + "] \u2500\u2500 " + name
                    + " \u2500\u2500 " + length + " bytes, Constructed");

            // T    L  标签行
            String tagPad = spaces(tag.length());
            String lenPad = spaces(lenHex.length());
            System.out.println(childPrefix + " T" + tagPad + "  L" + lenPad);

            // 子元素
            ArrayList<TLVElement> children = elem.getChildren();
            if (children.isEmpty() && length > 0) {
                diagnostics.add("Tag " + tag + ": Constructed but no children parsed");
            }
            for (int i = 0; i < children.size(); i++) {
                boolean last = (i == children.size() - 1);
                System.out.println(childPrefix + " \u2502");
                printElement(children.get(i), childPrefix + " ", last);
            }
        } else {
            byte[] value = elem.getValue();
            String valueHex = (value != null) ? TLVUtils.convertBytesToString(value) : "";

            // 长度校验
            if (value != null && value.length != length) {
                diagnostics.add("Tag " + tag + ": Declared length=" + length
                        + " but actual=" + value.length + " bytes");
            }

            if ("86".equals(tag) && value != null) {
                // APDU 特殊展示
                // [Tag] [Len] [ValueHex] ── Name
                System.out.println(prefix + connector
                        + "[" + tag + "] [" + lenHex + "] [" + valueHex + "] \u2500\u2500 " + name);

                // T    L       V  标签行
                String tagPad = spaces(tag.length());
                String lenPad = spaces(lenHex.length());
                String valPad = spaces(valueHex.length());
                System.out.println(childPrefix + " T" + tagPad + "  L" + lenPad + "  V" + valPad);

                // APDU 拆解
                String apduPrefix = childPrefix + spaces(tag.length() + lenHex.length() + 8);
                System.out.println(apduPrefix + "\u2514\u2500 APDU");
                printAPDU(value, apduPrefix + "   ");
            } else {
                // [Tag] [Len] [ValueHex] ── Name
                System.out.println(prefix + connector
                        + "[" + tag + "] [" + lenHex + "] [" + valueHex + "] \u2500\u2500 " + name);

                // T    L       V  标签行
                String tagPad = spaces(tag.length());
                String lenPad = spaces(lenHex.length());
                String valPad = spaces(valueHex.length());
                System.out.println(childPrefix + " T" + tagPad + "  L" + lenPad + "  V" + valPad);
            }
        }
    }

    private void printAPDU(byte[] apdu, String prefix) {
        List<String[]> fields = APDUParser.parse(apdu);
        for (int i = 0; i < fields.size(); i++) {
            String[] f = fields.get(i);
            boolean last = (i == fields.size() - 1);
            String conn = last ? "\u2514\u2500 " : "\u251c\u2500 ";
            StringBuilder sb = new StringBuilder();
            sb.append(prefix).append(conn);
            sb.append(padRight(f[0], 4)).append(" | ").append(padRight(f[1], 6));
            if (f[2] != null && !f[2].isEmpty()) {
                sb.append(" | ").append(f[2]);
            }
            System.out.println(sb.toString());
        }
    }

    private String formatHexSpaced(byte[] data, int from, int to) {
        StringBuilder sb = new StringBuilder();
        for (int i = from; i < to && i < data.length; i++) {
            if (i > from) sb.append(' ');
            sb.append(TLVUtils.bin2Hex(data[i]));
        }
        return sb.toString();
    }

    private String longToHex(long val) {
        if (val <= 0xFF) return String.format("%02X", val);
        if (val <= 0xFFFF) return String.format("%04X", val);
        return String.format("%06X", val);
    }

    private static String padRight(String s, int width) {
        if (s.length() >= width) return s;
        StringBuilder sb = new StringBuilder(s);
        for (int i = s.length(); i < width; i++) sb.append(' ');
        return sb.toString();
    }

    private static String spaces(int n) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) sb.append(' ');
        return sb.toString();
    }
}
