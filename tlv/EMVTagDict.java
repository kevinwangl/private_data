import java.util.HashMap;
import java.util.Map;

public class EMVTagDict {
    private static final Map<String, String> TAGS = new HashMap<String, String>();

    static {
        // EMV Book 3 - Data Elements
        TAGS.put("4F", "Application Identifier (AID)");
        TAGS.put("50", "Application Label");
        TAGS.put("57", "Track 2 Equivalent Data");
        TAGS.put("5A", "Application PAN");
        TAGS.put("5F20", "Cardholder Name");
        TAGS.put("5F24", "Application Expiration Date");
        TAGS.put("5F25", "Application Effective Date");
        TAGS.put("5F28", "Issuer Country Code");
        TAGS.put("5F2A", "Transaction Currency Code");
        TAGS.put("5F2D", "Language Preference");
        TAGS.put("5F34", "PAN Sequence Number");
        TAGS.put("61", "Application Template");
        TAGS.put("6F", "FCI Template");
        TAGS.put("70", "EMV Proprietary Template");
        TAGS.put("71", "Issuer Script Template 1");
        TAGS.put("72", "Issuer Script Template 2");
        TAGS.put("73", "Directory Discretionary Template");
        TAGS.put("77", "Response Message Template Format 2");
        TAGS.put("80", "Response Message Template Format 1");
        TAGS.put("82", "Application Interchange Profile");
        TAGS.put("84", "Dedicated File (DF) Name");
        TAGS.put("86", "Issuer Script Command");
        TAGS.put("87", "Application Priority Indicator");
        TAGS.put("88", "Short File Identifier (SFI)");
        TAGS.put("8A", "Authorisation Response Code");
        TAGS.put("8C", "CDOL1");
        TAGS.put("8D", "CDOL2");
        TAGS.put("8E", "CVM List");
        TAGS.put("8F", "CA Public Key Index");
        TAGS.put("90", "Issuer Public Key Certificate");
        TAGS.put("91", "Issuer Authentication Data");
        TAGS.put("92", "Issuer Public Key Remainder");
        TAGS.put("93", "Signed Static Application Data");
        TAGS.put("94", "Application File Locator (AFL)");
        TAGS.put("95", "Terminal Verification Results");
        TAGS.put("9A", "Transaction Date");
        TAGS.put("9B", "Transaction Status Information");
        TAGS.put("9C", "Transaction Type");
        TAGS.put("9F02", "Amount, Authorised");
        TAGS.put("9F03", "Amount, Other");
        TAGS.put("9F06", "AID - Terminal");
        TAGS.put("9F07", "Application Usage Control");
        TAGS.put("9F08", "Application Version Number (Card)");
        TAGS.put("9F09", "Application Version Number (Terminal)");
        TAGS.put("9F0D", "IAC - Default");
        TAGS.put("9F0E", "IAC - Denial");
        TAGS.put("9F0F", "IAC - Online");
        TAGS.put("9F10", "Issuer Application Data");
        TAGS.put("9F11", "Issuer Code Table Index");
        TAGS.put("9F12", "Application Preferred Name");
        TAGS.put("9F13", "Last Online ATC Register");
        TAGS.put("9F14", "Lower Consecutive Offline Limit");
        TAGS.put("9F17", "PIN Try Counter");
        TAGS.put("9F18", "Issuer Script Identifier");
        TAGS.put("9F1A", "Terminal Country Code");
        TAGS.put("9F1E", "IFD Serial Number");
        TAGS.put("9F21", "Transaction Time");
        TAGS.put("9F26", "Application Cryptogram");
        TAGS.put("9F27", "Cryptogram Information Data");
        TAGS.put("9F33", "Terminal Capabilities");
        TAGS.put("9F34", "CVM Results");
        TAGS.put("9F35", "Terminal Type");
        TAGS.put("9F36", "Application Transaction Counter (ATC)");
        TAGS.put("9F37", "Unpredictable Number");
        TAGS.put("9F38", "PDOL");
        TAGS.put("9F39", "POS Entry Mode");
        TAGS.put("9F40", "Additional Terminal Capabilities");
        TAGS.put("9F41", "Transaction Sequence Counter");
        TAGS.put("9F42", "Application Currency Code");
        TAGS.put("9F44", "Application Currency Exponent");
        TAGS.put("9F45", "Data Authentication Code");
        TAGS.put("9F46", "ICC Public Key Certificate");
        TAGS.put("9F47", "ICC Public Key Exponent");
        TAGS.put("9F48", "ICC Public Key Remainder");
        TAGS.put("9F49", "DDOL");
        TAGS.put("9F4A", "Static Data Authentication Tag List");
        TAGS.put("9F4C", "ICC Dynamic Number");
        TAGS.put("9F53", "Transaction Category Code");
        TAGS.put("9F5B", "Issuer Script Results");
        TAGS.put("9F6C", "Mag Stripe Application Version Number");
        TAGS.put("9F6E", "Form Factor Indicator");
        TAGS.put("A5", "FCI Proprietary Template");
        TAGS.put("BF0C", "FCI Issuer Discretionary Data");
        TAGS.put("DF8101", "Card Data Input Capability");
        TAGS.put("DF8102", "CVM Required Limit");
        TAGS.put("DF8103", "No CVM Limit");
        // Issuer Script tags
        TAGS.put("11", "Issuer Code Table Index");
    }

    public static String getName(String tag) {
        String name = TAGS.get(tag.toUpperCase());
        return name != null ? name : "Unknown";
    }

    public static boolean isKnown(String tag) {
        return TAGS.containsKey(tag.toUpperCase());
    }
}
