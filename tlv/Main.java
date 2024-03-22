import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.Vector;

public class Main {
    public static void main(String[] args) {
        String emv = "911001020304050607080910111213141516720E1111222222222233334444444444";
        byte[] bin;
        try {
            bin = TLVUtils.hex2Bin(emv);
        } catch (TLVParserException e) {
            throw new RuntimeException(e);
        }
        InputStream input = new ByteArrayInputStream(bin);
        TLVParser parser = new TLVParser(input);
        Vector<TLVElement> elements;
        try {
            elements = parser.parseAllTLVElement();
            System.out.println(elements.size());
            for (TLVElement element : elements) {
                String tag = element.getTag();
                byte[] byte_value = element.getValue();
                boolean isPrimitive = element.isPrimitive();
                if(isPrimitive){
                    String value = TLVUtils.convertBytesToString(byte_value);
                    System.out.println("Tag = "+tag);
                    System.out.println("Value = "+value);
                }else{
                   ArrayList<TLVElement> child = element.getChildren();
                   TLVElement child_element = child.get(0);
                   String value = TLVUtils.convertBytesToString(child_element.getValue());
                   System.out.println("Tag = "+tag);
                   System.out.println("Child Tag = "+child_element.getTag());
                   System.out.println("Value = "+value);
                }
            }


        } catch (TLVParserException e) {
            throw new RuntimeException(e);
        }
    }
}
