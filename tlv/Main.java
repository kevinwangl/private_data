import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        // 测试数据 1：原始数据
        printTLV("911001020304050607080910111213141516720E1111222222222233334444444444");

        // 测试数据 2：含 Issuer Script (9F18 + 86 APDU GET DATA)
        // 72 = Issuer Script Template 2
        //   9F18 04 00010204 = Script ID
        //   86 05 80CA9F3600 = Script Command (APDU: GET DATA for ATC)
        printTLV("720F9F180400010204860580CA9F3600");
    }

    private static void printTLV(String hex) {
        byte[] bin;
        try {
            bin = TLVUtils.hex2Bin(hex);
        } catch (TLVParserException e) {
            System.err.println("Invalid hex: " + e.getMessage());
            return;
        }
        InputStream input = new ByteArrayInputStream(bin);
        TLVParser parser = new TLVParser(input);
        try {
            ArrayList<TLVElement> elements = parser.parseAllTLVElement();
            TLVPrettyPrinter printer = new TLVPrettyPrinter();
            printer.print(bin, elements);
        } catch (TLVParserException e) {
            System.err.println("Parse error: " + e.getMessage());
        }
    }
}
