using DocProcessor;
using Document = DocProcessor.Document;
using DocumentType = DocProcessor.DocumentType;

Dictionary<string, string> paths = new();
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

string docPath = "/media/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates_dev/";

foreach (string template in templates)
{
    string path = docPath + template; 
    Document doc = new Document(path + ".docx", DocumentType.ExistingDocument);
    doc.SaveAs(path + "_NEW.docx");
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