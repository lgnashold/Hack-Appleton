
public class Time {
	// time in milliseconds since epoch
	// program began January 1 , 2010 in milliseconds
	
	// 1262304000 epoch time for January 1, 2010
	// 1523203223 epoch time for 11:00 am today (April 8)
	public long getTime()
	{
		// returns a random time from January 1, 2010 to now in epoch time
		long now = 1523203223;
		long start = 1262304000;
		long t = (long)(Math.random() * (now - start)) + start;
		return t;
	}
	
}
