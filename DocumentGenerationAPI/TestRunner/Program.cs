using DocProcessor;
using Document = DocProcessor.Document;
using DocumentType = DocProcessor.DocumentType;

Dictionary<string, string> paths = new();
/*
paths.Add("ASSESSOR SALUTATION", "<assessor.title>");
paths.Add("ASSESSOR FIRST", "<assessor.firstName>");
paths.Add("ASSESSOR LAST", "<assessor.lastName>");
paths.Add("ADJUSTER", "<adjuster>");
paths.Add("INSURANCE COMPANY", "<insuranceCompany>");
paths.Add("CLIENT SALUTATION", "<claimant.gender.title>");
paths.Add("CLIENT FIRST", "<claimant.firstName>");
paths.Add("CLIENT LAST", "<claimant.lastName>");
paths.Add("DOB", "<claimant.dateOfBirth>");
paths.Add("CLAIM NUMBER", "<claimNumber>");
paths.Add("DOL", "<claimant.dateOfLoss>");
paths.Add("DOA", "<dateOfAssessment>");
paths.Add("CLIENT AGE", "<claimant.age>");
paths.Add("MALE---FEMALE_Lower", "<claimant.gender.pronouns.p0>");
paths.Add("MALE---FEMALE_LOWER", "<claimant.gender.pronouns.p0>");
paths.Add("HE---SHE_Lower", "<claimant.gender.pronouns.p1_Lower>");
paths.Add("HIM---HER_Lower", "<claimant.gender.pronouns.p3_Lower>");
paths.Add("HE---SHE_Upper", "<claimant.gender.pronouns.p1_Upper>");
paths.Add("HIS---HER_Lower", "<claimant.gender.pronouns.p2_Lower>");
paths.Add("ASSESSOR QUALIFICATIONS", "<assessor.qualifications>");
paths.Add("CLIENT ADDRESS", "<claimant.address.addressLong>");
paths.Add("REFCOMP NAME", "<referralCompany.name>");
paths.Add("PICTURE", "<assessor.signature>");
paths.Add("ASSESSOR REGISTRATIONID", "<assessor.registrationId>");

paths.Add("REFCOMP ADDRESS", "<referralCompany.address.address>");
paths.Add("REFCOMP CITY", "<referralCompany.address.city>");
paths.Add("REFCOMP PHONE", "<referralCompany.phone>");
paths.Add("REFCOMP PROVINCEAB", "<referralCompany.address.provinceAb>");
paths.Add("REFCOMP FAX", "<referralCompany.fax>");
paths.Add("REFCOMP POSTALCODE", "<referralCompany.address.postalCode>");
paths.Add("REFCOMP EMAIL", "<referralCompany.email>");
*/
paths.Add("asmtType", "asmt_type");
paths.Add("insuranceCompany", "insurance_company");
paths.Add("claimNumber", "claim_number");
paths.Add("dateOfAssessment", "date_of_assessment");
paths.Add("seidenFileNumber", "seiden_file_number");
paths.Add("referralCompany", "referral_company");
paths.Add("commonName", "common_name");
paths.Add("provinceAbbreviated", "province_abbreviated");
paths.Add("provinceAb", "province_abbreviated");
paths.Add("provinceAB", "province_abbreviated");
paths.Add("postalCode", "postal_code");
paths.Add("addressLong", "address_long");
paths.Add("registrationID", "registration_id");
paths.Add("registrationId", "registration_id");
paths.Add("firstName", "first_name");
paths.Add("lastName", "last_name");
paths.Add("qualificationsParagraph", "qualifications_paragraph");
paths.Add("dateOfBirth", "date_of_birth");
paths.Add("dateOfLoss", "date_of_loss");
paths.Add("p0Lower", "p0_lower");
paths.Add("p1Lower", "p1_lower");
paths.Add("p2Lower", "p2_lower");
paths.Add("p3Lower", "p3_lower");
paths.Add("p0Upper", "p0_upper");
paths.Add("p1Upper", "p1_upper");
paths.Add("p2Upper", "p2_upper");
paths.Add("p3Upper", "p3_upper");

List<string> templates = new()
{
    "AC",
    "AC MRB",
    "CAT",
    "CAT AC",
    "CAT AC MRB",
    "CAT CAT_GOSE",
    "CAT_GOSE",
    "F1",
    "MRB",
    "NEB"
};

List<string> keys = new List<string>(paths.Keys);

string docPath = "/media/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates_dev/";
string newPath = "/media/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates_new/";

foreach (string template in templates)
{
    string path = docPath + template; 
    string savepath = newPath + template; 
    Document doc = new Document(path + ".docx", DocumentType.ExistingDocument);
    foreach (string key in keys)
    {
        doc.SearchAndReplaceText(key, paths[key]);
    }
    doc.SaveAs(savepath + ".docx");
    doc.Dispose();
}


string GetHelloStr(string match)
{
    if (paths.ContainsKey(match))
    {
        return paths[match];
    }

    return "NULL: " + match;

}