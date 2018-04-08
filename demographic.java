
public class demographic {
	
	// makes random demographics
	public String getContinent()
	{
		// array of all continents
		String[] continents = 
			{"NorthAmerica",
			   "SouthAmerica",
			    "Europe",
			    "Africa",
			    "Asia",
			    "Austrilia",
			    "Antarctica"};
		int index = (int) (Math.random() * (continents.length));
		return continents[index];
       
	}
	
	public String getAge()
	{
		// array of age brackets
		String[] ages = 
			{"UnderThirteen",
			    "ThirteenToEighteen",
			    "EighteenToThirty",
			    "ThirtyToFifty",
			    "FiftyAndOlder"};
		int index = (int) (Math.random() * (ages.length));
		return ages[index];
		
	}
	
	public String getGender()
	{
		int index = (int) (Math.random() * (3));
		String[] genders = {"Male", "Female", "Other"};
		return genders[index];
	}

}
