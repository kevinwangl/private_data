import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class APDUParser {
    private static final Map<Byte, String> INS_NAMES = new HashMap<Byte, String>();

    static {
        INS_NAMES.put((byte) 0xA4, "SELECT");
        INS_NAMES.put((byte) 0xB2, "READ RECORD");
        INS_NAMES.put((byte) 0xCA, "GET DATA");
        INS_NAMES.put((byte) 0xAE, "GENERATE AC");
        INS_NAMES.put((byte) 0x82, "EXTERNAL AUTHENTICATE");
        INS_NAMES.put((byte) 0x88, "GET PROCESSING OPTIONS");
        INS_NAMES.put((byte) 0x20, "VERIFY");
        INS_NAMES.put((byte) 0x24, "CHANGE PIN");
        INS_NAMES.put((byte) 0x04, "DEBIT");
        INS_NAMES.put((byte) 0x18, "UNBLOCK PIN");
        INS_NAMES.put((byte) 0xD6, "UPDATE BINARY");
        INS_NAMES.put((byte) 0xDC, "UPDATE RECORD");
        INS_NAMES.put((byte) 0xE2, "APPEND RECORD");
        INS_NAMES.put((byte) 0xDA, "PUT DATA");
    }

    /**
     * 解析 APDU 命令字节，返回字段列表 [field, hex, meaning]
     */
    public static List<String[]> parse(byte[] apdu) {
        List<String[]> fields = new ArrayList<String[]>();
        if (apdu == null || apdu.length < 4) {
            fields.add(new String[]{"RAW", bytesToHex(apdu), "Too short for APDU"});
            return fields;
        }

        String claHex = byteToHex(apdu[0]);
        String claDesc = (apdu[0] & 0x80) != 0 ? "Proprietary" : "ISO/IEC 7816";
        fields.add(new String[]{"CLA", claHex, claDesc});

        String insHex = byteToHex(apdu[1]);
        String insName = INS_NAMES.get(apdu[1]);
        fields.add(new String[]{"INS", insHex, insName != null ? insName : "Unknown"});

        String p1Hex = byteToHex(apdu[2]);
        String p2Hex = byteToHex(apdu[3]);
        String p1p2Combined = p1Hex + p2Hex;

        // 对 GET DATA 命令，P1P2 通常是 EMV tag
        if (apdu[1] == (byte) 0xCA || apdu[1] == (byte) 0xDA) {
            String tagName = EMVTagDict.getName(p1p2Combined);
            fields.add(new String[]{"P1P2", p1p2Combined, "\u2192 " + tagName});
        } else {
            fields.add(new String[]{"P1", p1Hex, ""});
            fields.add(new String[]{"P2", p2Hex, ""});
        }

        int pos = 4;
        if (pos < apdu.length) {
            int lcOrLe = apdu[pos] & 0xFF;
            if (pos + 1 + lcOrLe <= apdu.length && lcOrLe > 0) {
                // Case 3/4: has Lc + Data
                fields.add(new String[]{"Lc", byteToHex(apdu[pos]), String.valueOf(lcOrLe)});
                pos++;
                byte[] data = new byte[lcOrLe];
                System.arraycopy(apdu, pos, data, 0, lcOrLe);
                fields.add(new String[]{"Data", bytesToHex(data), lcOrLe + " bytes"});
                pos += lcOrLe;
                if (pos < apdu.length) {
                    fields.add(new String[]{"Le", byteToHex(apdu[pos]), ""});
                }
            } else {
                // Case 2: Le only
                fields.add(new String[]{"Le", byteToHex(apdu[pos]), ""});
            }
        }

        return fields;
    }

    private static String byteToHex(byte b) {
        return TLVUtils.bin2Hex(b);
    }

    private static String bytesToHex(byte[] data) {
        if (data == null) return "";
        return TLVUtils.convertBytesToString(data);
    }
}
