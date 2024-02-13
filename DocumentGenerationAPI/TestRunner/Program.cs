using DocumentFormat.OpenXml.Office.Word;
using Document = DocProcessor.Document;
using DocumentType = DocProcessor.DocumentType;
using List = DocProcessor.List;

string path = "/home/ben/projects/word_documents/checkbox.docx";
string savepath = "/home/ben/projects/word_documents/generated.docx";

//Document doc = new Document(path, DocumentType.ExistingDocument);
//doc.EditLegacyCheckbox("Check1", true);
// doc.EditLegacyCheckbox("Check2", true);
// doc.SaveAs(savepath);
// doc.Dispose();


Document doc = new Document(savepath, DocumentType.NewDocument);
List list = new List();

doc.InsertText("Hello, World!");
doc.InsertList(list.test());

doc.Save();
doc.Dispose();


/*

Dictionary<string, string> paths = new();
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

*/