
public class Purchase {
	// a purchase is an object for a user's demographic and time of purchase

	private String[] demographics; // continent, age, gender
	private long time;
	
	
	public Purchase(String[] demographics, long time)
	{
		this.demographics = demographics;
		this.time = time;
	}
	
	public String[] getDemographics()
	{
		return demographics;
	}
	
	public long getTime()
	{
		return time;
	}
}
