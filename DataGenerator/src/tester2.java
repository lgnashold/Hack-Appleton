import java.io.File;
import java.io.PrintWriter;

import org.json.JSONArray;
import org.json.JSONObject;

public class tester2 {

	public static void main(String[] args) {
		// second way of generating 1000 random data points in the correct format, using a JSON library
		String message;
		demographic d = new demographic();
		// demographic object to access its random generator
		Time t = new Time();
		// time object to access its random generator
		JSONObject json = new JSONObject();
		try
		{
			PrintWriter out = new PrintWriter("out.txt");
			out.print("[");
			for(int i = 0; i < 1000; i++)
			{
				json.put("age", d.getAge());
				json.put("gender", d.getGender());
				json.put("continent", d.getContinent());
				json.put("time", t.getTime());
				message = json.toString();
				out.print(message);
				out.println(", ");
				
			}
			out.print("]");
			out.close();
			// printed all the data onto a text file to be used with our website
		}
		catch(Exception e)
		{
			System.out.println("Error");
		}
		
		
	}

}
