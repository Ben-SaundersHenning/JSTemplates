namespace JSOT;

internal class Assessor {

    public string fName { get; set; }
    public string lName { get; set; }
    public string registrationID { get; set; }
    public string qualifications { get; set; }

    public Assessor(string first, string last, string id, string quals) {

        fName = first;
        lName = last;
        registrationID = id;
        qualifications = quals;

    }

    public override string ToString() {

        return $@"First: {fName}\nLast: {lName}\nID: {registrationID}\nQuals: {qualifications}";

    }

}
