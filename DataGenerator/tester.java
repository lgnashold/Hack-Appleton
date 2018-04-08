import java.io.File;
import java.io.IOException;
import java.io.PrintWriter;
import java.util.*;
public class tester {

	public static void main(String[] args) {
		tester t = new tester();
		// make 50 purchase objects
		Purchase[] ps = new Purchase[50];
		for(int i = 0; i < 50; i++)
		{
			ps[i] = new Purchase(t.makeDemographic(), t.makeTime());
		}
		
		// make a text file to send for testing
		//Dictionary<String, String> data = new HashMap<String, String>();
		File out;
		PrintWriter writer;
		try{
			out = new File("out.txt");
			writer = new PrintWriter(out);
			writer.print("[");
			for(int i = 0; i < 50; i++)
			{
				writer.println("{time:" + ps[i].getTime() + ", continent:" + 
						ps[i].getDemographics()[0] + ", gender:" + 
						ps[i].getDemographics()[2] + ", age:" + 
						ps[i].getDemographics()[1] + "},");
			}
			writer.print("]");
			System.out.println("hello");
			writer.close();
			
		}
		catch(Exception e)
		{
			System.out.println("Error");
		}
		System.out.println("hello");
		
	}
	
	public String[] makeDemographic()
	{
		demographic d = new demographic();
		// in format: continent, age, gender
		String[] out = new String[3];
		out[0] = d.getContinent();
		out[1] = d.getAge();
		out[2] = d.getGender();
		return out;
	}
	
	public long makeTime()
	{
		// milliseconds since epoch
		Time t = new Time();
		// in format: days from January 1, 2010 which is the date this program "began"
		return t.getTime();
	}
}
